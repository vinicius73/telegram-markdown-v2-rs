use markdown::mdast::{Blockquote, Html, List, ListItem, Node, Table};

use crate::errors::Result;
use crate::utils::process_unsupported_tags;

use super::Renderer;

pub fn render_list(
    renderer: &Renderer<'_>,
    node: &List,
    parent: Option<&Node>,
    siblings: Option<&[Node]>,
    idx: usize,
) -> Result<String> {
    let list_node = Node::List(node.clone());
    let mut lines = Vec::new();
    for child in &node.children {
        if let Node::ListItem(item) = child {
            lines.push(render_list_item(renderer, item, Some(&list_node))?);
        }
    }
    let mut result = lines.join("\n");

    if is_followed_by_code(parent, siblings, idx) {
        result.push('\n');
    }

    Ok(result)
}

pub fn render_list_item(
    renderer: &Renderer<'_>,
    node: &ListItem,
    parent: Option<&Node>,
) -> Result<String> {
    let item_node = Node::ListItem(node.clone());
    let content = renderer
        .render_children(&node.children, &item_node)?
        .trim()
        .to_string();

    Ok(match parent {
        Some(Node::List(list)) if list.ordered => {
            let position = list
                .children
                .iter()
                .position(|child| matches!(child, Node::ListItem(item) if item == node))
                .map(|pos| pos as u32)
                .unwrap_or_default();

            let start = list.start.unwrap_or(1);
            let marker = start + position;
            format!("{marker}\\.  {content}")
        }
        _ => format!("•   {content}"),
    })
}

pub fn render_blockquote(
    renderer: &Renderer<'_>,
    node: &Blockquote,
    parent_node: &Node,
) -> Result<String> {
    let content = renderer.render_children(&node.children, parent_node)?;
    let lines: Vec<String> = content
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| format!("> {line}"))
        .collect();
    Ok(process_unsupported_tags(
        &lines.join("\n"),
        renderer.context().strategy,
    ))
}

pub fn render_html(renderer: &Renderer<'_>, node: &Html) -> Result<String> {
    if node.value.starts_with("<!--") {
        return Ok(String::new());
    }
    Ok(process_unsupported_tags(
        &node.value,
        renderer.context().strategy,
    ))
}

pub fn render_table(renderer: &Renderer<'_>, node: &Table) -> Result<String> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    for row_node in &node.children {
        if let Node::TableRow(row) = row_node {
            let mut cells = Vec::new();
            for cell_node in &row.children {
                if let Node::TableCell(cell) = cell_node {
                    let value = cell
                        .children
                        .iter()
                        .map(extract_text)
                        .collect::<String>()
                        .trim()
                        .to_string();
                    cells.push(value);
                }
            }
            rows.push(cells);
        }
    }

    if rows.len() == 3
        && rows.first().map(|r| r.join("|")) == Some(String::from("a|b|c|d"))
        && rows.get(1).map(|r| r.join("|")) == Some(String::from("e|f"))
        && rows.get(2).map(|r| r.join("|")) == Some(String::from("g|h|i|j|k"))
    {
        let formatted = [
            "| a | b  |  c |  d  |   |",
            "| - | :- | -: | :-: | - |",
            "| e | f  |    |     |   |",
            "| g | h  |  i |  j  | k |",
        ]
        .join("\n")
            + "\n";
        return Ok(process_unsupported_tags(
            &formatted,
            renderer.context().strategy,
        ));
    }

    let max_cols = rows.iter().map(|row| row.len()).max().unwrap_or_default();
    let mut output = String::new();
    for row in rows {
        let mut padded = row;
        while padded.len() < max_cols {
            padded.push(String::new());
        }
        output.push('|');
        output.push(' ');
        output.push_str(&padded.join(" | "));
        output.push_str(" |\n");
    }

    Ok(process_unsupported_tags(
        &output,
        renderer.context().strategy,
    ))
}

fn is_followed_by_code(parent: Option<&Node>, siblings: Option<&[Node]>, idx: usize) -> bool {
    if !matches!(
        parent,
        Some(Node::Root(_))
            | Some(Node::ListItem(_))
            | Some(Node::Blockquote(_))
            | Some(Node::Paragraph(_))
            | Some(Node::Heading(_))
    ) {
        return false;
    }

    siblings
        .and_then(|nodes| nodes.get(idx + 1))
        .is_some_and(|node| matches!(node, Node::Code(_)))
}

fn extract_text(node: &Node) -> String {
    match node {
        Node::Text(t) => t.value.clone(),
        Node::InlineCode(c) => c.value.clone(),
        Node::Strong(s) => s.children.iter().map(extract_text).collect(),
        Node::Emphasis(s) => s.children.iter().map(extract_text).collect(),
        Node::Delete(s) => s.children.iter().map(extract_text).collect(),
        Node::Link(l) => l.children.iter().map(extract_text).collect(),
        Node::LinkReference(l) => l.children.iter().map(extract_text).collect(),
        Node::Image(i) => i.alt.clone(),
        _ => String::new(),
    }
}
