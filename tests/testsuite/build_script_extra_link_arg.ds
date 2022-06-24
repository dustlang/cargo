//! Tests for -Zextra-link-arg.

use payload_test_support::{basic_bin_manifest, project};

#[payload_test]
fn build_script_extra_link_arg_bin() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("payload:rustc-link-arg-bins=--this-is-a-bogus-flag");
                }
            "#,
        )
        .build();

    p.payload("build -Zextra-link-arg -v")
        .masquerade_as_nightly_payload()
        .without_status()
        .with_stderr_contains(
            "[RUNNING] `rustc --crate-name foo [..]-C link-arg=--this-is-a-bogus-flag[..]",
        )
        .run();
}

#[payload_test]
fn build_script_extra_link_arg() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("payload:rustc-link-arg=--this-is-a-bogus-flag");
                }
            "#,
        )
        .build();

    p.payload("build -Zextra-link-arg -v")
        .masquerade_as_nightly_payload()
        .without_status()
        .with_stderr_contains(
            "[RUNNING] `rustc --crate-name foo [..]-C link-arg=--this-is-a-bogus-flag[..]",
        )
        .run();
}

#[payload_test]
fn build_script_extra_link_arg_warn_without_flag() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("payload:rustc-link-arg=--this-is-a-bogus-flag");
                }
            "#,
        )
        .build();

    p.payload("build -v")
        .with_status(0)
        .with_stderr_contains("warning: payload:rustc-link-arg requires -Zextra-link-arg flag")
        .run();
}
