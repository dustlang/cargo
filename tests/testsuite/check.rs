//! Tests for the `payload check` command.

use std::fmt::{self, Write};

use payload_test_support::install::exe;
use payload_test_support::paths::PayloadPathExt;
use payload_test_support::registry::Package;
use payload_test_support::{basic_manifest, project};

#[payload_test]
fn check_success() {
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate bar; fn main() { ::bar::baz(); }",
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.payload("check").run();
}

#[payload_test]
fn check_fail() {
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate bar; fn main() { ::bar::baz(42); }",
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.payload("check")
        .with_status(101)
        .with_stderr_contains("[..]this function takes 0[..]")
        .run();
}

#[payload_test]
fn custom_derive() {
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file(
            "src/main.rs",
            r#"
            #[macro_use]
            extern crate bar;

            trait B {
                fn b(&self);
            }

            #[derive(B)]
            struct A;

            fn main() {
                let a = A;
                a.b();
            }
            "#,
        )
        .build();
    let _bar = project()
        .at("bar")
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "bar"
                version = "0.1.0"
                authors = []
                [lib]
                proc-macro = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            extern crate proc_macro;

            use proc_macro::TokenStream;

            #[proc_macro_derive(B)]
            pub fn derive(_input: TokenStream) -> TokenStream {
                format!("impl B for A {{ fn b(&self) {{}} }}").parse().unwrap()
            }
            "#,
        )
        .build();

    foo.payload("check").run();
}

#[payload_test]
fn check_build() {
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate bar; fn main() { ::bar::baz(); }",
        )
        .build();

    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.payload("check").run();
    foo.payload("build").run();
}

#[payload_test]
fn build_check() {
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate bar; fn main() { ::bar::baz(); }",
        )
        .build();

    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.payload("build -v").run();
    foo.payload("check -v").run();
}

// Checks that where a project has both a lib and a bin, the lib is only checked
// not built.
#[payload_test]
fn issue_3418() {
    let foo = project()
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .build();

    foo.payload("check -v")
        .with_stderr_contains("[..] --emit=[..]metadata [..]")
        .run();
}

// Some weirdness that seems to be caused by a crate being built as well as
// checked, but in this case with a proc macro too.
#[payload_test]
fn issue_3419() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies]
                rustc-serialize = "*"
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                extern crate rustc_serialize;

                use rustc_serialize::Decodable;

                pub fn take<T: Decodable>() {}
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                extern crate rustc_serialize;

                extern crate foo;

                #[derive(RustcDecodable)]
                pub struct Foo;

                fn main() {
                    foo::take::<Foo>();
                }
            "#,
        )
        .build();

    Package::new("rustc-serialize", "1.0.0")
        .file(
            "src/lib.rs",
            r#"
                pub trait Decodable: Sized {
                    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
                }
                pub trait Decoder {
                   type Error;
                   fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F)
                                        -> Result<T, Self::Error>
                   where F: FnOnce(&mut Self) -> Result<T, Self::Error>;
                }
            "#,
        )
        .publish();

    p.payload("check").run();
}

// Check on a dylib should have a different metadata hash than build.
#[payload_test]
fn dylib_check_preserves_build_cache() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.1.0"
                authors = []

                [lib]
                crate-type = ["dylib"]

                [dependencies]
            "#,
        )
        .file("src/lib.rs", "")
        .build();

    p.payload("build")
        .with_stderr(
            "\
[..]Compiling foo v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();

    p.payload("check").run();

    p.payload("build")
        .with_stderr("[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]")
        .run();
}

// test `payload rustc --profile check`
#[payload_test]
fn rustc_check() {
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate bar; fn main() { ::bar::baz(); }",
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.payload("rustc --profile check -- --emit=metadata").run();

    // Verify compatible usage of --profile with --release, issue #7488
    foo.payload("rustc --profile check --release -- --emit=metadata")
        .run();
    foo.payload("rustc --profile test --release -- --emit=metadata")
        .run();
}

#[payload_test]
fn rustc_check_err() {
    let foo = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [dependencies.bar]
                path = "../bar"
            "#,
        )
        .file(
            "src/main.rs",
            "extern crate bar; fn main() { ::bar::qux(); }",
        )
        .build();
    let _bar = project()
        .at("bar")
        .file("Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("src/lib.rs", "pub fn baz() {}")
        .build();

    foo.payload("rustc --profile check -- --emit=metadata")
        .with_status(101)
        .with_stderr_contains("[CHECKING] bar [..]")
        .with_stderr_contains("[CHECKING] foo [..]")
        .with_stderr_contains("[..]cannot find function `qux` in [..] `bar`")
        .run();
}

#[payload_test]
fn check_all() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []

                [workspace]
                [dependencies]
                b = { path = "b" }
            "#,
        )
        .file("src/main.rs", "fn main() {}")
        .file("examples/a.rs", "fn main() {}")
        .file("tests/a.rs", "")
        .file("src/lib.rs", "")
        .file("b/Payload.toml", &basic_manifest("b", "0.0.1"))
        .file("b/src/main.rs", "fn main() {}")
        .file("b/src/lib.rs", "")
        .build();

    p.payload("check --workspace -v")
        .with_stderr_contains("[..] --crate-name foo src/lib.rs [..]")
        .with_stderr_contains("[..] --crate-name foo src/main.rs [..]")
        .with_stderr_contains("[..] --crate-name b b/src/lib.rs [..]")
        .with_stderr_contains("[..] --crate-name b b/src/main.rs [..]")
        .run();
}

#[payload_test]
fn check_all_exclude() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["bar", "baz"]
            "#,
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() { break_the_build(); }")
        .build();

    p.payload("check --workspace --exclude baz")
        .with_stderr_does_not_contain("[CHECKING] baz v0.1.0 [..]")
        .with_stderr(
            "\
[CHECKING] bar v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn check_all_exclude_glob() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["bar", "baz"]
            "#,
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() { break_the_build(); }")
        .build();

    p.payload("check --workspace --exclude '*z'")
        .with_stderr_does_not_contain("[CHECKING] baz v0.1.0 [..]")
        .with_stderr(
            "\
[CHECKING] bar v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn check_virtual_all_implied() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["bar", "baz"]
            "#,
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.payload("check -v")
        .with_stderr_contains("[..] --crate-name bar bar/src/lib.rs [..]")
        .with_stderr_contains("[..] --crate-name baz baz/src/lib.rs [..]")
        .run();
}

#[payload_test]
fn check_virtual_manifest_one_project() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["bar", "baz"]
            "#,
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() { break_the_build(); }")
        .build();

    p.payload("check -p bar")
        .with_stderr_does_not_contain("[CHECKING] baz v0.1.0 [..]")
        .with_stderr(
            "\
[CHECKING] bar v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn check_virtual_manifest_glob() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["bar", "baz"]
            "#,
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {  break_the_build(); }")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.payload("check -p '*z'")
        .with_stderr_does_not_contain("[CHECKING] bar v0.1.0 [..]")
        .with_stderr(
            "\
[CHECKING] baz v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn exclude_warns_on_non_existing_package() {
    let p = project().file("src/lib.rs", "").build();
    p.payload("check --workspace --exclude bar")
        .with_stdout("")
        .with_stderr(
            "\
[WARNING] excluded package(s) `bar` not found in workspace `[CWD]`
[CHECKING] foo v0.0.1 ([CWD])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn targets_selected_default() {
    let foo = project()
        .file("src/main.rs", "fn main() {}")
        .file("src/lib.rs", "pub fn smth() {}")
        .file("examples/example1.rs", "fn main() {}")
        .file("tests/test2.rs", "#[test] fn t() {}")
        .file("benches/bench3.rs", "")
        .build();

    foo.payload("check -v")
        .with_stderr_contains("[..] --crate-name foo src/lib.rs [..]")
        .with_stderr_contains("[..] --crate-name foo src/main.rs [..]")
        .with_stderr_does_not_contain("[..] --crate-name example1 examples/example1.rs [..]")
        .with_stderr_does_not_contain("[..] --crate-name test2 tests/test2.rs [..]")
        .with_stderr_does_not_contain("[..] --crate-name bench3 benches/bench3.rs [..]")
        .run();
}

#[payload_test]
fn targets_selected_all() {
    let foo = project()
        .file("src/main.rs", "fn main() {}")
        .file("src/lib.rs", "pub fn smth() {}")
        .file("examples/example1.rs", "fn main() {}")
        .file("tests/test2.rs", "#[test] fn t() {}")
        .file("benches/bench3.rs", "")
        .build();

    foo.payload("check --all-targets -v")
        .with_stderr_contains("[..] --crate-name foo src/lib.rs [..]")
        .with_stderr_contains("[..] --crate-name foo src/main.rs [..]")
        .with_stderr_contains("[..] --crate-name example1 examples/example1.rs [..]")
        .with_stderr_contains("[..] --crate-name test2 tests/test2.rs [..]")
        .with_stderr_contains("[..] --crate-name bench3 benches/bench3.rs [..]")
        .run();
}

#[payload_test]
fn check_unit_test_profile() {
    let foo = project()
        .file(
            "src/lib.rs",
            r#"
                #[cfg(test)]
                mod tests {
                    #[test]
                    fn it_works() {
                        badtext
                    }
                }
            "#,
        )
        .build();

    foo.payload("check").run();
    foo.payload("check --profile test")
        .with_status(101)
        .with_stderr_contains("[..]badtext[..]")
        .run();
}

// Verify what is checked with various command-line filters.
#[payload_test]
fn check_filters() {
    let p = project()
        .file(
            "src/lib.rs",
            r#"
                fn unused_normal_lib() {}
                #[cfg(test)]
                mod tests {
                    fn unused_unit_lib() {}
                }
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                fn main() {}
                fn unused_normal_bin() {}
                #[cfg(test)]
                mod tests {
                    fn unused_unit_bin() {}
                }
            "#,
        )
        .file(
            "tests/t1.rs",
            r#"
                fn unused_normal_t1() {}
                #[cfg(test)]
                mod tests {
                    fn unused_unit_t1() {}
                }
            "#,
        )
        .file(
            "examples/ex1.rs",
            r#"
                fn main() {}
                fn unused_normal_ex1() {}
                #[cfg(test)]
                mod tests {
                    fn unused_unit_ex1() {}
                }
            "#,
        )
        .file(
            "benches/b1.rs",
            r#"
                fn unused_normal_b1() {}
                #[cfg(test)]
                mod tests {
                    fn unused_unit_b1() {}
                }
            "#,
        )
        .build();

    p.payload("check")
        .with_stderr_contains("[..]unused_normal_lib[..]")
        .with_stderr_contains("[..]unused_normal_bin[..]")
        .with_stderr_does_not_contain("[..]unused_normal_t1[..]")
        .with_stderr_does_not_contain("[..]unused_normal_ex1[..]")
        .with_stderr_does_not_contain("[..]unused_normal_b1[..]")
        .with_stderr_does_not_contain("[..]unused_unit_[..]")
        .run();
    p.root().join("target").rm_rf();
    p.payload("check --tests -v")
        .with_stderr_contains("[..] --crate-name foo src/lib.rs [..] --test [..]")
        .with_stderr_contains("[..] --crate-name foo src/lib.rs [..] --crate-type lib [..]")
        .with_stderr_contains("[..] --crate-name foo src/main.rs [..] --test [..]")
        .with_stderr_contains("[..]unused_unit_lib[..]")
        .with_stderr_contains("[..]unused_unit_bin[..]")
        .with_stderr_contains("[..]unused_normal_lib[..]")
        .with_stderr_contains("[..]unused_normal_bin[..]")
        .with_stderr_contains("[..]unused_unit_t1[..]")
        .with_stderr_does_not_contain("[..]unused_normal_ex1[..]")
        .with_stderr_does_not_contain("[..]unused_unit_ex1[..]")
        .with_stderr_does_not_contain("[..]unused_normal_b1[..]")
        .with_stderr_does_not_contain("[..]unused_unit_b1[..]")
        .with_stderr_does_not_contain("[..]--crate-type bin[..]")
        .run();
    p.root().join("target").rm_rf();
    p.payload("check --test t1 -v")
        .with_stderr_contains("[..]unused_normal_lib[..]")
        .with_stderr_contains("[..]unused_unit_t1[..]")
        .with_stderr_does_not_contain("[..]unused_unit_lib[..]")
        .with_stderr_does_not_contain("[..]unused_normal_bin[..]")
        .with_stderr_does_not_contain("[..]unused_unit_bin[..]")
        .with_stderr_does_not_contain("[..]unused_normal_ex1[..]")
        .with_stderr_does_not_contain("[..]unused_normal_b1[..]")
        .with_stderr_does_not_contain("[..]unused_unit_ex1[..]")
        .with_stderr_does_not_contain("[..]unused_unit_b1[..]")
        .run();
    p.root().join("target").rm_rf();
    p.payload("check --all-targets -v")
        .with_stderr_contains("[..]unused_normal_lib[..]")
        .with_stderr_contains("[..]unused_normal_bin[..]")
        .with_stderr_contains("[..]unused_normal_t1[..]")
        .with_stderr_contains("[..]unused_normal_ex1[..]")
        .with_stderr_contains("[..]unused_normal_b1[..]")
        .with_stderr_contains("[..]unused_unit_b1[..]")
        .with_stderr_contains("[..]unused_unit_t1[..]")
        .with_stderr_contains("[..]unused_unit_lib[..]")
        .with_stderr_contains("[..]unused_unit_bin[..]")
        .with_stderr_does_not_contain("[..]unused_unit_ex1[..]")
        .run();
}

#[payload_test]
fn check_artifacts() {
    // Verify which artifacts are created when running check (#4059).
    let p = project()
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .file("tests/t1.rs", "")
        .file("examples/ex1.rs", "fn main() {}")
        .file("benches/b1.rs", "")
        .build();

    p.payload("check").run();
    assert!(!p.root().join("target/debug/libfoo.rmeta").is_file());
    assert!(!p.root().join("target/debug/libfoo.rlib").is_file());
    assert!(!p.root().join("target/debug").join(exe("foo")).is_file());
    assert_eq!(p.glob("target/debug/deps/libfoo-*.rmeta").count(), 2);

    p.root().join("target").rm_rf();
    p.payload("check --lib").run();
    assert!(!p.root().join("target/debug/libfoo.rmeta").is_file());
    assert!(!p.root().join("target/debug/libfoo.rlib").is_file());
    assert!(!p.root().join("target/debug").join(exe("foo")).is_file());
    assert_eq!(p.glob("target/debug/deps/libfoo-*.rmeta").count(), 1);

    p.root().join("target").rm_rf();
    p.payload("check --bin foo").run();
    assert!(!p.root().join("target/debug/libfoo.rmeta").is_file());
    assert!(!p.root().join("target/debug/libfoo.rlib").is_file());
    assert!(!p.root().join("target/debug").join(exe("foo")).is_file());
    assert_eq!(p.glob("target/debug/deps/libfoo-*.rmeta").count(), 2);

    p.root().join("target").rm_rf();
    p.payload("check --test t1").run();
    assert!(!p.root().join("target/debug/libfoo.rmeta").is_file());
    assert!(!p.root().join("target/debug/libfoo.rlib").is_file());
    assert!(!p.root().join("target/debug").join(exe("foo")).is_file());
    assert_eq!(p.glob("target/debug/t1-*").count(), 0);
    assert_eq!(p.glob("target/debug/deps/libfoo-*.rmeta").count(), 1);
    assert_eq!(p.glob("target/debug/deps/libt1-*.rmeta").count(), 1);

    p.root().join("target").rm_rf();
    p.payload("check --example ex1").run();
    assert!(!p.root().join("target/debug/libfoo.rmeta").is_file());
    assert!(!p.root().join("target/debug/libfoo.rlib").is_file());
    assert!(!p
        .root()
        .join("target/debug/examples")
        .join(exe("ex1"))
        .is_file());
    assert_eq!(p.glob("target/debug/deps/libfoo-*.rmeta").count(), 1);
    assert_eq!(p.glob("target/debug/examples/libex1-*.rmeta").count(), 1);

    p.root().join("target").rm_rf();
    p.payload("check --bench b1").run();
    assert!(!p.root().join("target/debug/libfoo.rmeta").is_file());
    assert!(!p.root().join("target/debug/libfoo.rlib").is_file());
    assert!(!p.root().join("target/debug").join(exe("foo")).is_file());
    assert_eq!(p.glob("target/debug/b1-*").count(), 0);
    assert_eq!(p.glob("target/debug/deps/libfoo-*.rmeta").count(), 1);
    assert_eq!(p.glob("target/debug/deps/libb1-*.rmeta").count(), 1);
}

#[payload_test]
fn short_message_format() {
    let foo = project()
        .file("src/lib.rs", "fn foo() { let _x: bool = 'a'; }")
        .build();
    foo.payload("check --message-format=short")
        .with_status(101)
        .with_stderr_contains(
            "\
src/lib.rs:1:27: error[E0308]: mismatched types
error: aborting due to previous error
error: could not compile `foo`
",
        )
        .run();
}

#[payload_test]
fn proc_macro() {
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [package]
                name = "demo"
                version = "0.0.1"

                [lib]
                proc-macro = true
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                extern crate proc_macro;

                use proc_macro::TokenStream;

                #[proc_macro_derive(Foo)]
                pub fn demo(_input: TokenStream) -> TokenStream {
                    "".parse().unwrap()
                }
            "#,
        )
        .file(
            "src/main.rs",
            r#"
                #[macro_use]
                extern crate demo;

                #[derive(Foo)]
                struct A;

                fn main() {}
            "#,
        )
        .build();
    p.payload("check -v").env("PAYLOAD_LOG", "payload=trace").run();
}

#[payload_test]
fn does_not_use_empty_rustc_wrapper() {
    let p = project().file("src/lib.rs", "").build();
    p.payload("check").env("RUSTC_WRAPPER", "").run();
}

#[payload_test]
fn does_not_use_empty_rustc_workspace_wrapper() {
    let p = project().file("src/lib.rs", "").build();
    p.payload("check").env("RUSTC_WORKSPACE_WRAPPER", "").run();
}

#[payload_test]
fn error_from_deep_recursion() -> Result<(), fmt::Error> {
    let mut big_macro = String::new();
    writeln!(big_macro, "macro_rules! m {{")?;
    for i in 0..130 {
        writeln!(big_macro, "({}) => {{ m!({}); }};", i, i + 1)?;
    }
    writeln!(big_macro, "}}")?;
    writeln!(big_macro, "m!(0);")?;

    let p = project().file("src/lib.rs", &big_macro).build();
    p.payload("check --message-format=json")
        .with_status(101)
        .with_stdout_contains(
            "[..]\"message\":\"recursion limit reached while expanding [..]`m[..]`\"[..]",
        )
        .run();

    Ok(())
}

#[payload_test]
fn rustc_workspace_wrapper_affects_all_workspace_members() {
    use payload_test_support::paths;
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["bar", "baz"]
            "#,
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.payload("check")
        .env("RUSTC_WORKSPACE_WRAPPER", paths::echo_wrapper())
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name bar [..]")
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name baz [..]")
        .run();
}

#[payload_test]
fn rustc_workspace_wrapper_includes_path_deps() {
    use payload_test_support::paths;
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["bar"]

                [dependencies]
                baz = { path = "baz" }
            "#,
        )
        .file("src/lib.rs", "")
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.payload("check --workspace")
        .env("RUSTC_WORKSPACE_WRAPPER", paths::echo_wrapper())
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name foo [..]")
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name bar [..]")
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name baz [..]")
        .run();
}

#[payload_test]
fn rustc_workspace_wrapper_respects_primary_units() {
    use payload_test_support::paths;
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [workspace]
                members = ["bar", "baz"]
            "#,
        )
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .file("baz/Payload.toml", &basic_manifest("baz", "0.1.0"))
        .file("baz/src/lib.rs", "pub fn baz() {}")
        .build();

    p.payload("check -p bar")
        .env("RUSTC_WORKSPACE_WRAPPER", paths::echo_wrapper())
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name bar [..]")
        .with_stdout_does_not_contain("WRAPPER CALLED: rustc --crate-name baz [..]")
        .run();
}

#[payload_test]
fn rustc_workspace_wrapper_excludes_published_deps() {
    use payload_test_support::paths;
    let p = project()
        .file(
            "Payload.toml",
            r#"
                [project]
                name = "foo"
                version = "0.1.0"
                authors = []

                [workspace]
                members = ["bar"]

                [dependencies]
                baz = "1.0.0"
            "#,
        )
        .file("src/lib.rs", "")
        .file("bar/Payload.toml", &basic_manifest("bar", "0.1.0"))
        .file("bar/src/lib.rs", "pub fn bar() {}")
        .build();

    Package::new("baz", "1.0.0").publish();

    p.payload("check --workspace -v")
        .env("RUSTC_WORKSPACE_WRAPPER", paths::echo_wrapper())
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name foo [..]")
        .with_stderr_contains("WRAPPER CALLED: rustc --crate-name bar [..]")
        .with_stderr_contains("[CHECKING] baz [..]")
        .with_stdout_does_not_contain("WRAPPER CALLED: rustc --crate-name baz [..]")
        .run();
}
