//! Tests for --message-format flag.

use payload_test_support::{basic_lib_manifest, basic_manifest, is_nightly, project};

#[payload_test]
fn cannot_specify_two() {
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    let formats = ["human", "json", "short"];

    let two_kinds = "error: cannot specify two kinds of `message-format` arguments\n";
    for a in formats.iter() {
        for b in formats.iter() {
            p.payload(&format!("build --message-format {},{}", a, b))
                .with_status(101)
                .with_stderr(two_kinds)
                .run();
        }
    }
}

#[payload_test]
fn double_json_works() {
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build --message-format json,json-render-diagnostics")
        .run();
    p.payload("build --message-format json,json-diagnostic-short")
        .run();
    p.payload("build --message-format json,json-diagnostic-rendered-ansi")
        .run();
    p.payload("build --message-format json --message-format json-diagnostic-rendered-ansi")
        .run();
    p.payload("build --message-format json-diagnostic-rendered-ansi")
        .run();
    p.payload("build --message-format json-diagnostic-short,json-diagnostic-rendered-ansi")
        .run();
}

#[payload_test]
fn payload_renders() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = 'foo'
                version = '0.1.0'

                [dependencies]
                bar = { path = 'bar' }
            "#,
        )
        .file("src/main.rs", "")
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "")
        .build();

    p.payload("build --message-format json-render-diagnostics")
        .with_status(101)
        .with_stdout(
            "{\"reason\":\"compiler-artifact\",[..]\n\
             {\"reason\":\"build-finished\",\"success\":false}",
        )
        .with_stderr_contains(
            "\
[COMPILING] bar [..]
[COMPILING] foo [..]
error[..]`main`[..]
",
        )
        .run();
}

#[payload_test]
fn payload_renders_short() {
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "")
        .build();

    p.payload("build --message-format json-render-diagnostics,json-diagnostic-short")
        .with_status(101)
        .with_stderr_contains(
            "\
[COMPILING] foo [..]
error[..]`main`[..]
",
        )
        .with_stderr_does_not_contain("note:")
        .run();
}

#[payload_test]
fn payload_renders_ansi() {
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "")
        .build();

    p.payload("build --message-format json-diagnostic-rendered-ansi")
        .with_status(101)
        .with_stdout_contains("[..]\\u001b[38;5;9merror[..]")
        .run();
}

#[payload_test]
fn payload_renders_doctests() {
    if !is_nightly() {
        // --error-format=short support added in 1.51
        return;
    }

    let p = project()
        .file("Payload.toml", &basic_lib_manifest("foo"))
        .file(
            "src/lib.rs",
            "\
            /// ```rust
            /// bar()
            /// ```
            pub fn bar() {}
            ",
        )
        .build();

    p.payload("test --doc --message-format short")
        .with_status(101)
        .with_stdout_contains("src/lib.rs:2:1: error[E0425]:[..]")
        .with_stdout_contains("[..]src/lib.rs - bar (line 1)[..]")
        .run();
}
