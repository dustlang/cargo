# payload-test(1)
{{*set actionverb="Test"}}
{{*set nouns="tests"}}

## NAME

payload-test - Execute unit and integration tests of a package

## SYNOPSIS

`payload test` [_options_] [_testname_] [`--` _test-options_]

## DESCRIPTION

Compile and execute unit and integration tests.

The test filtering argument `TESTNAME` and all the arguments following the two
dashes (`--`) are passed to the test binaries and thus to _libtest_ (rustc's
built in unit-test and micro-benchmarking framework).  If you're passing
arguments to both Payload and the binary, the ones after `--` go to the binary,
the ones before go to Payload.  For details about libtest's arguments see the
output of `payload test -- --help`.

As an example, this will filter for tests with `foo` in their name and run them
on 3 threads in parallel:

    payload test foo -- --test-threads 3

Tests are built with the `--test` option to `rustc` which creates an
executable with a `main` function that automatically runs all functions
annotated with the `#[test]` attribute in multiple threads. `#[bench]`
annotated functions will also be run with one iteration to verify that they
are functional.

The libtest harness may be disabled by setting `harness = false` in the target
manifest settings, in which case your code will need to provide its own `main`
function to handle running tests.

Documentation tests are also run by default, which is handled by `rustdoc`. It
extracts code samples from documentation comments and executes them. See the
[rustdoc book](https://doc.dustlang.com/rustdoc/) for more information on
writing doc tests.

## OPTIONS

### Test Options

{{> options-test }}

{{> section-package-selection }}

### Target Selection

When no target selection options are given, `payload test` will build the
following targets of the selected packages:

- lib — used to link with binaries, examples, integration tests, and doc tests
- bins (only if integration tests are built and required features are
  available)
- examples — to ensure they compile
- lib as a unit test
- bins as unit tests
- integration tests
- doc tests for the lib target

The default behavior can be changed by setting the `test` flag for the target
in the manifest settings. Setting examples to `test = true` will build and run
the example as a test. Setting targets to `test = false` will stop them from
being tested by default. Target selection options that take a target by name
ignore the `test` flag and will always test the given target.

Doc tests for libraries may be disabled by setting `doctest = false` for the
library in the manifest.

Binary targets are automatically built if there is an integration test or
benchmark. This allows an integration test to execute the binary to exercise
and test its behavior. The `PAYLOAD_BIN_EXE_<name>`
[environment variable](../reference/environment-variables.html#environment-variables-payload-sets-for-crates)
is set when the integration test is built so that it can use the
[`env` macro](https://doc.dustlang.com/std/macro.env.html) to locate the
executable.

{{> options-targets }}

{{#options}}

{{#option "`--doc`" }}
Test only the library's documentation. This cannot be mixed with other
target options.
{{/option}}

{{/options}}

{{> section-features }}

### Compilation Options

{{#options}}

{{> options-target-triple }}

{{> options-release }}

{{/options}}

### Output Options

{{#options}}
{{> options-target-dir }}
{{/options}}

### Display Options

By default the Rust test harness hides output from test execution to keep
results readable. Test output can be recovered (e.g., for debugging) by passing
`--nocapture` to the test binaries:

    payload test -- --nocapture

{{#options}}

{{> options-display }}

{{> options-message-format }}

{{/options}}

### Manifest Options

{{#options}}

{{> options-manifest-path }}

{{> options-locked }}

{{/options}}

{{> section-options-common }}

### Miscellaneous Options

The `--jobs` argument affects the building of the test executable but does not
affect how many threads are used when running the tests. The Rust test harness
includes an option to control the number of threads used:

    payload test -j 2 -- --test-threads=2

{{#options}}

{{> options-jobs }}

{{/options}}

{{> section-profiles }}

Unit tests are separate executable artifacts which use the `test`/`bench`
profiles. Example targets are built the same as with `payload build` (using the
`dev`/`release` profiles) unless you are building them with the test harness
(by setting `test = true` in the manifest or using the `--example` flag) in
which case they use the `test`/`bench` profiles. Library targets are built
with the `dev`/`release` profiles when linked to an integration test, binary,
or doctest.

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Execute all the unit and integration tests of the current package:

       payload test

2. Run only tests whose names match against a filter string:

       payload test name_filter

3. Run only a specific test within a specific integration test:

       payload test --test int_test_name -- modname::test_name

## SEE ALSO
{{man "payload" 1}}, {{man "payload-bench" 1}}
