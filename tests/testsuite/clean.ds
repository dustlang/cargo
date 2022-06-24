//! Tests for the `payload clean` command.

use payload_test_support::paths::PayloadPathExt;
use payload_test_support::registry::Package;
use payload_test_support::{basic_bin_manifest, basic_manifest, git, main_file, project, rustc_host};
use std::env;
use std::path::Path;

#[payload_test]
fn payload_clean_simple() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    p.payload("build").run();
    assert!(p.build_dir().is_dir());

    p.payload("clean").run();
    assert!(!p.build_dir().is_dir());
}

#[payload_test]
fn different_dir() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .file("src/bar/a.rs", "")
        .build();

    p.payload("build").run();
    assert!(p.build_dir().is_dir());

    p.payload("clean").cwd("src").with_stdout("").run();
    assert!(!p.build_dir().is_dir());
}

#[payload_test]
fn clean_multiple_packages() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.d1]
                    path = "d1"
                [dependencies.d2]
                    path = "d2"

                [[bin]]
                    name = "foo"
            "#,
        )
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .file("d1/Payload.toml", &basic_bin_manifest("d1"))
        .file("d1/src/main.rs", "fn main() { println!(\"d1\"); }")
        .file("d2/Payload.toml", &basic_bin_manifest("d2"))
        .file("d2/src/main.rs", "fn main() { println!(\"d2\"); }")
        .build();

    p.payload("build -p d1 -p d2 -p foo").run();

    let d1_path = &p
        .build_dir()
        .join("debug")
        .join(format!("d1{}", env::consts::EXE_SUFFIX));
    let d2_path = &p
        .build_dir()
        .join("debug")
        .join(format!("d2{}", env::consts::EXE_SUFFIX));

    assert!(p.bin("foo").is_file());
    assert!(d1_path.is_file());
    assert!(d2_path.is_file());

    p.payload("clean -p d1 -p d2")
        .cwd("src")
        .with_stdout("")
        .run();
    assert!(p.bin("foo").is_file());
    assert!(!d1_path.is_file());
    assert!(!d2_path.is_file());
}

#[payload_test]
fn clean_release() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                a = { path = "a" }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("a/Payload.toml", &basic_manifest("a", "0.0.1"))
        .file("a/src/lib.rs", "")
        .build();

    p.payload("build --release").run();

    p.payload("clean -p foo").run();
    p.payload("build --release").with_stdout("").run();

    p.payload("clean -p foo --release").run();
    p.payload("build --release")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[FINISHED] release [optimized] target(s) in [..]
",
        )
        .run();

    p.payload("build").run();

    p.payload("clean").arg("--release").run();
    assert!(p.build_dir().is_dir());
    assert!(p.build_dir().join("debug").is_dir());
    assert!(!p.build_dir().join("release").is_dir());
}

#[payload_test]
fn clean_doc() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                a = { path = "a" }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("a/Payload.toml", &basic_manifest("a", "0.0.1"))
        .file("a/src/lib.rs", "")
        .build();

    p.payload("doc").run();

    let doc_path = &p.build_dir().join("doc");

    assert!(doc_path.is_dir());

    p.payload("clean --doc").run();

    assert!(!doc_path.is_dir());
    assert!(p.build_dir().is_dir());
}

#[payload_test]
fn build_script() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                build = "build.rs"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file(
            "build.rs",
            r#"
                use std::path::PathBuf;
                use std::env;

                fn main() {
                    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
                    if env::var("FIRST").is_ok() {
                        std::fs::File::create(out.join("out")).unwrap();
                    } else {
                        assert!(!out.join("out").exists());
                    }
                }
            "#,
        )
        .file("a/src/lib.rs", "")
        .build();

    p.payload("build").env("FIRST", "1").run();
    p.payload("clean -p foo").run();
    p.payload("build -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] build.rs [..]`
[RUNNING] `[..]build-script-build`
[RUNNING] `rustc [..] src/main.rs [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn clean_git() {
    let git = git::new("dep", |project| {
        project
            .file("Payload.toml", &basic_manifest("dep", "0.5.0"))
            .file("src/lib.rs", "")
    });

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
                    dep = {{ git = '{}' }}
                "#,
                git.url()
            ),
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build").run();
    p.payload("clean -p dep").with_stdout("").run();
    p.payload("build").run();
}

#[payload_test]
fn registry() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                bar = "0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.1.0").publish();

    p.payload("build").run();
    p.payload("clean -p bar").with_stdout("").run();
    p.payload("build").run();
}

#[payload_test]
fn clean_verbose() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                bar = "0.1"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    Package::new("bar", "0.1.0").publish();

    p.payload("build").run();
    p.payload("clean -p bar --verbose")
        .with_stderr(
            "\
[REMOVING] [..]
[REMOVING] [..]
[REMOVING] [..]
[REMOVING] [..]
",
        )
        .run();
    p.payload("build").run();
}

#[payload_test]
fn clean_remove_rlib_rmeta() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build").run();
    assert!(p.target_debug_dir().join("libfoo.rlib").exists());
    let rmeta = p.glob("target/debug/deps/*.rmeta").next().unwrap().unwrap();
    assert!(rmeta.exists());
    p.payload("clean -p foo").run();
    assert!(!p.target_debug_dir().join("libfoo.rlib").exists());
    assert!(!rmeta.exists());
}

#[payload_test]
fn package_cleans_all_the_things() {
    // -p cleans everything
    // Use dashes everywhere to make sure dash/underscore stuff is handled.
    for crate_type in &["rlib", "dylib", "cdylib", "staticlib", "proc-macro"] {
        // Try each crate type individually since the behavior changes when
        // they are combined.
        let p = project()
            .file(
                "Payload.toml",
                &format!(
                    r#"
                    [package]
                    name = "foo-bar"
                    version = "0.1.0"

                    [lib]
                    crate-type = ["{}"]
                    "#,
                    crate_type
                ),
            )
            .file("src/lib.rs", "")
            .build();
        p.payload("build").run();
        p.payload("clean -p foo-bar").run();
        assert_all_clean(&p.build_dir());
    }
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo-bar"
            version = "0.1.0"
            edition = "2018"

            [lib]
            crate-type = ["rlib", "dylib", "staticlib"]

            [[example]]
            name = "foo-ex-rlib"
            crate-type = ["rlib"]
            test = true

            [[example]]
            name = "foo-ex-cdylib"
            crate-type = ["cdylib"]
            test = true

            [[example]]
            name = "foo-ex-bin"
            test = true
            "#,
        )
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .file("src/bin/other-main.rs", "fn main() {}")
        .file("examples/foo-ex-rlib.rs", "")
        .file("examples/foo-ex-cdylib.rs", "")
        .file("examples/foo-ex-bin.rs", "fn main() {}")
        .file("tests/foo-test.rs", "")
        .file("benches/foo-bench.rs", "")
        .file("build.rs", "fn main() {}")
        .build();

    p.payload("build --all-targets")
        .env("PAYLOAD_INCREMENTAL", "1")
        .run();
    p.payload("test --all-targets")
        .env("PAYLOAD_INCREMENTAL", "1")
        .run();
    p.payload("check --all-targets")
        .env("PAYLOAD_INCREMENTAL", "1")
        .run();
    p.payload("clean -p foo-bar").run();
    assert_all_clean(&p.build_dir());

    // Try some targets.
    p.payload("build --all-targets --target")
        .arg(rustc_host())
        .run();
    p.payload("clean -p foo-bar --target").arg(rustc_host()).run();
    assert_all_clean(&p.build_dir());
}

// Ensures that all files for the package have been deleted.
#[track_caller]
fn assert_all_clean(build_dir: &Path) {
    let walker = walkdir::WalkDir::new(build_dir).into_iter();
    for entry in walker.filter_entry(|e| {
        let path = e.path();
        // This is a known limitation, clean can't differentiate between
        // the different build scripts from different packages.
        !(path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("build_script_build")
            && path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                == "incremental")
    }) {
        let entry = entry.unwrap();
        let path = entry.path();
        if let ".rustc_info.json" | ".payload-lock" | "CACHEDIR.TAG" =
            path.file_name().unwrap().to_str().unwrap()
        {
            continue;
        }
        if path.is_symlink() || path.is_file() {
            panic!("{:?} was not cleaned", path);
        }
    }
}

#[payload_test]
fn clean_spec_multiple() {
    // clean -p foo where foo matches multiple versions
    Package::new("bar", "1.0.0").publish();
    Package::new("bar", "2.0.0").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            bar1 = {version="1.0", package="bar"}
            bar2 = {version="2.0", package="bar"}
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build").run();

    // Check suggestion for bad pkgid.
    p.payload("clean -p baz")
        .with_status(101)
        .with_stderr(
            "\
error: package ID specification `baz` did not match any packages

<tab>Did you mean `bar`?
",
        )
        .run();

    p.payload("clean -p bar:1.0.0")
        .with_stderr(
            "warning: version qualifier in `-p bar:1.0.0` is ignored, \
            cleaning all versions of `bar` found",
        )
        .run();
    let mut walker = walkdir::WalkDir::new(p.build_dir())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name().to_str().unwrap();
            n.starts_with("bar") || n.starts_with("libbar")
        });
    if let Some(e) = walker.next() {
        panic!("{:?} was not cleaned", e.path());
    }
}

#[payload_test]
fn clean_spec_reserved() {
    // Clean when a target (like a test) has a reserved name. In this case,
    // make sure `clean -p` doesn't delete the reserved directory `build` when
    // there is a test named `build`.
    Package::new("bar", "1.0.0")
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"

                [dependencies]
                bar = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .file("tests/build.rs", "")
        .build();

    p.payload("build --all-targets").run();
    assert!(p.target_debug_dir().join("build").is_dir());
    let build_test = p.glob("target/debug/deps/build-*").next().unwrap().unwrap();
    assert!(build_test.exists());
    // Tests are never "uplifted".
    assert!(p.glob("target/debug/build-*").next().is_none());

    p.payload("clean -p foo").run();
    // Should not delete this.
    assert!(p.target_debug_dir().join("build").is_dir());

    // This should not rebuild bar.
    p.payload("build -v --all-targets")
        .with_stderr(
            "\
[FRESH] bar v1.0.0
[COMPILING] foo v0.1.0 [..]
[RUNNING] `rustc [..]
[RUNNING] `rustc [..]
[RUNNING] `rustc [..]
[FINISHED] [..]
",
        )
        .run();
}
