# Telegram MarkdownV2 showcase

This document exercises the official Bot API examples using **regular Markdown/GFM**
plus Telegram HTML extensions as library input.

## Basic formatting

**bold \*text**

_italic \*text_

<u>underline</u>

<ins>underline via ins</ins>

~~strikethrough~~

<span class="tg-spoiler">spoiler</span>

<tg-spoiler>spoiler via tg-spoiler</tg-spoiler>

## Nested formatting (official sample)

**bold _italic bold ~~italic bold strikethrough~~ <span class="tg-spoiler">italic bold strikethrough spoiler</span> <u>underline italic bold</u> bold**

## Links and mentions

[inline URL](http://www.example.com/)

[inline mention of a user](tg://user?id=123456789)

## Custom emoji and date/time (Markdown image syntax)

![👍](tg://emoji?id=5368324170671202286)

![22:45 tomorrow](tg://time?unix=1647531900&format=wDT)

![22:45 tomorrow](tg://time?unix=1647531900&format=t)

![22:45 tomorrow](tg://time?unix=1647531900&format=r)

![22:45 tomorrow](tg://time?unix=1647531900)

## Custom emoji and date/time (HTML syntax)

<tg-emoji emoji-id="5368324170671202286">👍</tg-emoji> from HTML

<tg-time unix="1647531900" format="wDT">22:45 tomorrow</tg-time> from HTML

<tg-time unix="1647531900">22:45 tomorrow without format</tg-time> from HTML

## Code

`inline fixed-width code`

```
pre-formatted fixed-width code block
```

```python
pre-formatted fixed-width code block written in the Python programming language
```

## Blockquotes

> Block quotation started
> Block quotation continued
> Block quotation continued
> The last line of the block quotation

<blockquote expandable>
The expandable block quotation started right after the previous block quotation
It is separated from the previous block quotation by an empty bold entity
Expandable block quotation continued
Hidden by default part of the expandable block quotation started
Expandable block quotation continued
The last line of the expandable block quotation with the expandability mark
</blockquote>

## Escaping stress (special chars outside code)

Price is 10\.50 with \(parentheses\) and a literal \*asterisk\*.

## List with inline formatting

- **bold item** with `code` and [link](https://example.com/path?q=1&x=y)
- _italic item_ with ![👍](tg://emoji?id=5368324170671202286)
- <u>underlined item</u> and <span class="tg-spoiler">hidden detail</span>

## Table (unsupported — kept as-is)

| Col A | Col B |
| --- | --- |
| *bold* | `code` |
