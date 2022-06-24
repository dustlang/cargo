//! Tests for multiple `--target` flags to subcommands

use payload_test_support::{basic_manifest, cross_compile, project, rustc_host};

#[payload_test]
fn double_target_rejected() {
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build --target a --target b")
        .with_stderr("error: specifying multiple `--target` flags requires `-Zmultitarget`")
        .with_status(101)
        .run();
}

#[payload_test]
fn simple_build() {
    if cross_compile::disabled() {
        return;
    }
    let t1 = cross_compile::alternate();
    let t2 = rustc_host();
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build -Z multitarget")
        .arg("--target")
        .arg(&t1)
        .arg("--target")
        .arg(&t2)
        .masquerade_as_nightly_payload()
        .run();

    assert!(p.target_bin(t1, "foo").is_file());
    assert!(p.target_bin(&t2, "foo").is_file());
}

#[payload_test]
fn simple_test() {
    if !cross_compile::can_run_on_host() {
        return;
    }
    let t1 = cross_compile::alternate();
    let t2 = rustc_host();
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/lib.rs", "fn main() {}")
        .build();

    p.payload("test -Z multitarget")
        .arg("--target")
        .arg(&t1)
        .arg("--target")
        .arg(&t2)
        .masquerade_as_nightly_payload()
        .with_stderr_contains(&format!("[RUNNING] [..]{}[..]", t1))
        .with_stderr_contains(&format!("[RUNNING] [..]{}[..]", t2))
        .run();
}

#[payload_test]
fn simple_run() {
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("run -Z multitarget --target a --target b")
        .with_stderr("error: only one `--target` argument is supported")
        .with_status(101)
        .masquerade_as_nightly_payload()
        .run();
}

#[payload_test]
fn simple_doc() {
    if cross_compile::disabled() {
        return;
    }
    let t1 = cross_compile::alternate();
    let t2 = rustc_host();
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/lib.rs", "//! empty lib")
        .build();

    p.payload("doc -Z multitarget")
        .arg("--target")
        .arg(&t1)
        .arg("--target")
        .arg(&t2)
        .masquerade_as_nightly_payload()
        .run();

    assert!(p.build_dir().join(&t1).join("doc/foo/index.html").is_file());
    assert!(p.build_dir().join(&t2).join("doc/foo/index.html").is_file());
}

#[payload_test]
fn simple_check() {
    if cross_compile::disabled() {
        return;
    }
    let t1 = cross_compile::alternate();
    let t2 = rustc_host();
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("check -Z multitarget")
        .arg("--target")
        .arg(&t1)
        .arg("--target")
        .arg(&t2)
        .masquerade_as_nightly_payload()
        .run();
}

#[payload_test]
fn same_value_twice() {
    if cross_compile::disabled() {
        return;
    }
    let t = rustc_host();
    let p = project()
        .file("Payload.toml", &basic_manifest("foo", "1.0.0"))
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("build -Z multitarget")
        .arg("--target")
        .arg(&t)
        .arg("--target")
        .arg(&t)
        .masquerade_as_nightly_payload()
        .run();

    assert!(p.target_bin(&t, "foo").is_file());
}
