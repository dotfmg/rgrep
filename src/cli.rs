//! Command-line interface definition and parsing

use clap::{error::ErrorKind as ClapErrorKind, CommandFactory, Parser};

use crate::errors::{ExitCode, RgrepError};

/// Configuration for the search operation
#[derive(Debug, Clone)]
pub struct Config {
    /// The search pattern (literal substring)
    pub pattern: String,
    /// Files to search (empty means stdin)
    pub files: Vec<String>,
    /// Case-insensitive matching
    pub ignore_case: bool,
    /// Show line numbers in output
    pub line_number: bool,
}

/// Actions that the CLI can perform
#[derive(Debug)]
pub enum CliAction {
    /// Show help and exit
    ShowHelp,
    /// Show version and exit
    ShowVersion,
    /// Run the search with given configuration
    Run(Config),
}

/// CLI argument definition using Clap derive
#[derive(Debug, Parser)]
#[command(
    name = "rgrep",
    version = "0.1.0",
    about = "A simple grep-like tool written in Rust",
    long_about = "rgrep searches for patterns in files or standard input.\n\
                  It supports case-insensitive matching and line number display."
)]
struct Cli {
    /// Case-insensitive matching
    #[arg(short = 'i', long = "ignore-case", help = "Ignore case when matching")]
    ignore_case: bool,

    /// Show line numbers
    #[arg(short = 'n', long = "line-number", help = "Show line numbers")]
    line_number: bool,

    /// The search pattern (literal substring)
    #[arg(help = "Pattern to search for", required = true)]
    pattern: String,

    /// Files to search (use '-' for stdin)
    #[arg(help = "Files to search (default: stdin)")]
    files: Vec<String>,
}

impl Config {
    /// Create a new configuration
    pub fn new(pattern: String, files: Vec<String>, ignore_case: bool, line_number: bool) -> Self {
        Self {
            pattern,
            files,
            ignore_case,
            line_number,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), RgrepError> {
        if self.pattern.is_empty() {
            return Err(RgrepError::invalid_args("Pattern cannot be empty", false));
        }

        // Check for conflicting file specifications
        if self.files.len() > 1 && self.files.contains(&"-".to_string()) {
            return Err(RgrepError::invalid_args(
                "Cannot mix stdin (-) with other files",
                false,
            ));
        }

        Ok(())
    }

    /// Check if we should read from stdin
    pub fn use_stdin(&self) -> bool {
        self.files.is_empty() || (self.files.len() == 1 && self.files[0] == "-")
    }

    /// Get the actual files to process (filters out stdin indicators)
    pub fn actual_files(&self) -> Vec<String> {
        if self.use_stdin() {
            Vec::new()
        } else {
            self.files.clone()
        }
    }
}

/// Parse command-line arguments into a CLI action
pub fn parse_args<I>(args: I) -> Result<CliAction, RgrepError>
where
    I: IntoIterator<Item = String>,
{
    // Clap expects argv[0] as the binary name
    let argv: Vec<String> = std::iter::once("rgrep".to_string())
        .chain(args)
        .collect();

    let cmd = build_command();

    match cmd.try_get_matches_from(argv) {
        Ok(matches) => {
            let ignore_case = matches.get_flag("ignore_case");
            let line_number = matches.get_flag("line_number");

            let pattern = matches
                .get_one::<String>("pattern")
                .expect("pattern is required")
                .to_string();

            let files: Vec<String> = matches
                .get_many::<String>("files")
                .map(|vals| vals.map(|s| s.to_string()).collect())
                .unwrap_or_default();

            let config = Config::new(pattern, files, ignore_case, line_number);
            config.validate()?;

            Ok(CliAction::Run(config))
        }
        Err(e) => match e.kind() {
            ClapErrorKind::DisplayHelp => Ok(CliAction::ShowHelp),
            ClapErrorKind::DisplayVersion => Ok(CliAction::ShowVersion),
            _ => Err(RgrepError::invalid_args(e.to_string(), true)),
        },
    }
}

/// Build the Clap command with additional configuration
fn build_command() -> clap::Command {
    let mut cmd = Cli::command();
    cmd = cmd.after_help(
        "Examples:\n  \
         rgrep foo file.txt              Search for 'foo' in file.txt\n  \
         rgrep -i error *.log            Case-insensitive search in log files\n  \
         rgrep -n pattern file1 file2    Show line numbers for matches\n  \
         echo 'test' | rgrep test        Search in stdin\n  \
         rgrep pattern -                 Explicitly search stdin",
    );
    cmd
}

/// Generate usage text
pub fn usage() -> String {
    let mut cmd = build_command();
    let mut buf = Vec::new();
    cmd.write_long_help(&mut buf).ok();
    String::from_utf8_lossy(&buf).to_string()
}

/// Generate version text
pub fn version() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

/// Handle CLI action results
pub fn handle_action(action: CliAction) -> Result<Config, ExitCode> {
    match action {
        CliAction::ShowHelp => {
            println!("{}", usage());
            Err(ExitCode::Success)
        }
        CliAction::ShowVersion => {
            println!("{}", version());
            Err(ExitCode::Success)
        }
        CliAction::Run(config) => Ok(config),
    }
}
