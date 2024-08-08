use std::fs;

use assert_cmd::Command;
use insta::{assert_debug_snapshot, glob};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TestCmd {
    cmd: String,
    args: Option<Vec<String>>,
}

#[test]
fn test_snapshots() {
    glob!("cli/inputs/*.toml", |path| {
        let input = fs::read_to_string(path).unwrap();
        let test_cmd: TestCmd = toml::from_str(&input).unwrap();
        let assert = Command::cargo_bin(test_cmd.cmd)
            .unwrap()
            .args(test_cmd.args.unwrap_or_default().as_slice())
            .assert();
        assert_debug_snapshot!(assert.get_output());
    });
}
