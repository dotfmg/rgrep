use rgrep::app;
use rgrep::search::{create_matcher, format_match, search_lines, MatchResult};
use rgrep::Config;

#[test]
fn search_literal_match() {
    let config = Config::new("test".into(), vec![], false, false);
    let matcher = create_matcher(&config);

    let input = "line 1\nthis is a test\nline 3";
    let lines = input.lines().map(|s| Ok(s.to_string()));

    let results: Vec<_> = search_lines(lines, matcher.as_ref()).collect();

    let matched: Vec<_> = results.into_iter().filter(|r| r.matched).collect();
    assert_eq!(matched.len(), 1);
    assert_eq!(matched[0].line, "this is a test");
    assert_eq!(matched[0].line_number, 2);
}

#[test]
fn search_case_insensitive_match() {
    let config = Config::new("TEST".into(), vec![], true, false);
    let matcher = create_matcher(&config);

    let input = "line 1\nthis is a test\nline 3";
    let lines = input.lines().map(|s| Ok(s.to_string()));

    let results: Vec<_> = search_lines(lines, matcher.as_ref()).collect();

    let matched: Vec<_> = results.into_iter().filter(|r| r.matched).collect();
    assert_eq!(matched.len(), 1);
    assert_eq!(matched[0].line, "this is a test");
}

#[test]
fn format_match_basic() {
    let match_result = MatchResult {
        line: "hello world".to_string(),
        line_number: 5,
        matched: true,
    };

    assert_eq!(
        format_match(&match_result, None, false, false),
        "hello world"
    );
}

#[test]
fn format_match_with_line_numbers() {
    let match_result = MatchResult {
        line: "hello world".to_string(),
        line_number: 5,
        matched: true,
    };

    assert_eq!(
        format_match(&match_result, None, false, true),
        "5:hello world"
    );
}

#[test]
fn format_match_with_filename() {
    let match_result = MatchResult {
        line: "hello world".to_string(),
        line_number: 5,
        matched: true,
    };

    assert_eq!(
        format_match(&match_result, Some("test.txt"), true, false),
        "test.txt:hello world"
    );
}

#[test]
fn format_match_with_filename_and_line_numbers() {
    let match_result = MatchResult {
        line: "hello world".to_string(),
        line_number: 5,
        matched: true,
    };

    assert_eq!(
        format_match(&match_result, Some("test.txt"), true, true),
        "test.txt:5:hello world"
    );
}

// Integration tests using the app module
mod integration {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn app_run_with_temp_file() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "line 1").unwrap();
        writeln!(temp_file, "this contains foo").unwrap();
        writeln!(temp_file, "line 3").unwrap();
        temp_file.flush().unwrap();

        let config = Config::new(
            "foo".to_string(),
            vec![temp_file.path().to_string_lossy().to_string()],
            false,
            false,
        );

        // This is a simplified test - in practice we'd need to capture stdout
        // For now we just verify the function doesn't panic
        let result = super::app::execute_search(&config);
        assert!(result.is_ok());
    }
}
