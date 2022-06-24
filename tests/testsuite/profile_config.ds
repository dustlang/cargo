//! Tests for profiles defined in config files.

use payload_test_support::paths::PayloadPathExt;
use payload_test_support::registry::Package;
use payload_test_support::{basic_lib_manifest, paths, project};

#[payload_test]
fn named_profile_gated() {
    // Named profile in config requires enabling in Payload.toml.
    let p = project()
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
            [profile.foo]
            inherits = 'dev'
            opt-level = 1
            "#,
        )
        .build();
    p.payload("build --profile foo -Zunstable-options")
        .masquerade_as_nightly_payload()
        .with_stderr(
            "\
[ERROR] config profile `foo` is not valid (defined in `[..]/foo/.payload/config`)

Caused by:
  feature `named-profiles` is required

  consider adding `payload-features = [\"named-profiles\"]` to the manifest
",
        )
        .with_status(101)
        .run();
}

#[payload_test]
fn profile_config_validate_warnings() {
    let p = project()
        .file("Payload.toml", &basic_lib_manifest("foo"))
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [profile.test]
                opt-level = 3

                [profile.asdf]
                opt-level = 3

                [profile.dev]
                bad-key = true

                [profile.dev.build-override]
                bad-key-bo = true

                [profile.dev.package.bar]
                bad-key-bar = true
            "#,
        )
        .build();

    p.payload("build")
        .with_stderr_unordered(
            "\
[WARNING] unused config key `profile.dev.bad-key` in `[..].payload/config`
[WARNING] unused config key `profile.dev.package.bar.bad-key-bar` in `[..].payload/config`
[WARNING] unused config key `profile.dev.build-override.bad-key-bo` in `[..].payload/config`
[COMPILING] foo [..]
[FINISHED] [..]
",
        )
        .run();
}

#[payload_test]
fn profile_config_error_paths() {
    // Errors in config show where the error is located.
    let p = project()
        .file("Payload.toml", &basic_lib_manifest("foo"))
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [profile.dev]
                opt-level = 3
            "#,
        )
        .file(
            paths::home().join(".payload/config"),
            r#"
            [profile.dev]
            rpath = "foo"
            "#,
        )
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] error in [..]/foo/.payload/config: could not load config key `profile.dev`

Caused by:
  error in [..]/home/.payload/config: `profile.dev.rpath` expected true/false, but found a string
",
        )
        .run();
}

#[payload_test]
fn profile_config_validate_errors() {
    let p = project()
        .file("Payload.toml", &basic_lib_manifest("foo"))
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [profile.dev.package.foo]
                panic = "abort"
            "#,
        )
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] config profile `dev` is not valid (defined in `[..]/foo/.payload/config`)

Caused by:
  `panic` may not be specified in a `package` profile
",
        )
        .run();
}

#[payload_test]
fn profile_config_syntax_errors() {
    let p = project()
        .file("Payload.toml", &basic_lib_manifest("foo"))
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [profile.dev]
                codegen-units = "foo"
            "#,
        )
        .build();

    p.payload("build")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] error in [..]/.payload/config: could not load config key `profile.dev`

Caused by:
  error in [..]/foo/.payload/config: `profile.dev.codegen-units` expected an integer, but found a string
",
        )
        .run();
}

#[payload_test]
fn profile_config_override_spec_multiple() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.0.1"

            [dependencies]
            bar = { path = "bar" }
            "#,
        )
        .file(
            ".payload/config",
            r#"
                [profile.dev.package.bar]
                opt-level = 3

                [profile.dev.package."bar:0.5.0"]
                opt-level = 3
            "#,
        )
        .file("src/lib.rs", "")
        .file("bar/Payload.toml", &basic_lib_manifest("bar"))
        .file("bar/src/lib.rs", "")
        .build();

    // Unfortunately this doesn't tell you which file, hopefully it's not too
    // much of a problem.
    p.payload("build -v")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] multiple package overrides in profile `dev` match package `bar v0.5.0 ([..])`
found package specs: bar, bar:0.5.0",
        )
        .run();
}

#[payload_test]
fn profile_config_all_options() {
    // Ensure all profile options are supported.
    let p = project()
        .file("src/main.rs", "fn main() {}")
        .file(
            ".payload/config",
            r#"
            [profile.release]
            opt-level = 1
            debug = true
            debug-assertions = true
            overflow-checks = false
            rpath = true
            lto = true
            codegen-units = 2
            panic = "abort"
            incremental = true
            "#,
        )
        .build();

    p.payload("build --release -v")
        .env_remove("PAYLOAD_INCREMENTAL")
        .with_stderr(
            "\
[COMPILING] foo [..]
[RUNNING] `rustc --crate-name foo [..] \
            -C opt-level=1 \
            -C panic=abort \
            -C lto[..]\
            -C codegen-units=2 \
            -C debuginfo=2 \
            -C debug-assertions=on \
            -C overflow-checks=off [..]\
            -C rpath [..]\
            -C incremental=[..]
[FINISHED] release [optimized + debuginfo] [..]
",
        )
        .run();
}

#[payload_test]
fn profile_config_override_precedence() {
    // Config values take precedence over manifest values.
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                bar = {path = "bar"}

                [profile.dev]
                codegen-units = 2

                [profile.dev.package.bar]
                opt-level = 3
            "#,
        )
        .file("src/lib.rs", "")
        .file("bar/Payload.toml", &basic_lib_manifest("bar"))
        .file("bar/src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [profile.dev.package.bar]
                opt-level = 2
            "#,
        )
        .build();

    p.payload("build -v")
        .with_stderr(
            "\
[COMPILING] bar [..]
[RUNNING] `rustc --crate-name bar [..] -C opt-level=2[..]-C codegen-units=2 [..]
[COMPILING] foo [..]
[RUNNING] `rustc --crate-name foo [..]-C codegen-units=2 [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]",
        )
        .run();
}

#[payload_test]
fn profile_config_no_warn_unknown_override() {
    let p = project()
        .file("Payload.toml", &basic_lib_manifest("foo"))
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [profile.dev.package.bar]
                codegen-units = 4
            "#,
        )
        .build();

    p.payload("build")
        .with_stderr_does_not_contain("[..]warning[..]")
        .run();
}

#[payload_test]
fn profile_config_mixed_types() {
    let p = project()
        .file("Payload.toml", &basic_lib_manifest("foo"))
        .file("src/lib.rs", "")
        .file(
            ".payload/config",
            r#"
                [profile.dev]
                opt-level = 3
            "#,
        )
        .file(
            paths::home().join(".payload/config"),
            r#"
            [profile.dev]
            opt-level = 's'
            "#,
        )
        .build();

    p.payload("build -v")
        .with_stderr_contains("[..]-C opt-level=3 [..]")
        .run();
}

#[payload_test]
fn named_config_profile() {
    // Exercises config named profies.
    // foo -> middle -> bar -> dev
    // middle exists in Payload.toml, the others in .payload/config
    use super::config::ConfigBuilder;
    use payload::core::compiler::CompileMode;
    use payload::core::profiles::{Profiles, UnitFor};
    use payload::core::{PackageId, Workspace};
    use payload::util::interning::InternedString;
    use std::fs;
    paths::root().join(".payload").mkdir_p();
    fs::write(
        paths::root().join(".payload/config"),
        r#"
            [profile.foo]
            inherits = "middle"
            codegen-units = 2
            [profile.foo.build-override]
            codegen-units = 6
            [profile.foo.package.dep]
            codegen-units = 7

            [profile.middle]
            inherits = "bar"
            codegen-units = 3

            [profile.bar]
            inherits = "dev"
            codegen-units = 4
            debug = 1
        "#,
    )
    .unwrap();
    fs::write(
        paths::root().join("Payload.toml"),
        r#"
            payload-features = ['named-profiles']

            [workspace]

            [profile.middle]
            inherits = "bar"
            codegen-units = 1
            opt-level = 1
            [profile.middle.package.dep]
            overflow-checks = false

            [profile.foo.build-override]
            codegen-units = 5
            debug-assertions = false
            [profile.foo.package.dep]
            codegen-units = 8
        "#,
    )
    .unwrap();
    let config = ConfigBuilder::new().nightly_features_allowed(true).build();
    let profile_name = InternedString::new("foo");
    let ws = Workspace::new(&paths::root().join("Payload.toml"), &config).unwrap();
    let profiles = Profiles::new(&ws, profile_name).unwrap();

    let crates_io = payload::core::source::SourceId::crates_io(&config).unwrap();
    let a_pkg = PackageId::new("a", "0.1.0", crates_io).unwrap();
    let dep_pkg = PackageId::new("dep", "0.1.0", crates_io).unwrap();

    // normal package
    let mode = CompileMode::Build;
    let p = profiles.get_profile(a_pkg, true, true, UnitFor::new_normal(), mode);
    assert_eq!(p.name, "foo");
    assert_eq!(p.codegen_units, Some(2)); // "foo" from config
    assert_eq!(p.opt_level, "1"); // "middle" from manifest
    assert_eq!(p.debuginfo, Some(1)); // "bar" from config
    assert_eq!(p.debug_assertions, true); // "dev" built-in (ignore build-override)
    assert_eq!(p.overflow_checks, true); // "dev" built-in (ignore package override)

    // build-override
    let bo = profiles.get_profile(a_pkg, true, true, UnitFor::new_host(false), mode);
    assert_eq!(bo.name, "foo");
    assert_eq!(bo.codegen_units, Some(6)); // "foo" build override from config
    assert_eq!(bo.opt_level, "0"); // default to zero
    assert_eq!(bo.debuginfo, Some(1)); // SAME as normal
    assert_eq!(bo.debug_assertions, false); // "foo" build override from manifest
    assert_eq!(bo.overflow_checks, true); // SAME as normal

    // package overrides
    let po = profiles.get_profile(dep_pkg, false, true, UnitFor::new_normal(), mode);
    assert_eq!(po.name, "foo");
    assert_eq!(po.codegen_units, Some(7)); // "foo" package override from config
    assert_eq!(po.opt_level, "1"); // SAME as normal
    assert_eq!(po.debuginfo, Some(1)); // SAME as normal
    assert_eq!(po.debug_assertions, true); // SAME as normal
    assert_eq!(po.overflow_checks, false); // "middle" package override from manifest
}

#[payload_test]
fn named_env_profile() {
    // Environment variables used to define a named profile.
    let p = project()
        .file(
            "Payload.toml",
            r#"
            payload-features = ["named-profiles"]
            [package]
            name = "foo"
            version = "0.1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build -v -Zunstable-options --profile=other")
        .masquerade_as_nightly_payload()
        .env("PAYLOAD_PROFILE_OTHER_CODEGEN_UNITS", "1")
        .env("PAYLOAD_PROFILE_OTHER_INHERITS", "dev")
        .with_stderr_contains("[..]-C codegen-units=1 [..]")
        .run();
}

#[payload_test]
fn test_with_dev_profile() {
    // `payload test` uses "dev" profile for dependencies.
    Package::new("somedep", "1.0.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            somedep = "1.0"
            "#,
        )
        .file("src/lib.rs", "")
        .build();
    p.payload("test --lib --no-run -v")
        .env("PAYLOAD_PROFILE_DEV_DEBUG", "0")
        .with_stderr(
            "\
[UPDATING] [..]
[DOWNLOADING] [..]
[DOWNLOADED] [..]
[COMPILING] somedep v1.0.0
[RUNNING] `rustc --crate-name somedep [..]-C debuginfo=0[..]
[COMPILING] foo v0.1.0 [..]
[RUNNING] `rustc --crate-name foo [..]-C debuginfo=2[..]
[FINISHED] [..]
",
        )
        .run();
}
