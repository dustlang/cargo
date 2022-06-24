### Common Options

{{#options}}

{{#option "`+`_toolchain_"}}
If Payload has been installed with rustup, and the first argument to `payload`
begins with `+`, it will be interpreted as a rustup toolchain name (such
as `+stable` or `+nightly`).
See the [rustup documentation](https://dustlang.github.io/rustup/overrides.html)
for more information about how toolchain overrides work.
{{/option}}

{{#option "`-h`" "`--help`"}}
Prints help information.
{{/option}}

{{#option "`-Z` _flag_"}}
Unstable (nightly-only) flags to Payload. Run `payload -Z help` for details.
{{/option}}

{{/options}}
