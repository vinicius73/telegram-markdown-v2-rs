use std::fs;
use std::path::PathBuf;

use rstest::rstest;
use telegram_markdown_v2_rs::{UnsupportedTagsStrategy, convert, convert_with_strategy};

fn parse_strategy_from_filename(path: &PathBuf) -> UnsupportedTagsStrategy {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if file_name.ends_with("__escape.input.md") {
        UnsupportedTagsStrategy::Escape
    } else if file_name.ends_with("__remove.input.md") {
        UnsupportedTagsStrategy::Remove
    } else {
        UnsupportedTagsStrategy::Keep
    }
}

fn expected_path_for_input(input_path: &PathBuf) -> PathBuf {
    let input_name = input_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("input fixture must have a filename");

    let expected_name = input_name.replace(".input.md", ".expected.md");
    input_path.with_file_name(expected_name)
}

#[rstest]
fn all_source_fixtures_match(
    #[files("tests/fixtures/*.input.md")] input_path: PathBuf,
) {
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|err| panic!("failed to read input fixture {}: {err}", input_path.display()));

    let expected_path = expected_path_for_input(&input_path);
    let expected = fs::read_to_string(&expected_path).unwrap_or_else(|err| {
        panic!(
            "failed to read expected fixture {}: {err}",
            expected_path.display()
        )
    });

    let strategy = parse_strategy_from_filename(&input_path);

    let actual = if strategy == UnsupportedTagsStrategy::Keep {
        convert(&input)
    } else {
        convert_with_strategy(&input, strategy)
    };

    assert_eq!(actual, expected, "fixture failed: {}", input_path.display());
}
