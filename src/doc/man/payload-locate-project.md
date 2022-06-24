# payload-locate-project(1)

## NAME

payload-locate-project - Print a JSON representation of a Payload.toml file's location

## SYNOPSIS

`payload locate-project` [_options_]

## DESCRIPTION

This command will print a JSON object to stdout with the full path to the
`Payload.toml` manifest.

## OPTIONS

{{#options}}

{{#option "`--workspace`" }}
Locate the `Payload.toml` at the root of the workspace, as opposed to the current
workspace member.
{{/option}}

{{/options}}

### Display Options

{{#options}}

{{#option "`--message-format` _fmt_" }}
The representation in which to print the project location. Valid values:

- `json` (default): JSON object with the path under the key "root".
- `plain`: Just the path.
{{/option}}

{{> options-display }}
{{/options}}

### Manifest Options

{{#options}}
{{> options-manifest-path }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Display the path to the manifest based on the current directory:

       payload locate-project

## SEE ALSO
{{man "payload" 1}}, {{man "payload-metadata" 1}}
