//! Tests specifically related to target handling (lib, bins, examples, tests, benches).

use payload_test_support::project;

#[payload_test]
fn reserved_windows_target_name() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [[bin]]
            name = "con"
            path = "src/main.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    if cfg!(windows) {
        p.payload("check")
            .with_stderr(
                "\
[WARNING] binary target `con` is a reserved Windows filename, \
this target will not work on Windows platforms
[CHECKING] foo[..]
[FINISHED][..]
",
            )
            .run();
    } else {
        p.payload("check")
            .with_stderr("[CHECKING] foo[..]\n[FINISHED][..]")
            .run();
    }
}
