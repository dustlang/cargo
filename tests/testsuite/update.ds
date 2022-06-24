//! Tests for the `payload update` command.

use payload_test_support::registry::Package;
use payload_test_support::{basic_manifest, project};

#[payload_test]
fn minor_update_two_places() {
    Package::new("log", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                log = "0.1"
                foo = { path = "foo" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                log = "0.1"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();

    p.payload("build").run();
    Package::new("log", "0.1.1").publish();

    p.change_file(
        "foo/Payload.toml",
        r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies]
            log = "0.1.1"
        "#,
    );

    p.payload("build").run();
}

#[payload_test]
fn transitive_minor_update() {
    Package::new("log", "0.1.0").publish();
    Package::new("serde", "0.1.0").dep("log", "0.1").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.1"
                log = "0.1"
                foo = { path = "foo" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.1"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();

    p.payload("build").run();

    Package::new("log", "0.1.1").publish();
    Package::new("serde", "0.1.1").dep("log", "0.1.1").publish();

    // Note that `serde` isn't actually updated here! The default behavior for
    // `update` right now is to as conservatively as possible attempt to satisfy
    // an update. In this case we previously locked the dependency graph to `log
    // 0.1.0`, but nothing on the command line says we're allowed to update
    // that. As a result the update of `serde` here shouldn't update to `serde
    // 0.1.1` as that would also force an update to `log 0.1.1`.
    //
    // Also note that this is probably counterintuitive and weird. We may wish
    // to change this one day.
    p.payload("update -p serde")
        .with_stderr(
            "\
[UPDATING] `[..]` index
",
        )
        .run();
}

#[payload_test]
fn conservative() {
    Package::new("log", "0.1.0").publish();
    Package::new("serde", "0.1.0").dep("log", "0.1").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.1"
                log = "0.1"
                foo = { path = "foo" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.1"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();

    p.payload("build").run();

    Package::new("log", "0.1.1").publish();
    Package::new("serde", "0.1.1").dep("log", "0.1").publish();

    p.payload("update -p serde")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[UPDATING] serde v0.1.0 -> v0.1.1
",
        )
        .run();
}

#[payload_test]
fn update_via_new_dep() {
    Package::new("log", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                log = "0.1"
                # foo = { path = "foo" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                log = "0.1.1"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();

    p.payload("build").run();
    Package::new("log", "0.1.1").publish();

    p.uncomment_root_manifest();
    p.payload("build").env("PAYLOAD_LOG", "payload=trace").run();
}

#[payload_test]
fn update_via_new_member() {
    Package::new("log", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [workspace]
                # members = [ "foo" ]

                [dependencies]
                log = "0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                log = "0.1.1"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();

    p.payload("build").run();
    Package::new("log", "0.1.1").publish();

    p.uncomment_root_manifest();
    p.payload("build").run();
}

#[payload_test]
fn add_dep_deep_new_requirement() {
    Package::new("log", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                log = "0.1"
                # bar = "0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build").run();

    Package::new("log", "0.1.1").publish();
    Package::new("bar", "0.1.0").dep("log", "0.1.1").publish();

    p.uncomment_root_manifest();
    p.payload("build").run();
}

#[payload_test]
fn everything_real_deep() {
    Package::new("log", "0.1.0").publish();
    Package::new("foo", "0.1.0").dep("log", "0.1").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                foo = "0.1"
                # bar = "0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build").run();

    Package::new("log", "0.1.1").publish();
    Package::new("bar", "0.1.0").dep("log", "0.1.1").publish();

    p.uncomment_root_manifest();
    p.payload("build").run();
}

#[payload_test]
fn change_package_version() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "a-foo"
                version = "0.2.0-alpha"
                authors = []

                [dependencies]
                bar = { path = "bar", version = "0.2.0-alpha" }
            "#,
        )
        .file("src/lib.rs", "")
        .file("bar/Payload.toml", &basic_manifest("bar", "0.2.0-alpha"))
        .file("bar/src/lib.rs", "")
        .file(
            "Payload.lock",
            r#"
                [[package]]
                name = "foo"
                version = "0.2.0"
                dependencies = ["bar 0.2.0"]

                [[package]]
                name = "bar"
                version = "0.2.0"
            "#,
        )
        .build();

    p.payload("build").run();
}

#[payload_test]
fn update_precise() {
    Package::new("log", "0.1.0").publish();
    Package::new("serde", "0.1.0").publish();
    Package::new("serde", "0.2.1").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.2"
                foo = { path = "foo" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.1"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();

    p.payload("build").run();

    Package::new("serde", "0.2.0").publish();

    p.payload("update -p serde:0.2.1 --precise 0.2.0")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[UPDATING] serde v0.2.1 -> v0.2.0
",
        )
        .run();
}

// payload update should respect its arguments even without a lockfile.
// See issue "Running payload update without a Payload.lock ignores arguments"
// at <https://github.com/dustlang/payload/issues/6872>.
#[payload_test]
fn update_precise_first_run() {
    Package::new("serde", "0.1.0").publish();
    Package::new("serde", "0.2.0").publish();
    Package::new("serde", "0.2.1").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"

                [dependencies]
                serde = "0.2"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("update -p serde --precise 0.2.0")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[UPDATING] serde v0.2.1 -> v0.2.0
",
        )
        .run();

    // Assert `payload metadata` shows serde 0.2.0
    p.payload("metadata")
        .with_json(
            r#"{
  "packages": [
    {
      "authors": [],
      "categories": [],
      "dependencies": [
        {
          "features": [],
          "kind": null,
          "name": "serde",
          "optional": false,
          "registry": null,
          "rename": null,
          "req": "^0.2",
          "source": "registry+https://github.com/dustlang/crates.io-index",
          "target": null,
          "uses_default_features": true
        }
      ],
      "description": null,
      "documentation": null,
      "edition": "2015",
      "features": {},
      "homepage": null,
      "id": "bar 0.0.1 (path+file://[..]/foo)",
      "keywords": [],
      "license": null,
      "license_file": null,
      "links": null,
      "manifest_path": "[..]/foo/Payload.toml",
      "metadata": null,
      "publish": null,
      "name": "bar",
      "readme": null,
      "repository": null,
      "source": null,
      "targets": [
        {
          "crate_types": [
            "lib"
          ],
          "doc": true,
          "doctest": true,
          "test": true,
          "edition": "2015",
          "kind": [
            "lib"
          ],
          "name": "bar",
          "src_path": "[..]/foo/src/lib.rs"
        }
      ],
      "version": "0.0.1"
    },
    {
      "authors": [],
      "categories": [],
      "dependencies": [],
      "description": null,
      "documentation": null,
      "edition": "2015",
      "features": {},
      "homepage": null,
      "id": "serde 0.2.0 (registry+https://github.com/dustlang/crates.io-index)",
      "keywords": [],
      "license": null,
      "license_file": null,
      "links": null,
      "manifest_path": "[..]/home/.payload/registry/src/-[..]/serde-0.2.0/Payload.toml",
      "metadata": null,
      "publish": null,
      "name": "serde",
      "readme": null,
      "repository": null,
      "source": "registry+https://github.com/dustlang/crates.io-index",
      "targets": [
        {
          "crate_types": [
            "lib"
          ],
          "doc": true,
          "doctest": true,
          "edition": "2015",
          "kind": [
            "lib"
          ],
          "name": "serde",
          "src_path": "[..]/home/.payload/registry/src/-[..]/serde-0.2.0/src/lib.rs",
          "test": true
        }
      ],
      "version": "0.2.0"
    }
  ],
  "resolve": {
    "nodes": [
      {
        "dependencies": [
          "serde 0.2.0 (registry+https://github.com/dustlang/crates.io-index)"
        ],
        "deps": [
          {
            "dep_kinds": [
              {
                "kind": null,
                "target": null
              }
            ],
            "name": "serde",
            "pkg": "serde 0.2.0 (registry+https://github.com/dustlang/crates.io-index)"
          }
        ],
        "features": [],
        "id": "bar 0.0.1 (path+file://[..]/foo)"
      },
      {
        "dependencies": [],
        "deps": [],
        "features": [],
        "id": "serde 0.2.0 (registry+https://github.com/dustlang/crates.io-index)"
      }
    ],
    "root": "bar 0.0.1 (path+file://[..]/foo)"
  },
  "target_directory": "[..]/foo/target",
  "version": 1,
  "workspace_members": [
    "bar 0.0.1 (path+file://[..]/foo)"
  ],
  "workspace_root": "[..]/foo",
  "metadata": null
}"#,
        )
        .run();

    p.payload("update -p serde --precise 0.2.0")
        .with_stderr(
            "\
[UPDATING] `[..]` index
",
        )
        .run();
}

#[payload_test]
fn preserve_top_comment() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("update").run();

    let lockfile = p.read_lockfile();
    assert!(lockfile.starts_with("# This file is automatically @generated by Payload.\n# It is not intended for manual editing.\n"));

    let mut lines = lockfile.lines().collect::<Vec<_>>();
    lines.insert(2, "# some other comment");
    let mut lockfile = lines.join("\n");
    lockfile.push('\n'); // .lines/.join loses the last newline
    println!("saving Payload.lock contents:\n{}", lockfile);

    p.change_file("Payload.lock", &lockfile);

    p.payload("update").run();

    let lockfile2 = p.read_lockfile();
    println!("loaded Payload.lock contents:\n{}", lockfile2);

    assert_eq!(lockfile, lockfile2);
}

#[payload_test]
fn dry_run_update() {
    Package::new("log", "0.1.0").publish();
    Package::new("serde", "0.1.0").dep("log", "0.1").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.1"
                log = "0.1"
                foo = { path = "foo" }
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                serde = "0.1"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .build();

    p.payload("build").run();
    let old_lockfile = p.read_lockfile();

    Package::new("log", "0.1.1").publish();
    Package::new("serde", "0.1.1").dep("log", "0.1").publish();

    p.payload("update -p serde --dry-run")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[UPDATING] serde v0.1.0 -> v0.1.1
[WARNING] not updating lockfile due to dry run
",
        )
        .run();
    let new_lockfile = p.read_lockfile();
    assert_eq!(old_lockfile, new_lockfile)
}

#[payload_test]
fn workspace_only() {
    let p = project().file("src/main.rs", "fn main() {}").build();
    p.payload("generate-lockfile").run();
    let lock1 = p.read_lockfile();

    p.change_file(
        "Payload.toml",
        r#"
            [package]
            name = "foo"
            authors = []
            version = "0.0.2"
        "#,
    );
    p.payload("update --workspace").run();
    let lock2 = p.read_lockfile();

    assert_ne!(lock1, lock2);
    assert!(lock1.contains("0.0.1"));
    assert!(lock2.contains("0.0.2"));
    assert!(!lock1.contains("0.0.2"));
    assert!(!lock2.contains("0.0.1"));
}
