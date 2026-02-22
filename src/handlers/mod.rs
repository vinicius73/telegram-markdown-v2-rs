use std::collections::HashMap;

use markdown::mdast::Node;

use crate::errors::Result;
use crate::types::{Definition, UnsupportedTagsStrategy};

pub mod blocks;
pub mod formatting;
pub mod links;
pub mod text;
pub mod utils;

/// Immutable data required while rendering Markdown AST nodes.
pub struct RenderContext<'a> {
    /// Link reference definitions collected from the source document.
    pub definitions: &'a HashMap<String, Definition>,
    /// Strategy for nodes unsupported by Telegram MarkdownV2.
    pub strategy: UnsupportedTagsStrategy,
}

/// Node renderer that dispatches each AST node to its Telegram renderer.
pub struct Renderer<'a> {
    ctx: &'a RenderContext<'a>,
}

impl<'a> Renderer<'a> {
    /// Creates a renderer bound to a given rendering context.
    pub fn new(ctx: &'a RenderContext<'a>) -> Self {
        Self { ctx }
    }

    /// Returns the rendering context currently used by this renderer.
    pub fn context(&self) -> &'a RenderContext<'a> {
        self.ctx
    }

    /// Renders a root node and joins top-level blocks with blank lines.
    ///
    /// For non-root nodes, this delegates to [`Self::render_node`].
    pub fn render_root(&self, node: &Node) -> Result<String> {
        if let Node::Root(root) = node {
            let mut chunks: Vec<String> = Vec::new();
            for (idx, child) in root.children.iter().enumerate() {
                let rendered = self.render_node(child, Some(node), Some(&root.children), idx)?;
                if !rendered.is_empty() {
                    chunks.push(rendered);
                }
            }

            if chunks.is_empty() {
                Ok(String::new())
            } else {
                let combined = chunks.join("\n\n");
                Ok(if combined.ends_with('\n') {
                    combined
                } else {
                    format!("{combined}\n")
                })
            }
        } else {
            self.render_node(node, None, None, 0)
        }
    }

    /// Renders all children in order without inserting separators.
    pub fn render_children(&self, children: &[Node], parent: &Node) -> Result<String> {
        let mut out = String::new();
        for (idx, child) in children.iter().enumerate() {
            out.push_str(&self.render_node(child, Some(parent), Some(children), idx)?);
        }
        Ok(out)
    }

    /// Renders a single AST node into Telegram MarkdownV2.
    ///
    /// Unknown or intentionally ignored node variants render as an empty string.
    pub fn render_node(
        &self,
        node: &Node,
        parent: Option<&Node>,
        siblings: Option<&[Node]>,
        idx: usize,
    ) -> Result<String> {
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
            Node::Definition(_) => Ok(String::new()),
            _ => Ok(String::new()),
        }
    }
}
