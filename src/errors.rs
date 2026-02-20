use markdown::message::Message;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to compile regex '{name}'")]
    RegexCompile {
        name: &'static str,
        pattern: &'static str,
        #[source]
        source: regex::Error,
    },

    #[error("failed to parse markdown input: {message}")]
    MarkdownParse { message: Message },
}
