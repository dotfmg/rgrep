use rgrep::{run, Config};

#[test]
fn run_no_match_exit_code() {
    let cfg = Config::new("xyz".into(), vec![], false, false);
    let code = run(cfg);
    assert_eq!(code, 1);
}
