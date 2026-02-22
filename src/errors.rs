use markdown::message::Message;
use thiserror::Error;

/// Convenience result type used across this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned while converting Markdown into Telegram MarkdownV2.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    /// A regular expression used internally could not be compiled.
    #[error("failed to compile regex '{name}'")]
    RegexCompile {
        /// Stable identifier for the regex used in diagnostics and tests.
        name: &'static str,
        /// Regex pattern string that failed compilation.
        pattern: &'static str,
        #[source]
        /// Underlying regex engine error.
        source: regex::Error,
    },

    /// Markdown parser failed to parse the input document.
    #[error("failed to parse markdown input: {message}")]
    MarkdownParse { message: Message },
}
