use markdown::mdast::{Delete, Emphasis, Heading, Node, Strong};

use crate::errors::Result;
use crate::utils::wrap;

use super::{utils::render_children, Renderer};

pub fn render_heading(renderer: &Renderer<'_>, node: &Heading, parent_node: &Node) -> Result<String> {
    Ok(wrap(
        &render_children(renderer, &node.children, parent_node)?,
        "*",
    ))
}

pub fn render_strong(renderer: &Renderer<'_>, node: &Strong, parent_node: &Node) -> Result<String> {
    Ok(wrap(
        &render_children(renderer, &node.children, parent_node)?,
        "*",
    ))
}

pub fn render_delete(renderer: &Renderer<'_>, node: &Delete, parent_node: &Node) -> Result<String> {
    Ok(wrap(
        &render_children(renderer, &node.children, parent_node)?,
        "~",
    ))
}

pub fn render_emphasis(
    renderer: &Renderer<'_>,
    node: &Emphasis,
    parent_node: &Node,
) -> Result<String> {
    Ok(wrap(
        &render_children(renderer, &node.children, parent_node)?,
        "_",
    ))
}
