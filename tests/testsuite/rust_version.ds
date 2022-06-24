//! Tests for targets with `rust-version`.

use payload_test_support::{project, registry::Package};

#[payload_test]
fn rust_version_gated() {
    project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                rust-version = "1.9999"
            "#,
        )
        .file("src/lib.rs", "")
        .build()
        .payload("build")
        .masquerade_as_nightly_payload()
        .with_stderr_contains(
            "warning: `rust-version` is not supported on this version of Payload and will be ignored\
            \n\nconsider adding `payload-features = [\"rust-version\"]` to the manifest",
        )
        .run();

    project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                rust-version = "1.9999"
            "#,
        )
        .file("src/lib.rs", "")
        .build()
        .payload("build")
        .with_stderr_contains(
            "warning: `rust-version` is not supported on this version of Payload and will be ignored\
            \n\nthis Payload does not support nightly features, but if you\n\
            switch to nightly channel you can add\n\
            `payload-features = [\"rust-version\"]` to enable this feature",
        )
        .run();
}

#[payload_test]
fn rust_version_satisfied() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["rust-version"]

            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.1.1"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build").masquerade_as_nightly_payload().run();
    p.payload("build --ignore-rust-version -Zunstable-options")
        .masquerade_as_nightly_payload()
        .run();
}

#[payload_test]
fn rust_version_bad_caret() {
    project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["rust-version"]

            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "^1.43"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .payload("build")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "error: failed to parse manifest at `[..]`\n\n\
             Caused by:\n  `rust-version` must be a value like \"1.32\"",
        )
        .run();
}

#[payload_test]
fn rust_version_bad_pre_release() {
    project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["rust-version"]

            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.43-beta.1"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .payload("build")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "error: failed to parse manifest at `[..]`\n\n\
             Caused by:\n  `rust-version` must be a value like \"1.32\"",
        )
        .run();
}

#[payload_test]
fn rust_version_bad_nonsense() {
    project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["rust-version"]

            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "foodaddle"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .payload("build")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "error: failed to parse manifest at `[..]`\n\n\
             Caused by:\n  `rust-version` must be a value like \"1.32\"",
        )
        .run();
}

#[payload_test]
fn rust_version_too_high() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["rust-version"]

            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.9876.0"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "error: package `foo v0.0.1 ([..])` cannot be built because it requires \
             rustc 1.9876.0 or newer, while the currently active rustc version is [..]",
        )
        .run();
    p.payload("build --ignore-rust-version -Zunstable-options")
        .masquerade_as_nightly_payload()
        .run();
}

#[payload_test]
fn rust_version_dependency_fails() {
    Package::new("bar", "0.0.1")
        .payload_feature("rust-version")
        .rust_version("1.2345.0")
        .file("src/lib.rs", "fn other_stuff() {}")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            [dependencies]
            bar = "0.0.1"
        "#,
        )
        .file("src/main.rs", "fn main(){}")
        .build();

    p.payload("build")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "    Updating `[..]` index\n \
             Downloading crates ...\n  \
             Downloaded bar v0.0.1 (registry `[..]`)\n\
             error: package `bar v0.0.1` cannot be built because it requires \
             rustc 1.2345.0 or newer, while the currently active rustc version is [..]",
        )
        .run();
    p.payload("build --ignore-rust-version -Zunstable-options")
        .masquerade_as_nightly_payload()
        .run();
}

#[payload_test]
fn rust_version_older_than_edition() {
    project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["rust-version"]

            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            rust-version = "1.1"
            edition = "2018"
            [[bin]]
            name = "foo"
        "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build()
        .payload("build")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr_contains("  rust-version 1.1 is older than first version (1.31.0) required by the specified edition (2018)",
        )
        .run();
}
