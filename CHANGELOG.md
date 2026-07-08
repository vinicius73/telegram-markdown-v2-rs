# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1](https://github.com/vinicius73/telegram-markdown-v2-rs/compare/telegram-markdown-v2-v0.2.0...telegram-markdown-v2-v0.2.1) (2026-07-08)


### Features

* **blockquote:** support expandable quotes via HTML expandable attribute ([d02a3e6](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/d02a3e695a43817cf2721dbe2dc73b38c5f22915))
* **errors:** enhance error handling for Markdown processing ([84184e2](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/84184e27f19bba6e6e913eab5d8baae3b94843e7))
* **html:** support ins, tg-spoiler, and tg-emoji tags ([62e56c3](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/62e56c367649c9617ed84184b89d35b4358529b6))
* **html:** support tg-time tags from Telegram HTML style ([a56b501](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/a56b50125eaef5637036f927bcdc7e73fe52d9e9))


### Bug Fixes

* avoid v2 underline/spoiler conversion inside code ([cf4303f](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/cf4303fde5d854f66b775294897a4ea061b7d2ed))
* **blockquote:** treat Telegram blockquotes as supported markup ([4f5f70d](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/4f5f70d70bde864c9fa10625b0cd03e50acacb8a))
* **code:** preserve fenced code language for Telegram pre entities ([f8c0645](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/f8c06456b880ffb5f7a40e9f80a7893f4c7f370c))
* **escape:** match official MarkdownV2 link escaping rules ([b87ff97](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/b87ff97198d14e8b957a2f0150e0350dbe66e586))
* **html:** escape tg-emoji/tg-time labels before Markdown parsing ([f88c430](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/f88c4302c3759d9257ac82208da3ab1b53205301))
* **html:** keep expandable blockquote markers literal inside code ([633f20f](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/633f20f792f41b22662bafb3d1e4b42da9f29c2c))
* **html:** keep expandable blockquotes literal inside code spans/blocks ([ed5b3cd](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/ed5b3cd7656f9a9d4c145570a7486f3bc2a3e758))
* **links:** keep bang prefix for Telegram emoji and time entities ([0e26430](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/0e264303cda98be4ade459b7eb477874a0bdecb9))
* **rendering:** optimize child rendering by handling empty children case and reducing reallocations ([5e3a952](https://github.com/vinicius73/telegram-markdown-v2-rs/commit/5e3a952d474ddbcfc572cd5d4b0a23f2c12cf92b))

## [0.2.0](https://github.com/vinicius73/telegram-markdown-v2-rs/tree/v0.2.0) (2026-07-08)

### Features

* Align public API with Telegram MarkdownV2 conversion
