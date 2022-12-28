use super::*;


#[test]
fn test_simple_parse() {
    let args = Args::try_parse(vec!["web-monitor", "-f", "bla.yml"])
        .expect("Must succeed to parse");
    assert_eq!("bla.yml", args.file_path.to_str().unwrap());
    assert!(args.check_time_s.is_none());
}
