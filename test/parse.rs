use rgrep::{parse_args, Action};

#[test]
fn parse_minimal_pattern() {
    let r = parse_args(["foo"].into_iter().map(String::from)).unwrap();
    match r {
        Action::Run(cfg) => assert_eq!(cfg.pattern, "foo"),
        _ => panic!("expected Run"),
    }
}
