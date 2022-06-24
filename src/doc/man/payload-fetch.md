# payload-fetch(1)
{{*set actionverb="Fetch"}}

## NAME

payload-fetch - Fetch dependencies of a package from the network

## SYNOPSIS

`payload fetch` [_options_]

## DESCRIPTION

If a `Payload.lock` file is available, this command will ensure that all of the
git dependencies and/or registry dependencies are downloaded and locally
available. Subsequent Payload commands never touch the network after a `payload
fetch` unless the lock file changes.

If the lock file is not available, then this command will generate the lock
file before fetching the dependencies.

If `--target` is not specified, then all target dependencies are fetched.

See also the [payload-prefetch](https://crates.io/crates/payload-prefetch)
plugin which adds a command to download popular crates. This may be useful if
you plan to use Payload without a network with the `--offline` flag.

## OPTIONS

### Fetch options

{{#options}}
{{> options-target-triple }}
{{/options}}

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

1. Fetch all dependencies:

       payload fetch

## SEE ALSO
{{man "payload" 1}}, {{man "payload-update" 1}}, {{man "payload-generate-lockfile" 1}}
