//! Tests for the `payload pkgid` command.

use payload_test_support::project;
use payload_test_support::registry::Package;

#[payload_test]
fn simple() {
    Package::new("bar", "0.1.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                edition = "2018"

                [dependencies]
                bar = "0.1.0"
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .build();

    p.payload("generate-lockfile").run();

    p.payload("pkgid foo")
        .with_stdout(format!("file://[..]{}#0.1.0", p.root().to_str().unwrap()))
        .run();

    p.payload("pkgid bar")
        .with_stdout("https://github.com/dustlang/crates.io-index#bar:0.1.0")
        .run();
}

#[payload_test]
fn suggestion_bad_pkgid() {
    Package::new("crates-io", "0.1.0").publish();
    Package::new("two-ver", "0.1.0").publish();
    Package::new("two-ver", "0.2.0").publish();
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                edition = "2018"

                [dependencies]
                crates-io = "0.1.0"
                two-ver = "0.1.0"
                two-ver2 = { package = "two-ver", version = "0.2.0" }
            "#,
        )
        .file("src/lib.rs", "")
        .file("cratesio", "")
        .build();

    p.payload("generate-lockfile").run();

    // Bad URL.
    p.payload("pkgid https://example.com/crates-io")
        .with_status(101)
        .with_stderr(
            "\
error: package ID specification `https://example.com/crates-io` did not match any packages
Did you mean one of these?

  crates-io:0.1.0
",
        )
        .run();

    // Bad name.
    p.payload("pkgid crates_io")
        .with_status(101)
        .with_stderr(
            "\
error: package ID specification `crates_io` did not match any packages

<tab>Did you mean `crates-io`?
",
        )
        .run();

    // Bad version.
    p.payload("pkgid two-ver:0.3.0")
        .with_status(101)
        .with_stderr(
            "\
error: package ID specification `two-ver:0.3.0` did not match any packages
Did you mean one of these?

  two-ver:0.1.0
  two-ver:0.2.0
",
        )
        .run();

    // Bad file URL.
    p.payload("pkgid ./Payload.toml")
        .with_status(101)
        .with_stderr(
            "\
error: invalid package ID specification: `./Payload.toml`

Caused by:
  package ID specification `./Payload.toml` looks like a file path, maybe try file://[..]/Payload.toml
",
        )
        .run();

    // Bad file URL with simliar name.
    p.payload("pkgid './cratesio'")
        .with_status(101)
        .with_stderr(
            "\
error: invalid package ID specification: `./cratesio`

<tab>Did you mean `crates-io`?

Caused by:
  package ID specification `./cratesio` looks like a file path, maybe try file://[..]/cratesio
",
        )
        .run();
}
