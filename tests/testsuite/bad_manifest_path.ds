//! Tests for invalid --manifest-path arguments.

use payload_test_support::{basic_bin_manifest, main_file, project};

#[track_caller]
fn assert_not_a_payload_toml(command: &str, manifest_path_argument: &str) {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload(command)
        .arg("--manifest-path")
        .arg(manifest_path_argument)
        .cwd(p.root().parent().unwrap())
        .with_status(101)
        .with_stderr(
            "[ERROR] the manifest-path must be a path \
             to a Payload.toml file",
        )
        .run();
}

#[track_caller]
fn assert_payload_toml_doesnt_exist(command: &str, manifest_path_argument: &str) {
    let p = project().build();
    let expected_path = manifest_path_argument
        .split('/')
        .collect::<Vec<_>>()
        .join("[..]");

    p.payload(command)
        .arg("--manifest-path")
        .arg(manifest_path_argument)
        .cwd(p.root().parent().unwrap())
        .with_status(101)
        .with_stderr(format!(
            "[ERROR] manifest path `{}` does not exist",
            expected_path
        ))
        .run();
}

#[payload_test]
fn bench_dir_containing_payload_toml() {
    assert_not_a_payload_toml("bench", "foo");
}

#[payload_test]
fn bench_dir_plus_file() {
    assert_not_a_payload_toml("bench", "foo/bar");
}

#[payload_test]
fn bench_dir_plus_path() {
    assert_not_a_payload_toml("bench", "foo/bar/baz");
}

#[payload_test]
fn bench_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("bench", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn build_dir_containing_payload_toml() {
    assert_not_a_payload_toml("build", "foo");
}

#[payload_test]
fn build_dir_plus_file() {
    assert_not_a_payload_toml("bench", "foo/bar");
}

#[payload_test]
fn build_dir_plus_path() {
    assert_not_a_payload_toml("bench", "foo/bar/baz");
}

#[payload_test]
fn build_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("build", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn clean_dir_containing_payload_toml() {
    assert_not_a_payload_toml("clean", "foo");
}

#[payload_test]
fn clean_dir_plus_file() {
    assert_not_a_payload_toml("clean", "foo/bar");
}

#[payload_test]
fn clean_dir_plus_path() {
    assert_not_a_payload_toml("clean", "foo/bar/baz");
}

#[payload_test]
fn clean_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("clean", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn doc_dir_containing_payload_toml() {
    assert_not_a_payload_toml("doc", "foo");
}

#[payload_test]
fn doc_dir_plus_file() {
    assert_not_a_payload_toml("doc", "foo/bar");
}

#[payload_test]
fn doc_dir_plus_path() {
    assert_not_a_payload_toml("doc", "foo/bar/baz");
}

#[payload_test]
fn doc_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("doc", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn fetch_dir_containing_payload_toml() {
    assert_not_a_payload_toml("fetch", "foo");
}

#[payload_test]
fn fetch_dir_plus_file() {
    assert_not_a_payload_toml("fetch", "foo/bar");
}

#[payload_test]
fn fetch_dir_plus_path() {
    assert_not_a_payload_toml("fetch", "foo/bar/baz");
}

#[payload_test]
fn fetch_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("fetch", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn generate_lockfile_dir_containing_payload_toml() {
    assert_not_a_payload_toml("generate-lockfile", "foo");
}

#[payload_test]
fn generate_lockfile_dir_plus_file() {
    assert_not_a_payload_toml("generate-lockfile", "foo/bar");
}

#[payload_test]
fn generate_lockfile_dir_plus_path() {
    assert_not_a_payload_toml("generate-lockfile", "foo/bar/baz");
}

#[payload_test]
fn generate_lockfile_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("generate-lockfile", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn package_dir_containing_payload_toml() {
    assert_not_a_payload_toml("package", "foo");
}

#[payload_test]
fn package_dir_plus_file() {
    assert_not_a_payload_toml("package", "foo/bar");
}

#[payload_test]
fn package_dir_plus_path() {
    assert_not_a_payload_toml("package", "foo/bar/baz");
}

#[payload_test]
fn package_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("package", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn pkgid_dir_containing_payload_toml() {
    assert_not_a_payload_toml("pkgid", "foo");
}

#[payload_test]
fn pkgid_dir_plus_file() {
    assert_not_a_payload_toml("pkgid", "foo/bar");
}

#[payload_test]
fn pkgid_dir_plus_path() {
    assert_not_a_payload_toml("pkgid", "foo/bar/baz");
}

#[payload_test]
fn pkgid_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("pkgid", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn publish_dir_containing_payload_toml() {
    assert_not_a_payload_toml("publish", "foo");
}

#[payload_test]
fn publish_dir_plus_file() {
    assert_not_a_payload_toml("publish", "foo/bar");
}

#[payload_test]
fn publish_dir_plus_path() {
    assert_not_a_payload_toml("publish", "foo/bar/baz");
}

#[payload_test]
fn publish_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("publish", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn read_manifest_dir_containing_payload_toml() {
    assert_not_a_payload_toml("read-manifest", "foo");
}

#[payload_test]
fn read_manifest_dir_plus_file() {
    assert_not_a_payload_toml("read-manifest", "foo/bar");
}

#[payload_test]
fn read_manifest_dir_plus_path() {
    assert_not_a_payload_toml("read-manifest", "foo/bar/baz");
}

#[payload_test]
fn read_manifest_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("read-manifest", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn run_dir_containing_payload_toml() {
    assert_not_a_payload_toml("run", "foo");
}

#[payload_test]
fn run_dir_plus_file() {
    assert_not_a_payload_toml("run", "foo/bar");
}

#[payload_test]
fn run_dir_plus_path() {
    assert_not_a_payload_toml("run", "foo/bar/baz");
}

#[payload_test]
fn run_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("run", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn rustc_dir_containing_payload_toml() {
    assert_not_a_payload_toml("rustc", "foo");
}

#[payload_test]
fn rustc_dir_plus_file() {
    assert_not_a_payload_toml("rustc", "foo/bar");
}

#[payload_test]
fn rustc_dir_plus_path() {
    assert_not_a_payload_toml("rustc", "foo/bar/baz");
}

#[payload_test]
fn rustc_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("rustc", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn test_dir_containing_payload_toml() {
    assert_not_a_payload_toml("test", "foo");
}

#[payload_test]
fn test_dir_plus_file() {
    assert_not_a_payload_toml("test", "foo/bar");
}

#[payload_test]
fn test_dir_plus_path() {
    assert_not_a_payload_toml("test", "foo/bar/baz");
}

#[payload_test]
fn test_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("test", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn update_dir_containing_payload_toml() {
    assert_not_a_payload_toml("update", "foo");
}

#[payload_test]
fn update_dir_plus_file() {
    assert_not_a_payload_toml("update", "foo/bar");
}

#[payload_test]
fn update_dir_plus_path() {
    assert_not_a_payload_toml("update", "foo/bar/baz");
}

#[payload_test]
fn update_dir_to_nonexistent_payload_toml() {
    assert_payload_toml_doesnt_exist("update", "foo/bar/baz/Payload.toml");
}

#[payload_test]
fn verify_project_dir_containing_payload_toml() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload("verify-project --manifest-path foo")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"the manifest-path must be a path to a Payload.toml file\"}\
             ",
        )
        .run();
}

#[payload_test]
fn verify_project_dir_plus_file() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload("verify-project --manifest-path foo/bar")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"the manifest-path must be a path to a Payload.toml file\"}\
             ",
        )
        .run();
}

#[payload_test]
fn verify_project_dir_plus_path() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload("verify-project --manifest-path foo/bar/baz")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"the manifest-path must be a path to a Payload.toml file\"}\
             ",
        )
        .run();
}

#[payload_test]
fn verify_project_dir_to_nonexistent_payload_toml() {
    let p = project().build();
    p.payload("verify-project --manifest-path foo/bar/baz/Payload.toml")
        .cwd(p.root().parent().unwrap())
        .with_status(1)
        .with_stdout(
            "{\"invalid\":\"manifest path `foo[..]bar[..]baz[..]Payload.toml` does not exist\"}\
             ",
        )
        .run();
}
