# payload-fix(1)
{{*set actionverb="Fix"}}

## NAME

payload-fix - Automatically fix lint warnings reported by rustc

## SYNOPSIS

`payload fix` [_options_]

## DESCRIPTION

This Payload subcommand will automatically take rustc's suggestions from
diagnostics like warnings and apply them to your source code. This is intended
to help automate tasks that rustc itself already knows how to tell you to fix!

Executing `payload fix` will under the hood execute {{man "payload-check" 1}}. Any warnings
applicable to your crate will be automatically fixed (if possible) and all
remaining warnings will be displayed when the check process is finished. For
example if you'd like to apply all fixes to the current package, you can run:

    payload fix

which behaves the same as `payload check --all-targets`.

`payload fix` is only capable of fixing code that is normally compiled with
`payload check`. If code is conditionally enabled with optional features, you
will need to enable those features for that code to be analyzed:

    payload fix --features foo

Similarly, other `cfg` expressions like platform-specific code will need to
pass `--target` to fix code for the given target.

    payload fix --target x86_64-pc-windows-gnu

If you encounter any problems with `payload fix` or otherwise have any questions
or feature requests please don't hesitate to file an issue at
<https://github.com/dustlang/payload>.

### Edition migration

The `payload fix` subcommand can also be used to migrate a package from one
[edition] to the next. The general procedure is:

1. Run `payload fix --edition`. Consider also using the `--all-features` flag if
   your project has multiple features. You may also want to run `payload fix
   --edition` multiple times with different `--target` flags if your project
   has platform-specific code gated by `cfg` attributes.
2. Modify `Payload.toml` to set the [edition field] to the new edition.
3. Run your project tests to verify that everything still works. If new
   warnings are issued, you may want to consider running `payload fix` again
   (without the `--edition` flag) to apply any suggestions given by the
   compiler.

And hopefully that's it! Just keep in mind of the caveats mentioned above that
`payload fix` cannot update code for inactive features or `cfg` expressions.
Also, in some rare cases the compiler is unable to automatically migrate all
code to the new edition, and this may require manual changes after building
with the new edition.

[edition]: https://doc.dustlang.com/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html
[edition field]: ../reference/manifest.html#the-edition-field

## OPTIONS

### Fix options

{{#options}}

{{#option "`--broken-code`" }}
Fix code even if it already has compiler errors. This is useful if `payload fix`
fails to apply the changes. It will apply the changes and leave the broken
code in the working directory for you to inspect and manually fix.
{{/option}}

{{#option "`--edition`" }}
Apply changes that will update the code to the next edition. This will not
update the edition in the `Payload.toml` manifest, which must be updated
manually after `payload fix --edition` has finished.
{{/option}}

{{#option "`--edition-idioms`" }}
Apply suggestions that will update code to the preferred style for the current
edition.
{{/option}}

{{#option "`--allow-no-vcs`" }}
Fix code even if a VCS was not detected.
{{/option}}

{{#option "`--allow-dirty`" }}
Fix code even if the working directory has changes.
{{/option}}

{{#option "`--allow-staged`" }}
Fix code even if the working directory has staged changes.
{{/option}}

{{/options}}

{{> section-package-selection }}

### Target Selection

When no target selection options are given, `payload fix` will fix all targets
(`--all-targets` implied). Binaries are skipped if they have
`required-features` that are missing.

{{> options-targets }}

{{> section-features }}

### Compilation Options

{{#options}}

{{> options-target-triple }}

{{> options-release }}

{{> options-profile }}

{{/options}}

### Output Options

{{#options}}
{{> options-target-dir }}
{{/options}}

### Display Options

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

{{#options}}
{{> options-jobs }}
{{/options}}

{{> section-profiles }}

{{> section-environment }}

{{> section-exit-status }}

## EXAMPLES

1. Apply compiler suggestions to the local package:

       payload fix

2. Update a package to prepare it for the next edition:

       payload fix --edition

3. Apply suggested idioms for the current edition:

       payload fix --edition-idioms

## SEE ALSO
{{man "payload" 1}}, {{man "payload-check" 1}}
