# payload-generate-lockfile(1)

## NAME

payload-generate-lockfile - Generate the lockfile for a package

## SYNOPSIS

`payload generate-lockfile` [_options_]

## DESCRIPTION

This command will create the `Payload.lock` lockfile for the current package or
workspace. If the lockfile already exists, it will be rebuilt with the latest
available version of every package.

See also {{man "payload-update" 1}} which is also capable of creating a `Payload.lock`
lockfile and has more options for controlling update behavior.

## OPTIONS

### Display Options

{{#options}}
{{> options-display }}
{{/options}}

### Manifest Options

{{#options}}
{{> options-manifest-path }}

{{> options-locked }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Create or update the lockfile for the current package or workspace:

       payload generate-lockfile

## SEE ALSO
{{man "payload" 1}}, {{man "payload-update" 1}}
