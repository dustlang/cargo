//! Tests for target filter flags rith glob patterns.

use payload_test_support::{project, Project};

#[payload_test]
fn build_example() {
    full_project()
        .payload("build -v --example 'ex*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name example1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn build_bin() {
    full_project()
        .payload("build -v --bin 'bi*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name bin1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn build_bench() {
    full_project()
        .payload("build -v --bench 'be*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bench1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn build_test() {
    full_project()
        .payload("build -v --test 'te*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name test1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn check_example() {
    full_project()
        .payload("check -v --example 'ex*1'")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name example1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn check_bin() {
    full_project()
        .payload("check -v --bin 'bi*1'")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name bin1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn check_bench() {
    full_project()
        .payload("check -v --bench 'be*1'")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name bench1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn check_test() {
    full_project()
        .payload("check -v --test 'te*1'")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name test1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn doc_bin() {
    full_project()
        .payload("doc -v --bin 'bi*1'")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc --crate-type bin --crate-name bin1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn fix_example() {
    full_project()
        .payload("fix -v --example 'ex*1' --allow-no-vcs")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `[..] rustc --crate-name example1 [..]`
[FIXING] examples/example1.rs
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn fix_bin() {
    full_project()
        .payload("fix -v --bin 'bi*1' --allow-no-vcs")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `[..] rustc --crate-name bin1 [..]`
[FIXING] src/bin/bin1.rs
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn fix_bench() {
    full_project()
        .payload("fix -v --bench 'be*1' --allow-no-vcs")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `[..] rustc --crate-name bench1 [..]`
[FIXING] benches/bench1.rs
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn fix_test() {
    full_project()
        .payload("fix -v --test 'te*1' --allow-no-vcs")
        .with_stderr(
            "\
[CHECKING] foo v0.0.1 ([CWD])
[RUNNING] `[..] rustc --crate-name test1 [..]`
[FIXING] tests/test1.rs
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn run_example_and_bin() {
    let p = full_project();
    p.payload("run -v --bin 'bi*1'")
        .with_status(101)
        .with_stderr("[ERROR] `payload run` does not support glob patterns on target selection")
        .run();

    p.payload("run -v --example 'ex*1'")
        .with_status(101)
        .with_stderr("[ERROR] `payload run` does not support glob patterns on target selection")
        .run();
}

#[payload_test]
fn test_example() {
    full_project()
        .payload("test -v --example 'ex*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name example1 [..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
[RUNNING] [..]example1[..]
",
        )
        .run();
}

#[payload_test]
fn test_bin() {
    full_project()
        .payload("test -v --bin 'bi*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name bin1 [..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
[RUNNING] [..]bin1[..]
",
        )
        .run();
}

#[payload_test]
fn test_bench() {
    full_project()
        .payload("test -v --bench 'be*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bench1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
[RUNNING] [..]bench1[..]
",
        )
        .run();
}

#[payload_test]
fn test_test() {
    full_project()
        .payload("test -v --test 'te*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name test1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] test [unoptimized + debuginfo] target(s) in [..]
[RUNNING] [..]test1[..]
",
        )
        .run();
}

#[payload_test]
fn bench_example() {
    full_project()
        .payload("bench -v --example 'ex*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name example1 [..]`
[FINISHED] bench [optimized] target(s) in [..]
[RUNNING] `[..]example1[..] --bench`
",
        )
        .run();
}

#[payload_test]
fn bench_bin() {
    full_project()
        .payload("bench -v --bin 'bi*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name bin1 [..]`
[FINISHED] bench [optimized] target(s) in [..]
[RUNNING] `[..]bin1[..] --bench`
",
        )
        .run();
}

#[payload_test]
fn bench_bench() {
    full_project()
        .payload("bench -v --bench 'be*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bench1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] bench [optimized] target(s) in [..]
[RUNNING] `[..]bench1[..] --bench`
",
        )
        .run();
}

#[payload_test]
fn bench_test() {
    full_project()
        .payload("bench -v --test 'te*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name test1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] bench [optimized] target(s) in [..]
[RUNNING] `[..]test1[..] --bench`
",
        )
        .run();
}

#[payload_test]
fn install_example() {
    full_project()
        .payload("install --path . --example 'ex*1'")
        .with_stderr(
            "\
[INSTALLING] foo v0.0.1 ([CWD])
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]/home/.payload/bin/example1[EXE]
[INSTALLED] package `foo v0.0.1 ([CWD])` (executable `example1[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[payload_test]
fn install_bin() {
    full_project()
        .payload("install --path . --bin 'bi*1'")
        .with_stderr(
            "\
[INSTALLING] foo v0.0.1 ([CWD])
[COMPILING] foo v0.0.1 ([CWD])
[FINISHED] release [optimized] target(s) in [..]
[INSTALLING] [..]/home/.payload/bin/bin1[EXE]
[INSTALLED] package `foo v0.0.1 ([CWD])` (executable `bin1[EXE]`)
[WARNING] be sure to add [..]
",
        )
        .run();
}

#[payload_test]
fn rustdoc_example() {
    full_project()
        .payload("rustdoc -v --example 'ex*1'")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc --crate-type bin --crate-name example1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustdoc_bin() {
    full_project()
        .payload("rustdoc -v --bin 'bi*1'")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc --crate-type bin --crate-name bin1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustdoc_bench() {
    full_project()
        .payload("rustdoc -v --bench 'be*1'")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc --crate-type bin --crate-name bench1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustdoc_test() {
    full_project()
        .payload("rustdoc -v --test 'te*1'")
        .with_stderr(
            "\
[DOCUMENTING] foo v0.0.1 ([CWD])
[RUNNING] `rustdoc --crate-type bin --crate-name test1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustc_example() {
    full_project()
        .payload("rustc -v --example 'ex*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name example1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustc_bin() {
    full_project()
        .payload("rustc -v --bin 'bi*1'")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name bin1 [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustc_bench() {
    full_project()
        .payload("rustc -v --bench 'be*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bench1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

#[payload_test]
fn rustc_test() {
    full_project()
        .payload("rustc -v --test 'te*1'")
        .with_stderr_contains("[RUNNING] `rustc --crate-name test1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin2 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name bin1 [..]`")
        .with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]`")
        .with_stderr(
            "\
[COMPILING] foo v0.0.1 ([CWD])
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[RUNNING] `rustc --crate-name [..]`
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        )
        .run();
}

fn full_project() -> Project {
    project()
        .file("examples/example1.rs", "fn main() { }")
        .file("examples/example2.rs", "fn main() { }")
        .file("benches/bench1.rs", "")
        .file("benches/bench2.rs", "")
        .file("tests/test1.rs", "")
        .file("tests/test2.rs", "")
        .file("src/main.rs", "fn main() { }")
        .file("src/bin/bin1.rs", "fn main() { }")
        .file("src/bin/bin2.rs", "fn main() { }")
        .build()
}
