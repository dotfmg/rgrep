//! Error handling and exit codes for rgrep

use std::fmt;
use std::io;

/// Unified error type for all rgrep operations
#[derive(Debug)]
pub enum RgrepError {
    /// Invalid command line arguments
    InvalidArgs { message: String, show_help: bool },
    /// File I/O error
    IoError { path: String, source: io::Error },
    /// General application error
    AppError { message: String },
}

impl fmt::Display for RgrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RgrepError::InvalidArgs { message, .. } => write!(f, "rgrep: {message}"),
            RgrepError::IoError { path, source } => write!(f, "rgrep: {path}: {source}"),
            RgrepError::AppError { message } => write!(f, "rgrep: {message}"),
        }
    }
}

impl std::error::Error for RgrepError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RgrepError::IoError { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Exit codes following POSIX conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Successful execution with matches found
    Success = 0,
    /// No matches found (normal operation)
    NoMatches = 1,
    /// Invalid arguments or usage error
    InvalidArgs = 2,
    /// File I/O or system error
    IoError = 3,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}

impl RgrepError {
    /// Get the appropriate exit code for this error
    pub fn exit_code(&self) -> ExitCode {
        match self {
            RgrepError::InvalidArgs { .. } => ExitCode::InvalidArgs,
            RgrepError::IoError { .. } => ExitCode::IoError,
            RgrepError::AppError { .. } => ExitCode::IoError,
        }
    }

    /// Create an invalid arguments error
    pub fn invalid_args(message: impl Into<String>, show_help: bool) -> Self {
        RgrepError::InvalidArgs {
            message: message.into(),
            show_help,
        }
    }

    /// Create an I/O error
    pub fn io_error(path: impl Into<String>, source: io::Error) -> Self {
        RgrepError::IoError {
            path: path.into(),
            source,
        }
    }

    /// Create a general application error
    pub fn app_error(message: impl Into<String>) -> Self {
        RgrepError::AppError {
            message: message.into(),
        }
    }

    /// Whether to show help text for this error
    pub fn should_show_help(&self) -> bool {
        matches!(
            self,
            RgrepError::InvalidArgs {
                show_help: true,
                ..
            }
        )
    }
}
