use markdown::mdast::{Code, InlineCode, Text};
use regex::Regex;

use crate::errors::{Error, Result};
use crate::types::TextType;
use crate::utils::escape_symbols;

pub fn render_text(node: &Text) -> Result<String> {
    Ok(escape_symbols(&node.value, TextType::Text))
}

pub fn render_inline_code(node: &InlineCode) -> Result<String> {
    Ok(format!("`{}`", escape_symbols(&node.value, TextType::Code)))
}

const SHEBANG_PATTERN: &str = r"^#![a-z]+\n";

pub fn render_code(node: &Code) -> Result<String> {
    let re = Regex::new(SHEBANG_PATTERN).map_err(|source| Error::RegexCompile {
        name: "code_block_shebang",
        pattern: SHEBANG_PATTERN,
        source,
    })?;

    let content = re.replace(&node.value, "");
    let escaped_content = escape_symbols(content.as_ref(), TextType::Code);
    Ok(format!("```\n{escaped_content}\n```"))
}
