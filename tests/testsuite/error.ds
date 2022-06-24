//! General error tests that don't belong anywhere else.

use payload_test_support::payload_process;

#[payload_test]
fn internal_error() {
    payload_process("init")
        .env("__PAYLOAD_TEST_INTERNAL_ERROR", "1")
        .with_status(101)
        .with_stderr(
            "\
[ERROR] internal error test
[NOTE] this is an unexpected payload internal error
[NOTE] we would appreciate a bug report: https://github.com/dustlang/payload/issues/
[NOTE] payload [..]
",
        )
        .run();
}
