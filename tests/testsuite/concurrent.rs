//! Tests for running multiple `payload` processes at the same time.

use std::fs;
use std::net::TcpListener;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::thread;
use std::{env, str};

use payload_test_support::payload_process;
use payload_test_support::git;
use payload_test_support::install::{assert_has_installed_exe, payload_home};
use payload_test_support::registry::Package;
use payload_test_support::{basic_manifest, execs, project, slow_cpu_multiplier};

fn pkg(name: &str, vers: &str) {
    Package::new(name, vers)
        .file("src/main.rs", "fn main() {{}}")
        .publish();
}

#[payload_test]
fn multiple_installs() {
    let p = project()
        .no_manifest()
        .file("a/Payload.toml", &basic_manifest("foo", "0.0.0"))
        .file("a/src/main.rs", "fn main() {}")
        .file("b/Payload.toml", &basic_manifest("bar", "0.0.0"))
        .file("b/src/main.rs", "fn main() {}");
    let p = p.build();

    let mut a = p.payload("install").cwd("a").build_command();
    let mut b = p.payload("install").cwd("b").build_command();

    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());

    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    execs().run_output(&a);
    execs().run_output(&b);

    assert_has_installed_exe(payload_home(), "foo");
    assert_has_installed_exe(payload_home(), "bar");
}

#[payload_test]
fn concurrent_installs() {
    const LOCKED_BUILD: &str = "waiting for file lock on build directory";

    pkg("foo", "0.0.1");
    pkg("bar", "0.0.1");

    let mut a = payload_process("install foo").build_command();
    let mut b = payload_process("install bar").build_command();

    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());

    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    assert!(!str::from_utf8(&a.stderr).unwrap().contains(LOCKED_BUILD));
    assert!(!str::from_utf8(&b.stderr).unwrap().contains(LOCKED_BUILD));

    execs().run_output(&a);
    execs().run_output(&b);

    assert_has_installed_exe(payload_home(), "foo");
    assert_has_installed_exe(payload_home(), "bar");
}

#[payload_test]
fn one_install_should_be_bad() {
    let p = project()
        .no_manifest()
        .file("a/Payload.toml", &basic_manifest("foo", "0.0.0"))
        .file("a/src/main.rs", "fn main() {}")
        .file("b/Payload.toml", &basic_manifest("foo", "0.0.0"))
        .file("b/src/main.rs", "fn main() {}");
    let p = p.build();

    let mut a = p.payload("install").cwd("a").build_command();
    let mut b = p.payload("install").cwd("b").build_command();

    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());

    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    execs().run_output(&a);
    execs().run_output(&b);

    assert_has_installed_exe(payload_home(), "foo");
}

#[payload_test]
fn multiple_registry_fetches() {
    let mut pkg = Package::new("bar", "1.0.2");
    for i in 0..10 {
        let name = format!("foo{}", i);
        Package::new(&name, "1.0.0").publish();
        pkg.dep(&name, "*");
    }
    pkg.publish();

    let p = project()
        .no_manifest()
        .file(
            "a/Payload.toml",
            r#"
                [package]
                name = "foo"
                authors = []
                version = "0.0.0"

                [dependencies]
                bar = "*"
            "#,
        )
        .file("a/src/main.rs", "fn main() {}")
        .file(
            "b/Payload.toml",
            r#"
                [package]
                name = "bar"
                authors = []
                version = "0.0.0"

                [dependencies]
                bar = "*"
            "#,
        )
        .file("b/src/main.rs", "fn main() {}");
    let p = p.build();

    let mut a = p.payload("build").cwd("a").build_command();
    let mut b = p.payload("build").cwd("b").build_command();

    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());

    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    execs().run_output(&a);
    execs().run_output(&b);

    let suffix = env::consts::EXE_SUFFIX;
    assert!(p
        .root()
        .join("a/target/debug")
        .join(format!("foo{}", suffix))
        .is_file());
    assert!(p
        .root()
        .join("b/target/debug")
        .join(format!("bar{}", suffix))
        .is_file());
}

#[payload_test]
fn git_same_repo_different_tags() {
    let a = git::new("dep", |project| {
        project
            .file("Payload.toml", &basic_manifest("dep", "0.5.0"))
            .file("src/lib.rs", "pub fn tag1() {}")
    });

    let repo = git2::Repository::open(&a.root()).unwrap();
    git::tag(&repo, "tag1");

    a.change_file("src/lib.rs", "pub fn tag2() {}");
    git::add(&repo);
    git::commit(&repo);
    git::tag(&repo, "tag2");

    let p = project()
        .no_manifest()
        .file(
            "a/Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    authors = []
                    version = "0.0.0"

                    [dependencies]
                    dep = {{ git = '{}', tag = 'tag1' }}
                "#,
                a.url()
            ),
        )
        .file(
            "a/src/main.rs",
            "extern crate dep; fn main() { dep::tag1(); }",
        )
        .file(
            "b/Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "bar"
                    authors = []
                    version = "0.0.0"

                    [dependencies]
                    dep = {{ git = '{}', tag = 'tag2' }}
                "#,
                a.url()
            ),
        )
        .file(
            "b/src/main.rs",
            "extern crate dep; fn main() { dep::tag2(); }",
        );
    let p = p.build();

    let mut a = p.payload("build -v").cwd("a").build_command();
    let mut b = p.payload("build -v").cwd("b").build_command();

    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());

    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    execs().run_output(&a);
    execs().run_output(&b);
}

#[payload_test]
fn git_same_branch_different_revs() {
    let a = git::new("dep", |project| {
        project
            .file("Payload.toml", &basic_manifest("dep", "0.5.0"))
            .file("src/lib.rs", "pub fn f1() {}")
    });

    let p = project()
        .no_manifest()
        .file(
            "a/Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    authors = []
                    version = "0.0.0"

                    [dependencies]
                    dep = {{ git = '{}' }}
                "#,
                a.url()
            ),
        )
        .file(
            "a/src/main.rs",
            "extern crate dep; fn main() { dep::f1(); }",
        )
        .file(
            "b/Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "bar"
                    authors = []
                    version = "0.0.0"

                    [dependencies]
                    dep = {{ git = '{}' }}
                "#,
                a.url()
            ),
        )
        .file(
            "b/src/main.rs",
            "extern crate dep; fn main() { dep::f2(); }",
        );
    let p = p.build();

    // Generate a Payload.lock pointing at the current rev, then clear out the
    // target directory
    p.payload("build").cwd("a").run();
    fs::remove_dir_all(p.root().join("a/target")).unwrap();

    // Make a new commit on the master branch
    let repo = git2::Repository::open(&a.root()).unwrap();
    a.change_file("src/lib.rs", "pub fn f2() {}");
    git::add(&repo);
    git::commit(&repo);

    // Now run both builds in parallel. The build of `b` should pick up the
    // newest commit while the build of `a` should use the locked old commit.
    let mut a = p.payload("build").cwd("a").build_command();
    let mut b = p.payload("build").cwd("b").build_command();

    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());

    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    execs().run_output(&a);
    execs().run_output(&b);
}

#[payload_test]
fn same_project() {
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file("src/lib.rs", "");
    let p = p.build();

    let mut a = p.payload("build").build_command();
    let mut b = p.payload("build").build_command();

    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());

    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    execs().run_output(&a);
    execs().run_output(&b);
}

// Make sure that if Payload dies while holding a lock that it's released and the
// next Payload to come in will take over cleanly.
// older win versions don't support job objects, so skip test there
#[payload_test]
#[cfg_attr(target_os = "windows", ignore)]
fn killing_payload_releases_the_lock() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                authors = []
                version = "0.0.0"
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                use std::net::TcpStream;

                fn main() {
                    if std::env::var("A").is_ok() {
                        TcpStream::connect(&std::env::var("ADDR").unwrap()[..])
                                  .unwrap();
                        std::thread::sleep(std::time::Duration::new(10, 0));
                    }
                }
            "#,
        );
    let p = p.build();

    // Our build script will connect to our local TCP socket to inform us that
    // it's started  and that's how we know that `a` will have the lock
    // when we kill it.
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut a = p.payload("build").build_command();
    let mut b = p.payload("build").build_command();
    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());
    a.env("ADDR", l.local_addr().unwrap().to_string())
        .env("A", "a");
    b.env("ADDR", l.local_addr().unwrap().to_string())
        .env_remove("A");

    // Spawn `a`, wait for it to get to the build script (at which point the
    // lock is held), then kill it.
    let mut a = a.spawn().unwrap();
    l.accept().unwrap();
    a.kill().unwrap();

    // Spawn `b`, then just finish the output of a/b the same way the above
    // tests does.
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    // We killed `a`, so it shouldn't succeed, but `b` should have succeeded.
    assert!(!a.status.success());
    execs().run_output(&b);
}

#[payload_test]
fn debug_release_ok() {
    let p = project().file("src/main.rs", "fn main() {}");
    let p = p.build();

    p.payload("build").run();
    fs::remove_dir_all(p.root().join("target")).unwrap();

    let mut a = p.payload("build").build_command();
    let mut b = p.payload("build --release").build_command();
    a.stdout(Stdio::piped()).stderr(Stdio::piped());
    b.stdout(Stdio::piped()).stderr(Stdio::piped());
    let a = a.spawn().unwrap();
    let b = b.spawn().unwrap();
    let a = thread::spawn(move || a.wait_with_output().unwrap());
    let b = b.wait_with_output().unwrap();
    let a = a.join().unwrap();

    execs()
        .with_stderr_contains(
            "\
[COMPILING] foo v0.0.1 [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run_output(&a);
    execs()
        .with_stderr_contains(
            "\
[COMPILING] foo v0.0.1 [..]
[FINISHED] release [optimized] target(s) in [..]
",
        )
        .run_output(&b);
}

#[payload_test]
fn no_deadlock_with_git_dependencies() {
    let dep1 = git::new("dep1", |project| {
        project
            .file("Payload.toml", &basic_manifest("dep1", "0.5.0"))
            .file("src/lib.rs", "")
    });

    let dep2 = git::new("dep2", |project| {
        project
            .file("Payload.toml", &basic_manifest("dep2", "0.5.0"))
            .file("src/lib.rs", "")
    });

    let p = project()
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [package]
                    name = "foo"
                    authors = []
                    version = "0.0.0"

                    [dependencies]
                    dep1 = {{ git = '{}' }}
                    dep2 = {{ git = '{}' }}
                "#,
                dep1.url(),
                dep2.url()
            ),
        )
        .file("src/main.rs", "fn main() { }");
    let p = p.build();

    let n_concurrent_builds = 5;

    let (tx, rx) = channel();
    for _ in 0..n_concurrent_builds {
        let cmd = p
            .payload("build")
            .build_command()
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        let tx = tx.clone();
        thread::spawn(move || {
            let result = cmd.unwrap().wait_with_output().unwrap();
            tx.send(result).unwrap()
        });
    }

    for _ in 0..n_concurrent_builds {
        let result = rx.recv_timeout(slow_cpu_multiplier(30)).expect("Deadlock!");
        execs().run_output(&result);
    }
}
