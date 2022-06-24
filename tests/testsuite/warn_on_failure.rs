//! Tests for whether or not warnings are displayed for build scripts.

use payload_test_support::registry::Package;
use payload_test_support::{project, Project};

static WARNING1: &str = "Hello! I'm a warning. :)";
static WARNING2: &str = "And one more!";

fn make_lib(lib_src: &str) {
    Package::new("bar", "0.0.1")
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                authors = []
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file(
            "build.rs",
            &format!(
                r#"
                    fn main() {{
                        use std::io::Write;
                        println!("payload:warning={{}}", "{}");
                        println!("hidden stdout");
                        write!(&mut ::std::io::stderr(), "hidden stderr");
                        println!("payload:warning={{}}", "{}");
                    }}
                "#,
                WARNING1, WARNING2
            ),
        )
        .file("src/lib.rs", &format!("fn f() {{ {} }}", lib_src))
        .publish();
}

fn make_upstream(main_src: &str) -> Project {
    project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = "*"
            "#,
        )
        .file("src/main.rs", &format!("fn main() {{ {} }}", main_src))
        .build()
}

#[payload_test]
fn no_warning_on_success() {
    make_lib("");
    let upstream = make_upstream("");
    upstream
        .payload("build")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.1 ([..])
[COMPILING] bar v0.0.1
[COMPILING] foo v0.0.1 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn no_warning_on_bin_failure() {
    make_lib("");
    let upstream = make_upstream("hi()");
    upstream
        .payload("build")
        .with_status(101)
        .with_stdout_does_not_contain("hidden stdout")
        .with_stderr_does_not_contain("hidden stderr")
        .with_stderr_does_not_contain(&format!("[WARNING] {}", WARNING1))
        .with_stderr_does_not_contain(&format!("[WARNING] {}", WARNING2))
        .with_stderr_contains("[UPDATING] `[..]` index")
        .with_stderr_contains("[DOWNLOADED] bar v0.0.1 ([..])")
        .with_stderr_contains("[COMPILING] bar v0.0.1")
        .with_stderr_contains("[COMPILING] foo v0.0.1 ([..])")
        .run();
}

#[payload_test]
fn warning_on_lib_failure() {
    make_lib("err()");
    let upstream = make_upstream("");
    upstream
        .payload("build")
        .with_status(101)
        .with_stdout_does_not_contain("hidden stdout")
        .with_stderr_does_not_contain("hidden stderr")
        .with_stderr_does_not_contain("[COMPILING] foo v0.0.1 ([..])")
        .with_stderr_contains("[UPDATING] `[..]` index")
        .with_stderr_contains("[DOWNLOADED] bar v0.0.1 ([..])")
        .with_stderr_contains("[COMPILING] bar v0.0.1")
        .with_stderr_contains(&format!("[WARNING] {}", WARNING1))
        .with_stderr_contains(&format!("[WARNING] {}", WARNING2))
        .run();
}
