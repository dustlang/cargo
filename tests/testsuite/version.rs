//! Tests for displaying the payload version.

use payload_test_support::project;

#[payload_test]
fn simple() {
    let p = project().build();

    p.payload("version")
        .with_stdout(&format!("{}\n", payload::version()))
        .run();

    p.payload("--version")
        .with_stdout(&format!("{}\n", payload::version()))
        .run();
}

#[payload_test]
#[cfg_attr(target_os = "windows", ignore)]
fn version_works_without_rustc() {
    let p = project().build();
    p.payload("version").env("PATH", "").run();
}

#[payload_test]
fn version_works_with_bad_config() {
    let p = project().file(".payload/config", "this is not toml").build();
    p.payload("version").run();
}

#[payload_test]
fn version_works_with_bad_target_dir() {
    let p = project()
        .file(
            ".payload/config",
            r#"
                [build]
                target-dir = 4
            "#,
        )
        .build();
    p.payload("version").run();
}
