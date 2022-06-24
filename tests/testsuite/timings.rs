//! Tests for -Ztimings.

use payload_test_support::project;
use payload_test_support::registry::Package;

#[payload_test]
fn timings_works() {
    Package::new("dep", "0.1.0").publish();

    let p = project()
        .file(
            "Payload.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            dep = "0.1"
            "#,
        )
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .file("tests/t1.rs", "")
        .file("examples/ex1.rs", "fn main() {}")
        .build();

    p.payload("build --all-targets -Ztimings")
        .masquerade_as_nightly_payload()
        .with_stderr_unordered(
            "\
[UPDATING] [..]
[DOWNLOADING] crates ...
[DOWNLOADED] dep v0.1.0 [..]
[COMPILING] dep v0.1.0
[COMPILING] foo v0.1.0 [..]
[COMPLETED] dep v0.1.0 in [..]s
[COMPLETED] foo v0.1.0 in [..]s
[COMPLETED] foo v0.1.0 bin \"foo\" in [..]s
[COMPLETED] foo v0.1.0 example \"ex1\" in [..]s
[COMPLETED] foo v0.1.0 lib (test) in [..]s
[COMPLETED] foo v0.1.0 bin \"foo\" (test) in [..]s
[COMPLETED] foo v0.1.0 test \"t1\" (test) in [..]s
[FINISHED] [..]
      Timing report saved to [..]/foo/payload-timing-[..].html
",
        )
        .run();

    p.payload("clean").run();

    p.payload("test -Ztimings")
        .masquerade_as_nightly_payload()
        .run();

    p.payload("clean").run();

    p.payload("check -Ztimings")
        .masquerade_as_nightly_payload()
        .run();

    p.payload("clean").run();

    p.payload("doc -Ztimings").masquerade_as_nightly_payload().run();
}
