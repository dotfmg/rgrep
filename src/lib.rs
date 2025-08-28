use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Config {
    pub pattern: String,
    pub files: Vec<String>,
    pub ignore_case: bool,
    pub line_number: bool,
}

pub enum Action {
    ShowHelp,
    ShowVersion,
    Run(Config),
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub show_usage: bool,
}

impl Config {
    pub fn new(pattern: String, files: Vec<String>, ignore_case: bool, line_number: bool) -> Self {
        Self {
            pattern,
            files,
            ignore_case,
            line_number,
        }
    }
}

pub fn usage() -> String {
    r#"Usage: rgrep [OPTIONS] <pattern> [files...]
Options:
  -h, --help           Show this help message and exit
  -v, --version        Show version information and exit
  -i, --ignore-case    Case-insensitive matching
  -n, --line-number    Show line numbers
  --                   End of options (treat following args as files)
"#
    .to_string()
}

pub fn version() -> String {
    "rgrep 0.1.0".to_string()
}

pub fn parse_args<I>(args: I) -> Result<Action, ParseError>
where
    I: IntoIterator<Item = String>,
{
    let mut ignore_case = false;
    let mut line_number = false;
    let mut positionals: Vec<String> = Vec::new();
    let mut after_double_dash = false;

    for arg in args {
        if !after_double_dash {
            match arg.as_str() {
                "--help" | "-h" => return Ok(Action::ShowHelp),
                "--version" | "-v" => return Ok(Action::ShowVersion),
                "--ignore-case" | "-i" => {
                    ignore_case = true;
                    continue;
                }
                "--line-number" | "-n" => {
                    line_number = true;
                    continue;
                }
                "--" => {
                    after_double_dash = true;
                    continue;
                }
                _ => {
                    if arg.starts_with('-') {
                        return Err(ParseError {
                            message: format!("Unknown argument: {arg}"),
                            show_usage: true,
                        });
                    }
                }
            }
        }
        positionals.push(arg);
    }

    if positionals.is_empty() {
        return Err(ParseError {
            message: "Missing required <pattern>".to_string(),
            show_usage: true,
        });
    }

    let pattern = positionals[0].clone();
    let files = if positionals.len() > 1 {
        positionals[1..].to_vec()
    } else {
        Vec::new()
    };

    let cfg = Config::new(pattern, files, ignore_case, line_number);
    Ok(Action::Run(cfg))
}

pub fn run(cfg: Config) -> i32 {
    let multi_file = cfg.files.len() > 1;

    let pattern_norm = if cfg.ignore_case {
        cfg.pattern.to_lowercase()
    } else {
        cfg.pattern.clone()
    };

    let mut any_match = false;

    if cfg.files.is_empty() {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let matched = search_reader(reader, None, false, &cfg, &pattern_norm);
        if matched {
            any_match = true;
        }
    } else {
        for file in &cfg.files {
            match File::open(file) {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    let show_prefix = multi_file;
                    let matched = search_reader(
                        reader,
                        Some(file.as_str()),
                        show_prefix,
                        &cfg,
                        &pattern_norm,
                    );
                    if matched {
                        any_match = true;
                    }
                }
                Err(e) => {
                    eprintln!("rgrep: {}: {}", file, e);
                }
            }
        }
    }

    if any_match {
        0
    } else {
        1
    }
}

fn search_reader<R: BufRead>(
    reader: R,
    source_name: Option<&str>,
    show_prefix: bool,
    cfg: &Config,
    pattern_norm: &str,
) -> bool {
    let mut matched_any = false;

    for (idx, line_res) in reader.lines().enumerate() {
        let mut line = match line_res {
            Ok(s) => s,
            Err(_) => continue,
        };

        if line.ends_with('\r') {
            line.pop();
        }

        let hay = if cfg.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        if hay.contains(pattern_norm) {
            matched_any = true;

            let mut prefix = String::new();
            if show_prefix {
                if let Some(name) = source_name {
                    prefix.push_str(name);
                    prefix.push(':');
                }
            }
            if cfg.line_number {
                let n = idx + 1;
                prefix.push_str(&format!("{n}:"));
            }

            if prefix.is_empty() {
                println!("{}", line);
            } else {
                println!("{}{}", prefix, line);
            }
        }
    }

    matched_any
}
