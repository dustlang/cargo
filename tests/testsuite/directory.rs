//! Tests for directory sources.

use std::collections::HashMap;
use std::fs;
use std::str;

use serde::Serialize;

use payload_test_support::payload_process;
use payload_test_support::git;
use payload_test_support::paths;
use payload_test_support::registry::{cksum, Package};
use payload_test_support::{basic_manifest, project, t, ProjectBuilder};

fn setup() {
    let root = paths::root();
    t!(fs::create_dir(&root.join(".payload")));
    t!(fs::write(
        root.join(".payload/config"),
        r#"
            [source.crates-io]
            replace-with = 'my-awesome-local-registry'

            [source.my-awesome-local-registry]
            directory = 'index'
        "#
    ));
}

struct VendorPackage {
    p: Option<ProjectBuilder>,
    cksum: Checksum,
}

#[derive(Serialize)]
struct Checksum {
    package: Option<String>,
    files: HashMap<String, String>,
}

impl VendorPackage {
    fn new(name: &str) -> VendorPackage {
        VendorPackage {
            p: Some(project().at(&format!("index/{}", name))),
            cksum: Checksum {
                package: Some(String::new()),
                files: HashMap::new(),
            },
        }
    }

    fn file(&mut self, name: &str, contents: &str) -> &mut VendorPackage {
        self.p = Some(self.p.take().unwrap().file(name, contents));
        self.cksum
            .files
            .insert(name.to_string(), cksum(contents.as_bytes()));
        self
    }

    fn disable_checksum(&mut self) -> &mut VendorPackage {
        self.cksum.package = None;
        self
    }

    fn no_manifest(mut self) -> Self {
        self.p = self.p.map(|pb| pb.no_manifest());
        self
    }

    fn build(&mut self) {
        let p = self.p.take().unwrap();
        let json = serde_json::to_string(&self.cksum).unwrap();
        let p = p.file(".payload-checksum.json", &json);
        let _ = p.build();
    }
}

#[payload_test]
fn simple() {
    setup();

    VendorPackage::new("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn bar() {}")
        .build();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::bar(); }",
        )
        .build();

    p.payload("build")
        .with_stderr(
            "\
[COMPILING] bar v0.1.0
[COMPILING] foo v0.1.0 ([CWD])
[FINISHED] [..]
",
        )
        .run();
}

#[payload_test]
fn simple_install() {
    setup();

    VendorPackage::new("foo")
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    VendorPackage::new("bar")
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = "0.0.1"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate foo; pub fn main() { foo::foo(); }",
        )
        .build();

    payload_process("install bar")
        .with_stderr(
            "\
[INSTALLING] bar v0.1.0
[COMPILING] foo v0.0.1
[COMPILING] bar v0.1.0
[FINISHED] release [optimized] target(s) in [..]s
[INSTALLING] [..]bar[..]
[INSTALLED] package `bar v0.1.0` (executable `bar[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
}

#[payload_test]
fn simple_install_fail() {
    setup();

    VendorPackage::new("foo")
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    VendorPackage::new("bar")
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = "0.1.0"
                baz = "9.8.7"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate foo; pub fn main() { foo::foo(); }",
        )
        .build();

    payload_process("install bar")
        .with_status(101)
        .with_stderr(
            "  Installing bar v0.1.0
error: failed to compile `bar v0.1.0`, intermediate artifacts can be found at `[..]`

Caused by:
  no matching package named `baz` found
  location searched: registry `https://github.com/dustlang/crates.io-index`
  perhaps you meant: bar or foo
  required by package `bar v0.1.0`
",
        )
        .run();
}

#[payload_test]
fn install_without_feature_dep() {
    setup();

    VendorPackage::new("foo")
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    VendorPackage::new("bar")
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = "0.0.1"
                baz = { version = "9.8.7", optional = true }

                [features]
                wantbaz = ["baz"]
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate foo; pub fn main() { foo::foo(); }",
        )
        .build();

    payload_process("install bar")
        .with_stderr(
            "\
[INSTALLING] bar v0.1.0
[COMPILING] foo v0.0.1
[COMPILING] bar v0.1.0
[FINISHED] release [optimized] target(s) in [..]s
[INSTALLING] [..]bar[..]
[INSTALLED] package `bar v0.1.0` (executable `bar[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
}

#[payload_test]
fn not_there() {
    setup();

    let _ = project().at("index").build();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::bar(); }",
        )
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
error: no matching package named `bar` found
location searched: [..]
required by package `foo v0.1.0 ([..])`
",
        )
        .run();
}

#[payload_test]
fn multiple() {
    setup();

    VendorPackage::new("bar-0.1.0")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn bar() {}")
        .file(".payload-checksum", "")
        .build();

    VendorPackage::new("bar-0.2.0")
        .file("Payload.toml", &basic_manifest("bar", "0.2.0"))
        .file("src/lib.rs", "pub fn bar() {}")
        .file(".payload-checksum", "")
        .build();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::bar(); }",
        )
        .build();

    p.payload("build")
        .with_stderr(
            "\
[COMPILING] bar v0.1.0
[COMPILING] foo v0.1.0 ([CWD])
[FINISHED] [..]
",
        )
        .run();
}

#[payload_test]
fn crates_io_then_directory() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file(
            "src/lib.rs",
            "extern crate bar; pub fn foo() { bar::bar(); }",
        )
        .build();

    let cksum = Package::new("bar", "0.1.0")
        .file("src/lib.rs", "pub fn bar() -> u32 { 0 }")
        .publish();

    p.payload("build")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.1.0 ([..])
[COMPILING] bar v0.1.0
[COMPILING] foo v0.1.0 ([CWD])
[FINISHED] [..]
",
        )
        .run();

    setup();

    let mut v = VendorPackage::new("bar");
    v.file("Payload.toml", &basic_manifest("bar", "0.1.0"));
    v.file("src/lib.rs", "pub fn bar() -> u32 { 1 }");
    v.cksum.package = Some(cksum);
    v.build();

    p.payload("build")
        .with_stderr(
            "\
[COMPILING] bar v0.1.0
[COMPILING] foo v0.1.0 ([CWD])
[FINISHED] [..]
",
        )
        .run();
}

#[payload_test]
fn crates_io_then_bad_checksum() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    Package::new("bar", "0.1.0").publish();

    p.payload("build").run();
    setup();

    VendorPackage::new("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "")
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
error: checksum for `bar v0.1.0` changed between lock files

this could be indicative of a few possible errors:

    * the lock file is corrupt
    * a replacement source in use (e.g., a mirror) returned a different checksum
    * the source itself may be corrupt in one way or another

unable to verify that `bar v0.1.0` is the same as when the lockfile was generated

",
        )
        .run();
}

#[payload_test]
fn bad_file_checksum() {
    setup();

    VendorPackage::new("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "")
        .build();

    t!(fs::write(
        paths::root().join("index/bar/src/lib.rs"),
        "fn bar() -> u32 { 0 }"
    ));

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
error: the listed checksum of `[..]lib.rs` has changed:
expected: [..]
actual:   [..]

directory sources are not intended to be edited, if modifications are \
required then it is recommended that [replace] is used with a forked copy of \
the source
",
        )
        .run();
}

#[payload_test]
fn only_dot_files_ok() {
    setup();

    VendorPackage::new("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "")
        .build();
    VendorPackage::new("foo")
        .no_manifest()
        .file(".bar", "")
        .build();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build").run();
}

#[payload_test]
fn random_files_ok() {
    setup();

    VendorPackage::new("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "")
        .build();
    VendorPackage::new("foo")
        .no_manifest()
        .file("bar", "")
        .file("../test", "")
        .build();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build").run();
}

#[payload_test]
fn git_lock_file_doesnt_change() {
    let git = git::new("git", |p| {
        p.file("Payload.toml", &basic_manifest("git", "0.5.0"))
            .file("src/lib.rs", "")
    });

    VendorPackage::new("git")
        .file("Payload.toml", &basic_manifest("git", "0.5.0"))
        .file("src/lib.rs", "")
        .disable_checksum()
        .build();

    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    version = "0.0.1"
                    authors = []

                    [dependencies]
                    git = {{ git = '{0}' }}
                "#,
                git.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build").run();

    let lock1 = p.read_lockfile();

    let root = paths::root();
    t!(fs::create_dir(&root.join(".payload")));
    t!(fs::write(
        root.join(".payload/config"),
        format!(
            r#"
                [source.my-git-repo]
                git = '{}'
                replace-with = 'my-awesome-local-registry'

                [source.my-awesome-local-registry]
                directory = 'index'
            "#,
            git.url()
        )
    ));

    p.payload("build")
        .with_stderr(
            "\
[COMPILING] [..]
[COMPILING] [..]
[FINISHED] [..]
",
        )
        .run();

    let lock2 = p.read_lockfile();
    assert_eq!(lock1, lock2, "lock files changed");
}

#[payload_test]
fn git_override_requires_lockfile() {
    VendorPackage::new("git")
        .file("Payload.toml", &basic_manifest("git", "0.5.0"))
        .file("src/lib.rs", "")
        .disable_checksum()
        .build();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                git = { git = 'https://example.com/' }
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    let root = paths::root();
    t!(fs::create_dir(&root.join(".payload")));
    t!(fs::write(
        root.join(".payload/config"),
        r#"
            [source.my-git-repo]
            git = 'https://example.com/'
            replace-with = 'my-awesome-local-registry'

            [source.my-awesome-local-registry]
            directory = 'index'
        "#
    ));

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to get `git` as a dependency of package `foo v0.0.1 ([..])`

Caused by:
  failed to load source for dependency `git`

Caused by:
  Unable to update [..]

Caused by:
  the source my-git-repo requires a lock file to be present first before it can be
  used against vendored source code

  remove the source replacement configuration, generate a lock file, and then
  restore the source replacement configuration to continue the build
",
        )
        .run();
}

#[payload_test]
fn workspace_different_locations() {
    let p = project()
        .no_manifest()
        .file(
            "foo/Payload.toml",
            r#"
                [package]
                name = 'foo'
                version = '0.1.0'

                [dependencies]
                baz = "*"
            "#,
        )
        .file("foo/src/lib.rs", "")
        .file("foo/vendor/baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("foo/vendor/baz/src/lib.rs", "")
        .file("foo/vendor/baz/.payload-checksum.json", "{\"files\":{}}")
        .file(
            "bar/Payload.toml",
            r#"
                [package]
                name = 'bar'
                version = '0.1.0'

                [dependencies]
                baz = "*"
            "#,
        )
        .file("bar/src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [build]
                target-dir = './target'

                [source.crates-io]
                replace-with = 'my-awesome-local-registry'

                [source.my-awesome-local-registry]
                directory = 'foo/vendor'
            "#,
        )
        .build();

    p.payload("build").cwd("foo").run();
    p.payload("build")
        .cwd("bar")
        .with_stderr(
            "\
[COMPILING] bar [..]
[FINISHED] [..]
",
        )
        .run();
}

#[payload_test]
fn version_missing() {
    setup();

    VendorPackage::new("foo")
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    VendorPackage::new("bar")
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = "2"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    payload_process("install bar")
        .with_stderr(
            "\
[INSTALLING] bar v0.1.0
error: failed to compile [..]

Caused by:
  failed to select a version for the requirement `foo = \"^2\"`
  candidate versions found which didn't match: 0.0.1
  location searched: directory source `[..] (which is replacing registry `[..]`)
  required by package `bar v0.1.0`
  perhaps a crate was updated and forgotten to be re-vendored?
",
        )
        .with_status(101)
        .run();
}
