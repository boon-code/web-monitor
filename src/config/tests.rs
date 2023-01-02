use super::*;
use crate::testutils::get_path;

fn load_cfg(filename: &str) -> Config {
    Config::from_yaml_file(get_path(filename))
        .unwrap()
}

#[test]
fn test_simple_parse() {
    let cfg = load_cfg("example_only_url.yml");
    assert_eq!(cfg.websites.len(), 1);
    let w = &cfg.websites["google"];
    assert_eq!(w.url, "https://www.google.com");
    assert!(w.interval.is_none());
    assert_eq!(cfg.global.default_interval, 60_u64);
}

#[test]
fn test_global_default_is_used_with_empty_globals() {
    let cfg = load_cfg("example_only_url_empty_globals.yml");
    assert_eq!(cfg.global.default_interval, 60_u64);
}

#[test]
fn test_unsupported_globals_are_ignored() {
    let cfg = load_cfg("example_only_url_unsupported_globals.yml");
    assert_eq!(cfg.global.default_interval, 60_u64);
}

#[test]
fn test_cmd_notifier() {
    let cfg = load_cfg("example_notifier.yml");
    assert_eq!(cfg.notifiers.len(), 1);
    let n = &cfg.notifiers["shell"];
    let c = n.as_command().unwrap();

    assert_eq!(c.cmd, "echo");
}
