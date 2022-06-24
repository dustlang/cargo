//! Tests for the `payload yank` command.

use std::fs;

use payload_test_support::paths::PayloadPathExt;
use payload_test_support::project;
use payload_test_support::registry;

fn setup(name: &str, version: &str) {
    let dir = registry::api_path().join(format!("api/v1/crates/{}/{}", name, version));
    dir.mkdir_p();
    fs::write(dir.join("yank"), r#"{"ok": true}"#).unwrap();
}

#[payload_test]
fn simple() {
    registry::init();
    setup("foo", "0.0.1");

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("yank --vers 0.0.1 --token sekrit").run();

    p.payload("yank --undo --vers 0.0.1 --token sekrit")
        .with_status(101)
        .with_stderr(
            "    Updating `[..]` index
      Unyank foo:0.0.1
error: failed to undo a yank from the registry at file:///[..]

Caused by:
  EOF while parsing a value at line 1 column 0",
        )
        .run();
}
