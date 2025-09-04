//! Main application logic

use crate::cli::{self, Config};
use crate::errors::{ExitCode, RgrepError};
use crate::io::create_input_source;
use crate::search::{create_matcher, format_match, search_lines};

/// Main entry point for the application
pub fn run() -> ExitCode {
    match run_impl() {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("{error}");

            if error.should_show_help() {
                eprintln!("\n{}", cli::usage());
            }

            error.exit_code()
        }
    }
}

/// Internal implementation that can return errors
fn run_impl() -> Result<ExitCode, RgrepError> {
    let args = std::env::args().skip(1);
    let action = cli::parse_args(args)?;

    let config = match cli::handle_action(action) {
        Ok(config) => config,
        Err(exit_code) => return Ok(exit_code),
    };

    execute_search(&config)
}

/// Execute the search operation
pub fn execute_search(config: &Config) -> Result<ExitCode, RgrepError> {
    let matcher = create_matcher(config);
    let mut any_match = false;

    let files = config.actual_files();
    let multiple_files = files.len() > 1;

    if config.use_stdin() {
        // Search stdin
        let input_source = create_input_source(None)?;

        let matches = search_lines(input_source.lines, matcher.as_ref());
        let matched_any = process_matches(
            matches,
            input_source.name.as_deref(),
            false,
            config.line_number,
        );

        if matched_any {
            any_match = true;
        }
    } else {
        // Search files
        for file_path in &files {
            match process_file(file_path, &*matcher, multiple_files, config) {
                Ok(matched) => {
                    if matched {
                        any_match = true;
                    }
                }
                Err(error) => {
                    // Print error but continue with other files
                    eprintln!("{error}");
                }
            }
        }
    }

    if any_match {
        Ok(ExitCode::Success)
    } else {
        Ok(ExitCode::NoMatches)
    }
}

/// Process a single file
fn process_file(
    file_path: &str,
    matcher: &dyn crate::search::Matcher,
    show_filename: bool,
    config: &Config,
) -> Result<bool, RgrepError> {
    let input_source = create_input_source(Some(file_path))?;

    let matches = search_lines(input_source.lines, matcher);
    let matched_any = process_matches(
        matches,
        input_source.name.as_deref(),
        show_filename,
        config.line_number,
    );

    Ok(matched_any)
}

/// Process match results and print output
fn process_matches<I>(
    matches: I,
    source_name: Option<&str>,
    show_filename: bool,
    show_line_numbers: bool,
) -> bool
where
    I: Iterator<Item = crate::search::MatchResult>,
{
    let mut any_match = false;

    for match_result in matches {
        if match_result.matched {
            any_match = true;

            let output = format_match(&match_result, source_name, show_filename, show_line_numbers);

            println!("{output}");
        }
    }

    any_match
}
