use crate::types::{TextType, UnsupportedTagsStrategy};

/// Wraps `value` with the same marker on both sides.
pub fn wrap(value: &str, marker: &str) -> String {
    format!("{marker}{value}{marker}")
}

/// Returns `true` when `value` parses as an absolute URL.
pub fn is_url(value: &str) -> bool {
    url::Url::parse(value).is_ok()
}

/// Escapes Telegram MarkdownV2 special characters based on text context.
///
/// The escaping rules differ for normal text, code spans/blocks, and link
/// destinations.
pub fn escape_symbols(text: &str, text_type: TextType) -> String {
    match text_type {
        TextType::Code => text.replace('\\', "\\\\").replace('`', "\\`"),
        TextType::Link => {
            let mut result = text
                .replace('\\', "\\\\")
                .replace(')', "\\)")
                .replace('(', "\\(");
            // Telegram deep links frequently carry query params, so escape them
            // only in this scheme to preserve regular URL readability.
            if text.starts_with("tg://") {
                result = result.replace('?', "\\?").replace('=', "\\=");
            }
            result
        }
        TextType::Text => text
            .replace('\\', "\\\\")
            .replace('_', "\\_")
            .replace('*', "\\*")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('~', "\\~")
            .replace('`', "\\`")
            .replace('>', "\\>")
            .replace('#', "\\#")
            .replace('+', "\\+")
            .replace('-', "\\-")
            .replace('=', "\\=")
            .replace('|', "\\|")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('.', "\\.")
            .replace('!', "\\!"),
    }
}

/// Applies the configured strategy to unsupported Markdown content.
pub fn process_unsupported_tags(content: &str, strategy: UnsupportedTagsStrategy) -> String {
    match strategy {
        UnsupportedTagsStrategy::Escape => escape_symbols(content, TextType::Text),
        UnsupportedTagsStrategy::Remove => String::new(),
        UnsupportedTagsStrategy::Keep => content.to_owned(),
    }
}
