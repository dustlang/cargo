# payload-locate-project(1)

## NAME

payload-locate-project - Print a JSON representation of a Payload.toml file's location

## SYNOPSIS

`payload locate-project` [_options_]

## DESCRIPTION

This command will print a JSON object to stdout with the full path to the
`Payload.toml` manifest.

## OPTIONS

<dl>

<dt class="option-term" id="option-payload-locate-project---workspace"><a class="option-anchor" href="#option-payload-locate-project---workspace"></a><code>--workspace</code></dt>
<dd class="option-desc">Locate the <code>Payload.toml</code> at the root of the workspace, as opposed to the current
workspace member.</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-payload-locate-project---message-format"><a class="option-anchor" href="#option-payload-locate-project---message-format"></a><code>--message-format</code> <em>fmt</em></dt>
<dd class="option-desc">The representation in which to print the project location. Valid values:</p>
<ul>
<li><code>json</code> (default): JSON object with the path under the key &quot;root&quot;.</li>
<li><code>plain</code>: Just the path.</li>
</ul></dd>


<dt class="option-term" id="option-payload-locate-project--v"><a class="option-anchor" href="#option-payload-locate-project--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-locate-project---verbose"><a class="option-anchor" href="#option-payload-locate-project---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-locate-project--q"><a class="option-anchor" href="#option-payload-locate-project--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-locate-project---quiet"><a class="option-anchor" href="#option-payload-locate-project---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-locate-project---color"><a class="option-anchor" href="#option-payload-locate-project---color"></a><code>--color</code> <em>when</em></dt>
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

### Manifest Options

<dl>
<dt class="option-term" id="option-payload-locate-project---manifest-path"><a class="option-anchor" href="#option-payload-locate-project---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>


</dl>

### Common Options

<dl>

<dt class="option-term" id="option-payload-locate-project-+toolchain"><a class="option-anchor" href="#option-payload-locate-project-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-locate-project--h"><a class="option-anchor" href="#option-payload-locate-project--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-locate-project---help"><a class="option-anchor" href="#option-payload-locate-project---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-locate-project--Z"><a class="option-anchor" href="#option-payload-locate-project--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Display the path to the manifest based on the current directory:

       payload locate-project

## SEE ALSO
[payload(1)](payload.html), [payload-metadata(1)](payload-metadata.html)
