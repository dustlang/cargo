//! Tests for the `payload install` command.

use std::fs::{self, OpenOptions};
use std::io::prelude::*;

use payload_test_support::cross_compile;
use payload_test_support::git;
use payload_test_support::registry::{registry_path, registry_url, Package};
use payload_test_support::{
    basic_manifest, payload_process, no_such_file_err_msg, project, symlink_supported, t,
};

use payload_test_support::install::{
    assert_has_installed_exe, assert_has_not_installed_exe, payload_home,
};
use payload_test_support::paths;
use std::env;
use std::path::PathBuf;

fn pkg(name: &str, vers: &str) {
    Package::new(name, vers)
        .file("src/lib.rs", "")
        .file(
            "src/main.rs",
            &format!("extern crate {}; fn main() {{}}", name),
        )
        .publish();
}

#[payload_test]
fn simple() {
    pkg("foo", "0.0.1");

    payload_process("install foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry [..])
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(payload_home(), "foo");

    payload_process("uninstall foo")
        .with_stderr("[REMOVING] [CWD]/home/.payload/bin/foo[EXE]")
        .run();
    assert_has_not_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn with_index() {
    pkg("foo", "0.0.1");

    payload_process("install foo --index")
        .arg(registry_url().to_string())
        .with_stderr(&format!(
            "\
[UPDATING] `{reg}` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `{reg}`)
[INSTALLING] foo v0.0.1 (registry `{reg}`)
[COMPILING] foo v0.0.1 (registry `{reg}`)
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1 (registry `{reg}`)` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
            reg = registry_path().to_str().unwrap()
        ))
        .run();
    assert_has_installed_exe(payload_home(), "foo");

    payload_process("uninstall foo")
        .with_stderr("[REMOVING] [CWD]/home/.payload/bin/foo[EXE]")
        .run();
    assert_has_not_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn multiple_pkgs() {
    pkg("foo", "0.0.1");
    pkg("bar", "0.0.2");

    payload_process("install foo bar baz")
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `[CWD]/registry`)
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.2 (registry `[CWD]/registry`)
[INSTALLING] bar v0.0.2
[COMPILING] bar v0.0.2
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/bar[EXE]
[INSTALLED] package `bar v0.0.2` (executable `bar[EXE]`)
[ERROR] could not find `baz` in registry `[..]` with version `*`
[SUMMARY] Successfully installed foo, bar! Failed to install baz (see error(s) above).
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
[ERROR] some crates failed to install
",
        )
        .run();
    assert_has_installed_exe(payload_home(), "foo");
    assert_has_installed_exe(payload_home(), "bar");

    payload_process("uninstall foo bar")
        .with_stderr(
            "\
[REMOVING] [CWD]/home/.payload/bin/foo[EXE]
[REMOVING] [CWD]/home/.payload/bin/bar[EXE]
[SUMMARY] Successfully uninstalled foo, bar!
",
        )
        .run();

    assert_has_not_installed_exe(payload_home(), "foo");
    assert_has_not_installed_exe(payload_home(), "bar");
}

fn path() -> Vec<PathBuf> {
    env::split_paths(&env::var_os("PATH").unwrap_or_default()).collect()
}

#[payload_test]
fn multiple_pkgs_path_set() {
    // confirm partial failure results in 101 status code and does not have the
    //      '[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries'
    //  even if PAYLOAD_HOME/bin is in the PATH
    pkg("foo", "0.0.1");
    pkg("bar", "0.0.2");

    // add PAYLOAD_HOME/bin to path
    let mut path = path();
    path.push(payload_home().join("bin"));
    let new_path = env::join_paths(path).unwrap();
    payload_process("install foo bar baz")
        .env("PATH", new_path)
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.0.1 (registry `[CWD]/registry`)
[INSTALLING] foo v0.0.1
[COMPILING] foo v0.0.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1` (executable `foo[EXE]`)
[DOWNLOADING] crates ...
[DOWNLOADED] bar v0.0.2 (registry `[CWD]/registry`)
[INSTALLING] bar v0.0.2
[COMPILING] bar v0.0.2
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/bar[EXE]
[INSTALLED] package `bar v0.0.2` (executable `bar[EXE]`)
[ERROR] could not find `baz` in registry `[..]` with version `*`
[SUMMARY] Successfully installed foo, bar! Failed to install baz (see error(s) above).
[ERROR] some crates failed to install
",
        )
        .run();
    assert_has_installed_exe(payload_home(), "foo");
    assert_has_installed_exe(payload_home(), "bar");

    payload_process("uninstall foo bar")
        .with_stderr(
            "\
[REMOVING] [CWD]/home/.payload/bin/foo[EXE]
[REMOVING] [CWD]/home/.payload/bin/bar[EXE]
[SUMMARY] Successfully uninstalled foo, bar!
",
        )
        .run();

    assert_has_not_installed_exe(payload_home(), "foo");
    assert_has_not_installed_exe(payload_home(), "bar");
}

#[payload_test]
fn pick_max_version() {
    pkg("foo", "0.1.0");
    pkg("foo", "0.2.0");
    pkg("foo", "0.2.1");
    pkg("foo", "0.2.1-pre.1");
    pkg("foo", "0.3.0-pre.2");

    payload_process("install foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.2.1 (registry [..])
[INSTALLING] foo v0.2.1
[COMPILING] foo v0.2.1
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/foo[EXE]
[INSTALLED] package `foo v0.2.1` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn installs_beta_version_by_explicit_name_from_git() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Payload.toml", &basic_manifest("foo", "0.3.0-beta.1"))
        .file("src/main.rs", "fn main() {}")
        .build();

    payload_process("install --git")
        .arg(p.url().to_string())
        .arg("foo")
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn missing() {
    pkg("foo", "0.0.1");
    payload_process("install bar")
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..] index
[ERROR] could not find `bar` in registry `[..]` with version `*`
",
        )
        .run();
}

#[payload_test]
fn missing_current_working_directory() {
    payload_process("install .")
        .with_status(101)
        .with_stderr(
            "\
error: To install the binaries for the package in current working \
directory use `payload install --path .`. Use `payload build` if you \
want to simply build the package.
",
        )
        .run();
}

#[payload_test]
fn bad_version() {
    pkg("foo", "0.0.1");
    payload_process("install foo --vers=0.2.0")
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] [..] index
[ERROR] could not find `foo` in registry `[..]` with version `=0.2.0`
",
        )
        .run();
}

#[payload_test]
fn bad_paths() {
    payload_process("install")
        .with_status(101)
        .with_stderr("[ERROR] `[CWD]` is not a crate root; specify a crate to install [..]")
        .run();

    payload_process("install --path .")
        .with_status(101)
        .with_stderr("[ERROR] `[CWD]` does not contain a Payload.toml file[..]")
        .run();

    let toml = paths::root().join("Payload.toml");
    fs::write(toml, "").unwrap();
    payload_process("install --path Payload.toml")
        .with_status(101)
        .with_stderr("[ERROR] `[CWD]/Payload.toml` is not a directory[..]")
        .run();

    payload_process("install --path .")
        .with_status(101)
        .with_stderr_contains("[ERROR] failed to parse manifest at `[CWD]/Payload.toml`")
        .run();
}

#[payload_test]
fn install_location_precedence() {
    pkg("foo", "0.0.1");

    let root = paths::root();
    let t1 = root.join("t1");
    let t2 = root.join("t2");
    let t3 = root.join("t3");
    let t4 = payload_home();

    fs::create_dir(root.join(".payload")).unwrap();
    fs::write(
        root.join(".payload/config"),
        &format!(
            "[install]
             root = '{}'
            ",
            t3.display()
        ),
    )
    .unwrap();

    println!("install --root");

    payload_process("install foo --root")
        .arg(&t1)
        .env("PAYLOAD_INSTALL_ROOT", &t2)
        .run();
    assert_has_installed_exe(&t1, "foo");
    assert_has_not_installed_exe(&t2, "foo");

    println!("install PAYLOAD_INSTALL_ROOT");

    payload_process("install foo")
        .env("PAYLOAD_INSTALL_ROOT", &t2)
        .run();
    assert_has_installed_exe(&t2, "foo");
    assert_has_not_installed_exe(&t3, "foo");

    println!("install install.root");

    payload_process("install foo").run();
    assert_has_installed_exe(&t3, "foo");
    assert_has_not_installed_exe(&t4, "foo");

    fs::remove_file(root.join(".payload/config")).unwrap();

    println!("install payload home");

    payload_process("install foo").run();
    assert_has_installed_exe(&t4, "foo");
}

#[payload_test]
fn install_path() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    payload_process("install --path").arg(p.root()).run();
    assert_has_installed_exe(payload_home(), "foo");
    // path-style installs force a reinstall
    p.payload("install --path .")
        .with_stderr(
            "\
[INSTALLING] foo v0.0.1 [..]
[FINISHED] release [..]
[REPLACING] [..]/.payload/bin/foo[EXE]
[REPLACED] package `foo v0.0.1 [..]` with `foo v0.0.1 [..]` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[payload_test]
fn install_target_dir() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.payload("install --target-dir td_test")
        .with_stderr(
            "\
[WARNING] Using `payload install` [..]
[INSTALLING] foo v0.0.1 [..]
[COMPILING] foo v0.0.1 [..]
[FINISHED] release [..]
[INSTALLING] [..]foo[EXE]
[INSTALLED] package `foo v0.0.1 [..]foo[..]` (executable `foo[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();

    let mut path = p.root();
    path.push("td_test");
    assert!(path.exists());

    #[cfg(not(windows))]
    path.push("release/foo");
    #[cfg(windows)]
    path.push("release/foo.exe");
    assert!(path.exists());
}

#[payload_test]
fn multiple_crates_error() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .file("a/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/main.rs", "fn main() {}")
        .build();

    payload_process("install --git")
        .arg(p.url().to_string())
        .with_status(101)
        .with_stderr(
            "\
[UPDATING] git repository [..]
[ERROR] multiple packages with binaries found: bar, foo
",
        )
        .run();
}

#[payload_test]
fn multiple_crates_select() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .file("a/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/main.rs", "fn main() {}")
        .build();

    payload_process("install --git")
        .arg(p.url().to_string())
        .arg("foo")
        .run();
    assert_has_installed_exe(payload_home(), "foo");
    assert_has_not_installed_exe(payload_home(), "bar");

    payload_process("install --git")
        .arg(p.url().to_string())
        .arg("bar")
        .run();
    assert_has_installed_exe(payload_home(), "bar");
}

#[payload_test]
fn multiple_crates_git_all() {
    let p = git::repo(&paths::root().join("foo"))
        .file(
            "Payload.toml",
            r#"
            [workspace]
            members = ["bin1", "bin2"]
            "#,
        )
        .file("bin1/Payload.toml", &basic_manifest("bin1", "0.1.0"))
        .file("bin2/Payload.toml", &basic_manifest("bin2", "0.1.0"))
        .file(
            "bin1/src/main.rs",
            r#"fn main() { println!("Hello, world!"); }"#,
        )
        .file(
            "bin2/src/main.rs",
            r#"fn main() { println!("Hello, world!"); }"#,
        )
        .build();

    payload_process(&format!("install --git {} bin1 bin2", p.url().to_string())).run();
}

#[payload_test]
fn multiple_crates_auto_binaries() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/main.rs", "extern crate bar; fn main() {}")
        .file("a/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    payload_process("install --path").arg(p.root()).run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn multiple_crates_auto_examples() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/lib.rs", "extern crate bar;")
        .file(
            "examples/foo.rs",
            "
            extern crate bar;
            extern crate foo;
            fn main() {}
        ",
        )
        .file("a/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    payload_process("install --path")
        .arg(p.root())
        .arg("--example=foo")
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn no_binaries_or_examples() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/lib.rs", "")
        .file("a/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    payload_process("install --path")
        .arg(p.root())
        .with_status(101)
        .with_stderr("[ERROR] no packages found with binaries or examples")
        .run();
}

#[payload_test]
fn no_binaries() {
    let p = project()
        .file("src/lib.rs", "")
        .file("examples/foo.rs", "fn main() {}")
        .build();

    payload_process("install --path")
        .arg(p.root())
        .arg("foo")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] there is nothing to install in `foo v0.0.1 ([..])`, because it has no binaries[..]
[..]
[..]",
        )
        .run();
}

#[payload_test]
fn examples() {
    let p = project()
        .file("src/lib.rs", "")
        .file("examples/foo.rs", "extern crate foo; fn main() {}")
        .build();

    payload_process("install --path")
        .arg(p.root())
        .arg("--example=foo")
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn install_force() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    payload_process("install --path").arg(p.root()).run();

    let p = project()
        .at("foo2")
        .file("Payload.toml", &basic_manifest("foo", "0.2.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    payload_process("install --force --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] foo v0.2.0 ([..])
[COMPILING] foo v0.2.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[REPLACING] [CWD]/home/.payload/bin/foo[EXE]
[REPLACED] package `foo v0.0.1 ([..]/foo)` with `foo v0.2.0 ([..]/foo2)` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    payload_process("install --list")
        .with_stdout(
            "\
foo v0.2.0 ([..]):
    foo[..]
",
        )
        .run();
}

#[payload_test]
fn install_force_partial_overlap() {
    let p = project()
        .file("src/bin/foo-bin1.rs", "fn main() {}")
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .build();

    payload_process("install --path").arg(p.root()).run();

    let p = project()
        .at("foo2")
        .file("Payload.toml", &basic_manifest("foo", "0.2.0"))
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .file("src/bin/foo-bin3.rs", "fn main() {}")
        .build();

    payload_process("install --force --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] foo v0.2.0 ([..])
[COMPILING] foo v0.2.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/foo-bin3[EXE]
[REPLACING] [CWD]/home/.payload/bin/foo-bin2[EXE]
[REMOVING] executable `[..]/bin/foo-bin1[EXE]` from previous version foo v0.0.1 [..]
[INSTALLED] package `foo v0.2.0 ([..]/foo2)` (executable `foo-bin3[EXE]`)
[REPLACED] package `foo v0.0.1 ([..]/foo)` with `foo v0.2.0 ([..]/foo2)` (executable `foo-bin2[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    payload_process("install --list")
        .with_stdout(
            "\
foo v0.2.0 ([..]):
    foo-bin2[..]
    foo-bin3[..]
",
        )
        .run();
}

#[payload_test]
fn install_force_bin() {
    let p = project()
        .file("src/bin/foo-bin1.rs", "fn main() {}")
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .build();

    payload_process("install --path").arg(p.root()).run();

    let p = project()
        .at("foo2")
        .file("Payload.toml", &basic_manifest("foo", "0.2.0"))
        .file("src/bin/foo-bin1.rs", "fn main() {}")
        .file("src/bin/foo-bin2.rs", "fn main() {}")
        .build();

    payload_process("install --force --bin foo-bin2 --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] foo v0.2.0 ([..])
[COMPILING] foo v0.2.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[REPLACING] [CWD]/home/.payload/bin/foo-bin2[EXE]
[REPLACED] package `foo v0.0.1 ([..]/foo)` with `foo v0.2.0 ([..]/foo2)` (executable `foo-bin2[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    payload_process("install --list")
        .with_stdout(
            "\
foo v0.0.1 ([..]):
    foo-bin1[..]
foo v0.2.0 ([..]):
    foo-bin2[..]
",
        )
        .run();
}

#[payload_test]
fn compile_failure() {
    let p = project().file("src/main.rs", "").build();

    payload_process("install --path")
        .arg(p.root())
        .with_status(101)
        .with_stderr_contains(
            "\
[ERROR] failed to compile `foo v0.0.1 ([..])`, intermediate artifacts can be \
    found at `[..]target`

Caused by:
  could not compile `foo`

To learn more, run the command again with --verbose.
",
        )
        .run();
}

#[payload_test]
fn git_repo() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    // Use `--locked` to test that we don't even try to write a lock file.
    payload_process("install --locked --git")
        .arg(p.url().to_string())
        .with_stderr(
            "\
[UPDATING] git repository `[..]`
[WARNING] no Payload.lock file published in foo v0.1.0 ([..])
[INSTALLING] foo v0.1.0 ([..])
[COMPILING] foo v0.1.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [CWD]/home/.payload/bin/foo[EXE]
[INSTALLED] package `foo v0.1.0 ([..]/foo#[..])` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(payload_home(), "foo");
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn list() {
    pkg("foo", "0.0.1");
    pkg("bar", "0.2.1");
    pkg("bar", "0.2.2");

    payload_process("install --list").with_stdout("").run();

    payload_process("install bar --vers =0.2.1").run();
    payload_process("install foo").run();
    payload_process("install --list")
        .with_stdout(
            "\
bar v0.2.1:
    bar[..]
foo v0.0.1:
    foo[..]
",
        )
        .run();
}

#[payload_test]
fn list_error() {
    pkg("foo", "0.0.1");
    payload_process("install foo").run();
    payload_process("install --list")
        .with_stdout(
            "\
foo v0.0.1:
    foo[..]
",
        )
        .run();
    let mut worldfile_path = payload_home();
    worldfile_path.push(".crates.toml");
    let mut worldfile = OpenOptions::new()
        .write(true)
        .open(worldfile_path)
        .expect(".crates.toml should be there");
    worldfile.write_all(b"\x00").unwrap();
    drop(worldfile);
    payload_process("install --list --verbose")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] failed to parse crate metadata at `[..]`

Caused by:
  invalid TOML found for metadata

Caused by:
  unexpected character[..]
",
        )
        .run();
}

#[payload_test]
fn uninstall_pkg_does_not_exist() {
    payload_process("uninstall foo")
        .with_status(101)
        .with_stderr("[ERROR] package ID specification `foo` did not match any packages")
        .run();
}

#[payload_test]
fn uninstall_bin_does_not_exist() {
    pkg("foo", "0.0.1");

    payload_process("install foo").run();
    payload_process("uninstall foo --bin=bar")
        .with_status(101)
        .with_stderr("[ERROR] binary `bar[..]` not installed as part of `foo v0.0.1`")
        .run();
}

#[payload_test]
fn uninstall_piecemeal() {
    let p = project()
        .file("src/bin/foo.rs", "fn main() {}")
        .file("src/bin/bar.rs", "fn main() {}")
        .build();

    payload_process("install --path").arg(p.root()).run();
    assert_has_installed_exe(payload_home(), "foo");
    assert_has_installed_exe(payload_home(), "bar");

    payload_process("uninstall foo --bin=bar")
        .with_stderr("[REMOVING] [..]bar[..]")
        .run();

    assert_has_installed_exe(payload_home(), "foo");
    assert_has_not_installed_exe(payload_home(), "bar");

    payload_process("uninstall foo --bin=foo")
        .with_stderr("[REMOVING] [..]foo[..]")
        .run();
    assert_has_not_installed_exe(payload_home(), "foo");

    payload_process("uninstall foo")
        .with_status(101)
        .with_stderr("[ERROR] package ID specification `foo` did not match any packages")
        .run();
}

#[payload_test]
fn subcommand_works_out_of_the_box() {
    Package::new("payload-foo", "1.0.0")
        .file("src/main.rs", r#"fn main() { println!("bar"); }"#)
        .publish();
    payload_process("install payload-foo").run();
    payload_process("foo").with_stdout("bar\n").run();
    payload_process("--list")
        .with_stdout_contains("    foo\n")
        .run();
}

#[payload_test]
fn installs_from_cwd_by_default() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.payload("install")
        .with_stderr_contains(
            "warning: Using `payload install` to install the binaries for the \
             package in current working directory is deprecated, \
             use `payload install --path .` instead. \
             Use `payload build` if you want to simply build the package.",
        )
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn installs_from_cwd_with_2018_warnings() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []
                edition = "2018"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("install")
        .with_status(101)
        .with_stderr_contains(
            "error: Using `payload install` to install the binaries for the \
             package in current working directory is no longer supported, \
             use `payload install --path .` instead. \
             Use `payload build` if you want to simply build the package.",
        )
        .run();
    assert_has_not_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn uninstall_cwd() {
    let p = project().file("src/main.rs", "fn main() {}").build();
    p.payload("install --path .")
        .with_stderr(&format!(
            "\
[INSTALLING] foo v0.0.1 ([CWD])
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] {home}/bin/foo[EXE]
[INSTALLED] package `foo v0.0.1 ([..]/foo)` (executable `foo[EXE]`)
[WARNING] be sure to add `{home}/bin` to your PATH to be able to run the installed binaries",
            home = payload_home().display(),
        ))
        .run();
    assert_has_installed_exe(payload_home(), "foo");

    p.payload("uninstall")
        .with_stdout("")
        .with_stderr(&format!(
            "[REMOVING] {home}/bin/foo[EXE]",
            home = payload_home().display()
        ))
        .run();
    assert_has_not_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn uninstall_cwd_not_installed() {
    let p = project().file("src/main.rs", "fn main() {}").build();
    p.payload("uninstall")
        .with_status(101)
        .with_stdout("")
        .with_stderr("error: package `foo v0.0.1 ([CWD])` is not installed")
        .run();
}

#[payload_test]
fn uninstall_cwd_no_project() {
    payload_process("uninstall")
        .with_status(101)
        .with_stdout("")
        .with_stderr(format!(
            "\
[ERROR] failed to read `[CWD]/Payload.toml`

Caused by:
  {err_msg}",
            err_msg = no_such_file_err_msg(),
        ))
        .run();
}

#[payload_test]
fn do_not_rebuilds_on_local_install() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    p.payload("build --release").run();
    payload_process("install --path")
        .arg(p.root())
        .with_stderr(
            "\
[INSTALLING] [..]
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]
[INSTALLED] package `foo v0.0.1 ([..]/foo)` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();

    assert!(p.build_dir().exists());
    assert!(p.release_bin("foo").exists());
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn reports_unsuccessful_subcommand_result() {
    Package::new("payload-fail", "1.0.0")
        .file("src/main.rs", "fn main() { panic!(); }")
        .publish();
    payload_process("install payload-fail").run();
    payload_process("--list")
        .with_stdout_contains("    fail\n")
        .run();
    payload_process("fail")
        .with_status(101)
        .with_stderr_contains("thread '[..]' panicked at 'explicit panic', [..]")
        .run();
}

#[payload_test]
fn git_with_lockfile() {
    let p = git::repo(&paths::root().join("foo"))
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dependencies]
                bar = { path = "bar" }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "fn main() {}")
        .file(
            "Payload.lock",
            r#"
                [[package]]
                name = "foo"
                version = "0.1.0"
                dependencies = [ "bar 0.1.0" ]

                [[package]]
                name = "bar"
                version = "0.1.0"
            "#,
        )
        .build();

    payload_process("install --git")
        .arg(p.url().to_string())
        .run();
}

#[payload_test]
fn q_silences_warnings() {
    let p = project().file("src/main.rs", "fn main() {}").build();

    payload_process("install -q --path")
        .arg(p.root())
        .with_stderr("")
        .run();
}

#[payload_test]
fn readonly_dir() {
    pkg("foo", "0.0.1");

    let root = paths::root();
    let dir = &root.join("readonly");
    fs::create_dir(root.join("readonly")).unwrap();
    let mut perms = fs::metadata(dir).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir, perms).unwrap();

    payload_process("install foo").cwd(dir).run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn use_path_workspace() {
    Package::new("foo", "1.0.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["baz"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "baz/Payload.toml",
            r#"
                [package]
                name = "baz"
                version = "0.1.0"
                authors = []

                [dependencies]
                foo = "1"
            "#,
        )
        .file("baz/src/lib.rs", "")
        .build();

    p.payload("build").run();
    let lock = p.read_lockfile();
    p.payload("install").run();
    let lock2 = p.read_lockfile();
    assert_eq!(lock, lock2, "different lockfiles");
}

#[payload_test]
fn dev_dependencies_no_check() {
    Package::new("foo", "1.0.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [dev-dependencies]
                baz = "1.0.0"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr_contains("[..] no matching package named `baz` found")
        .run();
    p.payload("install").run();
}

#[payload_test]
fn dev_dependencies_lock_file_untouched() {
    Package::new("foo", "1.0.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [dev-dependencies]
                bar = { path = "a" }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("a/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("a/src/lib.rs", "")
        .build();

    p.payload("build").run();
    let lock = p.read_lockfile();
    p.payload("install").run();
    let lock2 = p.read_lockfile();
    assert!(lock == lock2, "different lockfiles");
}

#[payload_test]
fn install_target_native() {
    pkg("foo", "0.1.0");

    payload_process("install foo --target")
        .arg(payload_test_support::rustc_host())
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn install_target_foreign() {
    if cross_compile::disabled() {
        return;
    }

    pkg("foo", "0.1.0");

    payload_process("install foo --target")
        .arg(cross_compile::alternate())
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn vers_precise() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    payload_process("install foo --vers 0.1.1")
        .with_stderr_contains("[DOWNLOADED] foo v0.1.1 (registry [..])")
        .run();
}

#[payload_test]
fn version_too() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    payload_process("install foo --version 0.1.1")
        .with_stderr_contains("[DOWNLOADED] foo v0.1.1 (registry [..])")
        .run();
}

#[payload_test]
fn not_both_vers_and_version() {
    pkg("foo", "0.1.1");
    pkg("foo", "0.1.2");

    payload_process("install foo --version 0.1.1 --vers 0.1.2")
        .with_status(1)
        .with_stderr_contains(
            "\
error: The argument '--version <VERSION>' was provided more than once, \
but cannot be used multiple times
",
        )
        .run();
}

#[payload_test]
fn test_install_git_cannot_be_a_base_url() {
    payload_process("install --git github.com:dustlang-nursery/rustfmt.git")
        .with_status(101)
        .with_stderr("\
[ERROR] invalid url `github.com:dustlang-nursery/rustfmt.git`: cannot-be-a-base-URLs are not supported")
        .run();
}

#[payload_test]
fn uninstall_multiple_and_specifying_bin() {
    payload_process("uninstall foo bar --bin baz")
        .with_status(101)
        .with_stderr("\
[ERROR] A binary can only be associated with a single installed package, specifying multiple specs with --bin is redundant.")
        .run();
}

#[payload_test]
fn uninstall_with_empty_pakcage_option() {
    payload_process("uninstall -p")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] \"--package <SPEC>\" requires a SPEC format value.
Run `payload help pkgid` for more information about SPEC format.
",
        )
        .run();
}

#[payload_test]
fn uninstall_multiple_and_some_pkg_does_not_exist() {
    pkg("foo", "0.0.1");

    payload_process("install foo").run();

    payload_process("uninstall foo bar")
        .with_status(101)
        .with_stderr(
            "\
[REMOVING] [CWD]/home/.payload/bin/foo[EXE]
error: package ID specification `bar` did not match any packages
[SUMMARY] Successfully uninstalled foo! Failed to uninstall bar (see error(s) above).
error: some packages failed to uninstall
",
        )
        .run();

    assert_has_not_installed_exe(payload_home(), "foo");
    assert_has_not_installed_exe(payload_home(), "bar");
}

#[payload_test]
fn custom_target_dir_for_git_source() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    payload_process("install --git")
        .arg(p.url().to_string())
        .run();
    assert!(!paths::root().join("target/release").is_dir());

    payload_process("install --force --git")
        .arg(p.url().to_string())
        .env("PAYLOAD_TARGET_DIR", "target")
        .run();
    assert!(paths::root().join("target/release").is_dir());
}

#[payload_test]
fn install_respects_lock_file() {
    // `payload install` now requires --locked to use a Payload.lock.
    Package::new("bar", "0.1.0").publish();
    Package::new("bar", "0.1.1")
        .file("src/lib.rs", "not rust")
        .publish();
    Package::new("foo", "0.1.0")
        .dep("bar", "0.1")
        .file("src/lib.rs", "")
        .file(
            "src/main.rs",
            "extern crate foo; extern crate bar; fn main() {}",
        )
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

    payload_process("install foo")
        .with_stderr_contains("[..]not rust[..]")
        .with_status(101)
        .run();
    payload_process("install --locked foo").run();
}

#[payload_test]
fn install_path_respects_lock_file() {
    // --path version of install_path_respects_lock_file, --locked is required
    // to use Payload.lock.
    Package::new("bar", "0.1.0").publish();
    Package::new("bar", "0.1.1")
        .file("src/lib.rs", "not rust")
        .publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            bar = "0.1"
            "#,
        )
        .file("src/main.rs", "extern crate bar; fn main() {}")
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
        .build();

    p.payload("install --path .")
        .with_stderr_contains("[..]not rust[..]")
        .with_status(101)
        .run();
    p.payload("install --path . --locked").run();
}

#[payload_test]
fn lock_file_path_deps_ok() {
    Package::new("bar", "0.1.0").publish();

    Package::new("foo", "0.1.0")
        .dep("bar", "0.1")
        .file("src/lib.rs", "")
        .file(
            "src/main.rs",
            "extern crate foo; extern crate bar; fn main() {}",
        )
        .file(
            "Payload.lock",
            r#"
            [[package]]
            name = "bar"
            version = "0.1.0"

            [[package]]
            name = "foo"
            version = "0.1.0"
            dependencies = [
             "bar 0.1.0",
            ]
            "#,
        )
        .publish();

    payload_process("install foo").run();
}

#[payload_test]
fn install_empty_argument() {
    // Bug 5229
    payload_process("install")
        .arg("")
        .with_status(1)
        .with_stderr_contains(
            "[ERROR] The argument '<crate>...' requires a value but none was supplied",
        )
        .run();
}

#[payload_test]
fn git_repo_replace() {
    let p = git::repo(&paths::root().join("foo"))
        .file("Payload.toml", &basic_manifest("foo", "0.1.0"))
        .file("src/main.rs", "fn main() {}")
        .build();
    let repo = git2::Repository::open(&p.root()).unwrap();
    let old_rev = repo.revparse_single("HEAD").unwrap().id();
    payload_process("install --git")
        .arg(p.url().to_string())
        .run();
    git::commit(&repo);
    let new_rev = repo.revparse_single("HEAD").unwrap().id();
    let mut path = paths::home();
    path.push(".payload/.crates.toml");

    assert_ne!(old_rev, new_rev);
    assert!(fs::read_to_string(path.clone())
        .unwrap()
        .contains(&format!("{}", old_rev)));
    payload_process("install --force --git")
        .arg(p.url().to_string())
        .run();
    assert!(fs::read_to_string(path)
        .unwrap()
        .contains(&format!("{}", new_rev)));
}

#[payload_test]
fn workspace_uses_workspace_target_dir() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [workspace]

                [dependencies]
                bar = { path = 'bar' }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/main.rs", "fn main() {}")
        .build();

    p.payload("build --release").cwd("bar").run();
    payload_process("install --path")
        .arg(p.root().join("bar"))
        .with_stderr(
            "[INSTALLING] [..]
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]
[INSTALLED] package `bar v0.1.0 ([..]/bar)` (executable `bar[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
}

#[payload_test]
fn install_ignores_local_payload_config() {
    pkg("bar", "0.0.1");

    let p = project()
        .file(
            ".payload/config",
            r#"
                [build]
                target = "non-existing-target"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("install bar").run();
    assert_has_installed_exe(payload_home(), "bar");
}

#[payload_test]
fn install_ignores_unstable_table_in_local_payload_config() {
    pkg("bar", "0.0.1");

    let p = project()
        .file(
            ".payload/config",
            r#"
                [unstable]
                build-std = ["core"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("install bar").masquerade_as_nightly_payload().run();
    assert_has_installed_exe(payload_home(), "bar");
}

#[payload_test]
fn install_global_payload_config() {
    pkg("bar", "0.0.1");

    let config = payload_home().join("config");
    let mut toml = fs::read_to_string(&config).unwrap_or_default();

    toml.push_str(
        r#"
            [build]
            target = 'nonexistent'
        "#,
    );
    fs::write(&config, toml).unwrap();

    payload_process("install bar")
        .with_status(101)
        .with_stderr_contains("[..]--target nonexistent[..]")
        .run();
}

#[payload_test]
fn install_path_config() {
    project()
        .file(
            ".payload/config",
            r#"
            [build]
            target = 'nonexistent'
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();
    payload_process("install --path foo")
        .with_status(101)
        .with_stderr_contains("[..]--target nonexistent[..]")
        .run();
}

#[payload_test]
fn install_version_req() {
    // Try using a few versionreq styles.
    pkg("foo", "0.0.3");
    pkg("foo", "1.0.4");
    pkg("foo", "1.0.5");
    payload_process("install foo --version=*")
        .with_stderr_does_not_contain("[WARNING][..]is not a valid semver[..]")
        .with_stderr_contains("[INSTALLING] foo v1.0.5")
        .run();
    payload_process("uninstall foo").run();
    payload_process("install foo --version=^1.0")
        .with_stderr_does_not_contain("[WARNING][..]is not a valid semver[..]")
        .with_stderr_contains("[INSTALLING] foo v1.0.5")
        .run();
    payload_process("uninstall foo").run();
    payload_process("install foo --version=0.0.*")
        .with_stderr_does_not_contain("[WARNING][..]is not a valid semver[..]")
        .with_stderr_contains("[INSTALLING] foo v0.0.3")
        .run();
}

#[payload_test]
fn git_install_reads_workspace_manifest() {
    let p = git::repo(&paths::root().join("foo"))
        .file(
            "Payload.toml",
            r#"
            [workspace]
            members = ["bin1"]

            [profile.release]
            incremental = 3
            "#,
        )
        .file("bin1/Payload.toml", &basic_manifest("bin1", "0.1.0"))
        .file(
            "bin1/src/main.rs",
            r#"fn main() { println!("Hello, world!"); }"#,
        )
        .build();

    payload_process(&format!("install --git {}", p.url().to_string()))
        .with_status(101)
        .with_stderr_contains("  invalid type: integer `3`[..]")
        .run();
}

#[payload_test]
fn install_git_with_symlink_home() {
    // Ensure that `payload install` with a git repo is OK when PAYLOAD_HOME is a
    // symlink, and uses an build script.
    if !symlink_supported() {
        return;
    }
    let p = git::new("foo", |p| {
        p.file("Payload.toml", &basic_manifest("foo", "1.0.0"))
            .file("src/main.rs", "fn main() {}")
            // This triggers discover_git_and_list_files for detecting changed files.
            .file("build.rs", "fn main() {}")
    });
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(windows)]
    use std::os::windows::fs::symlink_dir as symlink;

    let actual = paths::root().join("actual-home");
    t!(std::fs::create_dir(&actual));
    t!(symlink(&actual, paths::home().join(".payload")));
    payload_process("install --git")
        .arg(p.url().to_string())
        .with_stderr(
            "\
[UPDATING] git repository [..]
[INSTALLING] foo v1.0.0 [..]
[COMPILING] foo v1.0.0 [..]
[FINISHED] [..]
[INSTALLING] [..]home/.payload/bin/foo[..]
[INSTALLED] package `foo [..]
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[payload_test]
fn install_yanked_payload_package() {
    Package::new("baz", "0.0.1").yanked(true).publish();
    payload_process("install baz --version 0.0.1")
        .with_status(101)
        .with_stderr_contains(
            "error: cannot install package `baz`, it has been yanked from registry \
         `https://github.com/dustlang/crates.io-index`",
        )
        .run();
}

#[payload_test]
fn install_payload_package_in_a_patched_workspace() {
    pkg("foo", "0.1.0");
    pkg("fizz", "1.0.0");

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["baz"]
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "baz/Payload.toml",
            r#"
                [package]
                name = "baz"
                version = "0.1.0"
                authors = []

                [dependencies]
                fizz = "1"

                [patch.crates-io]
                fizz = { version = "=1.0.0" }
            "#,
        )
        .file("baz/src/lib.rs", "")
        .build();

    let stderr = "\
[WARNING] patch for the non root package will be ignored, specify patch at the workspace root:
package:   [..]/foo/baz/Payload.toml
workspace: [..]/foo/Payload.toml
";
    p.payload("check").with_stderr_contains(&stderr).run();

    // A crate installation must not emit any message from a workspace under
    // current working directory.
    // See https://github.com/dustlang/payload/issues/8619
    p.payload("install foo")
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] foo v0.1.0 (registry [..])
[INSTALLING] foo v0.1.0
[COMPILING] foo v0.1.0
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]foo[EXE]
[INSTALLED] package `foo v0.1.0` (executable `foo[EXE]`)
[WARNING] be sure to add `[..]` to your PATH to be able to run the installed binaries
",
        )
        .run();
    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn locked_install_without_published_lockfile() {
    Package::new("foo", "0.1.0")
        .file("src/main.rs", "//! Some docs\nfn main() {}")
        .publish();

    payload_process("install foo --locked")
        .with_stderr_contains("[WARNING] no Payload.lock file published in foo v0.1.0")
        .run();
}
