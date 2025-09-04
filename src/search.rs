//! Search functionality for pattern matching

use std::borrow::Cow;

use crate::cli::Config;

/// Result of a line match
#[derive(Debug)]
pub struct MatchResult {
    /// The original line content
    pub line: String,
    /// Line number (1-based)
    pub line_number: usize,
    /// Whether this line matched the pattern
    pub matched: bool,
}

/// A matcher that can check if a line matches the pattern
pub trait Matcher {
    /// Check if the given line matches
    fn matches(&self, line: &str) -> bool;
}

/// Case-sensitive literal matcher
pub struct LiteralMatcher {
    pattern: String,
}

impl LiteralMatcher {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }
}

impl Matcher for LiteralMatcher {
    fn matches(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}

/// Case-insensitive literal matcher  
pub struct CaseInsensitiveMatcher {
    pattern_lower: String,
}

impl CaseInsensitiveMatcher {
    pub fn new(pattern: String) -> Self {
        Self {
            pattern_lower: pattern.to_lowercase(),
        }
    }
}

impl Matcher for CaseInsensitiveMatcher {
    fn matches(&self, line: &str) -> bool {
        // Use Cow to avoid unnecessary allocations for ASCII-only content
        let line_lower: Cow<str> = if line.is_ascii() {
            Cow::Owned(line.to_ascii_lowercase())
        } else {
            Cow::Owned(line.to_lowercase())
        };

        line_lower.contains(&self.pattern_lower)
    }
}

/// Create the appropriate matcher based on configuration
pub fn create_matcher(config: &Config) -> Box<dyn Matcher> {
    if config.ignore_case {
        Box::new(CaseInsensitiveMatcher::new(config.pattern.clone()))
    } else {
        Box::new(LiteralMatcher::new(config.pattern.clone()))
    }
}

/// Search through lines and yield match results
pub fn search_lines<I>(
    lines: I,
    matcher: &dyn Matcher,
) -> impl Iterator<Item = MatchResult> + use<'_, I>
where
    I: Iterator<Item = Result<String, std::io::Error>>,
{
    lines.enumerate().filter_map(|(idx, line_result)| {
        match line_result {
            Ok(line) => {
                let line_number = idx + 1;
                let matched = matcher.matches(&line);
                Some(MatchResult {
                    line,
                    line_number,
                    matched,
                })
            }
            Err(_) => None, // Skip lines that couldn't be read
        }
    })
}

/// Format output for a match
pub fn format_match(
    match_result: &MatchResult,
    source_name: Option<&str>,
    show_filename: bool,
    show_line_numbers: bool,
) -> String {
    let mut output = String::new();

    // Add filename prefix if needed
    if show_filename {
        if let Some(name) = source_name {
            output.push_str(name);
            output.push(':');
        }
    }

    // Add line number if needed
    if show_line_numbers {
        output.push_str(&format!("{}:", match_result.line_number));
    }

    // Add the actual line content
    output.push_str(&match_result.line);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_matcher() {
        let matcher = LiteralMatcher::new("test".to_string());
        assert!(matcher.matches("this is a test"));
        assert!(matcher.matches("testing"));
        assert!(!matcher.matches("this is a TEST"));
        assert!(!matcher.matches("no match here"));
    }

    #[test]
    fn test_case_insensitive_matcher() {
        let matcher = CaseInsensitiveMatcher::new("Test".to_string());
        assert!(matcher.matches("this is a test"));
        assert!(matcher.matches("this is a TEST"));
        assert!(matcher.matches("Testing"));
        assert!(!matcher.matches("no match here"));
    }

    #[test]
    fn test_format_match() {
        let match_result = MatchResult {
            line: "hello world".to_string(),
            line_number: 42,
            matched: true,
        };

        // Basic formatting
        assert_eq!(
            format_match(&match_result, None, false, false),
            "hello world"
        );

        // With line numbers
        assert_eq!(
            format_match(&match_result, None, false, true),
            "42:hello world"
        );

        // With filename
        assert_eq!(
            format_match(&match_result, Some("test.txt"), true, false),
            "test.txt:hello world"
        );

        // With both filename and line numbers
        assert_eq!(
            format_match(&match_result, Some("test.txt"), true, true),
            "test.txt:42:hello world"
        );
    }
}
