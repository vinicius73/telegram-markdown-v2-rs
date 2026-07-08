/// Strategy used to handle Markdown nodes that cannot be represented in
/// Telegram MarkdownV2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnsupportedTagsStrategy {
    /// Convert unsupported content into plain escaped text.
    Escape,
    /// Remove unsupported content from the output.
    Remove,
    /// Keep unsupported content unchanged.
    #[default]
    Keep,
}

/// Escaping context used by `escape_symbols`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextType {
    /// Regular message text context.
    Text,
    /// Inline or block code context.
    Code,
    /// URL/link destination context.
    Link,
    /// Markdown image label before parsing (`\`, `[`, `]` only).
    MarkdownLabel,
}

/// Reference-style link definition extracted from the Markdown AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    /// Optional title associated with the definition.
    pub title: Option<String>,
    /// Destination URL used by the definition.
    pub url: String,
}
