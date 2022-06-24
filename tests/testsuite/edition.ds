//! Tests for edition setting.

use payload::core::Edition;
use payload_test_support::{basic_lib_manifest, is_nightly, project};

#[payload_test]
fn edition_works_for_build_script() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = 'foo'
                version = '0.1.0'
                edition = '2018'

                [build-dependencies]
                a = { path = 'a' }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() {
                    a::foo();
                }
            "#,
        )
        .file("a/Payload.toml", &basic_lib_manifest("a"))
        .file("a/src/lib.rs", "pub fn foo() {}")
        .build();

    p.payload("build -v").run();
}

#[payload_test]
fn edition_unstable_gated() {
    // During the period where a new edition is coming up, but not yet stable,
    // this test will verify that it cannot be used on stable. If there is no
    // next edition, it does nothing.
    let next = match Edition::LATEST_UNSTABLE {
        Some(next) => next,
        None => {
            eprintln!("Next edition is currently not available, skipping test.");
            return;
        }
    };
    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                [package]
                name = "foo"
                version = "0.1.0"
                edition = "{}"
            "#,
                next
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("check")
        .with_status(101)
        .with_stderr(&format!(
            "\
[ERROR] failed to parse manifest at `[..]/foo/Payload.toml`

Caused by:
  feature `edition{next}` is required

  this Payload does not support nightly features, but if you
  switch to nightly channel you can add
  `payload-features = [\"edition{next}\"]` to enable this feature
",
            next = next
        ))
        .run();
}

#[payload_test]
fn edition_unstable() {
    // During the period where a new edition is coming up, but not yet stable,
    // this test will verify that it can be used with `payload-features`. If
    // there is no next edition, it does nothing.
    if !is_nightly() {
        // This test is fundamentally always nightly.
        return;
    }
    let next = match Edition::LATEST_UNSTABLE {
        Some(next) => next,
        None => {
            eprintln!("Next edition is currently not available, skipping test.");
            return;
        }
    };
    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                payload-features = ["edition{next}"]

                [package]
                name = "foo"
                version = "0.1.0"
                edition = "{next}"
            "#,
                next = next
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("check")
        .masquerade_as_nightly_payload()
        .with_stderr(
            "\
[CHECKING] foo [..]
[FINISHED] [..]
",
        )
        .run();
}
