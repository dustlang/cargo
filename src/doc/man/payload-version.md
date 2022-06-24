# payload-version(1)

## NAME

payload-version - Show version information

## SYNOPSIS

`payload version` [_options_]

## DESCRIPTION

Displays the version of Payload.

## OPTIONS

{{#options}}

{{#option "`-v`" "`--verbose`" }}
Display additional version information.
{{/option}}

{{/options}}

## EXAMPLES

1. Display the version:

       payload version

2. The version is also available via flags:

       payload --version
       payload -V

3. Display extra version information:

       payload -Vv

## SEE ALSO
{{man "payload" 1}}
