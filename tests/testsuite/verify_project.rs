//! Tests for the `payload verify-project` command.

use payload_test_support::{basic_bin_manifest, main_file, project};

fn verify_project_success_output() -> String {
    r#"{"success":"true"}"#.into()
}

#[payload_test]
fn payload_verify_project_path_to_payload_toml_relative() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload("verify-project --manifest-path foo/Payload.toml")
        .cwd(p.root().parent().unwrap())
        .with_stdout(verify_project_success_output())
        .run();
}

#[payload_test]
fn payload_verify_project_path_to_payload_toml_absolute() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload("verify-project --manifest-path")
        .arg(p.root().join("Payload.toml"))
        .cwd(p.root().parent().unwrap())
        .with_stdout(verify_project_success_output())
        .run();
}

#[payload_test]
fn payload_verify_project_cwd() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload("verify-project")
        .with_stdout(verify_project_success_output())
        .run();
}

#[payload_test]
fn payload_verify_project_honours_unstable_features() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["test-dummy-unstable"]

                [package]
                name = "foo"
                version = "0.0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("verify-project")
        .masquerade_as_nightly_payload()
        .with_stdout(verify_project_success_output())
        .run();

    p.payload("verify-project")
        .with_status(1)
        .with_stdout(r#"{"invalid":"failed to parse manifest at `[CWD]/Payload.toml`"}"#)
        .run();
}
