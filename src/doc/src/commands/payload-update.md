# payload-update(1)

## NAME

payload-update - Update dependencies as recorded in the local lock file

## SYNOPSIS

`payload update` [_options_]

## DESCRIPTION

This command will update dependencies in the `Payload.lock` file to the latest
version. If the `Payload.lock` file does not exist, it will be created with the
latest available versions.

## OPTIONS

### Update Options

<dl>

<dt class="option-term" id="option-payload-update--p"><a class="option-anchor" href="#option-payload-update--p"></a><code>-p</code> <em>spec</em>...</dt>
<dt class="option-term" id="option-payload-update---package"><a class="option-anchor" href="#option-payload-update---package"></a><code>--package</code> <em>spec</em>...</dt>
<dd class="option-desc">Update only the specified packages. This flag may be specified
multiple times. See <a href="payload-pkgid.html">payload-pkgid(1)</a> for the SPEC format.</p>
<p>If packages are specified with the <code>-p</code> flag, then a conservative update of
the lockfile will be performed. This means that only the dependency specified
by SPEC will be updated. Its transitive dependencies will be updated only if
SPEC cannot be updated without updating dependencies.  All other dependencies
will remain locked at their currently recorded versions.</p>
<p>If <code>-p</code> is not specified, all dependencies are updated.</dd>


<dt class="option-term" id="option-payload-update---aggressive"><a class="option-anchor" href="#option-payload-update---aggressive"></a><code>--aggressive</code></dt>
<dd class="option-desc">When used with <code>-p</code>, dependencies of <em>spec</em> are forced to update as well.
Cannot be used with <code>--precise</code>.</dd>


<dt class="option-term" id="option-payload-update---precise"><a class="option-anchor" href="#option-payload-update---precise"></a><code>--precise</code> <em>precise</em></dt>
<dd class="option-desc">When used with <code>-p</code>, allows you to specify a specific version number to set
the package to. If the package comes from a git repository, this can be a git
revision (such as a SHA hash or tag).</dd>


<dt class="option-term" id="option-payload-update--w"><a class="option-anchor" href="#option-payload-update--w"></a><code>-w</code></dt>
<dt class="option-term" id="option-payload-update---workspace"><a class="option-anchor" href="#option-payload-update---workspace"></a><code>--workspace</code></dt>
<dd class="option-desc">Attempt to update only packages defined in the workspace. Other packages
are updated only if they don't already exist in the lockfile. This
option is useful for updating <code>Payload.lock</code> after you've changed version
numbers in <code>Payload.toml</code>.</dd>


<dt class="option-term" id="option-payload-update---dry-run"><a class="option-anchor" href="#option-payload-update---dry-run"></a><code>--dry-run</code></dt>
<dd class="option-desc">Displays what would be updated, but doesn't actually write the lockfile.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-payload-update--v"><a class="option-anchor" href="#option-payload-update--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-update---verbose"><a class="option-anchor" href="#option-payload-update---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-update--q"><a class="option-anchor" href="#option-payload-update--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-update---quiet"><a class="option-anchor" href="#option-payload-update---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-update---color"><a class="option-anchor" href="#option-payload-update---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-payload-update---manifest-path"><a class="option-anchor" href="#option-payload-update---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-payload-update---frozen"><a class="option-anchor" href="#option-payload-update---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-update---locked"><a class="option-anchor" href="#option-payload-update---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-update---offline"><a class="option-anchor" href="#option-payload-update---offline"></a><code>--offline</code></dt>
<dd class="option-desc">Prevents Payload from accessing the network for any reason. Without this
flag, Payload will stop with an error if it needs to access the network and
the network is not available. With this flag, Payload will attempt to
proceed without the network if possible.</p>
<p>Beware that this may result in different dependency resolution than online
mode. Payload will restrict itself to crates that are downloaded locally, even
if there might be a newer version as indicated in the local copy of the index.
See the <a href="payload-fetch.html">payload-fetch(1)</a> command to download dependencies before going
offline.</p>
<p>May also be specified with the <code>net.offline</code> <a href="../reference/config.html">config value</a>.</dd>



</dl>

### Common Options

<dl>

<dt class="option-term" id="option-payload-update-+toolchain"><a class="option-anchor" href="#option-payload-update-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-update--h"><a class="option-anchor" href="#option-payload-update--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-update---help"><a class="option-anchor" href="#option-payload-update---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-update--Z"><a class="option-anchor" href="#option-payload-update--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Update all dependencies in the lockfile:

       payload update

2. Update only specific dependencies:

       payload update -p foo -p bar

3. Set a specific dependency to a specific version:

       payload update -p foo --precise 1.2.3

## SEE ALSO
[payload(1)](payload.html), [payload-generate-lockfile(1)](payload-generate-lockfile.html)
