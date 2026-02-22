use markdown::mdast::Node;

use super::Renderer;
use crate::errors::Result;

/// Small forwarding helper used by handler modules for child rendering.
pub fn render_children(
    renderer: &Renderer<'_>,
    children: &[Node],
    parent: &Node,
) -> Result<String> {
    renderer.render_children(children, parent)
}
