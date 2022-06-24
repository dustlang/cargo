//! Tests for `[alias]` config command aliases.

use payload_test_support::{basic_bin_manifest, project};

#[payload_test]
fn alias_incorrect_config_type() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
                [alias]
                b-payload-test = 5
            "#,
        )
        .build();

    p.payload("b-payload-test -v")
        .with_status(101)
        .with_stderr_contains(
            "\
[ERROR] invalid configuration for key `alias.b-payload-test`
expected a list, but found a integer for [..]",
        )
        .run();
}

#[payload_test]
fn alias_config() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
                [alias]
                b-payload-test = "build"
            "#,
        )
        .build();

    p.payload("b-payload-test -v")
        .with_stderr_contains(
            "\
[COMPILING] foo v0.5.0 [..]
[RUNNING] `rustc --crate-name foo [..]",
        )
        .run();
}

#[payload_test]
fn recursive_alias() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
                [alias]
                b-payload-test = "build"
                a-payload-test = ["b-payload-test", "-v"]
            "#,
        )
        .build();

    p.payload("a-payload-test")
        .with_stderr_contains(
            "\
[COMPILING] foo v0.5.0 [..]
[RUNNING] `rustc --crate-name foo [..]",
        )
        .run();
}

#[payload_test]
fn alias_list_test() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
               [alias]
               b-payload-test = ["build", "--release"]
            "#,
        )
        .build();

    p.payload("b-payload-test -v")
        .with_stderr_contains("[COMPILING] foo v0.5.0 [..]")
        .with_stderr_contains("[RUNNING] `rustc --crate-name [..]")
        .run();
}

#[payload_test]
fn alias_with_flags_config() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
               [alias]
               b-payload-test = "build --release"
            "#,
        )
        .build();

    p.payload("b-payload-test -v")
        .with_stderr_contains("[COMPILING] foo v0.5.0 [..]")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]")
        .run();
}

#[payload_test]
fn alias_cannot_shadow_builtin_command() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
               [alias]
               build = "fetch"
            "#,
        )
        .build();

    p.payload("build")
        .with_stderr(
            "\
[WARNING] user-defined alias `build` is ignored, because it is shadowed by a built-in command
[COMPILING] foo v0.5.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn alias_override_builtin_alias() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
               [alias]
               b = "run"
            "#,
        )
        .build();

    p.payload("b")
        .with_stderr(
            "\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] `target/debug/foo[EXE]`
",
        )
        .run();
}

#[payload_test]
fn builtin_alias_takes_options() {
    // #6381
    let p = project()
        .file("src/lib.rs", "")
        .file(
            "examples/ex1.rs",
            r#"fn main() { println!("{}", std::env::args().skip(1).next().unwrap()) }"#,
        )
        .build();

    p.payload("r --example ex1 -- asdf").with_stdout("asdf").run();
}

#[payload_test]
fn global_options_with_alias() {
    // Check that global options are passed through.
    let p = project().file("src/lib.rs", "").build();

    p.payload("-v c")
        .with_stderr(
            "\
[CHECKING] foo [..]
[RUNNING] `rustc [..]
[FINISHED] dev [..]
",
        )
        .run();
}
