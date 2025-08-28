use std::env;
use std::process;

use rgrep::{parse_args, run, usage, version, Action};

fn main() {
    let args = env::args().skip(1);

    match parse_args(args) {
        Ok(Action::ShowHelp) => {
            println!("{}", usage());
            process::exit(0);
        }
        Ok(Action::ShowVersion) => {
            println!("{}", version());
            process::exit(0);
        }
        Ok(Action::Run(cfg)) => {
            let code = run(cfg);
            process::exit(code);
        }
        Err(err) => {
            eprintln!("{}", err.message);
            if err.show_usage {
                eprintln!("{}", usage());
            }
            process::exit(2);
        }
    }
}
