# payload-yank(1)

## NAME

payload-yank - Remove a pushed crate from the index

## SYNOPSIS

`payload yank` [_options_] `--vers` _version_ [_crate_]

## DESCRIPTION

The yank command removes a previously published crate's version from the
server's index. This command does not delete any data, and the crate will
still be available for download via the registry's download link.

Note that existing crates locked to a yanked version will still be able to
download the yanked version to use it. Payload will, however, not allow any new
crates to be locked to any yanked version.

This command requires you to be authenticated with either the `--token` option
or using {{man "payload-login" 1}}.

If the crate name is not specified, it will use the package name from the
current directory.

## OPTIONS

### Yank Options

{{#options}}

{{#option "`--vers` _version_" }}
The version to yank or un-yank.
{{/option}}

{{#option "`--undo`" }}
Undo a yank, putting a version back into the index.
{{/option}}

{{> options-token }}

{{> options-index }}

{{> options-registry }}

{{/options}}

### Display Options

{{#options}}

{{> options-display }}

{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Yank a crate from the index:

       payload yank --vers 1.0.7 foo

## SEE ALSO
{{man "payload" 1}}, {{man "payload-login" 1}}, {{man "payload-publish" 1}}
