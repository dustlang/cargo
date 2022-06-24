//! Tests for setting custom rustdoc flags.

use payload_test_support::project;

#[payload_test]
fn parses_env() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("doc -v")
        .env("RUSTDOCFLAGS", "--cfg=foo")
        .with_stderr_contains("[RUNNING] `rustdoc [..] --cfg=foo[..]`")
        .run();
}

#[payload_test]
fn parses_config() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [build]
                rustdocflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.payload("doc -v")
        .with_stderr_contains("[RUNNING] `rustdoc [..] --cfg foo[..]`")
        .run();
}

#[payload_test]
fn bad_flags() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("doc")
        .env("RUSTDOCFLAGS", "--bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn rerun() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("doc").env("RUSTDOCFLAGS", "--cfg=foo").run();
    p.payload("doc")
        .env("RUSTDOCFLAGS", "--cfg=foo")
        .with_stderr("[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]")
        .run();
    p.payload("doc")
        .env("RUSTDOCFLAGS", "--cfg=bar")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustdocflags_passed_to_rustdoc_through_payload_test() {
    let p = project()
        .file(
            "src/lib.rs",
            r#"
                //! ```
                //! assert!(cfg!(do_not_choke));
                //! ```
            "#,
        )
        .build();

    p.payload("test --doc")
        .env("RUSTDOCFLAGS", "--cfg do_not_choke")
        .run();
}

#[payload_test]
fn rustdocflags_passed_to_rustdoc_through_payload_test_only_once() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("test --doc")
        .env("RUSTDOCFLAGS", "--markdown-no-toc")
        .run();
}

#[payload_test]
fn rustdocflags_misspelled() {
    let p = project().file("src/main.rs", "fn main() { }").build();

    p.payload("doc")
        .env("RUSTDOC_FLAGS", "foo")
        .with_stderr_contains("[WARNING] Payload does not read `RUSTDOC_FLAGS` environment variable. Did you mean `RUSTDOCFLAGS`?")
        .run();
}

#[payload_test]
fn whitespace() {
    // Checks behavior of different whitespace characters.
    let p = project().file("src/lib.rs", "").build();

    // "too many operands"
    p.payload("doc")
        .env("RUSTDOCFLAGS", "--crate-version this has spaces")
        .with_stderr_contains("[ERROR] could not document `foo`")
        .with_status(101)
        .run();

    const SPACED_VERSION: &str = "a\nb\tc\u{00a0}d";
    p.payload("doc")
        .env(
            "RUSTDOCFLAGS",
            format!("--crate-version {}", SPACED_VERSION),
        )
        .run();

    let contents = p.read_file("target/doc/foo/index.html");
    assert!(contents.contains(SPACED_VERSION));
}
