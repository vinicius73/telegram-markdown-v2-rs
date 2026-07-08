use markdown::mdast::{Code, InlineCode, Text};
use regex::Regex;
use std::sync::OnceLock;

use crate::errors::Result;
use crate::types::TextType;
use crate::utils::escape_symbols;

/// Renders plain text with Telegram MarkdownV2 escaping.
pub fn render_text(node: &Text) -> Result<String> {
    Ok(escape_symbols(&node.value, TextType::Text))
}

/// Renders inline code wrapped in backticks with code-context escaping.
pub fn render_inline_code(node: &InlineCode) -> Result<String> {
    Ok(format!("`{}`", escape_symbols(&node.value, TextType::Code)))
}

const SHEBANG_PATTERN: &str = r"^#![a-z]+\n";

static SHEBANG_RE: OnceLock<Regex> = OnceLock::new();

fn shebang_re() -> &'static Regex {
    SHEBANG_RE.get_or_init(|| Regex::new(SHEBANG_PATTERN).expect("invalid shebang regex"))
}

/// Renders a fenced code block.
///
/// A leading shebang line is stripped to keep output consistent with Telegram
/// code block rendering.
pub fn render_code(node: &Code) -> Result<String> {
    let content = shebang_re().replace(&node.value, "");
    let escaped_content = escape_symbols(content.as_ref(), TextType::Code);
    let opening_fence = match node.lang.as_deref() {
        Some(lang) if !lang.is_empty() => format!("```{lang}"),
        _ => "```".to_owned(),
    };
    Ok(format!("{opening_fence}\n{escaped_content}\n```"))
}
