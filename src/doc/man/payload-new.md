# payload-new(1)

## NAME

payload-new - Create a new Payload package

## SYNOPSIS

`payload new` [_options_] _path_

## DESCRIPTION

This command will create a new Payload package in the given directory. This
includes a simple template with a `Payload.toml` manifest, sample source file,
and a VCS ignore file. If the directory is not already in a VCS repository,
then a new repository is created (see `--vcs` below).

{{> description-new-authors }}

See {{man "payload-init" 1}} for a similar command which will create a new manifest
in an existing directory.

## OPTIONS

### New Options

{{> options-new }}

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Create a binary Payload package in the given directory:

       payload new foo

## SEE ALSO
{{man "payload" 1}}, {{man "payload-init" 1}}
