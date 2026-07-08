use markdown::{ParseOptions, to_mdast};
use regex::Regex;
use std::sync::OnceLock;

use crate::definitions::collect_definitions;
use crate::errors::{Error, Result};
use crate::handlers::{RenderContext, Renderer};
use crate::types::UnsupportedTagsStrategy;

const U_START: &str = "TGMDV2USTART";
const U_END: &str = "TGMDV2UEND";
const S_START: &str = "TGMDV2SSTART";
const S_END: &str = "TGMDV2SEND";
const EXPAND_FIRST: &str = "TGMDV2EXPFIRST ";
const EXPAND_END: &str = "TGMDV2EXPENDMARK";

const UNDERLINE_PATTERN: &str = r"(?s)<(?:u|ins)>(.*?)</(?:u|ins)>";
const SPOILER_PATTERN: &str =
    r#"(?s)<(?:span class="tg-spoiler"|tg-spoiler)>(.*?)</(?:span|tg-spoiler)>"#;
const TG_EMOJI_PATTERN: &str = r#"(?s)<tg-emoji emoji-id="([^"]+)">(.*?)</tg-emoji>"#;
const TG_TIME_PATTERN: &str =
    r#"(?s)<tg-time unix="([^"]+)"(?: format="([^"]*)")?>(.*?)</tg-time>"#;
const EXPANDABLE_BLOCKQUOTE_PATTERN: &str = r"(?s)<blockquote\s+expandable>(.*?)</blockquote>";

static UNDERLINE_RE: OnceLock<Regex> = OnceLock::new();
static SPOILER_RE: OnceLock<Regex> = OnceLock::new();
static EXPANDABLE_BLOCKQUOTE_RE: OnceLock<Regex> = OnceLock::new();
static TG_EMOJI_RE: OnceLock<Regex> = OnceLock::new();
static TG_TIME_RE: OnceLock<Regex> = OnceLock::new();

fn underline_re() -> &'static Regex {
    // Avoid compiling regexes on every conversion; cache them for the process lifetime.
    UNDERLINE_RE.get_or_init(|| Regex::new(UNDERLINE_PATTERN).expect("invalid underline regex"))
}

fn spoiler_re() -> &'static Regex {
    // Same rationale as `underline_re()`.
    SPOILER_RE.get_or_init(|| Regex::new(SPOILER_PATTERN).expect("invalid spoiler regex"))
}

fn expandable_blockquote_re() -> &'static Regex {
    EXPANDABLE_BLOCKQUOTE_RE.get_or_init(|| {
        Regex::new(EXPANDABLE_BLOCKQUOTE_PATTERN).expect("invalid expandable blockquote regex")
    })
}

fn tg_emoji_re() -> &'static Regex {
    TG_EMOJI_RE.get_or_init(|| Regex::new(TG_EMOJI_PATTERN).expect("invalid tg-emoji regex"))
}

fn tg_time_re() -> &'static Regex {
    TG_TIME_RE.get_or_init(|| Regex::new(TG_TIME_PATTERN).expect("invalid tg-time regex"))
}

fn ends_with_blockquote_line(text: &str) -> bool {
    text.trim_end().lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with('>') && trimmed.len() > 1
    })
}

fn ensure_blank_line_before_expandable(out: &mut String, before: &str) {
    if before.trim_end().is_empty() {
        return;
    }
    if before.ends_with("\n\n") {
        return;
    }
    if before.ends_with('\n') {
        out.push('\n');
    } else {
        out.push_str("\n\n");
    }
}

fn expandable_blockquote_to_markdown(content: &str, after_blockquote: bool) -> String {
    let lines: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    if lines.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        let is_first = i == 0;
        let is_last = i + 1 == lines.len();
        let first_prefix = if is_first && after_blockquote {
            EXPAND_FIRST
        } else {
            ""
        };
        let end_marker = if is_last { EXPAND_END } else { "" };

        out.push_str(&format!("> {first_prefix}{line}{end_marker}\n"));
    }
    out
}

fn preprocess_expandable_blockquotes(text: &str) -> Result<String> {
    let re = expandable_blockquote_re();
    let mut out = String::with_capacity(text.len());
    let mut last = 0;

    for cap in re.captures_iter(text) {
        let m = cap.get(0).expect("expandable blockquote match should exist");
        let before = &text[last..m.start()];
        out.push_str(before);

        let after_blockquote = ends_with_blockquote_line(before);
        if after_blockquote {
            ensure_blank_line_before_expandable(&mut out, before);
        }

        let content = cap.get(1).map(|matched| matched.as_str()).unwrap_or("");
        out.push_str(&expandable_blockquote_to_markdown(content, after_blockquote));
        last = m.end();
    }

    out.push_str(&text[last..]);
    Ok(out)
}

#[derive(Clone, Copy)]
struct Fence {
    marker: u8,
    len: usize,
}

fn fence_marker(line: &str) -> Option<Fence> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i] == b' ' && i < 3 {
        i += 1;
    }

    let marker = *bytes.get(i)?;
    if marker != b'`' && marker != b'~' {
        return None;
    }

    let mut len = 0;
    while i + len < bytes.len() && bytes[i + len] == marker {
        len += 1;
    }

    (len >= 3).then_some(Fence { marker, len })
}

fn transform_outside_inline_code<F>(input: &str, transform: &mut F) -> String
where
    F: FnMut(&str) -> String,
{
    if !input.as_bytes().contains(&b'`') {
        return transform(input);
    }

    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let mut plain_start = 0;

    while i < bytes.len() {
        if bytes[i] != b'`' {
            i += 1;
            continue;
        }

        // Inline code can be delimited by N backticks and must close with the same N.
        // We keep code spans verbatim so preprocessing doesn't mutate literal code.
        let mut tick_len = 1;
        while i + tick_len < bytes.len() && bytes[i + tick_len] == b'`' {
            tick_len += 1;
        }

        let mut j = i + tick_len;
        let mut close_start: Option<usize> = None;
        while j < bytes.len() {
            if bytes[j] != b'`' {
                j += 1;
                continue;
            }

            let mut run_len = 1;
            while j + run_len < bytes.len() && bytes[j + run_len] == b'`' {
                run_len += 1;
            }

            if run_len == tick_len {
                close_start = Some(j);
                break;
            }

            j += run_len;
        }

        let Some(close_start) = close_start else {
            // Unclosed delimiter: treat it as plain text.
            i += tick_len;
            continue;
        };

        if plain_start < i {
            out.push_str(&transform(&input[plain_start..i]));
        }

        let end = close_start + tick_len;
        out.push_str(&input[i..end]);
        i = end;
        plain_start = i;
    }

    if plain_start < input.len() {
        out.push_str(&transform(&input[plain_start..]));
    }

    out
}

fn transform_outside_code<F>(input: &str, mut transform: F) -> String
where
    F: FnMut(&str) -> String,
{
    let mut out = String::with_capacity(input.len());
    let mut fence: Option<Fence> = None;

    for line in input.split_inclusive('\n') {
        if let Some(marker) = fence_marker(line) {
            match fence {
                None => {
                    // Enter fenced code block (``` / ~~~). Contents must remain literal.
                    fence = Some(marker);
                    out.push_str(line);
                    continue;
                }
                Some(open) if open.marker == marker.marker && marker.len >= open.len => {
                    // Close fence: same marker and at least the opening fence length.
                    fence = None;
                    out.push_str(line);
                    continue;
                }
                Some(_) => {}
            }
        }

        if fence.is_some() {
            out.push_str(line);
        } else {
            out.push_str(&transform_outside_inline_code(line, &mut transform));
        }
    }

    out
}

fn preprocess_v2_html_tags(text: &str) -> Result<String> {
    let with_expandable = preprocess_expandable_blockquotes(text)?;
    let underline = underline_re();
    let spoiler = spoiler_re();
    let tg_emoji = tg_emoji_re();
    let tg_time = tg_time_re();

    Ok(transform_outside_code(&with_expandable, |chunk| {
        let with_emojis = tg_emoji.replace_all(chunk, "![$2](tg://emoji?id=$1)");
        let with_times = tg_time.replace_all(with_emojis.as_ref(), |caps: &regex::Captures| {
            let unix = &caps[1];
            let text = &caps[3];
            match caps.get(2).map(|matched| matched.as_str()).filter(|value| !value.is_empty()) {
                Some(format) => format!("![{text}](tg://time?unix={unix}&format={format})"),
                None => format!("![{text}](tg://time?unix={unix})"),
            }
        });
        let with_underlines =
            underline.replace_all(with_times.as_ref(), format!("{U_START}${{1}}{U_END}"));
        spoiler
            .replace_all(
                with_underlines.as_ref(),
                format!("{S_START}${{1}}{S_END}"),
            )
            .to_string()
    }))
}

fn postprocess_v2_formatting(text: &str) -> String {
    let with_expandable = text
        .replace(&format!("> {EXPAND_FIRST}"), "**>")
        .replace(EXPAND_END, "||")
        .replace("\n\n**>", "\n**>");
    transform_outside_code(&with_expandable, |chunk| {
        let with_underlines = chunk.replace(U_START, "__").replace(U_END, "__");
        with_underlines.replace(S_START, "||").replace(S_END, "||")
    })
}

/// Converts a Markdown document into Telegram **MarkdownV2**.
///
/// This function renders the supported Markdown constructs and escapes Telegram MarkdownV2
/// reserved characters in plain text, so the output can be sent using Telegram's
/// `parse_mode = MarkdownV2`.
///
/// It is equivalent to calling [`convert_with_strategy`] with
/// [`UnsupportedTagsStrategy::Keep`].
///
/// The returned string ends with a trailing newline (`\n`) when the rendered output is
/// non-empty.
///
/// # Examples
///
/// ```rust
/// # fn main() -> telegram_markdown_v2::Result<()> {
/// use telegram_markdown_v2::convert;
///
/// let out = convert("Hello world!")?;
/// assert_eq!(out, "Hello world\\!\n");
/// # Ok(())
/// # }
/// ```
pub fn convert(markdown: &str) -> Result<String> {
    convert_with_strategy(markdown, UnsupportedTagsStrategy::Keep)
}

/// Converts a Markdown document into Telegram **MarkdownV2**, controlling how unsupported
/// constructs are handled.
///
/// Telegram MarkdownV2 does not support some Markdown/HTML constructs (for example:
/// tables and raw HTML blocks). When such nodes appear in the input, they
/// are handled according to `strategy`:
///
/// - [`UnsupportedTagsStrategy::Keep`]: keep the unsupported content as-is.
/// - [`UnsupportedTagsStrategy::Escape`]: escape the unsupported content as plain text.
/// - [`UnsupportedTagsStrategy::Remove`]: drop the unsupported content entirely.
///
/// Independently of `strategy`, this converter recognizes the following HTML patterns
/// outside inline/fenced code and turns them into Telegram MarkdownV2 markers:
///
/// - `<u>…</u>` → `__…__` (underline)
/// - `<ins>…</ins>` → `__…__` (underline)
/// - `<span class="tg-spoiler">…</span>` / `<tg-spoiler>…</tg-spoiler>` → `||…||` (spoiler)
/// - `<tg-emoji emoji-id="…">…</tg-emoji>` → `![…](tg://emoji?id=…)` (custom emoji)
/// - `<tg-time unix="…" format="…">…</tg-time>` → `![…](tg://time?unix=…&format=…)` (date/time)
/// - `<blockquote expandable>…</blockquote>` → expandable blockquote (`> …||`)
///
/// # Examples
///
/// ```rust
/// # fn main() -> telegram_markdown_v2::Result<()> {
/// use telegram_markdown_v2::{UnsupportedTagsStrategy, convert_with_strategy};
///
/// let out = convert_with_strategy("<div>test</div>", UnsupportedTagsStrategy::Escape)?;
/// assert_eq!(out, "<div\\>test</div\\>\n");
/// # Ok(())
/// # }
/// ```
pub fn convert_with_strategy(markdown: &str, strategy: UnsupportedTagsStrategy) -> Result<String> {
    let processed_markdown = preprocess_v2_html_tags(markdown)?;
    let tree = to_mdast(&processed_markdown, &ParseOptions::gfm())
        .map_err(|message| Error::MarkdownParse { message })?;

    let definitions = collect_definitions(&tree);
    let context = RenderContext {
        definitions: &definitions,
        strategy,
    };

    let renderer = Renderer::new(&context);
    let result = renderer.render_root(&tree)?;
    // Strip parser placeholders, but never inside code (inline/fenced).
    let cleaned = transform_outside_code(&result, |chunk| chunk.replace("<!---->\n", ""));
    Ok(postprocess_v2_formatting(&cleaned))
}
