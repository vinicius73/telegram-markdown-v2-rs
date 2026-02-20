use markdown::mdast::{Image, ImageReference, Link, LinkReference, Node};

use crate::types::TextType;
use crate::utils::{escape_symbols, is_url};

use super::{utils::render_children, Renderer};

pub fn render_link(renderer: &Renderer<'_>, node: &Link, parent_node: &Node) -> String {
    let text = {
        let rendered = render_children(renderer, &node.children, parent_node);
        if rendered.is_empty() {
            node.title
                .as_ref()
                .map(|value| escape_symbols(value, TextType::Text))
                .unwrap_or_default()
        } else {
            rendered
        }
    };

    let url = node.url.as_str();
    if !is_url(url) {
        if text.is_empty() {
            return escape_symbols(url, TextType::Text);
        }
        return text;
    }

    if text.is_empty() {
        format!(
            "[{}]({})",
            escape_symbols(url, TextType::Text),
            escape_symbols(url, TextType::Link)
        )
    } else {
        format!("[{text}]({})", escape_symbols(url, TextType::Link))
    }
}

pub fn render_link_reference(
    renderer: &Renderer<'_>,
    node: &LinkReference,
    parent_node: &Node,
) -> String {
    let definition = renderer.context().definitions.get(&node.identifier);
    let text = {
        let rendered = render_children(renderer, &node.children, parent_node);
        if rendered.is_empty() {
            definition
                .and_then(|def| def.title.as_ref())
                .map(|value| escape_symbols(value, TextType::Text))
                .unwrap_or_default()
        } else {
            rendered
        }
    };

    let Some(definition) = definition else {
        return text;
    };

    if !is_url(&definition.url) {
        return text;
    }

    if text.is_empty() {
        format!(
            "[{}]({})",
            escape_symbols(&definition.url, TextType::Text),
            escape_symbols(&definition.url, TextType::Link)
        )
    } else {
        format!(
            "[{text}]({})",
            escape_symbols(&definition.url, TextType::Link)
        )
    }
}

pub fn render_image(node: &Image) -> String {
    let text = if node.alt.is_empty() {
        node.title.clone().unwrap_or_default()
    } else {
        node.alt.clone()
    };
    let url = node.url.as_str();

    if !is_url(url) {
        if text.is_empty() {
            return escape_symbols(url, TextType::Text);
        }
        return escape_symbols(&text, TextType::Text);
    }

    if text.is_empty() {
        format!(
            "[{}]({})",
            escape_symbols(url, TextType::Text),
            escape_symbols(url, TextType::Link)
        )
    } else {
        format!(
            "[{}]({})",
            escape_symbols(&text, TextType::Text),
            escape_symbols(url, TextType::Link)
        )
    }
}

pub fn render_image_reference(renderer: &Renderer<'_>, node: &ImageReference) -> String {
    let definition = renderer.context().definitions.get(&node.identifier);
    let text = if node.alt.is_empty() {
        definition
            .and_then(|def| def.title.clone())
            .unwrap_or_default()
    } else {
        node.alt.clone()
    };

    let Some(definition) = definition else {
        return escape_symbols(&text, TextType::Text);
    };

    if !is_url(&definition.url) {
        return escape_symbols(&text, TextType::Text);
    }

    if text.is_empty() {
        format!(
            "[{}]({})",
            escape_symbols(&definition.url, TextType::Text),
            escape_symbols(&definition.url, TextType::Link)
        )
    } else {
        format!(
            "[{}]({})",
            escape_symbols(&text, TextType::Text),
            escape_symbols(&definition.url, TextType::Link)
        )
    }
}
