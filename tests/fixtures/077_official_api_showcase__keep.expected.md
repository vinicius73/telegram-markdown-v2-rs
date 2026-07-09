*Telegram MarkdownV2 showcase*

This document exercises the official Bot API examples using *regular Markdown/GFM*
plus Telegram HTML extensions as library input\.

*Basic formatting*

*bold \*text*

_italic \*text_

__underline__

__underline via ins__

~strikethrough~

||spoiler||

||spoiler via tg\-spoiler||

*Nested formatting \(official sample\)*

*bold \_italic bold ~italic bold strikethrough~ ||italic bold strikethrough spoiler|| __underline italic bold__ bold*

*Links and mentions*

[inline URL](http://www.example.com/)

[inline mention of a user](tg://user?id=123456789)

*Custom emoji and date/time \(Markdown image syntax\)*

![👍](tg://emoji?id=5368324170671202286)

![22:45 tomorrow](tg://time?unix=1647531900&format=wDT)

![22:45 tomorrow](tg://time?unix=1647531900&format=t)

![22:45 tomorrow](tg://time?unix=1647531900&format=r)

![22:45 tomorrow](tg://time?unix=1647531900)

*Custom emoji and date/time \(HTML syntax\)*

![👍](tg://emoji?id=5368324170671202286) from HTML

![22:45 tomorrow](tg://time?unix=1647531900&format=wDT) from HTML

![22:45 tomorrow without format](tg://time?unix=1647531900) from HTML

*Code*

`inline fixed-width code`

```
pre-formatted fixed-width code block
```

```python
pre-formatted fixed-width code block written in the Python programming language
```

*Blockquotes*

> Block quotation started
> Block quotation continued
> Block quotation continued
> The last line of the block quotation
**>The expandable block quotation started right after the previous block quotation
> It is separated from the previous block quotation by an empty bold entity
> Expandable block quotation continued
> Hidden by default part of the expandable block quotation started
> Expandable block quotation continued
> The last line of the expandable block quotation with the expandability mark||

*Escaping stress \(special chars outside code\)*

Price is 10\.50 with \(parentheses\) and a literal \*asterisk\*\.

*List with inline formatting*

•   *bold item* with `code` and [link](https://example.com/path?q=1&x=y)
•   _italic item_ with ![👍](tg://emoji?id=5368324170671202286)
•   __underlined item__ and ||hidden detail||

*Table \(unsupported — kept as\-is\)*

| Col A | Col B |
| -     | -     |
| bold  | code  |
