use std::collections::HashMap;

use markdown::mdast::Node;

use crate::types::Definition;

pub fn collect_definitions(root: &Node) -> HashMap<String, Definition> {
    let mut definitions = HashMap::new();
    collect_recursive(root, &mut definitions);
    definitions
}

fn collect_recursive(node: &Node, definitions: &mut HashMap<String, Definition>) {
    if let Node::Definition(definition) = node {
        definitions.insert(
            definition.identifier.clone(),
            Definition {
                title: definition.title.clone(),
                url: definition.url.clone(),
            },
        );
    }

    for child in child_nodes(node) {
        collect_recursive(child, definitions);
    }
}

fn child_nodes(node: &Node) -> &[Node] {
    match node {
        Node::Root(n) => &n.children,
        Node::Blockquote(n) => &n.children,
        Node::List(n) => &n.children,
        Node::ListItem(n) => &n.children,
        Node::Paragraph(n) => &n.children,
        Node::Heading(n) => &n.children,
        Node::Strong(n) => &n.children,
        Node::Emphasis(n) => &n.children,
        Node::Delete(n) => &n.children,
        Node::Link(n) => &n.children,
        Node::LinkReference(n) => &n.children,
        Node::Table(n) => &n.children,
        Node::TableRow(n) => &n.children,
        Node::TableCell(n) => &n.children,
        _ => &[],
    }
}
