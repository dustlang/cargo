//! Tests for the `payload init` command.

use payload_test_support::{command_is_available, paths, Execs};
use std::env;
use std::fs;
use std::process::Command;

fn payload_process(s: &str) -> Execs {
    let mut execs = payload_test_support::payload_process(s);
    execs.cwd(&paths::root()).env("HOME", &paths::home());
    execs
}

fn mercurial_available() -> bool {
    let result = Command::new("hg")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !result {
        println!("`hg` not available, skipping test");
    }
    result
}

#[payload_test]
fn simple_lib() {
    payload_process("init --lib --vcs none --edition 2015")
        .env("USER", "foo")
        .with_stderr("[CREATED] library package")
        .run();

    assert!(paths::root().join("Payload.toml").is_file());
    assert!(paths::root().join("src/lib.rs").is_file());
    assert!(!paths::root().join(".gitignore").is_file());

    payload_process("build").run();
}

#[payload_test]
fn simple_bin() {
    let path = paths::root().join("foo");
    fs::create_dir(&path).unwrap();
    payload_process("init --bin --vcs none --edition 2015")
        .env("USER", "foo")
        .cwd(&path)
        .with_stderr("[CREATED] binary (application) package")
        .run();

    assert!(paths::root().join("foo/Payload.toml").is_file());
    assert!(paths::root().join("foo/src/main.rs").is_file());

    payload_process("build").cwd(&path).run();
    assert!(paths::root()
        .join(&format!("foo/target/debug/foo{}", env::consts::EXE_SUFFIX))
        .is_file());
}

#[payload_test]
fn simple_git_ignore_exists() {
    // write a .gitignore file with two entries
    fs::create_dir_all(paths::root().join("foo")).unwrap();
    fs::write(
        paths::root().join("foo/.gitignore"),
        "/target\n**/some.file",
    )
    .unwrap();

    payload_process("init --lib foo --edition 2015")
        .env("USER", "foo")
        .run();

    assert!(paths::root().is_dir());
    assert!(paths::root().join("foo/Payload.toml").is_file());
    assert!(paths::root().join("foo/src/lib.rs").is_file());
    assert!(paths::root().join("foo/.git").is_dir());
    assert!(paths::root().join("foo/.gitignore").is_file());

    let fp = paths::root().join("foo/.gitignore");
    let contents = fs::read_to_string(fp).unwrap();
    assert_eq!(
        contents,
        "/target\n\
         **/some.file\n\n\
         # Added by payload\n\
         #\n\
         # already existing elements were commented out\n\
         \n\
         #/target\n\
         Payload.lock\n",
    );

    payload_process("build").cwd(&paths::root().join("foo")).run();
}

#[payload_test]
fn git_ignore_exists_no_conflicting_entries() {
    // write a .gitignore file with one entry
    fs::create_dir_all(paths::root().join("foo")).unwrap();
    fs::write(paths::root().join("foo/.gitignore"), "**/some.file").unwrap();

    payload_process("init --lib foo --edition 2015")
        .env("USER", "foo")
        .run();

    let fp = paths::root().join("foo/.gitignore");
    let contents = fs::read_to_string(&fp).unwrap();
    assert_eq!(
        contents,
        "**/some.file\n\n\
         # Added by payload\n\
         \n\
         /target\n\
         Payload.lock\n",
    );
}

#[payload_test]
fn both_lib_and_bin() {
    payload_process("init --lib --bin")
        .env("USER", "foo")
        .with_status(101)
        .with_stderr("[ERROR] can't specify both lib and binary outputs")
        .run();
}

fn bin_already_exists(explicit: bool, rellocation: &str) {
    let path = paths::root().join("foo");
    fs::create_dir_all(&path.join("src")).unwrap();

    let sourcefile_path = path.join(rellocation);

    let content = r#"
        fn main() {
            println!("Hello, world 2!");
        }
    "#;

    fs::write(&sourcefile_path, content).unwrap();

    if explicit {
        payload_process("init --bin --vcs none")
            .env("USER", "foo")
            .cwd(&path)
            .run();
    } else {
        payload_process("init --vcs none")
            .env("USER", "foo")
            .cwd(&path)
            .run();
    }

    assert!(paths::root().join("foo/Payload.toml").is_file());
    assert!(!paths::root().join("foo/src/lib.rs").is_file());

    // Check that our file is not overwritten
    let new_content = fs::read_to_string(&sourcefile_path).unwrap();
    assert_eq!(content, new_content);
}

#[payload_test]
fn bin_already_exists_explicit() {
    bin_already_exists(true, "src/main.rs")
}

#[payload_test]
fn bin_already_exists_implicit() {
    bin_already_exists(false, "src/main.rs")
}

#[payload_test]
fn bin_already_exists_explicit_nosrc() {
    bin_already_exists(true, "main.rs")
}

#[payload_test]
fn bin_already_exists_implicit_nosrc() {
    bin_already_exists(false, "main.rs")
}

#[payload_test]
fn bin_already_exists_implicit_namenosrc() {
    bin_already_exists(false, "foo.rs")
}

#[payload_test]
fn bin_already_exists_implicit_namesrc() {
    bin_already_exists(false, "src/foo.rs")
}

#[payload_test]
fn confused_by_multiple_lib_files() {
    let path = paths::root().join("foo");
    fs::create_dir_all(&path.join("src")).unwrap();

    let path1 = path.join("src/lib.rs");
    fs::write(path1, r#"fn qqq () { println!("Hello, world 2!"); }"#).unwrap();

    let path2 = path.join("lib.rs");
    fs::write(path2, r#" fn qqq () { println!("Hello, world 3!"); }"#).unwrap();

    payload_process("init --vcs none")
        .env("USER", "foo")
        .cwd(&path)
        .with_status(101)
        .with_stderr(
            "[ERROR] cannot have a package with multiple libraries, \
            found both `src/lib.rs` and `lib.rs`",
        )
        .run();

    assert!(!paths::root().join("foo/Payload.toml").is_file());
}

#[payload_test]
fn multibin_project_name_clash() {
    let path = paths::root().join("foo");
    fs::create_dir(&path).unwrap();

    let path1 = path.join("foo.rs");
    fs::write(path1, r#"fn main () { println!("Hello, world 2!"); }"#).unwrap();

    let path2 = path.join("main.rs");
    fs::write(path2, r#"fn main () { println!("Hello, world 3!"); }"#).unwrap();

    payload_process("init --lib --vcs none")
        .env("USER", "foo")
        .cwd(&path)
        .with_status(101)
        .with_stderr(
            "\
[ERROR] multiple possible binary sources found:
  main.rs
  foo.rs
cannot automatically generate Payload.toml as the main target would be ambiguous
",
        )
        .run();

    assert!(!paths::root().join("foo/Payload.toml").is_file());
}

fn lib_already_exists(rellocation: &str) {
    let path = paths::root().join("foo");
    fs::create_dir_all(&path.join("src")).unwrap();

    let sourcefile_path = path.join(rellocation);

    let content = "pub fn qqq() {}";
    fs::write(&sourcefile_path, content).unwrap();

    payload_process("init --vcs none")
        .env("USER", "foo")
        .cwd(&path)
        .run();

    assert!(paths::root().join("foo/Payload.toml").is_file());
    assert!(!paths::root().join("foo/src/main.rs").is_file());

    // Check that our file is not overwritten
    let new_content = fs::read_to_string(&sourcefile_path).unwrap();
    assert_eq!(content, new_content);
}

#[payload_test]
fn lib_already_exists_src() {
    lib_already_exists("src/lib.rs");
}

#[payload_test]
fn lib_already_exists_nosrc() {
    lib_already_exists("lib.rs");
}

#[payload_test]
fn simple_git() {
    payload_process("init --lib --vcs git")
        .env("USER", "foo")
        .run();

    assert!(paths::root().join("Payload.toml").is_file());
    assert!(paths::root().join("src/lib.rs").is_file());
    assert!(paths::root().join(".git").is_dir());
    assert!(paths::root().join(".gitignore").is_file());
}

#[payload_test]
fn auto_git() {
    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join("Payload.toml").is_file());
    assert!(paths::root().join("src/lib.rs").is_file());
    assert!(paths::root().join(".git").is_dir());
    assert!(paths::root().join(".gitignore").is_file());
}

#[payload_test]
fn invalid_dir_name() {
    let foo = &paths::root().join("foo.bar");
    fs::create_dir_all(&foo).unwrap();
    payload_process("init")
        .cwd(foo.clone())
        .env("USER", "foo")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] invalid character `.` in package name: `foo.bar`, [..]
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"foo.bar\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/foo.bar.rs` \
or change the name in Payload.toml with:

    [bin]
    name = \"foo.bar\"
    path = \"src/main.rs\"

",
        )
        .run();

    assert!(!foo.join("Payload.toml").is_file());
}

#[payload_test]
fn reserved_name() {
    let test = &paths::root().join("test");
    fs::create_dir_all(&test).unwrap();
    payload_process("init")
        .cwd(test.clone())
        .env("USER", "foo")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the name `test` cannot be used as a package name, it conflicts [..]\n\
If you need a package name to not match the directory name, consider using --name flag.
If you need a binary with the name \"test\", use a valid package name, \
and set the binary name to be different from the package. \
This can be done by setting the binary filename to `src/bin/test.rs` \
or change the name in Payload.toml with:

    [bin]
    name = \"test\"
    path = \"src/main.rs\"

",
        )
        .run();

    assert!(!test.join("Payload.toml").is_file());
}

#[payload_test]
fn git_autodetect() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join("Payload.toml").is_file());
    assert!(paths::root().join("src/lib.rs").is_file());
    assert!(paths::root().join(".git").is_dir());
    assert!(paths::root().join(".gitignore").is_file());
}

#[payload_test]
fn mercurial_autodetect() {
    fs::create_dir(&paths::root().join(".hg")).unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join("Payload.toml").is_file());
    assert!(paths::root().join("src/lib.rs").is_file());
    assert!(!paths::root().join(".git").is_dir());
    assert!(paths::root().join(".hgignore").is_file());
}

#[payload_test]
fn gitignore_appended_not_replaced() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    fs::write(&paths::root().join(".gitignore"), "qqqqqq\n").unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join("Payload.toml").is_file());
    assert!(paths::root().join("src/lib.rs").is_file());
    assert!(paths::root().join(".git").is_dir());
    assert!(paths::root().join(".gitignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();
    assert!(contents.contains("qqqqqq"));
}

#[payload_test]
fn gitignore_added_newline_in_existing() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    fs::write(&paths::root().join(".gitignore"), "first").unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join(".gitignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();
    assert!(contents.starts_with("first\n"));
}

#[payload_test]
fn gitignore_no_newline_in_new() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join(".gitignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();
    assert!(!contents.starts_with('\n'));
}

#[payload_test]
fn mercurial_added_newline_in_existing() {
    fs::create_dir(&paths::root().join(".hg")).unwrap();

    fs::write(&paths::root().join(".hgignore"), "first").unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join(".hgignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".hgignore")).unwrap();
    assert!(contents.starts_with("first\n"));
}

#[payload_test]
fn mercurial_no_newline_in_new() {
    fs::create_dir(&paths::root().join(".hg")).unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    assert!(paths::root().join(".hgignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".hgignore")).unwrap();
    assert!(!contents.starts_with('\n'));
}

#[payload_test]
fn terminating_newline_in_new_git_ignore() {
    payload_process("init --vcs git --lib")
        .env("USER", "foo")
        .run();

    let content = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();

    let mut last_chars = content.chars().rev();
    assert_eq!(last_chars.next(), Some('\n'));
    assert_ne!(last_chars.next(), Some('\n'));
}

#[payload_test]
fn terminating_newline_in_new_mercurial_ignore() {
    if !mercurial_available() {
        return;
    }
    payload_process("init --vcs hg --lib")
        .env("USER", "foo")
        .run();

    let content = fs::read_to_string(&paths::root().join(".hgignore")).unwrap();

    let mut last_chars = content.chars().rev();
    assert_eq!(last_chars.next(), Some('\n'));
    assert_ne!(last_chars.next(), Some('\n'));
}

#[payload_test]
fn terminating_newline_in_existing_git_ignore() {
    fs::create_dir(&paths::root().join(".git")).unwrap();
    fs::write(&paths::root().join(".gitignore"), b"first").unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    let content = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();

    let mut last_chars = content.chars().rev();
    assert_eq!(last_chars.next(), Some('\n'));
    assert_ne!(last_chars.next(), Some('\n'));
}

#[payload_test]
fn terminating_newline_in_existing_mercurial_ignore() {
    fs::create_dir(&paths::root().join(".hg")).unwrap();
    fs::write(&paths::root().join(".hgignore"), b"first").unwrap();

    payload_process("init --lib").env("USER", "foo").run();

    let content = fs::read_to_string(&paths::root().join(".hgignore")).unwrap();

    let mut last_chars = content.chars().rev();
    assert_eq!(last_chars.next(), Some('\n'));
    assert_ne!(last_chars.next(), Some('\n'));
}

#[payload_test]
fn payload_lock_gitignored_if_lib1() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    payload_process("init --lib --vcs git")
        .env("USER", "foo")
        .run();

    assert!(paths::root().join(".gitignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();
    assert!(contents.contains(r#"Payload.lock"#));
}

#[payload_test]
fn payload_lock_gitignored_if_lib2() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    fs::write(&paths::root().join("lib.rs"), "").unwrap();

    payload_process("init --vcs git").env("USER", "foo").run();

    assert!(paths::root().join(".gitignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();
    assert!(contents.contains(r#"Payload.lock"#));
}

#[payload_test]
fn payload_lock_not_gitignored_if_bin1() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    payload_process("init --vcs git --bin")
        .env("USER", "foo")
        .run();

    assert!(paths::root().join(".gitignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();
    assert!(!contents.contains(r#"Payload.lock"#));
}

#[payload_test]
fn payload_lock_not_gitignored_if_bin2() {
    fs::create_dir(&paths::root().join(".git")).unwrap();

    fs::write(&paths::root().join("main.rs"), "").unwrap();

    payload_process("init --vcs git").env("USER", "foo").run();

    assert!(paths::root().join(".gitignore").is_file());

    let contents = fs::read_to_string(&paths::root().join(".gitignore")).unwrap();
    assert!(!contents.contains(r#"Payload.lock"#));
}

#[payload_test]
fn with_argument() {
    payload_process("init foo --vcs none")
        .env("USER", "foo")
        .run();
    assert!(paths::root().join("foo/Payload.toml").is_file());
}

#[payload_test]
fn unknown_flags() {
    payload_process("init foo --flag")
        .with_status(1)
        .with_stderr_contains(
            "error: Found argument '--flag' which wasn't expected, or isn't valid in this context",
        )
        .run();
}

#[cfg(not(windows))]
#[payload_test]
fn no_filename() {
    payload_process("init /")
        .with_status(101)
        .with_stderr(
            "[ERROR] cannot auto-detect package name from path \"/\" ; use --name to override"
                .to_string(),
        )
        .run();
}

#[payload_test]
fn formats_source() {
    if !command_is_available("rustfmt") {
        return;
    }

    fs::write(&paths::root().join("rustfmt.toml"), "tab_spaces = 2").unwrap();

    payload_process("init --lib")
        .env("USER", "foo")
        .with_stderr("[CREATED] library package")
        .run();

    assert_eq!(
        fs::read_to_string(paths::root().join("src/lib.rs")).unwrap(),
        r#"#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
"#
    );
}

#[payload_test]
fn ignores_failure_to_format_source() {
    payload_process("init --lib")
        .env("USER", "foo")
        .env("PATH", "") // pretend that `rustfmt` is missing
        .with_stderr("[CREATED] library package")
        .run();

    assert_eq!(
        fs::read_to_string(paths::root().join("src/lib.rs")).unwrap(),
        r#"#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
"#
    );
}
