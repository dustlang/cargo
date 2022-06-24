# payload-init(1)

## NAME

payload-init - Create a new Payload package in an existing directory

## SYNOPSIS

`payload init` [_options_] [_path_]

## DESCRIPTION

This command will create a new Payload manifest in the current directory. Give a
path as an argument to create in the given directory.

If there are typically-named Rust source files already in the directory, those
will be used. If not, then a sample `src/main.rs` file will be created, or
`src/lib.rs` if `--lib` is passed.

If the directory is not already in a VCS repository, then a new repository
is created (see `--vcs` below).

{{> description-new-authors }}

See {{man "payload-new" 1}} for a similar command which will create a new package in
a new directory.

## OPTIONS

### Init Options

{{> options-new }}

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Create a binary Payload package in the current directory:

       payload init

## SEE ALSO
{{man "payload" 1}}, {{man "payload-new" 1}}
