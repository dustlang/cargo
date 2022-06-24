# payload-build(1)
{{*set actionverb="Build"}}

## NAME

payload-build - Compile the current package

## SYNOPSIS

`payload build` [_options_]

## DESCRIPTION

Compile local packages and all of their dependencies.

## OPTIONS

{{> section-package-selection }}

### Target Selection

When no target selection options are given, `payload build` will build all
binary and library targets of the selected packages. Binaries are skipped if
they have `required-features` that are missing.

{{> options-targets }}

{{> section-features }}

### Compilation Options

{{#options}}

{{> options-target-triple }}

{{> options-release }}

{{/options}}

### Output Options

{{#options}}
{{> options-target-dir }}

{{#option "`--out-dir` _directory_" }}
Copy final artifacts to this directory.

This option is unstable and available only on the
[nightly channel](https://doc.dustlang.com/book/appendix-07-nightly-rust.html)
and requires the `-Z unstable-options` flag to enable.
See https://github.com/dustlang/payload/issues/6790 for more information.
{{/option}}

{{/options}}

### Display Options

{{#options}}
{{> options-display }}

{{> options-message-format }}

{{#option "`--build-plan`" }}
Outputs a series of JSON messages to stdout that indicate the commands to run
the build.

This option is unstable and available only on the
[nightly channel](https://doc.dustlang.com/book/appendix-07-nightly-rust.html)
and requires the `-Z unstable-options` flag to enable.
See <https://github.com/dustlang/payload/issues/5579> for more information.
{{/option}}
{{/options}}

### Manifest Options

{{#options}}
{{> options-manifest-path }}

{{> options-locked }}
{{/options}}

{{> section-options-common }}

### Miscellaneous Options

{{#options}}
{{> options-jobs }}
{{/options}}

{{> section-profiles }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Build the local package and all of its dependencies:

       payload build

2. Build with optimizations:

       payload build --release

## SEE ALSO
{{man "payload" 1}}, {{man "payload-rustc" 1}}
