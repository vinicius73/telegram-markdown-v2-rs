## telegram-markdown-v2 (Rust)

Rust library to transform “regular” Markdown into **Telegram MarkdownV2** (ready to send with `parse_mode = MarkdownV2`).

This project is a **Rust port** of the TypeScript library [AndyRightNow/telegram-markdown-v2](https://github.com/AndyRightNow/telegram-markdown-v2).

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
telegram-markdown-v2 = "0.1"
```

### Usage

#### Basic conversion

```rust
use telegram_markdown_v2::convert;

fn main() -> telegram_markdown_v2::Result<()> {
    let out = convert("Hello world!")?;
    assert_eq!(out, "Hello world\\!\n");
    Ok(())
}
```

#### Strategy for “unsupported” constructs

Telegram MarkdownV2 does not support some Markdown/HTML constructs (for example: blockquotes, tables, HTML blocks). Use `UnsupportedTagsStrategy` to decide what to do:

- `Keep`: keep the content as-is
- `Escape`: treat it as plain text and escape special characters
- `Remove`: drop the content entirely

```rust
use telegram_markdown_v2::{convert_with_strategy, UnsupportedTagsStrategy};

fn main() -> telegram_markdown_v2::Result<()> {
    let out = convert_with_strategy("> test", UnsupportedTagsStrategy::Escape)?;
    assert_eq!(out, "\\> test\n");
    Ok(())
}
```

#### Telegram-specific extensions

Outside inline/fenced code, the converter recognizes:

- `<u>…</u>` → `__…__` (underline)
- `<span class="tg-spoiler">…</span>` → `||…||` (spoiler)

```rust
use telegram_markdown_v2::convert;

fn main() -> telegram_markdown_v2::Result<()> {
    let out = convert(r#"This is <u>underlined</u> and <span class="tg-spoiler">hidden</span>."#)?;
    assert_eq!(out, "This is __underlined__ and ||hidden||\\.\n");
    Ok(())
}
```

### Notes

- The output ends with a trailing `\n` when the rendered result is non-empty.
- Telegram limits messages to 4096 characters; split messages in your application if needed.
