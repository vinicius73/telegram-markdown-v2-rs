use markdown::mdast::Node;

use super::Renderer;

pub fn render_children(renderer: &Renderer<'_>, children: &[Node], parent: &Node) -> String {
    renderer.render_children(children, parent)
}
