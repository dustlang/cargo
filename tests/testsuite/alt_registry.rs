//! Tests for alternative registries.

use payload::util::IntoUrl;
use payload_test_support::publish::validate_alt_upload;
use payload_test_support::registry::{self, Package};
use payload_test_support::{basic_manifest, git, paths, project};
use std::fs;

#[payload_test]
fn depend_on_alt_registry() {
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.0.1").alternative(true).publish();

    p.payload("build")
        .with_stderr(&format!(
            "\
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
            reg = registry::alt_registry_path().to_str().unwrap()
        ))
        .run();

    p.payload("clean").run();

    // Don't download a second time
    p.payload("build")
        .with_stderr(
            "\
[COMPILING] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
        )
        .run();
}

#[payload_test]
fn depend_on_alt_registry_depends_on_same_registry_no_index() {
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("baz", "0.0.1").alternative(true).publish();
    Package::new("bar", "0.0.1")
        .registry_dep("baz", "0.0.1")
        .alternative(true)
        .publish();

    p.payload("build")
        .with_stderr(&format!(
            "\
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] [..] v0.0.1 (registry `[ROOT][..]`)
[DOWNLOADED] [..] v0.0.1 (registry `[ROOT][..]`)
[COMPILING] baz v0.0.1 (registry `[ROOT][..]`)
[COMPILING] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
            reg = registry::alt_registry_path().to_str().unwrap()
        ))
        .run();
}

#[payload_test]
fn depend_on_alt_registry_depends_on_same_registry() {
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("baz", "0.0.1").alternative(true).publish();
    Package::new("bar", "0.0.1")
        .registry_dep("baz", "0.0.1")
        .alternative(true)
        .publish();

    p.payload("build")
        .with_stderr(&format!(
            "\
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] [..] v0.0.1 (registry `[ROOT][..]`)
[DOWNLOADED] [..] v0.0.1 (registry `[ROOT][..]`)
[COMPILING] baz v0.0.1 (registry `[ROOT][..]`)
[COMPILING] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
            reg = registry::alt_registry_path().to_str().unwrap()
        ))
        .run();
}

#[payload_test]
fn depend_on_alt_registry_depends_on_crates_io() {
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("baz", "0.0.1").publish();
    Package::new("bar", "0.0.1")
        .dep("baz", "0.0.1")
        .alternative(true)
        .publish();

    p.payload("build")
        .with_stderr_unordered(&format!(
            "\
[UPDATING] `{alt_reg}` index
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] baz v0.0.1 (registry `[ROOT][..]`)
[DOWNLOADED] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] baz v0.0.1
[COMPILING] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
            alt_reg = registry::alt_registry_path().to_str().unwrap(),
            reg = registry::registry_path().to_str().unwrap()
        ))
        .run();
}

#[payload_test]
fn registry_and_path_dep_works() {
    registry::alt_init();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "bar"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Payload.toml", &basic_manifest("bar", "0.0.1"))
        .file("bar/src/lib.rs", "")
        .build();

    p.payload("build")
        .with_stderr(
            "\
[COMPILING] bar v0.0.1 ([CWD]/bar)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
        )
        .run();
}

#[payload_test]
fn registry_incompatible_with_git() {
    registry::alt_init();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                git = ""
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr_contains(
            "  dependency (bar) specification is ambiguous. \
             Only one of `git` or `registry` is allowed.",
        )
        .run();
}

#[payload_test]
fn cannot_publish_to_crates_io_with_registry_dependency() {
    registry::alt_init();
    let fakeio_path = paths::root().join("fake.io");
    let fakeio_url = fakeio_path.into_url().unwrap();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []
                [dependencies.bar]
                version = "0.0.1"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            &format!(
                r#"
                    [registries.fakeio]
                    index = "{}"
                "#,
                fakeio_url
            ),
        )
        .build();

    Package::new("bar", "0.0.1").alternative(true).publish();

    // Since this can't really call plain `publish` without fetching the real
    // crates.io index, create a fake one that points to the real crates.io.
    git::repo(&fakeio_path)
        .file(
            "config.json",
            r#"
                {"dl": "https://crates.io/api/v1/crates", "api": "https://crates.io"}
            "#,
        )
        .build();

    // Login so that we have the token available
    p.payload("login --registry fakeio TOKEN").run();

    p.payload("publish --registry fakeio")
        .with_status(101)
        .with_stderr_contains("[ERROR] crates cannot be published to crates.io[..]")
        .run();

    p.payload("publish --token sekrit --index")
        .arg(fakeio_url.to_string())
        .with_status(101)
        .with_stderr_contains("[ERROR] crates cannot be published to crates.io[..]")
        .run();
}

#[payload_test]
fn publish_with_registry_dependency() {
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.0.1").alternative(true).publish();

    // Login so that we have the token available
    p.payload("login --registry alternative TOKEN").run();

    p.payload("publish --registry alternative").run();

    validate_alt_upload(
        r#"{
            "authors": [],
            "badges": {},
            "categories": [],
            "deps": [
              {
                "default_features": true,
                "features": [],
                "kind": "normal",
                "name": "bar",
                "optional": false,
                "target": null,
                "version_req": "^0.0.1"
              }
            ],
            "description": null,
            "documentation": null,
            "features": {},
            "homepage": null,
            "keywords": [],
            "license": null,
            "license_file": null,
            "links": null,
            "name": "foo",
            "readme": null,
            "readme_file": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "vers": "0.0.1"
        }"#,
        "foo-0.0.1.crate",
        &["Payload.lock", "Payload.toml", "Payload.toml.orig", "src/main.rs"],
    );
}

#[payload_test]
fn alt_registry_and_crates_io_deps() {
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                crates_io_dep = "0.0.1"

                [dependencies.alt_reg_dep]
                version = "0.1.0"
                registry = "alternative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("crates_io_dep", "0.0.1").publish();
    Package::new("alt_reg_dep", "0.1.0")
        .alternative(true)
        .publish();

    p.payload("build")
        .with_stderr_contains(format!(
            "[UPDATING] `{}` index",
            registry::alt_registry_path().to_str().unwrap()
        ))
        .with_stderr_contains(&format!(
            "[UPDATING] `{}` index",
            registry::registry_path().to_str().unwrap()
        ))
        .with_stderr_contains("[DOWNLOADED] crates_io_dep v0.0.1 (registry `[ROOT][..]`)")
        .with_stderr_contains("[DOWNLOADED] alt_reg_dep v0.1.0 (registry `[ROOT][..]`)")
        .with_stderr_contains("[COMPILING] alt_reg_dep v0.1.0 (registry `[ROOT][..]`)")
        .with_stderr_contains("[COMPILING] crates_io_dep v0.0.1")
        .with_stderr_contains("[COMPILING] foo v0.0.1 ([CWD])")
        .with_stderr_contains("[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s")
        .run();
}

#[payload_test]
fn block_publish_due_to_no_token() {
    registry::alt_init();
    let p = project().file("src/lib.rs", "").build();

    fs::remove_file(paths::home().join(".payload/credentials")).unwrap();

    // Now perform the actual publish
    p.payload("publish --registry alternative")
        .with_status(101)
        .with_stderr_contains(
            "error: no upload token found, \
            please run `payload login` or pass `--token`",
        )
        .run();
}

#[payload_test]
fn publish_to_alt_registry() {
    registry::alt_init();
    let p = project().file("src/main.rs", "fn main() {}").build();

    // Setup the registry by publishing a package
    Package::new("bar", "0.0.1").alternative(true).publish();

    // Login so that we have the token available
    p.payload("login --registry alternative TOKEN").run();

    // Now perform the actual publish
    p.payload("publish --registry alternative").run();

    validate_alt_upload(
        r#"{
            "authors": [],
            "badges": {},
            "categories": [],
            "deps": [],
            "description": null,
            "documentation": null,
            "features": {},
            "homepage": null,
            "keywords": [],
            "license": null,
            "license_file": null,
            "links": null,
            "name": "foo",
            "readme": null,
            "readme_file": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "vers": "0.0.1"
        }"#,
        "foo-0.0.1.crate",
        &["Payload.lock", "Payload.toml", "Payload.toml.orig", "src/main.rs"],
    );
}

#[payload_test]
fn publish_with_crates_io_dep() {
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = ["me"]
                license = "MIT"
                description = "foo"

                [dependencies.bar]
                version = "0.0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.0.1").publish();

    // Login so that we have the token available
    p.payload("login --registry alternative TOKEN").run();

    p.payload("publish --registry alternative").run();

    validate_alt_upload(
        r#"{
            "authors": ["me"],
            "badges": {},
            "categories": [],
            "deps": [
              {
                "default_features": true,
                "features": [],
                "kind": "normal",
                "name": "bar",
                "optional": false,
                "registry": "https://github.com/dustlang/crates.io-index",
                "target": null,
                "version_req": "^0.0.1"
              }
            ],
            "description": "foo",
            "documentation": null,
            "features": {},
            "homepage": null,
            "keywords": [],
            "license": "MIT",
            "license_file": null,
            "links": null,
            "name": "foo",
            "readme": null,
            "readme_file": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "vers": "0.0.1"
        }"#,
        "foo-0.0.1.crate",
        &["Payload.lock", "Payload.toml", "Payload.toml.orig", "src/main.rs"],
    );
}

#[payload_test]
fn passwords_in_registries_index_url_forbidden() {
    registry::alt_init();

    let config = paths::home().join(".payload/config");

    fs::write(
        config,
        r#"
        [registries.alternative]
        index = "ssh://git:secret@foobar.com"
        "#,
    )
    .unwrap();

    let p = project().file("src/main.rs", "fn main() {}").build();

    p.payload("publish --registry alternative")
        .with_status(101)
        .with_stderr(
            "\
error: invalid index URL for registry `alternative` defined in [..]/home/.payload/config

Caused by:
  registry URLs may not contain passwords
",
        )
        .run();
}

#[payload_test]
fn patch_alt_reg() {
    registry::alt_init();
    Package::new("bar", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                bar = { version = "0.1.0", registry = "alternative" }

                [patch.alternative]
                bar = { path = "bar" }
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate bar;
            pub fn f() { bar::bar(); }
            ",
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .build();

    p.payload("build")
        .with_stderr(
            "\
[UPDATING] `[ROOT][..]` index
[COMPILING] bar v0.1.0 ([CWD]/bar)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn bad_registry_name() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "bad name"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse manifest at `[CWD]/Payload.toml`

Caused by:
  invalid character ` ` in registry name: `bad name`, [..]",
        )
        .run();

    for cmd in &[
        "init",
        "install foo",
        "login",
        "owner",
        "publish",
        "search",
        "yank --vers 0.0.1",
    ] {
        p.payload(cmd)
            .arg("--registry")
            .arg("bad name")
            .with_status(101)
            .with_stderr("[ERROR] invalid character ` ` in registry name: `bad name`, [..]")
            .run();
    }
}

#[payload_test]
fn no_api() {
    registry::alt_init();
    Package::new("bar", "0.0.1").alternative(true).publish();
    // Configure without `api`.
    let repo = git2::Repository::open(registry::alt_registry_path()).unwrap();
    let cfg_path = registry::alt_registry_path().join("config.json");
    fs::write(
        cfg_path,
        format!(r#"{{"dl": "{}"}}"#, registry::alt_dl_url()),
    )
    .unwrap();
    git::add(&repo);
    git::commit(&repo);

    // First check that a dependency works.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies.bar]
                version = "0.0.1"
                registry = "alternative"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build")
        .with_stderr(&format!(
            "\
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
            reg = registry::alt_registry_path().to_str().unwrap()
        ))
        .run();

    // Check all of the API commands.
    let err = format!(
        "[ERROR] registry `{}` does not support API commands",
        registry::alt_registry_path().display()
    );

    p.payload("login --registry alternative TOKEN")
        .with_status(101)
        .with_stderr_contains(&err)
        .run();

    p.payload("publish --registry alternative")
        .with_status(101)
        .with_stderr_contains(&err)
        .run();

    p.payload("search --registry alternative")
        .with_status(101)
        .with_stderr_contains(&err)
        .run();

    p.payload("owner --registry alternative --list")
        .with_status(101)
        .with_stderr_contains(&err)
        .run();

    p.payload("yank --registry alternative --vers=0.0.1 bar")
        .with_status(101)
        .with_stderr_contains(&err)
        .run();

    p.payload("yank --registry alternative --vers=0.0.1 bar")
        .with_stderr_contains(&err)
        .with_status(101)
        .run();
}

#[payload_test]
fn alt_reg_metadata() {
    // Check for "registry" entries in `payload metadata` with alternative registries.
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                altdep = { version = "0.0.1", registry = "alternative" }
                iodep = { version = "0.0.1" }
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("bar", "0.0.1").publish();
    Package::new("altdep", "0.0.1")
        .dep("bar", "0.0.1")
        .alternative(true)
        .publish();
    Package::new("altdep2", "0.0.1").alternative(true).publish();
    Package::new("iodep", "0.0.1")
        .registry_dep("altdep2", "0.0.1")
        .publish();

    // The important thing to check here is the "registry" value in `deps`.
    // They should be:
    // foo -> altdep: alternative-registry
    // foo -> iodep: null (because it is in crates.io)
    // altdep -> bar: null (because it is in crates.io)
    // iodep -> altdep2: alternative-registry
    p.payload("metadata --format-version=1 --no-deps")
        .with_json(
            r#"
            {
                "packages": [
                    {
                        "name": "foo",
                        "version": "0.0.1",
                        "id": "foo 0.0.1 (path+file:[..]/foo)",
                        "license": null,
                        "license_file": null,
                        "description": null,
                        "source": null,
                        "dependencies": [
                            {
                                "name": "altdep",
                                "source": "registry+file:[..]/alternative-registry",
                                "req": "^0.0.1",
                                "kind": null,
                                "rename": null,
                                "optional": false,
                                "uses_default_features": true,
                                "features": [],
                                "target": null,
                                "registry": "file:[..]/alternative-registry"
                            },
                            {
                                "name": "iodep",
                                "source": "registry+https://github.com/dustlang/crates.io-index",
                                "req": "^0.0.1",
                                "kind": null,
                                "rename": null,
                                "optional": false,
                                "uses_default_features": true,
                                "features": [],
                                "target": null,
                                "registry": null
                            }
                        ],
                        "targets": "{...}",
                        "features": {},
                        "manifest_path": "[..]/foo/Payload.toml",
                        "metadata": null,
                        "publish": null,
                        "authors": [],
                        "categories": [],
                        "keywords": [],
                        "readme": null,
                        "repository": null,
                        "homepage": null,
                        "documentation": null,
                        "edition": "2015",
                        "links": null
                    }
                ],
                "workspace_members": [
                    "foo 0.0.1 (path+file:[..]/foo)"
                ],
                "resolve": null,
                "target_directory": "[..]/foo/target",
                "version": 1,
                "workspace_root": "[..]/foo",
                "metadata": null
            }"#,
        )
        .run();

    // --no-deps uses a different code path, make sure both work.
    p.payload("metadata --format-version=1")
        .with_json(
            r#"
             {
                "packages": [
                    {
                        "name": "altdep",
                        "version": "0.0.1",
                        "id": "altdep 0.0.1 (registry+file:[..]/alternative-registry)",
                        "license": null,
                        "license_file": null,
                        "description": null,
                        "source": "registry+file:[..]/alternative-registry",
                        "dependencies": [
                            {
                                "name": "bar",
                                "source": "registry+https://github.com/dustlang/crates.io-index",
                                "req": "^0.0.1",
                                "kind": null,
                                "rename": null,
                                "optional": false,
                                "uses_default_features": true,
                                "features": [],
                                "target": null,
                                "registry": null
                            }
                        ],
                        "targets": "{...}",
                        "features": {},
                        "manifest_path": "[..]/altdep-0.0.1/Payload.toml",
                        "metadata": null,
                        "publish": null,
                        "authors": [],
                        "categories": [],
                        "keywords": [],
                        "readme": null,
                        "repository": null,
                        "homepage": null,
                        "documentation": null,
                        "edition": "2015",
                        "links": null
                    },
                    {
                        "name": "altdep2",
                        "version": "0.0.1",
                        "id": "altdep2 0.0.1 (registry+file:[..]/alternative-registry)",
                        "license": null,
                        "license_file": null,
                        "description": null,
                        "source": "registry+file:[..]/alternative-registry",
                        "dependencies": [],
                        "targets": "{...}",
                        "features": {},
                        "manifest_path": "[..]/altdep2-0.0.1/Payload.toml",
                        "metadata": null,
                        "publish": null,
                        "authors": [],
                        "categories": [],
                        "keywords": [],
                        "readme": null,
                        "repository": null,
                        "homepage": null,
                        "documentation": null,
                        "edition": "2015",
                        "links": null
                    },
                    {
                        "name": "bar",
                        "version": "0.0.1",
                        "id": "bar 0.0.1 (registry+https://github.com/dustlang/crates.io-index)",
                        "license": null,
                        "license_file": null,
                        "description": null,
                        "source": "registry+https://github.com/dustlang/crates.io-index",
                        "dependencies": [],
                        "targets": "{...}",
                        "features": {},
                        "manifest_path": "[..]/bar-0.0.1/Payload.toml",
                        "metadata": null,
                        "publish": null,
                        "authors": [],
                        "categories": [],
                        "keywords": [],
                        "readme": null,
                        "repository": null,
                        "homepage": null,
                        "documentation": null,
                        "edition": "2015",
                        "links": null
                    },
                    {
                        "name": "foo",
                        "version": "0.0.1",
                        "id": "foo 0.0.1 (path+file:[..]/foo)",
                        "license": null,
                        "license_file": null,
                        "description": null,
                        "source": null,
                        "dependencies": [
                            {
                                "name": "altdep",
                                "source": "registry+file:[..]/alternative-registry",
                                "req": "^0.0.1",
                                "kind": null,
                                "rename": null,
                                "optional": false,
                                "uses_default_features": true,
                                "features": [],
                                "target": null,
                                "registry": "file:[..]/alternative-registry"
                            },
                            {
                                "name": "iodep",
                                "source": "registry+https://github.com/dustlang/crates.io-index",
                                "req": "^0.0.1",
                                "kind": null,
                                "rename": null,
                                "optional": false,
                                "uses_default_features": true,
                                "features": [],
                                "target": null,
                                "registry": null
                            }
                        ],
                        "targets": "{...}",
                        "features": {},
                        "manifest_path": "[..]/foo/Payload.toml",
                        "metadata": null,
                        "publish": null,
                        "authors": [],
                        "categories": [],
                        "keywords": [],
                        "readme": null,
                        "repository": null,
                        "homepage": null,
                        "documentation": null,
                        "edition": "2015",
                        "links": null
                    },
                    {
                        "name": "iodep",
                        "version": "0.0.1",
                        "id": "iodep 0.0.1 (registry+https://github.com/dustlang/crates.io-index)",
                        "license": null,
                        "license_file": null,
                        "description": null,
                        "source": "registry+https://github.com/dustlang/crates.io-index",
                        "dependencies": [
                            {
                                "name": "altdep2",
                                "source": "registry+file:[..]/alternative-registry",
                                "req": "^0.0.1",
                                "kind": null,
                                "rename": null,
                                "optional": false,
                                "uses_default_features": true,
                                "features": [],
                                "target": null,
                                "registry": "file:[..]/alternative-registry"
                            }
                        ],
                        "targets": "{...}",
                        "features": {},
                        "manifest_path": "[..]/iodep-0.0.1/Payload.toml",
                        "metadata": null,
                        "publish": null,
                        "authors": [],
                        "categories": [],
                        "keywords": [],
                        "readme": null,
                        "repository": null,
                        "homepage": null,
                        "documentation": null,
                        "edition": "2015",
                        "links": null
                    }
                ],
                "workspace_members": [
                    "foo 0.0.1 (path+file:[..]/foo)"
                ],
                "resolve": "{...}",
                "target_directory": "[..]/foo/target",
                "version": 1,
                "workspace_root": "[..]/foo",
                "metadata": null
            }"#,
        )
        .run();
}

#[payload_test]
fn unknown_registry() {
    // A known registry refers to an unknown registry.
    // foo -> bar(crates.io) -> baz(alt)
    registry::alt_init();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("baz", "0.0.1").alternative(true).publish();
    Package::new("bar", "0.0.1")
        .registry_dep("baz", "0.0.1")
        .publish();

    // Remove "alternative" from config.
    let cfg_path = paths::home().join(".payload/config");
    let mut config = fs::read_to_string(&cfg_path).unwrap();
    let start = config.find("[registries.alternative]").unwrap();
    config.insert(start, '#');
    let start_index = &config[start..].find("index =").unwrap();
    config.insert(start + start_index, '#');
    fs::write(&cfg_path, config).unwrap();

    p.payload("build").run();

    // Important parts:
    // foo -> bar registry = null
    // bar -> baz registry = alternate
    p.payload("metadata --format-version=1")
        .with_json(
            r#"
            {
              "packages": [
                {
                  "name": "bar",
                  "version": "0.0.1",
                  "id": "bar 0.0.1 (registry+https://github.com/dustlang/crates.io-index)",
                  "license": null,
                  "license_file": null,
                  "description": null,
                  "source": "registry+https://github.com/dustlang/crates.io-index",
                  "dependencies": [
                    {
                      "name": "baz",
                      "source": "registry+file://[..]/alternative-registry",
                      "req": "^0.0.1",
                      "kind": null,
                      "rename": null,
                      "optional": false,
                      "uses_default_features": true,
                      "features": [],
                      "target": null,
                      "registry": "file:[..]/alternative-registry"
                    }
                  ],
                  "targets": "{...}",
                  "features": {},
                  "manifest_path": "[..]",
                  "metadata": null,
                  "publish": null,
                  "authors": [],
                  "categories": [],
                  "keywords": [],
                  "readme": null,
                  "repository": null,
                  "homepage": null,
                  "documentation": null,
                  "edition": "2015",
                  "links": null
                },
                {
                  "name": "baz",
                  "version": "0.0.1",
                  "id": "baz 0.0.1 (registry+file://[..]/alternative-registry)",
                  "license": null,
                  "license_file": null,
                  "description": null,
                  "source": "registry+file://[..]/alternative-registry",
                  "dependencies": [],
                  "targets": "{...}",
                  "features": {},
                  "manifest_path": "[..]",
                  "metadata": null,
                  "publish": null,
                  "authors": [],
                  "categories": [],
                  "keywords": [],
                  "readme": null,
                  "repository": null,
                  "homepage": null,
                  "documentation": null,
                  "edition": "2015",
                  "links": null
                },
                {
                  "name": "foo",
                  "version": "0.0.1",
                  "id": "foo 0.0.1 (path+file://[..]/foo)",
                  "license": null,
                  "license_file": null,
                  "description": null,
                  "source": null,
                  "dependencies": [
                    {
                      "name": "bar",
                      "source": "registry+https://github.com/dustlang/crates.io-index",
                      "req": "^0.0.1",
                      "kind": null,
                      "rename": null,
                      "optional": false,
                      "uses_default_features": true,
                      "features": [],
                      "target": null,
                      "registry": null
                    }
                  ],
                  "targets": "{...}",
                  "features": {},
                  "manifest_path": "[..]/foo/Payload.toml",
                  "metadata": null,
                  "publish": null,
                  "authors": [],
                  "categories": [],
                  "keywords": [],
                  "readme": null,
                  "repository": null,
                  "homepage": null,
                  "documentation": null,
                  "edition": "2015",
                  "links": null
                }
              ],
              "workspace_members": [
                "foo 0.0.1 (path+file://[..]/foo)"
              ],
              "resolve": "{...}",
              "target_directory": "[..]/foo/target",
              "version": 1,
              "workspace_root": "[..]/foo",
              "metadata": null
            }
            "#,
        )
        .run();
}

#[payload_test]
fn registries_index_relative_url() {
    registry::alt_init();
    let config = paths::root().join(".payload/config");
    fs::create_dir_all(config.parent().unwrap()).unwrap();
    fs::write(
        &config,
        r#"
            [registries.relative]
            index = "file:alternative-registry"
        "#,
    )
    .unwrap();

    registry::init();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "relative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.0.1").alternative(true).publish();

    p.payload("build")
        .with_stderr(&format!(
            "\
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] bar v0.0.1 (registry `[ROOT][..]`)
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]s
",
            reg = registry::alt_registry_path().to_str().unwrap()
        ))
        .run();
}

#[payload_test]
fn registries_index_relative_path_not_allowed() {
    registry::alt_init();
    let config = paths::root().join(".payload/config");
    fs::create_dir_all(config.parent().unwrap()).unwrap();
    fs::write(
        &config,
        r#"
            [registries.relative]
            index = "alternative-registry"
        "#,
    )
    .unwrap();

    registry::init();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                version = "0.0.1"
                registry = "relative"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.0.1").alternative(true).publish();

    p.payload("build")
        .with_stderr(&format!(
            "\
error: failed to parse manifest at `{root}/foo/Payload.toml`

Caused by:
  invalid index URL for registry `relative` defined in [..]/.payload/config

Caused by:
  invalid url `alternative-registry`: relative URL without a base
",
            root = paths::root().to_str().unwrap()
        ))
        .with_status(101)
        .run();
}

#[payload_test]
fn both_index_and_registry() {
    let p = project().file("src/lib.rs", "").build();
    for cmd in &["publish", "owner", "search", "yank --vers 1.0.0"] {
        p.payload(cmd)
            .arg("--registry=foo")
            .arg("--index=foo")
            .with_status(101)
            .with_stderr(
                "[ERROR] both `--index` and `--registry` \
                should not be set at the same time",
            )
            .run();
    }
}
