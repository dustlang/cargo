//! Tests for setting custom rustc flags.

use payload_test_support::registry::Package;
use payload_test_support::{
    basic_lib_manifest, basic_manifest, paths, project, project_in_home, rustc_host,
};
use std::fs;

#[payload_test]
fn env_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .build();

    // Use RUSTFLAGS to pass an argument that will generate an error
    p.payload("build --lib")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --bin=a")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --example=b")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("test")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("bench")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn env_rustflags_build_script() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { }
                #[cfg(not(foo))]
                fn main() { }
            "#,
        )
        .build();

    p.payload("build").env("RUSTFLAGS", "--cfg foo").run();
}

#[payload_test]
fn env_rustflags_build_script_dep() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.payload("build").env("RUSTFLAGS", "--cfg foo").run();
}

#[payload_test]
fn env_rustflags_plugin() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(not(foo))]
                fn main() { }
            "#,
        )
        .build();

    p.payload("build").env("RUSTFLAGS", "--cfg foo").run();
}

#[payload_test]
fn env_rustflags_plugin_dep() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.payload("build").env("RUSTFLAGS", "--cfg foo").run();
}

#[payload_test]
fn env_rustflags_normal_source_with_target() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .build();

    let host = &rustc_host();

    // Use RUSTFLAGS to pass an argument that will generate an error
    p.payload("build --lib --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --bin=a --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --example=b --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("test --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("bench --target")
        .arg(host)
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn env_rustflags_build_script_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { }
                #[cfg(foo)]
                fn main() { }
            "#,
        )
        .build();

    let host = rustc_host();
    p.payload("build --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[payload_test]
fn env_rustflags_build_script_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.payload("build --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[payload_test]
fn env_rustflags_plugin_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(foo)]
                fn main() { }
            "#,
        )
        .build();

    let host = rustc_host();
    p.payload("build --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[payload_test]
fn env_rustflags_plugin_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.payload("build --target")
        .arg(host)
        .env("RUSTFLAGS", "--cfg foo")
        .run();
}

#[payload_test]
fn env_rustflags_recompile() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("build").run();
    // Setting RUSTFLAGS forces a recompile
    p.payload("build")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn env_rustflags_recompile2() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("build").env("RUSTFLAGS", "--cfg foo").run();
    // Setting RUSTFLAGS forces a recompile
    p.payload("build")
        .env("RUSTFLAGS", "-Z bogus")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn env_rustflags_no_recompile() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("build").env("RUSTFLAGS", "--cfg foo").run();
    p.payload("build")
        .env("RUSTFLAGS", "--cfg foo")
        .with_stdout("")
        .run();
}

#[payload_test]
fn build_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["-Z", "bogus"]
            "#,
        )
        .build();

    p.payload("build --lib")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --bin=a")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --example=b")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("test")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("bench")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn build_rustflags_build_script() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { }
                #[cfg(not(foo))]
                fn main() { }
            "#,
        )
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.payload("build").run();
}

#[payload_test]
fn build_rustflags_build_script_dep() {
    // RUSTFLAGS should be passed to rustc for build scripts
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.payload("build").run();
}

#[payload_test]
fn build_rustflags_plugin() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(not(foo))]
                fn main() { }
            "#,
        )
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.payload("build").run();
}

#[payload_test]
fn build_rustflags_plugin_dep() {
    // RUSTFLAGS should be passed to rustc for plugins
    // when --target is not specified.
    // In this test if --cfg foo is not passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(not(foo))]
                fn bar() { }
            "#,
        )
        .build();

    foo.payload("build").run();
}

#[payload_test]
fn build_rustflags_normal_source_with_target() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["-Z", "bogus"]
            "#,
        )
        .build();

    let host = &rustc_host();

    // Use RUSTFLAGS to pass an argument that will generate an error
    p.payload("build --lib --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --bin=a --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --example=b --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("test --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("bench --target")
        .arg(host)
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn build_rustflags_build_script_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file(
            "build.rs",
            r#"
                fn main() { }
                #[cfg(foo)]
                fn main() { }
            "#,
        )
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    let host = rustc_host();
    p.payload("build --target").arg(host).run();
}

#[payload_test]
fn build_rustflags_build_script_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for build scripts
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"

                [build-dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.0.1"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.payload("build --target").arg(host).run();
}

#[payload_test]
fn build_rustflags_plugin_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                fn main() { }
                #[cfg(foo)]
                fn main() { }
            "#,
        )
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    let host = rustc_host();
    p.payload("build --target").arg(host).run();
}

#[payload_test]
fn build_rustflags_plugin_dep_with_target() {
    // RUSTFLAGS should not be passed to rustc for plugins
    // when --target is specified.
    // In this test if --cfg foo is passed the build will fail.
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [lib]
                name = "foo"
                plugin = true

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file("src/lib.rs", "fn foo() {}")
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_lib_manifest("bar"))
        .file(
            "src/lib.rs",
            r#"
                fn bar() { }
                #[cfg(foo)]
                fn bar() { }
            "#,
        )
        .build();

    let host = rustc_host();
    foo.payload("build --target").arg(host).run();
}

#[payload_test]
fn build_rustflags_recompile() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("build").run();

    // Setting RUSTFLAGS forces a recompile
    let config = r#"
        [build]
        rustflags = ["-Z", "bogus"]
        "#;
    let config_file = paths::root().join("foo/.payload/config");
    fs::create_dir_all(config_file.parent().unwrap()).unwrap();
    fs::write(config_file, config).unwrap();

    p.payload("build")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn build_rustflags_recompile2() {
    let p = project().file("src/lib.rs", "").build();

    p.payload("build").env("RUSTFLAGS", "--cfg foo").run();

    // Setting RUSTFLAGS forces a recompile
    let config = r#"
        [build]
        rustflags = ["-Z", "bogus"]
        "#;
    let config_file = paths::root().join("foo/.payload/config");
    fs::create_dir_all(config_file.parent().unwrap()).unwrap();
    fs::write(config_file, config).unwrap();

    p.payload("build")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn build_rustflags_no_recompile() {
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p.payload("build").env("RUSTFLAGS", "--cfg foo").run();
    p.payload("build")
        .env("RUSTFLAGS", "--cfg foo")
        .with_stdout("")
        .run();
}

#[payload_test]
fn build_rustflags_with_home_config() {
    // We need a config file inside the home directory
    let home = paths::home();
    let home_config = home.join(".payload");
    fs::create_dir(&home_config).unwrap();
    fs::write(
        &home_config.join("config"),
        r#"
            [build]
            rustflags = ["-Cllvm-args=-x86-asm-syntax=intel"]
        "#,
    )
    .unwrap();

    // And we need the project to be inside the home directory
    // so the walking process finds the home project twice.
    let p = project_in_home("foo").file("src/lib.rs", "").build();

    p.payload("build -v").run();
}

#[payload_test]
fn target_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            "benches/d.rs",
            r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            "#,
        )
        .file(
            ".payload/config",
            &format!(
                "
            [target.{}]
            rustflags = [\"-Z\", \"bogus\"]
            ",
                rustc_host()
            ),
        )
        .build();

    p.payload("build --lib")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --bin=a")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --example=b")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("test")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("bench")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

// target.{}.rustflags takes precedence over build.rustflags
#[payload_test]
fn target_rustflags_precedence() {
    let p = project()
        .file("src/lib.rs", "")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            ".payload/config",
            &format!(
                "
            [build]
            rustflags = [\"--cfg\", \"foo\"]

            [target.{}]
            rustflags = [\"-Z\", \"bogus\"]
            ",
                rustc_host()
            ),
        )
        .build();

    p.payload("build --lib")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --bin=a")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("build --example=b")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("test")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
    p.payload("bench")
        .with_status(101)
        .with_stderr_contains("[..]bogus[..]")
        .run();
}

#[payload_test]
fn cfg_rustflags_normal_source() {
    let p = project()
        .file("src/lib.rs", "pub fn t() {}")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            ".payload/config",
            &format!(
                r#"
                [target.'cfg({})']
                rustflags = ["--cfg", "bar"]
                "#,
                if rustc_host().contains("-windows-") {
                    "windows"
                } else {
                    "not(windows)"
                }
            ),
        )
        .build();

    p.payload("build --lib -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("build --bin=a -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("build --example=b -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("test --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("bench --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] bench [optimized] target(s) in [..]
",
        )
        .run();
}

// target.'cfg(...)'.rustflags takes precedence over build.rustflags
#[payload_test]
fn cfg_rustflags_precedence() {
    let p = project()
        .file("src/lib.rs", "pub fn t() {}")
        .file("src/bin/a.rs", "fn main() {}")
        .file("examples/b.rs", "fn main() {}")
        .file("tests/c.rs", "#[test] fn f() { }")
        .file(
            ".payload/config",
            &format!(
                r#"
                [build]
                rustflags = ["--cfg", "foo"]

                [target.'cfg({})']
                rustflags = ["--cfg", "bar"]
                "#,
                if rustc_host().contains("-windows-") {
                    "windows"
                } else {
                    "not(windows)"
                }
            ),
        )
        .build();

    p.payload("build --lib -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("build --bin=a -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("build --example=b -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("test --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("bench --no-run -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[RUNNING] `rustc [..] --cfg bar[..]`
[FINISHED] bench [optimized] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn target_rustflags_string_and_array_form1() {
    let p1 = project()
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = ["--cfg", "foo"]
            "#,
        )
        .build();

    p1.payload("build -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    let p2 = project()
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
            [build]
            rustflags = "--cfg foo"
            "#,
        )
        .build();

    p2.payload("build -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn target_rustflags_string_and_array_form2() {
    let p1 = project()
        .file(
            ".payload/config",
            &format!(
                r#"
                    [target.{}]
                    rustflags = ["--cfg", "foo"]
                "#,
                rustc_host()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p1.payload("build -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    let p2 = project()
        .file(
            ".payload/config",
            &format!(
                r#"
                    [target.{}]
                    rustflags = "--cfg foo"
                "#,
                rustc_host()
            ),
        )
        .file("src/lib.rs", "")
        .build();

    p2.payload("build -v")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] --cfg foo[..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn two_matching_in_config() {
    let p1 = project()
        .file(
            ".payload/config",
            r#"
                [target.'cfg(unix)']
                rustflags = ["--cfg", 'foo="a"']
                [target.'cfg(windows)']
                rustflags = ["--cfg", 'foo="a"']
                [target.'cfg(target_pointer_width = "32")']
                rustflags = ["--cfg", 'foo="b"']
                [target.'cfg(target_pointer_width = "64")']
                rustflags = ["--cfg", 'foo="b"']
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                fn main() {
                    if cfg!(foo = "a") {
                        println!("a");
                    } else if cfg!(foo = "b") {
                        println!("b");
                    } else {
                        panic!()
                    }
                }
            "#,
        )
        .build();

    p1.payload("run").run();
    p1.payload("build").with_stderr("[FINISHED] [..]").run();
}

#[payload_test]
fn env_rustflags_misspelled() {
    let p = project().file("src/main.rs", "fn main() { }").build();

    for cmd in &["check", "build", "run", "test", "bench"] {
        p.payload(cmd)
            .env("RUST_FLAGS", "foo")
            .with_stderr_contains("[WARNING] Payload does not read `RUST_FLAGS` environment variable. Did you mean `RUSTFLAGS`?")
            .run();
    }
}

#[payload_test]
fn env_rustflags_misspelled_build_script() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                build = "build.rs"
            "#,
        )
        .file("src/lib.rs", "")
        .file("build.rs", "fn main() { }")
        .build();

    p.payload("build")
        .env("RUST_FLAGS", "foo")
        .with_stderr_contains("[WARNING] Payload does not read `RUST_FLAGS` environment variable. Did you mean `RUSTFLAGS`?")
        .run();
}

#[payload_test]
fn remap_path_prefix_ignored() {
    // Ensure that --remap-path-prefix does not affect metadata hash.
    let p = project().file("src/lib.rs", "").build();
    p.payload("build").run();
    let rlibs = p
        .glob("target/debug/deps/*.rlib")
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rlibs.len(), 1);
    p.payload("clean").run();

    let check_metadata_same = || {
        let rlibs2 = p
            .glob("target/debug/deps/*.rlib")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rlibs, rlibs2);
    };

    p.payload("build")
        .env(
            "RUSTFLAGS",
            "--remap-path-prefix=/abc=/zoo --remap-path-prefix /spaced=/zoo",
        )
        .run();
    check_metadata_same();

    p.payload("clean").run();
    p.payload("rustc -- --remap-path-prefix=/abc=/zoo --remap-path-prefix /spaced=/zoo")
        .run();
    check_metadata_same();
}

#[payload_test]
fn remap_path_prefix_works() {
    // Check that remap-path-prefix works.
    Package::new("bar", "0.1.0")
        .file("src/lib.rs", "pub fn f() -> &'static str { file!() }")
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
        .file(
            "src/main.rs",
            r#"
            fn main() {
                println!("{}", bar::f());
            }
            "#,
        )
        .build();

    p.payload("run")
        .env(
            "RUSTFLAGS",
            format!("--remap-path-prefix={}=/foo", paths::root().display()),
        )
        .with_stdout("/foo/home/.payload/registry/src/[..]/bar-0.1.0/src/lib.rs")
        .run();
}
