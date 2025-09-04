//! I/O operations and file handling

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::errors::RgrepError;

/// Input source with its name and line iterator
pub struct InputSource {
    /// Source name for display (None for stdin)
    pub name: Option<String>,
    /// Iterator over lines
    pub lines: Box<dyn Iterator<Item = io::Result<String>>>,
}

impl InputSource {
    /// Create input source from stdin
    pub fn stdin() -> Self {
        let stdin = io::stdin();
        let reader = stdin.lock();
        Self {
            name: None,
            lines: Box::new(normalize_lines(reader.lines())),
        }
    }

    /// Create input source from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, RgrepError> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let file = File::open(&path).map_err(|e| RgrepError::io_error(&path_str, e))?;
        let reader = BufReader::new(file);

        Ok(Self {
            name: Some(path_str),
            lines: Box::new(normalize_lines(reader.lines())),
        })
    }
}

/// Create appropriate input source based on file path
pub fn create_input_source(file_path: Option<&str>) -> Result<InputSource, RgrepError> {
    match file_path {
        None | Some("-") => Ok(InputSource::stdin()),
        Some(path) => InputSource::from_file(path),
    }
}

/// Normalize line endings (remove trailing \r)
fn normalize_lines<I>(lines: I) -> impl Iterator<Item = io::Result<String>>
where
    I: Iterator<Item = io::Result<String>>,
{
    lines.map(|line_result| {
        line_result.map(|mut line| {
            if line.ends_with('\r') {
                line.pop();
            }
            line
        })
    })
}
