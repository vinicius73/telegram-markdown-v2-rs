#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnsupportedTagsStrategy {
    Escape,
    Remove,
    #[default]
    Keep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextType {
    Text,
    Code,
    Link,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub title: Option<String>,
    pub url: String,
}
