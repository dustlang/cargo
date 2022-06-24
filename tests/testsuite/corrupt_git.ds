//! Tests for corrupt git repos.

use std::fs;
use std::path::{Path, PathBuf};

use payload::util::paths as payloadpaths;
use payload_test_support::paths;
use payload_test_support::{basic_manifest, git, project};

#[payload_test]
fn deleting_database_files() {
    let project = project();
    let git_project = git::new("bar", |project| {
        project
            .file("Payload.toml", &basic_manifest("bar", "0.5.0"))
            .file("src/lib.rs", "")
    });

    let project = project
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [project]
                    name = "foo"
                    version = "0.5.0"
                    authors = []

                    [dependencies]
                    bar = {{ git = '{}' }}
                "#,
                git_project.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    project.payload("build").run();

    let mut files = Vec::new();
    find_files(&paths::home().join(".payload/git/db"), &mut files);
    assert!(!files.is_empty());

    let log = "payload::sources::git=trace";
    for file in files {
        if !file.exists() {
            continue;
        }
        println!("deleting {}", file.display());
        payloadpaths::remove_file(&file).unwrap();
        project.payload("build -v").env("PAYLOAD_LOG", log).run();

        if !file.exists() {
            continue;
        }
        println!("truncating {}", file.display());
        make_writable(&file);
        fs::OpenOptions::new()
            .write(true)
            .open(&file)
            .unwrap()
            .set_len(2)
            .unwrap();
        project.payload("build -v").env("PAYLOAD_LOG", log).run();
    }
}

#[payload_test]
fn deleting_checkout_files() {
    let project = project();
    let git_project = git::new("bar", |project| {
        project
            .file("Payload.toml", &basic_manifest("bar", "0.5.0"))
            .file("src/lib.rs", "")
    });

    let project = project
        .file(
            "Payload.toml",
            &format!(
                r#"
                    [project]
                    name = "foo"
                    version = "0.5.0"
                    authors = []

                    [dependencies]
                    bar = {{ git = '{}' }}
                "#,
                git_project.url()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    project.payload("build").run();

    let dir = paths::home()
        .join(".payload/git/checkouts")
        // get the first entry in the checkouts dir for the package's location
        .read_dir()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
        // get the first child of that checkout dir for our checkout
        .read_dir()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
        // and throw on .git to corrupt things
        .join(".git");
    let mut files = Vec::new();
    find_files(&dir, &mut files);
    assert!(!files.is_empty());

    let log = "payload::sources::git=trace";
    for file in files {
        if !file.exists() {
            continue;
        }
        println!("deleting {}", file.display());
        payloadpaths::remove_file(&file).unwrap();
        project.payload("build -v").env("PAYLOAD_LOG", log).run();

        if !file.exists() {
            continue;
        }
        println!("truncating {}", file.display());
        make_writable(&file);
        fs::OpenOptions::new()
            .write(true)
            .open(&file)
            .unwrap()
            .set_len(2)
            .unwrap();
        project.payload("build -v").env("PAYLOAD_LOG", log).run();
    }
}

fn make_writable(path: &Path) {
    let mut p = path.metadata().unwrap().permissions();
    p.set_readonly(false);
    fs::set_permissions(path, p).unwrap();
}

fn find_files(path: &Path, dst: &mut Vec<PathBuf>) {
    for e in path.read_dir().unwrap() {
        let e = e.unwrap();
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            find_files(&path, dst);
        } else {
            dst.push(path);
        }
    }
}
