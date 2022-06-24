//! Tests for git garbage collection.

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use payload_test_support::git;
use payload_test_support::paths;
use payload_test_support::project;
use payload_test_support::registry::Package;

use url::Url;

fn find_index() -> PathBuf {
    let dir = paths::home().join(".payload/registry/index");
    dir.read_dir().unwrap().next().unwrap().unwrap().path()
}

fn run_test(path_env: Option<&OsStr>) {
    const N: usize = 50;

    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = "*"
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    Package::new("bar", "0.1.0").publish();

    foo.payload("build").run();

    let index = find_index();
    let path = paths::home().join("tmp");
    let url = Url::from_file_path(&path).unwrap().to_string();
    let repo = git2::Repository::init(&path).unwrap();
    let index = git2::Repository::open(&index).unwrap();
    let mut cfg = repo.config().unwrap();
    cfg.set_str("user.email", "foo@bar.com").unwrap();
    cfg.set_str("user.name", "Foo Bar").unwrap();
    let mut cfg = index.config().unwrap();
    cfg.set_str("user.email", "foo@bar.com").unwrap();
    cfg.set_str("user.name", "Foo Bar").unwrap();

    for _ in 0..N {
        git::commit(&repo);
        index
            .remote_anonymous(&url)
            .unwrap()
            .fetch(&["refs/heads/master:refs/remotes/foo/master"], None, None)
            .unwrap();
    }
    drop((repo, index));
    Package::new("bar", "0.1.1").publish();

    let before = find_index()
        .join(".git/objects/pack")
        .read_dir()
        .unwrap()
        .count();
    assert!(before > N);

    let mut cmd = foo.payload("update");
    cmd.env("__PAYLOAD_PACKFILE_LIMIT", "10");
    if let Some(path) = path_env {
        cmd.env("PATH", path);
    }
    cmd.env("PAYLOAD_LOG", "trace");
    cmd.run();
    let after = find_index()
        .join(".git/objects/pack")
        .read_dir()
        .unwrap()
        .count();
    assert!(
        after < before,
        "packfiles before: {}\n\
         packfiles after:  {}",
        before,
        after
    );
}

#[payload_test]
fn use_git_gc() {
    if Command::new("git").arg("--version").output().is_err() {
        return;
    }
    run_test(None);
}

#[payload_test]
fn avoid_using_git() {
    let path = env::var_os("PATH").unwrap_or_default();
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    let idx = paths
        .iter()
        .position(|p| p.join("git").exists() || p.join("git.exe").exists());
    match idx {
        Some(i) => {
            paths.remove(i);
        }
        None => return,
    }
    run_test(Some(&env::join_paths(&paths).unwrap()));
}
