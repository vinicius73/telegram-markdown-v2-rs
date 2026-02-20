use markdown::{ParseOptions, to_mdast};
use regex::Regex;

use crate::definitions::collect_definitions;
use crate::handlers::{RenderContext, Renderer};
use crate::types::UnsupportedTagsStrategy;

const U_START: &str = "TGMDV2USTART";
const U_END: &str = "TGMDV2UEND";
const S_START: &str = "TGMDV2SSTART";
const S_END: &str = "TGMDV2SEND";

fn preprocess_v2_html_tags(text: &str) -> String {
    let underline = Regex::new(r"(?s)<u>(.*?)</u>").expect("valid underline regex");
    let spoiler =
        Regex::new(r#"(?s)<span class="tg-spoiler">(.*?)</span>"#).expect("valid spoiler regex");

    let with_underlines = underline.replace_all(text, format!("{U_START}${{1}}{U_END}"));
    spoiler
        .replace_all(with_underlines.as_ref(), format!("{S_START}${{1}}{S_END}"))
        .to_string()
}

fn postprocess_v2_formatting(text: &str) -> String {
    let with_underlines = text.replace(U_START, "__").replace(U_END, "__");
    with_underlines.replace(S_START, "||").replace(S_END, "||")
}

pub fn convert(markdown: &str) -> String {
    convert_with_strategy(markdown, UnsupportedTagsStrategy::Keep)
}

pub fn convert_with_strategy(markdown: &str, strategy: UnsupportedTagsStrategy) -> String {
    let processed_markdown = preprocess_v2_html_tags(markdown);
    let tree =
        to_mdast(&processed_markdown, &ParseOptions::gfm()).expect("valid markdown parse result");

    let definitions = collect_definitions(&tree);
    let context = RenderContext {
        definitions: &definitions,
        strategy,
    };

    let renderer = Renderer::new(&context);
    let result = renderer.render_root(&tree).replace("<!---->\n", "");
    postprocess_v2_formatting(&result)
}
