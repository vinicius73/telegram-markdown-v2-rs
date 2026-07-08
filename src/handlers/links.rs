use markdown::mdast::{Image, ImageReference, Link, LinkReference, Node};

use crate::errors::Result;
use crate::types::TextType;
use crate::utils::{escape_symbols, is_url};

use super::{Renderer, utils::render_children};

/// Returns true when the image URL maps to a Telegram custom emoji or date/time entity.
fn is_telegram_special_image(url: &str) -> bool {
    url.starts_with("tg://emoji") || url.starts_with("tg://time")
}

fn format_image_markdown(text: &str, url: &str) -> String {
    let prefix = if is_telegram_special_image(url) {
        "!"
    } else {
        ""
    };
    let escaped_url = escape_symbols(url, TextType::Link);

    if text.is_empty() {
        format!(
            "{prefix}[{}]({escaped_url})",
            escape_symbols(url, TextType::Text)
        )
    } else {
        format!(
            "{prefix}[{}]({escaped_url})",
            escape_symbols(text, TextType::Text)
        )
    }
}

/// Renders an inline link.
///
/// If the URL is invalid, this falls back to escaped text content.
pub fn render_link(renderer: &Renderer<'_>, node: &Link, parent_node: &Node) -> Result<String> {
    let text = {
        let rendered = render_children(renderer, &node.children, parent_node)?;
        if rendered.is_empty() {
            match node.title.as_ref() {
                Some(value) => escape_symbols(value, TextType::Text),
                None => String::new(),
            }
        } else {
            rendered
        }
    };

    let url = node.url.as_str();
    if !is_url(url) {
        if text.is_empty() {
            return Ok(escape_symbols(url, TextType::Text));
        }
        return Ok(text);
    }

    if text.is_empty() {
        Ok(format!(
            "[{}]({})",
            escape_symbols(url, TextType::Text),
            escape_symbols(url, TextType::Link)
        ))
    } else {
        Ok(format!("[{text}]({})", escape_symbols(url, TextType::Link)))
    }
}

/// Renders a reference-style link using a previously collected definition.
///
/// If the definition is missing or invalid, this falls back to plain text.
pub fn render_link_reference(
    renderer: &Renderer<'_>,
    node: &LinkReference,
    parent_node: &Node,
) -> Result<String> {
    let definition = renderer.context().definitions.get(&node.identifier);
    let text = {
        let rendered = render_children(renderer, &node.children, parent_node)?;
        if rendered.is_empty() {
            match definition.and_then(|def| def.title.as_ref()) {
                Some(value) => escape_symbols(value, TextType::Text),
                None => String::new(),
            }
        } else {
            rendered
        }
    };

    let Some(definition) = definition else {
        return Ok(text);
    };

    if !is_url(&definition.url) {
        return Ok(text);
    }

    if text.is_empty() {
        Ok(format!(
            "[{}]({})",
            escape_symbols(&definition.url, TextType::Text),
            escape_symbols(&definition.url, TextType::Link)
        ))
    } else {
        Ok(format!(
            "[{text}]({})",
            escape_symbols(&definition.url, TextType::Link)
        ))
    }
}

/// Renders an image as a Telegram link using `alt` or `title` as visible text.
///
/// If the URL is invalid, this falls back to escaped text.
pub fn render_image(node: &Image) -> Result<String> {
    let text = if node.alt.is_empty() {
        match node.title.as_ref() {
            Some(title) => title.clone(),
            None => String::new(),
        }
    } else {
        node.alt.clone()
    };
    let url = node.url.as_str();

    if !is_url(url) {
        if text.is_empty() {
            return Ok(escape_symbols(url, TextType::Text));
        }
        return Ok(escape_symbols(&text, TextType::Text));
    }

    Ok(format_image_markdown(&text, url))
}

/// Renders a reference-style image using definition URL and optional title.
///
/// If the definition is missing or invalid, this falls back to escaped text.
pub fn render_image_reference(renderer: &Renderer<'_>, node: &ImageReference) -> Result<String> {
    let definition = renderer.context().definitions.get(&node.identifier);
    let text = if node.alt.is_empty() {
        match definition.and_then(|def| def.title.as_ref()) {
            Some(title) => title.clone(),
            None => String::new(),
        }
    } else {
        node.alt.clone()
    };

    let Some(definition) = definition else {
        return Ok(escape_symbols(&text, TextType::Text));
    };

    if !is_url(&definition.url) {
        return Ok(escape_symbols(&text, TextType::Text));
    }

    Ok(format_image_markdown(&text, &definition.url))
}
