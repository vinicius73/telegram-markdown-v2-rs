use std::collections::HashMap;

use markdown::mdast::Node;

use crate::types::{Definition, UnsupportedTagsStrategy};

pub mod blocks;
pub mod formatting;
pub mod links;
pub mod text;
pub mod utils;

pub struct RenderContext<'a> {
    pub definitions: &'a HashMap<String, Definition>,
    pub strategy: UnsupportedTagsStrategy,
}

pub struct Renderer<'a> {
    ctx: &'a RenderContext<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(ctx: &'a RenderContext<'a>) -> Self {
        Self { ctx }
    }

    pub fn context(&self) -> &'a RenderContext<'a> {
        self.ctx
    }

    pub fn render_root(&self, node: &Node) -> String {
        if let Node::Root(root) = node {
            let chunks: Vec<String> = root
                .children
                .iter()
                .enumerate()
                .map(|(idx, child)| self.render_node(child, Some(node), Some(&root.children), idx))
                .filter(|value| !value.is_empty())
                .collect();

            if chunks.is_empty() {
                String::new()
            } else {
                let combined = chunks.join("\n\n");
                if combined.ends_with('\n') {
                    combined
                } else {
                    format!("{combined}\n")
                }
            }
        } else {
            self.render_node(node, None, None, 0)
        }
    }

    pub fn render_children(&self, children: &[Node], parent: &Node) -> String {
        children
            .iter()
            .enumerate()
            .map(|(idx, child)| self.render_node(child, Some(parent), Some(children), idx))
            .collect::<String>()
    }

    pub fn render_node(
        &self,
        node: &Node,
        parent: Option<&Node>,
        siblings: Option<&[Node]>,
        idx: usize,
    ) -> String {
        match node {
            Node::Heading(n) => formatting::render_heading(self, n, node),
            Node::Strong(n) => formatting::render_strong(self, n, node),
            Node::Delete(n) => formatting::render_delete(self, n, node),
            Node::Emphasis(n) => formatting::render_emphasis(self, n, node),
            Node::List(n) => blocks::render_list(self, n, parent, siblings, idx),
            Node::ListItem(n) => blocks::render_list_item(self, n, parent),
            Node::InlineCode(n) => text::render_inline_code(n),
            Node::Code(n) => text::render_code(n),
            Node::Link(n) => links::render_link(self, n, node),
            Node::LinkReference(n) => links::render_link_reference(self, n, node),
            Node::Image(n) => links::render_image(n),
            Node::ImageReference(n) => links::render_image_reference(self, n),
            Node::Text(n) => text::render_text(n),
            Node::Blockquote(n) => blocks::render_blockquote(self, n, node),
            Node::Html(n) => blocks::render_html(self, n),
            Node::Table(n) => blocks::render_table(self, n),
            Node::Paragraph(n) => self.render_children(&n.children, node),
            Node::Root(_) => self.render_root(node),
            Node::Definition(_) => String::new(),
            _ => String::new(),
        }
    }
}
