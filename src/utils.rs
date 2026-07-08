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
    fn escape_with_rules<F>(text: &str, should_escape: F) -> String
    where
        F: Fn(u8) -> bool,
    {
        let extra = text
            .as_bytes()
            .iter()
            .filter(|&&byte| should_escape(byte))
            .count();

        if extra == 0 {
            return text.to_owned();
        }

        let mut escaped = String::with_capacity(text.len() + extra);
        for ch in text.chars() {
            if ch.is_ascii() && should_escape(ch as u8) {
                escaped.push('\\');
            }
            escaped.push(ch);
        }

        escaped
    }

    match text_type {
        TextType::Code => escape_with_rules(text, |byte| matches!(byte, b'\\' | b'`')),
        TextType::Link => {
            let is_telegram_deep_link = text.starts_with("tg://");
            escape_with_rules(text, |byte| {
                matches!(byte, b'\\' | b'(' | b')')
                    || (is_telegram_deep_link && matches!(byte, b'?' | b'=' | b'&'))
            })
        }
        TextType::Text => escape_with_rules(text, |byte| {
            matches!(
                byte,
                b'\\'
                    | b'_'
                    | b'*'
                    | b'['
                    | b']'
                    | b'('
                    | b')'
                    | b'~'
                    | b'`'
                    | b'>'
                    | b'#'
                    | b'+'
                    | b'-'
                    | b'='
                    | b'|'
                    | b'{'
                    | b'}'
                    | b'.'
                    | b'!'
            )
        }),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn wrap_adds_marker_on_both_sides() {
        assert_eq!(wrap("value", "*"), "*value*");
    }

    #[test]
    fn wrap_with_empty_marker_keeps_original_value() {
        assert_eq!(wrap("value", ""), "value");
    }

    #[test]
    fn is_url_accepts_absolute_and_rejects_relative() {
        assert!(is_url("https://example.com/path"));
        assert!(!is_url("/relative/path"));
    }

    mod escape_symbols_cases {
        use super::*;

        #[rstest]
        #[case(
            "_*[]()~`>#+-=|{}.!\\",
            "\\_\\*\\[\\]\\(\\)\\~\\`\\>\\#\\+\\-\\=\\|\\{\\}\\.\\!\\\\"
        )]
        #[case("", "")]
        #[case("plain text", "plain text")]
        #[case("Olá_日本_テスト", "Olá\\_日本\\_テスト")]
        fn escapes_text_type_text(#[case] input: &str, #[case] expected: &str) {
            assert_eq!(escape_symbols(input, TextType::Text), expected);
        }

        #[rstest]
        #[case("a`b\\c_d", "a\\`b\\\\c_d")]
        #[case("abc123", "abc123")]
        #[case("\\`", "\\\\\\`")]
        fn escapes_text_type_code(#[case] input: &str, #[case] expected: &str) {
            assert_eq!(escape_symbols(input, TextType::Code), expected);
        }

        #[rstest]
        #[case("https://example.com/a(b)c?x=1", "https://example.com/a\\(b\\)c?x=1")]
        #[case(
            "tg://resolve?domain=test(abc)",
            "tg://resolve\\?domain\\=test\\(abc\\)"
        )]
        #[case(
            "tg://time?unix=1647531900&format=wDT",
            "tg://time\\?unix\\=1647531900\\&format\\=wDT"
        )]
        #[case("TG://resolve?domain=test(abc)", "TG://resolve?domain=test\\(abc\\)")]
        fn escapes_text_type_link(#[case] input: &str, #[case] expected: &str) {
            assert_eq!(escape_symbols(input, TextType::Link), expected);
        }
    }

    mod unsupported_tags_escape {
        use super::*;

        #[rstest]
        #[case("<blockquote>quote</blockquote>", "<blockquote\\>quote</blockquote\\>")]
        #[case("a_b", "a\\_b")]
        #[case("plain text", "plain text")]
        #[case("", "")]
        fn escapes_content(#[case] content: &str, #[case] expected: &str) {
            assert_eq!(
                process_unsupported_tags(content, UnsupportedTagsStrategy::Escape),
                expected
            );
        }
    }

    mod unsupported_tags_remove {
        use super::*;

        #[rstest]
        #[case("<blockquote>quote</blockquote>")]
        #[case("a_b")]
        #[case("plain text")]
        #[case("")]
        fn removes_content(#[case] content: &str) {
            assert_eq!(
                process_unsupported_tags(content, UnsupportedTagsStrategy::Remove),
                ""
            );
        }
    }

    mod unsupported_tags_keep {
        use super::*;

        #[rstest]
        #[case("<blockquote>quote</blockquote>")]
        #[case("a_b")]
        #[case("plain text")]
        #[case("")]
        fn keeps_content(#[case] content: &str) {
            assert_eq!(
                process_unsupported_tags(content, UnsupportedTagsStrategy::Keep),
                content
            );
        }
    }
}
