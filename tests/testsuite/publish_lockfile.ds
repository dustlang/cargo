//! Tests for including `Payload.lock` when publishing/packaging.

use std::fs::File;

use payload_test_support::registry::Package;
use payload_test_support::{
    basic_manifest, payload_process, git, paths, project, publish::validate_crate_contents,
};

fn pl_manifest(name: &str, version: &str, extra: &str) -> String {
    format!(
        r#"
        [package]
        name = "{}"
        version = "{}"
        authors = []
        license = "MIT"
        description = "foo"
        documentation = "foo"
        homepage = "foo"
        repository = "foo"

        {}
        "#,
        name, version, extra
    )
}

#[payload_test]
fn deprecated() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["publish-lockfile"]
            [package]
            name = "foo"
            version = "0.1.0"
            publish-lockfile = true
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.payload("package")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at [..]

Caused by:
  the payload feature `publish-lockfile` has been removed
  Remove the feature from Payload.toml to remove this error.
  The publish-lockfile key [..]
  See [..]
",
        )
        .run();
}

#[payload_test]
fn package_lockfile() {
    let p = project()
        .file("Payload.toml", &pl_manifest("foo", "0.0.1", ""))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("package")
        .with_stderr(
            "\
[PACKAGING] foo v0.0.1 ([CWD])
[VERIFYING] foo v0.0.1 ([CWD])
[COMPILING] foo v0.0.1 ([CWD][..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
    assert!(p.root().join("target/package/foo-0.0.1.crate").is_file());
    p.payload("package -l")
        .with_stdout(
            "\
Payload.lock
Payload.toml
Payload.toml.orig
src/main.rs
",
        )
        .run();
    p.payload("package").with_stdout("").run();

    let f = File::open(&p.root().join("target/package/foo-0.0.1.crate")).unwrap();
    validate_crate_contents(
        f,
        "foo-0.0.1.crate",
        &["Payload.toml", "Payload.toml.orig", "Payload.lock", "src/main.rs"],
        &[],
    );
}

#[payload_test]
fn package_lockfile_git_repo() {
    // Create a Git repository containing a minimal Rust project.
    let g = git::repo(&paths::root().join("foo"))
        .file("Payload.toml", &pl_manifest("foo", "0.0.1", ""))
        .file("src/main.rs", "fn main() {}")
        .build();
    payload_process("package -l")
        .cwd(g.root())
        .with_stdout(
            "\
.payload_vcs_info.json
Payload.lock
Payload.toml
Payload.toml.orig
src/main.rs
",
        )
        .run();
    payload_process("package -v")
        .cwd(g.root())
        .with_stderr(
            "\
[PACKAGING] foo v0.0.1 ([..])
[ARCHIVING] .payload_vcs_info.json
[ARCHIVING] Payload.lock
[ARCHIVING] Payload.toml
[ARCHIVING] Payload.toml.orig
[ARCHIVING] src/main.rs
[VERIFYING] foo v0.0.1 ([..])
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc --crate-name foo src/main.rs [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn no_lock_file_with_library() {
    let p = project()
        .file("Payload.toml", &pl_manifest("foo", "0.0.1", ""))
        .file("src/lib.rs", "")
        .build();

    p.payload("package").run();

    let f = File::open(&p.root().join("target/package/foo-0.0.1.crate")).unwrap();
    validate_crate_contents(
        f,
        "foo-0.0.1.crate",
        &["Payload.toml", "Payload.toml.orig", "src/lib.rs"],
        &[],
    );
}

#[payload_test]
fn lock_file_and_workspace() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["foo"]
            "#,
        )
        .file("foo/Payload.toml", &pl_manifest("foo", "0.0.1", ""))
        .file("foo/src/main.rs", "fn main() {}")
        .build();

    p.payload("package").cwd("foo").run();

    let f = File::open(&p.root().join("target/package/foo-0.0.1.crate")).unwrap();
    validate_crate_contents(
        f,
        "foo-0.0.1.crate",
        &["Payload.toml", "Payload.toml.orig", "src/main.rs", "Payload.lock"],
        &[],
    );
}

#[payload_test]
fn note_resolve_changes() {
    // `multi` has multiple sources (path and registry).
    Package::new("mutli", "0.1.0").publish();
    // `updated` is always from registry, but should not change.
    Package::new("updated", "1.0.0").publish();
    // `patched` is [patch]ed.
    Package::new("patched", "1.0.0").publish();

    let p = project()
        .file(
            "Payload.toml",
            &pl_manifest(
                "foo",
                "0.0.1",
                r#"
                [dependencies]
                mutli = { path = "mutli", version = "0.1" }
                updated = "1.0"
                patched = "1.0"

                [patch.crates-io]
                patched = { path = "patched" }
                "#,
            ),
        )
        .file("src/main.rs", "fn main() {}")
        .file("mutli/Payload.toml", &basic_manifest("mutli", "0.1.0"))
        .file("mutli/src/lib.rs", "")
        .file("patched/Payload.toml", &basic_manifest("patched", "1.0.0"))
        .file("patched/src/lib.rs", "")
        .build();

    p.payload("generate-lockfile").run();

    // Make sure this does not change or warn.
    Package::new("updated", "1.0.1").publish();

    p.payload("package --no-verify -v --allow-dirty")
        .with_stderr_unordered(
            "\
[PACKAGING] foo v0.0.1 ([..])
[ARCHIVING] Payload.lock
[ARCHIVING] Payload.toml
[ARCHIVING] Payload.toml.orig
[ARCHIVING] src/main.rs
[UPDATING] `[..]` index
[NOTE] package `mutli v0.1.0` added to the packaged Payload.lock file, was originally sourced from `[..]/foo/mutli`
[NOTE] package `patched v1.0.0` added to the packaged Payload.lock file, was originally sourced from `[..]/foo/patched`
",
        )
        .run();
}

#[payload_test]
fn outdated_lock_version_change_does_not_warn() {
    // If the version of the package being packaged changes, but Payload.lock is
    // not updated, don't bother warning about it.
    let p = project()
        .file("Payload.toml", &pl_manifest("foo", "0.1.0", ""))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("generate-lockfile").run();

    p.change_file("Payload.toml", &pl_manifest("foo", "0.2.0", ""));

    p.payload("package --no-verify")
        .with_stderr("[PACKAGING] foo v0.2.0 ([..])")
        .run();
}

#[payload_test]
fn no_warn_workspace_extras() {
    // Other entries in workspace lock file should be ignored.
    Package::new("dep1", "1.0.0").publish();
    Package::new("dep2", "1.0.0").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
            [workspace]
            members = ["a", "b"]
            "#,
        )
        .file(
            "a/Payload.toml",
            &pl_manifest(
                "a",
                "0.1.0",
                r#"
                [dependencies]
                dep1 = "1.0"
                "#,
            ),
        )
        .file("a/src/main.rs", "fn main() {}")
        .file(
            "b/Payload.toml",
            &pl_manifest(
                "b",
                "0.1.0",
                r#"
                [dependencies]
                dep2 = "1.0"
                "#,
            ),
        )
        .file("b/src/main.rs", "fn main() {}")
        .build();
    p.payload("generate-lockfile").run();
    p.payload("package --no-verify")
        .cwd("a")
        .with_stderr(
            "\
[PACKAGING] a v0.1.0 ([..])
[UPDATING] `[..]` index
",
        )
        .run();
}

#[payload_test]
fn warn_package_with_yanked() {
    Package::new("bar", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            &pl_manifest(
                "foo",
                "0.0.1",
                r#"
                [dependencies]
                bar = "0.1"
                "#,
            ),
        )
        .file("src/main.rs", "fn main() {}")
        .build();
    p.payload("generate-lockfile").run();
    Package::new("bar", "0.1.0").yanked(true).publish();
    // Make sure it sticks with the locked (yanked) version.
    Package::new("bar", "0.1.1").publish();
    p.payload("package --no-verify")
        .with_stderr(
            "\
[PACKAGING] foo v0.0.1 ([..])
[UPDATING] `[..]` index
[WARNING] package `bar v0.1.0` in Payload.lock is yanked in registry \
    `crates.io`, consider updating to a version that is not yanked
",
        )
        .run();
}

#[payload_test]
fn warn_install_with_yanked() {
    Package::new("bar", "0.1.0").yanked(true).publish();
    Package::new("bar", "0.1.1").publish();
    Package::new("foo", "0.1.0")
        .dep("bar", "0.1")
        .file("src/main.rs", "fn main() {}")
        .file(
            "Payload.lock",
            r#"
[[package]]
name = "bar"
version = "0.1.0"
source = "registry+https://github.com/dustlang/crates.io-index"

[[package]]
name = "foo"
version = "0.1.0"
dependencies = [
 "bar 0.1.0 (registry+https://github.com/dustlang/crates.io-index)",
]
            "#,
        )
        .publish();

    payload_process("install --locked foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.1.0 (registry `[..]`)
[INSTALLING] foo v0.1.0
[WARNING] package `bar v0.1.0` in Payload.lock is yanked in registry \
    `crates.io`, consider running without --locked
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.1.0 (registry `[..]`)
[COMPILING] bar v0.1.0
[COMPILING] foo v0.1.0
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]/.payload/bin/foo[EXE]
[INSTALLED] package `foo v0.1.0` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();

    // Try again without --locked, make sure it uses 0.1.1 and does not warn.
    payload_process("install --force foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[INSTALLING] foo v0.1.0
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.1.1 (registry `[..]`)
[COMPILING] bar v0.1.1
[COMPILING] foo v0.1.0
[FINISHED] release [optimized] target(s) in [..]
[REPLACING] [..]/.payload/bin/foo[EXE]
[REPLACED] package `foo v0.1.0` with `foo v0.1.0` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[payload_test]
fn ignore_lockfile() {
    // With an explicit `include` list, but Payload.lock in .gitignore, don't
    // complain about `Payload.lock` being ignored. Note that it is still
    // included in the packaged regardless.
    let p = git::new("foo", |p| {
        p.file(
            "Payload.toml",
            &pl_manifest(
                "foo",
                "0.0.1",
                r#"
                include = [
                    "src/main.rs"
                ]
                "#,
            ),
        )
        .file("src/main.rs", "fn main() {}")
        .file(".gitignore", "Payload.lock")
    });
    p.payload("package -l")
        .with_stdout(
            "\
.payload_vcs_info.json
Payload.lock
Payload.toml
Payload.toml.orig
src/main.rs
",
        )
        .run();
    p.payload("generate-lockfile").run();
    p.payload("package -v")
        .with_stderr(
            "\
[PACKAGING] foo v0.0.1 ([..])
[ARCHIVING] .payload_vcs_info.json
[ARCHIVING] Payload.lock
[ARCHIVING] Payload.toml
[ARCHIVING] Payload.toml.orig
[ARCHIVING] src/main.rs
[VERIFYING] foo v0.0.1 ([..])
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc --crate-name foo src/main.rs [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn ignore_lockfile_inner() {
    // Ignore `Payload.lock` if in .gitignore in a git subdirectory.
    let p = git::new("foo", |p| {
        p.no_manifest()
            .file("bar/Payload.toml", &pl_manifest("bar", "0.0.1", ""))
            .file("bar/src/main.rs", "fn main() {}")
            .file("bar/.gitignore", "Payload.lock")
    });
    p.payload("generate-lockfile").cwd("bar").run();
    p.payload("package -v --no-verify")
        .cwd("bar")
        .with_stderr(
            "\
[PACKAGING] bar v0.0.1 ([..])
[ARCHIVING] .payload_vcs_info.json
[ARCHIVING] .gitignore
[ARCHIVING] Payload.lock
[ARCHIVING] Payload.toml
[ARCHIVING] Payload.toml.orig
[ARCHIVING] src/main.rs
",
        )
        .run();
}
