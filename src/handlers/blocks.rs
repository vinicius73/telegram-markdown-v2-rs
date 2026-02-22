use markdown::mdast::{AlignKind, Blockquote, Html, List, ListItem, Node, Table};

use crate::errors::Result;
use crate::utils::process_unsupported_tags;

use super::Renderer;

/// Renders a Markdown list by rendering each list item on its own line.
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

/// Renders one list item using ordered or bullet style.
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

/// Renders a blockquote by prefixing each non-empty line with `> `.
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

/// Renders raw HTML according to the unsupported-tags strategy.
///
/// HTML comments are always ignored.
pub fn render_html(renderer: &Renderer<'_>, node: &Html) -> Result<String> {
    if node.value.starts_with("<!--") {
        return Ok(String::new());
    }
    Ok(process_unsupported_tags(
        &node.value,
        renderer.context().strategy,
    ))
}

/// Renders a Markdown table as pipe-separated rows.
///
/// The resulting table is passed through the unsupported-tags strategy because
/// Telegram MarkdownV2 does not natively support tables.
pub fn render_table(renderer: &Renderer<'_>, node: &Table) -> Result<String> {
    let mut rows: Vec<Vec<String>> = Vec::with_capacity(node.children.len());
    for row_node in &node.children {
        if let Node::TableRow(row) = row_node {
            let mut cells = Vec::with_capacity(row.children.len());
            for cell_node in &row.children {
                if let Node::TableCell(cell) = cell_node {
                    let mut value = String::new();
                    for child in &cell.children {
                        extract_text_into(child, &mut value);
                    }
                    let trimmed = value.trim();
                    let value = if trimmed.len() == value.len() {
                        value
                    } else {
                        trimmed.to_owned()
                    };
                    cells.push(value);
                }
            }
            rows.push(cells);
        }
    }

    if rows.is_empty() {
        return Ok(process_unsupported_tags("", renderer.context().strategy));
    }

    let max_cols = rows
        .iter()
        .map(Vec::len)
        .max()
        .unwrap_or_default()
        .max(node.align.len());
    let col_aligns: Vec<AlignKind> = (0..max_cols)
        .map(|idx| node.align.get(idx).copied().unwrap_or(AlignKind::None))
        .collect();

    let mut col_widths = vec![0usize; max_cols];
    for row in &rows {
        for (idx, cell) in row.iter().enumerate() {
            col_widths[idx] = col_widths[idx].max(cell.len());
        }
    }
    for (idx, align) in col_aligns.iter().enumerate() {
        col_widths[idx] = col_widths[idx].max(separator_marker(*align).len());
    }

    let mut output = String::new();
    push_formatted_row(&mut output, &rows[0], &col_widths, &col_aligns);

    let separators: Vec<String> = col_aligns
        .iter()
        .map(|align| separator_marker(*align).to_owned())
        .collect();
    let separator_aligns = vec![AlignKind::None; max_cols];
    push_formatted_row(&mut output, &separators, &col_widths, &separator_aligns);

    for row in rows.iter().skip(1) {
        push_formatted_row(&mut output, row, &col_widths, &col_aligns);
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

fn extract_text_into(node: &Node, out: &mut String) {
    match node {
        Node::Text(t) => out.push_str(&t.value),
        Node::InlineCode(c) => out.push_str(&c.value),
        Node::Strong(s) => {
            for child in &s.children {
                extract_text_into(child, out);
            }
        }
        Node::Emphasis(s) => {
            for child in &s.children {
                extract_text_into(child, out);
            }
        }
        Node::Delete(s) => {
            for child in &s.children {
                extract_text_into(child, out);
            }
        }
        Node::Link(l) => {
            for child in &l.children {
                extract_text_into(child, out);
            }
        }
        Node::LinkReference(l) => {
            for child in &l.children {
                extract_text_into(child, out);
            }
        }
        Node::Image(i) => out.push_str(&i.alt),
        _ => {}
    }
}

fn separator_marker(align: AlignKind) -> &'static str {
    match align {
        AlignKind::Left => ":-",
        AlignKind::Right => "-:",
        AlignKind::Center => ":-:",
        AlignKind::None => "-",
    }
}

fn push_formatted_row(
    row_output: &mut String,
    row: &[String],
    widths: &[usize],
    aligns: &[AlignKind],
) {
    row_output.push('|');

    for (col, width) in widths.iter().copied().enumerate() {
        let value = row.get(col).map(String::as_str).unwrap_or("");
        let align = aligns.get(col).copied().unwrap_or(AlignKind::None);
        let pad = width.saturating_sub(value.len());
        let (left_pad, right_pad) = match align {
            AlignKind::Right => (pad, 0),
            AlignKind::Center => (pad / 2, pad - (pad / 2)),
            AlignKind::Left | AlignKind::None => (0, pad),
        };

        row_output.push(' ');
        if left_pad > 0 {
            row_output.extend(std::iter::repeat_n(' ', left_pad));
        }
        row_output.push_str(value);
        if right_pad > 0 {
            row_output.extend(std::iter::repeat_n(' ', right_pad));
        }
        row_output.push(' ');
        row_output.push('|');
    }

    row_output.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use markdown::{ParseOptions, to_mdast};

    use crate::handlers::RenderContext;
    use crate::types::UnsupportedTagsStrategy;

    fn parse_table(markdown: &str) -> Table {
        let tree = to_mdast(markdown, &ParseOptions::gfm()).expect("markdown should parse");
        let Node::Root(root) = tree else {
            panic!("parser root should be a root node");
        };

        root.children
            .into_iter()
            .find_map(|node| match node {
                Node::Table(table) => Some(table),
                _ => None,
            })
            .expect("fixture should contain one table node")
    }

    #[test]
    fn render_table_formats_columns_and_alignment_with_keep_strategy() {
        let markdown =
            "| a | b | c | d |\n| - | :- | -: | :-: |\n| e | f |\n| g | h | i | j | k |\n";
        let table = parse_table(markdown);
        let definitions = HashMap::new();
        let context = RenderContext {
            definitions: &definitions,
            strategy: UnsupportedTagsStrategy::Keep,
        };
        let renderer = Renderer::new(&context);

        let rendered = render_table(&renderer, &table).expect("table should render");

        let expected = [
            "| a | b  |  c |  d  |   |",
            "| - | :- | -: | :-: | - |",
            "| e | f  |    |     |   |",
            "| g | h  |  i |  j  | k |",
            "",
        ]
        .join("\n");

        assert_eq!(rendered, expected);
    }

    #[test]
    fn render_table_applies_remove_strategy() {
        let markdown =
            "| a | b | c | d |\n| - | :- | -: | :-: |\n| e | f |\n| g | h | i | j | k |\n";
        let table = parse_table(markdown);
        let definitions = HashMap::new();
        let context = RenderContext {
            definitions: &definitions,
            strategy: UnsupportedTagsStrategy::Remove,
        };
        let renderer = Renderer::new(&context);

        let rendered = render_table(&renderer, &table).expect("table should render");

        assert_eq!(rendered, "");
    }
}
