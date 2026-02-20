use markdown::mdast::{Image, ImageReference, Link, LinkReference, Node};

use crate::errors::Result;
use crate::types::TextType;
use crate::utils::{escape_symbols, is_url};

use super::{Renderer, utils::render_children};

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

    if text.is_empty() {
        Ok(format!(
            "[{}]({})",
            escape_symbols(url, TextType::Text),
            escape_symbols(url, TextType::Link)
        ))
    } else {
        Ok(format!(
            "[{}]({})",
            escape_symbols(&text, TextType::Text),
            escape_symbols(url, TextType::Link)
        ))
    }
}

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

    if text.is_empty() {
        Ok(format!(
            "[{}]({})",
            escape_symbols(&definition.url, TextType::Text),
            escape_symbols(&definition.url, TextType::Link)
        ))
    } else {
        Ok(format!(
            "[{}]({})",
            escape_symbols(&text, TextType::Text),
            escape_symbols(&definition.url, TextType::Link)
        ))
    }
}
