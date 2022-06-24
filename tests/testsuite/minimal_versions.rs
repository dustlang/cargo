//! Tests for minimal-version resolution.
//!
//! Note: Some tests are located in the resolver-tests package.

use payload_test_support::project;
use payload_test_support::registry::Package;

// Ensure that the "-Z minimal-versions" CLI option works and the minimal
// version of a dependency ends up in the lock file.
#[payload_test]
fn minimal_version_cli() {
    Package::new("dep", "1.0.0").publish();
    Package::new("dep", "1.1.0").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                authors = []
                version = "0.0.1"

                [dependencies]
                dep = "1.0"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("generate-lockfile -Zminimal-versions")
        .masquerade_as_nightly_payload()
        .run();

    let lock = p.read_lockfile();

    assert!(!lock.contains("1.1.0"));
}
