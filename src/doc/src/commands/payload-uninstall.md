# payload-uninstall(1)

## NAME

payload-uninstall - Remove a Rust binary

## SYNOPSIS

`payload uninstall` [_options_] [_spec_...]

## DESCRIPTION

This command removes a package installed with [payload-install(1)](payload-install.html). The _spec_
argument is a package ID specification of the package to remove (see
[payload-pkgid(1)](payload-pkgid.html)).

By default all binaries are removed for a crate but the `--bin` and
`--example` flags can be used to only remove particular binaries.

The installation root is determined, in order of precedence:

- `--root` option
- `PAYLOAD_INSTALL_ROOT` environment variable
- `install.root` Payload [config value](../reference/config.html)
- `PAYLOAD_HOME` environment variable
- `$HOME/.payload`


## OPTIONS

### Install Options

<dl>

<dt class="option-term" id="option-payload-uninstall--p"><a class="option-anchor" href="#option-payload-uninstall--p"></a><code>-p</code></dt>
<dt class="option-term" id="option-payload-uninstall---package"><a class="option-anchor" href="#option-payload-uninstall---package"></a><code>--package</code> <em>spec</em>...</dt>
<dd class="option-desc">Package to uninstall.</dd>


<dt class="option-term" id="option-payload-uninstall---bin"><a class="option-anchor" href="#option-payload-uninstall---bin"></a><code>--bin</code> <em>name</em>...</dt>
<dd class="option-desc">Only uninstall the binary <em>name</em>.</dd>


<dt class="option-term" id="option-payload-uninstall---root"><a class="option-anchor" href="#option-payload-uninstall---root"></a><code>--root</code> <em>dir</em></dt>
<dd class="option-desc">Directory to uninstall packages from.</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-payload-uninstall--v"><a class="option-anchor" href="#option-payload-uninstall--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-uninstall---verbose"><a class="option-anchor" href="#option-payload-uninstall---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-uninstall--q"><a class="option-anchor" href="#option-payload-uninstall--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-uninstall---quiet"><a class="option-anchor" href="#option-payload-uninstall---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-uninstall---color"><a class="option-anchor" href="#option-payload-uninstall---color"></a><code>--color</code> <em>when</em></dt>
<dd class="option-desc">Control when colored output is used. Valid values:</p>
<ul>
<li><code>auto</code> (default): Automatically detect if color support is available on the
terminal.</li>
<li><code>always</code>: Always display colors.</li>
<li><code>never</code>: Never display colors.</li>
</ul>
<p>May also be specified with the <code>term.color</code>
<a href="../reference/config.html">config value</a>.</dd>



</dl>

### Common Options

<dl>

<dt class="option-term" id="option-payload-uninstall-+toolchain"><a class="option-anchor" href="#option-payload-uninstall-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-uninstall--h"><a class="option-anchor" href="#option-payload-uninstall--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-uninstall---help"><a class="option-anchor" href="#option-payload-uninstall---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-uninstall--Z"><a class="option-anchor" href="#option-payload-uninstall--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Uninstall a previously installed package.

       payload uninstall ripgrep

## SEE ALSO
[payload(1)](payload.html), [payload-install(1)](payload-install.html)
