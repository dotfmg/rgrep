use rgrep::cli::{parse_args, CliAction};

#[test]
fn parse_minimal_pattern() {
    let r = parse_args(["foo"].into_iter().map(String::from)).unwrap();
    match r {
        CliAction::Run(cfg) => assert_eq!(cfg.pattern, "foo"),
        _ => panic!("expected Run"),
    }
}

#[test]
fn parse_with_flags() {
    let r = parse_args(
        ["-i", "-n", "pattern", "file.txt"]
            .into_iter()
            .map(String::from),
    )
    .unwrap();
    match r {
        CliAction::Run(cfg) => {
            assert_eq!(cfg.pattern, "pattern");
            assert!(cfg.ignore_case);
            assert!(cfg.line_number);
            assert_eq!(cfg.files, vec!["file.txt"]);
        }
        _ => panic!("expected Run"),
    }
}

#[test]
fn parse_help() {
    let r = parse_args(["--help"].into_iter().map(String::from)).unwrap();
    matches!(r, CliAction::ShowHelp);
}

#[test]
fn parse_version() {
    let r = parse_args(["--version"].into_iter().map(String::from)).unwrap();
    matches!(r, CliAction::ShowVersion);
}

#[test]
fn parse_stdin_explicit() {
    let r = parse_args(["pattern", "-"].into_iter().map(String::from)).unwrap();
    match r {
        CliAction::Run(cfg) => {
            assert_eq!(cfg.pattern, "pattern");
            assert!(cfg.use_stdin());
        }
        _ => panic!("expected Run"),
    }
}

#[test]
fn parse_multiple_files() {
    let r = parse_args(
        ["pattern", "file1.txt", "file2.txt"]
            .into_iter()
            .map(String::from),
    )
    .unwrap();
    match r {
        CliAction::Run(cfg) => {
            assert_eq!(cfg.pattern, "pattern");
            assert!(!cfg.use_stdin());
            assert_eq!(cfg.actual_files(), vec!["file1.txt", "file2.txt"]);
        }
        _ => panic!("expected Run"),
    }
}

#[test]
fn parse_invalid_mixing_stdin_and_files() {
    let r = parse_args(["pattern", "-", "file.txt"].into_iter().map(String::from));
    assert!(r.is_err());
}
