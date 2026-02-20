use crate::types::{TextType, UnsupportedTagsStrategy};

pub fn wrap(value: &str, marker: &str) -> String {
    format!("{marker}{value}{marker}")
}

pub fn is_url(value: &str) -> bool {
    url::Url::parse(value).is_ok()
}

pub fn escape_symbols(text: &str, text_type: TextType) -> String {
    match text_type {
        TextType::Code => text.replace('\\', "\\\\").replace('`', "\\`"),
        TextType::Link => {
            let mut result = text
                .replace('\\', "\\\\")
                .replace(')', "\\)")
                .replace('(', "\\(");
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

pub fn process_unsupported_tags(content: &str, strategy: UnsupportedTagsStrategy) -> String {
    match strategy {
        UnsupportedTagsStrategy::Escape => escape_symbols(content, TextType::Text),
        UnsupportedTagsStrategy::Remove => String::new(),
        UnsupportedTagsStrategy::Keep => content.to_owned(),
    }
}
