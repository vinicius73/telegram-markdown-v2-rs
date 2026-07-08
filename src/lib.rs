//! Convert Markdown into Telegram **MarkdownV2**.
//!
//! Telegram's MarkdownV2 is a restricted dialect with its own escaping rules. This crate
//! takes a Markdown document and renders it into a string suitable to be sent with
//! Telegram's `parse_mode = MarkdownV2`.
//!
//! The main entry points are [`convert`] (default behavior) and [`convert_with_strategy`]
//! (custom handling for unsupported constructs).
//!
//! ## Basic usage
//!
//! ```rust
//! # fn main() -> telegram_markdown_v2::Result<()> {
//! use telegram_markdown_v2::convert;
//!
//! let out = convert("Hello world!")?;
//! assert_eq!(out, "Hello world\\!\n");
//! # Ok(())
//! # }
//! ```
//!
//! ## Unsupported constructs
//!
//! Telegram MarkdownV2 does not support some Markdown/HTML constructs (e.g. tables
//! and raw HTML blocks). Use [`UnsupportedTagsStrategy`] to decide what to do
//! with such input:
//!
//! - [`UnsupportedTagsStrategy::Keep`]: keep the unsupported content as-is.
//! - [`UnsupportedTagsStrategy::Escape`]: escape the unsupported content as plain text.
//! - [`UnsupportedTagsStrategy::Remove`]: drop the unsupported content entirely.
//!
//! ```rust
//! # fn main() -> telegram_markdown_v2::Result<()> {
//! use telegram_markdown_v2::{convert_with_strategy, UnsupportedTagsStrategy};
//!
//! let out = convert_with_strategy("<div>test</div>", UnsupportedTagsStrategy::Escape)?;
//! assert_eq!(out, "<div\\>test</div\\>\n");
//! # Ok(())
//! # }
//! ```
//!
//! ## Telegram-specific extensions
//!
//! Outside inline/fenced code, the converter recognizes:
//!
//! - `<u>…</u>` and `<ins>…</ins>` as underline (`__…__`)
//! - `<span class="tg-spoiler">…</span>` and `<tg-spoiler>…</tg-spoiler>` as spoiler (`||…||`)
//! - `<tg-emoji emoji-id="…">…</tg-emoji>` as custom emoji (`![…](tg://emoji?id=…)`)
//! - `<tg-time unix="…" format="…">…</tg-time>` as date/time (`![…](tg://time?unix=…&format=…)`)
//! - `<blockquote expandable>…</blockquote>` as expandable blockquote (`> …||`)
mod convert;
mod definitions;
mod errors;
mod handlers;
mod types;
mod utils;

pub use convert::{convert, convert_with_strategy};
pub use errors::{Error, Result};
pub use types::{Definition, TextType, UnsupportedTagsStrategy};
