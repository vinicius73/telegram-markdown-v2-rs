use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use rstest::rstest;
use telegram_markdown_v2::{UnsupportedTagsStrategy, convert, convert_with_strategy};

fn parse_strategy_from_filename(path: &Path) -> UnsupportedTagsStrategy {
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

fn expected_path_for_input(input_path: &Path) -> io::Result<PathBuf> {
    let input_name = input_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "input fixture path has no filename: {}",
                    input_path.display()
                ),
            )
        })?;

    let expected_name = input_name.replace(".input.md", ".expected.md");
    Ok(input_path.with_file_name(expected_name))
}

#[rstest]
fn all_source_fixtures_match(
    #[files("tests/fixtures/*.input.md")] input_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(&input_path)?;

    let expected_path = expected_path_for_input(&input_path)?;
    let expected = fs::read_to_string(&expected_path)?;

    let strategy = parse_strategy_from_filename(&input_path);

    let actual = if strategy == UnsupportedTagsStrategy::Keep {
        convert(&input)?
    } else {
        convert_with_strategy(&input, strategy)?
    };

    assert_eq!(actual, expected, "fixture failed: {}", input_path.display());
    Ok(())
}

/// End-to-end showcase of official Bot API MarkdownV2 constructs.
///
/// Exercises regular Markdown/GFM and Telegram HTML extensions in a single
/// large document, asserting the full converted output in one shot.
#[test]
fn official_api_showcase_e2e() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("tests/fixtures/077_official_api_showcase__keep.input.md")?;
    let expected =
        fs::read_to_string("tests/fixtures/077_official_api_showcase__keep.expected.md")?;
    let actual = convert(&input)?;
    assert_eq!(actual, expected);
    Ok(())
}
