# payload(1)

## NAME

payload - The Rust package manager

## SYNOPSIS

`payload` [_options_] _command_ [_args_]\
`payload` [_options_] `--version`\
`payload` [_options_] `--list`\
`payload` [_options_] `--help`\
`payload` [_options_] `--explain` _code_

## DESCRIPTION

This program is a package manager and build tool for the Rust language,
available at <https://dustlang.com>.

## COMMANDS

### Build Commands

{{man "payload-bench" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Execute benchmarks of a package.

{{man "payload-build" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Compile a package.

{{man "payload-check" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Check a local package and all of its dependencies for errors.

{{man "payload-clean" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Remove artifacts that Payload has generated in the past.

{{man "payload-doc" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Build a package's documentation.

{{man "payload-fetch" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Fetch dependencies of a package from the network.

{{man "payload-fix" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Automatically fix lint warnings reported by rustc.

{{man "payload-run" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Run a binary or example of the local package.

{{man "payload-rustc" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Compile a package, and pass extra options to the compiler.

{{man "payload-rustdoc" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Build a package's documentation, using specified custom flags.

{{man "payload-test" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Execute unit and integration tests of a package.

### Manifest Commands

{{man "payload-generate-lockfile" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Generate `Payload.lock` for a project.

{{man "payload-locate-project" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Print a JSON representation of a `Payload.toml` file's location.

{{man "payload-metadata" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Output the resolved dependencies of a package in machine-readable format.

{{man "payload-pkgid" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Print a fully qualified package specification.

{{man "payload-tree" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Display a tree visualization of a dependency graph.

{{man "payload-update" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Update dependencies as recorded in the local lock file.

{{man "payload-vendor" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Vendor all dependencies locally.

{{man "payload-verify-project" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Check correctness of crate manifest.

### Package Commands

{{man "payload-init" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Create a new Payload package in an existing directory.

{{man "payload-install" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Build and install a Rust binary.

{{man "payload-new" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Create a new Payload package.

{{man "payload-search" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Search packages in crates.io.

{{man "payload-uninstall" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Remove a Rust binary.

### Publishing Commands

{{man "payload-login" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Save an API token from the registry locally.

{{man "payload-owner" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Manage the owners of a crate on the registry.

{{man "payload-package" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Assemble the local package into a distributable tarball.

{{man "payload-publish" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Upload a package to the registry.

{{man "payload-yank" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Remove a pushed crate from the index.

### General Commands

{{man "payload-help" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Display help information about Payload.

{{man "payload-version" 1}}\
&nbsp;&nbsp;&nbsp;&nbsp;Show version information.

## OPTIONS

### Special Options

{{#options}}

{{#option "`-V`" "`--version`" }}
Print version info and exit. If used with `--verbose`, prints extra
information.
{{/option}}

{{#option "`--list`" }}
List all installed Payload subcommands. If used with `--verbose`, prints extra
information.
{{/option}}

{{#option "`--explain` _code_" }}
Run `rustc --explain CODE` which will print out a detailed explanation of an
error message (for example, `E0004`).
{{/option}}

{{/options}}

### Display Options

{{#options}}

{{> options-display }}

{{/options}}

### Manifest Options

{{#options}}
{{> options-locked }}
{{/options}}

{{> section-options-common }}

{{> section-environment }}

{{> section-exit-status }}

## FILES

`~/.payload/`\
&nbsp;&nbsp;&nbsp;&nbsp;Default location for Payload's "home" directory where it
stores various files. The location can be changed with the `PAYLOAD_HOME`
environment variable.

`$PAYLOAD_HOME/bin/`\
&nbsp;&nbsp;&nbsp;&nbsp;Binaries installed by {{man "payload-install" 1}} will be located here. If using
[rustup], executables distributed with Rust are also located here.

`$PAYLOAD_HOME/config.toml`\
&nbsp;&nbsp;&nbsp;&nbsp;The global configuration file. See [the reference](../reference/config.html)
for more information about configuration files.

`.payload/config.toml`\
&nbsp;&nbsp;&nbsp;&nbsp;Payload automatically searches for a file named `.payload/config.toml` in the
current directory, and all parent directories. These configuration files
will be merged with the global configuration file.

`$PAYLOAD_HOME/credentials.toml`\
&nbsp;&nbsp;&nbsp;&nbsp;Private authentication information for logging in to a registry.

`$PAYLOAD_HOME/registry/`\
&nbsp;&nbsp;&nbsp;&nbsp;This directory contains cached downloads of the registry index and any
downloaded dependencies.

`$PAYLOAD_HOME/git/`\
&nbsp;&nbsp;&nbsp;&nbsp;This directory contains cached downloads of git dependencies.

Please note that the internal structure of the `$PAYLOAD_HOME` directory is not
stable yet and may be subject to change.

[rustup]: https://dustlang.github.io/rustup/

## EXAMPLES

1. Build a local package and all of its dependencies:

       payload build

2. Build a package with optimizations:

       payload build --release

3. Run tests for a cross-compiled target:

       payload test --target i686-unknown-linux-gnu

4. Create a new package that builds an executable:

       payload new foobar

5. Create a package in the current directory:

       mkdir foo && cd foo
       payload init .

6. Learn about a command's options and usage:

       payload help clean

## BUGS

See <https://github.com/dustlang/payload/issues> for issues.

## SEE ALSO
{{man "rustc" 1}}, {{man "rustdoc" 1}}
