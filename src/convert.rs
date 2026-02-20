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

const UNDERLINE_PATTERN: &str = r"(?s)<u>(.*?)</u>";
const SPOILER_PATTERN: &str = r#"(?s)<span class="tg-spoiler">(.*?)</span>"#;

static UNDERLINE_RE: OnceLock<Regex> = OnceLock::new();
static SPOILER_RE: OnceLock<Regex> = OnceLock::new();

fn underline_re() -> &'static Regex {
    // Avoid compiling regexes on every conversion; cache them for the process lifetime.
    UNDERLINE_RE.get_or_init(|| Regex::new(UNDERLINE_PATTERN).expect("invalid underline regex"))
}

fn spoiler_re() -> &'static Regex {
    // Same rationale as `underline_re()`.
    SPOILER_RE.get_or_init(|| Regex::new(SPOILER_PATTERN).expect("invalid spoiler regex"))
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
    let underline = underline_re();
    let spoiler = spoiler_re();

    Ok(transform_outside_code(text, |chunk| {
        let with_underlines = underline.replace_all(chunk, format!("{U_START}${{1}}{U_END}"));
        spoiler
            .replace_all(with_underlines.as_ref(), format!("{S_START}${{1}}{S_END}"))
            .to_string()
    }))
}

fn postprocess_v2_formatting(text: &str) -> String {
    transform_outside_code(text, |chunk| {
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
/// blockquotes, tables, and raw HTML blocks). When such nodes appear in the input, they
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
/// - `<span class="tg-spoiler">…</span>` → `||…||` (spoiler)
///
/// # Examples
///
/// ```rust
/// # fn main() -> telegram_markdown_v2::Result<()> {
/// use telegram_markdown_v2::{UnsupportedTagsStrategy, convert_with_strategy};
///
/// // Blockquotes are not supported by Telegram MarkdownV2; escape them as plain text.
/// let out = convert_with_strategy("> test", UnsupportedTagsStrategy::Escape)?;
/// assert_eq!(out, "\\> test\n");
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
