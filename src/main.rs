use std::process;

use rgrep::app;

fn main() {
    let exit_code = app::run();
    process::exit(exit_code.into());
}
