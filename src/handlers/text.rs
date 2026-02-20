use markdown::mdast::{Code, InlineCode, Text};
use regex::Regex;

use crate::types::TextType;
use crate::utils::escape_symbols;

pub fn render_text(node: &Text) -> String {
    escape_symbols(&node.value, TextType::Text)
}

pub fn render_inline_code(node: &InlineCode) -> String {
    format!("`{}`", escape_symbols(&node.value, TextType::Code))
}

pub fn render_code(node: &Code) -> String {
    let re = Regex::new(r"^#![a-z]+\n").expect("valid regex");
    let content = re.replace(&node.value, "");
    let escaped_content = escape_symbols(content.as_ref(), TextType::Code);
    format!("```\n{escaped_content}\n```")
}
