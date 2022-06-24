//! Tests for workspace member discovery.

use payload::core::{Shell, Workspace};
use payload::util::config::Config;

use payload_test_support::install::payload_home;
use payload_test_support::project;
use payload_test_support::registry;

/// Tests exclusion of non-directory files from workspace member discovery using glob `*`.
#[payload_test]
fn bad_file_member_exclusion() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = [ "crates/*" ]
            "#,
        )
        .file("crates/.DS_Store", "PLACEHOLDER")
        .file(
            "crates/bar/Payload.toml",
            r#"
                [project]
                name = "bar"
                version = "0.1.0"
                authors = []
            "#,
        )
        .file("crates/bar/src/main.rs", "fn main() {}")
        .build();

    // Prevent this test from accessing the network by setting up .payload/config.
    registry::init();
    let config = Config::new(
        Shell::from_write(Box::new(Vec::new())),
        payload_home(),
        payload_home(),
    );
    let ws = Workspace::new(&p.root().join("Payload.toml"), &config).unwrap();
    assert_eq!(ws.members().count(), 1);
    assert_eq!(ws.members().next().unwrap().name(), "bar");
}
