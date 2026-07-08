## telegram-markdown-v2 (Rust)

Rust library to transform “regular” Markdown into **Telegram MarkdownV2** (ready to send with `parse_mode = MarkdownV2`).

This project is a **Rust port** of the TypeScript library [AndyRightNow/telegram-markdown-v2](https://github.com/AndyRightNow/telegram-markdown-v2).

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
telegram-markdown-v2 = "0.1"
```

### Input mapping

This library converts **regular Markdown/GFM** (plus Telegram HTML extensions) into **Telegram MarkdownV2**. It does not accept MarkdownV2 syntax as input.

| Telegram result | Library input | MarkdownV2 output |
| --- | --- | --- |
| bold | `**text**` | `*text*` |
| italic | `*text*` or `_text_` | `_text_` |
| underline | `<u>text</u>` | `__text__` |
| spoiler | `<span class="tg-spoiler">text</span>` | `\|\|text\|\|` |
| link / mention | `[label](https://…)` or `[label](tg://user?id=…)` | `[label](escaped-url)` |
| custom emoji | `![emoji](tg://emoji?id=…)` | `![emoji](tg://emoji\?id\=…)` |
| date/time | `![label](tg://time?unix=…&format=…)` | `![label](tg://time\?unix\=…&format\=…)` |
| code with language | fenced block with language tag | fenced block with language tag |
| blockquote | line starting with `>` | line starting with `>` |

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
