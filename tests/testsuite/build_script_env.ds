//! Tests for build.rs rerun-if-env-changed and rustc-env

use payload_test_support::project;
use payload_test_support::sleep_ms;

#[payload_test]
fn rerun_if_env_changes() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("payload:rerun-if-env-changed=FOO");
                }
            "#,
        )
        .build();

    p.payload("build")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.payload("build")
        .env("FOO", "bar")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.payload("build")
        .env("FOO", "baz")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.payload("build")
        .env("FOO", "baz")
        .with_stderr("[FINISHED] [..]")
        .run();
    p.payload("build")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
}

#[payload_test]
fn rerun_if_env_or_file_changes() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                fn main() {
                    println!("payload:rerun-if-env-changed=FOO");
                    println!("payload:rerun-if-changed=foo");
                }
            "#,
        )
        .file("foo", "")
        .build();

    p.payload("build")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.payload("build")
        .env("FOO", "bar")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
    p.payload("build")
        .env("FOO", "bar")
        .with_stderr("[FINISHED] [..]")
        .run();
    sleep_ms(1000);
    p.change_file("foo", "");
    p.payload("build")
        .env("FOO", "bar")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] [..]
",
        )
        .run();
}

#[payload_test]
fn rustc_bootstrap() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
            fn main() {
                println!("payload:rustc-env=RUSTC_BOOTSTRAP=1");
            }
        "#,
        )
        .build();
    p.payload("build")
        .with_stderr_contains("error: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .with_stderr_contains("help: [..] set the environment variable `RUSTC_BOOTSTRAP=foo` [..]")
        .with_status(101)
        .run();
    p.payload("build")
        .masquerade_as_nightly_payload()
        .with_stderr_contains("warning: Cannot set `RUSTC_BOOTSTRAP=1` [..]")
        .run();
}
