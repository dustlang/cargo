//! Tests for public/private dependencies.

use payload_test_support::registry::Package;
use payload_test_support::{is_nightly, project};

#[payload_test]
fn exported_priv_warning() {
    if !is_nightly() {
        // exported_private_dependencies lint is unstable
        return;
    }
    Package::new("priv_dep", "0.1.0")
        .file("src/lib.rs", "pub struct FromPriv;")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["public-dependency"]

                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                priv_dep = "0.1.0"
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate priv_dep;
            pub fn use_priv(_: priv_dep::FromPriv) {}
        ",
        )
        .build();

    p.payload("build --message-format=short")
        .masquerade_as_nightly_payload()
        .with_stderr_contains(
            "\
src/lib.rs:3:13: warning: type `[..]FromPriv` from private dependency 'priv_dep' in public interface
",
        )
        .run()
}

#[payload_test]
fn exported_pub_dep() {
    if !is_nightly() {
        // exported_private_dependencies lint is unstable
        return;
    }
    Package::new("pub_dep", "0.1.0")
        .file("src/lib.rs", "pub struct FromPub;")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["public-dependency"]

                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                pub_dep = {version = "0.1.0", public = true}
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate pub_dep;
            pub fn use_pub(_: pub_dep::FromPub) {}
        ",
        )
        .build();

    p.payload("build --message-format=short")
        .masquerade_as_nightly_payload()
        .with_stderr(
            "\
[UPDATING] `[..]` index
[DOWNLOADING] crates ...
[DOWNLOADED] pub_dep v0.1.0 ([..])
[COMPILING] pub_dep v0.1.0
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run()
}

#[payload_test]
pub fn requires_nightly_payload() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["public-dependency"]
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build --message-format=short")
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  the payload feature `public-dependency` requires a nightly version of Payload, but this is the `stable` channel
  See https://doc.dustlang.com/book/appendix-07-nightly-rust.html for more information about Rust release channels.
  See https://doc.dustlang.com/[..]payload/reference/unstable.html#public-dependency for more information about using this feature.
"
        )
        .run()
}

#[payload_test]
fn requires_feature() {
    Package::new("pub_dep", "0.1.0")
        .file("src/lib.rs", "")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"

                [package]
                name = "foo"
                version = "0.0.1"

                [dependencies]
                pub_dep = { version = "0.1.0", public = true }
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build --message-format=short")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  feature `public-dependency` is required

  consider adding `payload-features = [\"public-dependency\"]` to the manifest
",
        )
        .run()
}

#[payload_test]
fn pub_dev_dependency() {
    Package::new("pub_dep", "0.1.0")
        .file("src/lib.rs", "pub struct FromPub;")
        .publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
                payload-features = ["public-dependency"]

                [package]
                name = "foo"
                version = "0.0.1"

                [dev-dependencies]
                pub_dep = {version = "0.1.0", public = true}
            "#,
        )
        .file(
            "src/lib.rs",
            "
            extern crate pub_dep;
            pub fn use_pub(_: pub_dep::FromPub) {}
        ",
        )
        .build();

    p.payload("build --message-format=short")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "\
error: failed to parse manifest at `[..]`

Caused by:
  'public' specifier can only be used on regular dependencies, not Development dependencies
",
        )
        .run()
}
