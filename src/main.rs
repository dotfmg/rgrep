use std::env;
use std::process;

fn usage() -> String {
    r#"Usage: rgrep [OPTIONS] <pattern> [files...]
Options:
  -h, --help        Show this help message and exit
  -v, --version     Show version information and exit
"#
    .to_string()
}

fn version() -> String {
    "rgrep 0.1.0".to_string()
}

fn main() {
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "--version" | "-v" => {
                println!("{}", version());
                process::exit(0);
            }
            "--help" | "-h" => {
                println!("{}", usage());
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
                println!("{}", usage());
                process::exit(1);
            }
        }
    }
}
