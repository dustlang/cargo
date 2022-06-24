# payload-clean(1)


## NAME

payload-clean - Remove generated artifacts

## SYNOPSIS

`payload clean` [_options_]

## DESCRIPTION

Remove artifacts from the target directory that Payload has generated in the
past.

With no options, `payload clean` will delete the entire target directory.

## OPTIONS

### Package Selection

When no packages are selected, all packages and all dependencies in the
workspace are cleaned.

<dl>
<dt class="option-term" id="option-payload-clean--p"><a class="option-anchor" href="#option-payload-clean--p"></a><code>-p</code> <em>spec</em>...</dt>
<dt class="option-term" id="option-payload-clean---package"><a class="option-anchor" href="#option-payload-clean---package"></a><code>--package</code> <em>spec</em>...</dt>
<dd class="option-desc">Clean only the specified packages. This flag may be specified
multiple times. See <a href="payload-pkgid.html">payload-pkgid(1)</a> for the SPEC format.</dd>

</dl>

### Clean Options

<dl>

<dt class="option-term" id="option-payload-clean---doc"><a class="option-anchor" href="#option-payload-clean---doc"></a><code>--doc</code></dt>
<dd class="option-desc">This option will cause <code>payload clean</code> to remove only the <code>doc</code> directory in
the target directory.</dd>


<dt class="option-term" id="option-payload-clean---release"><a class="option-anchor" href="#option-payload-clean---release"></a><code>--release</code></dt>
<dd class="option-desc">Clean all artifacts that were built with the <code>release</code> or <code>bench</code> profiles.</dd>


<dt class="option-term" id="option-payload-clean---target-dir"><a class="option-anchor" href="#option-payload-clean---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>PAYLOAD_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to <code>target</code> in the root of the workspace.</dd>



<dt class="option-term" id="option-payload-clean---target"><a class="option-anchor" href="#option-payload-clean---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Clean for the given architecture. The default is the host
architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Payload run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



</dl>

### Display Options

<dl>
<dt class="option-term" id="option-payload-clean--v"><a class="option-anchor" href="#option-payload-clean--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-clean---verbose"><a class="option-anchor" href="#option-payload-clean---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-clean--q"><a class="option-anchor" href="#option-payload-clean--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-clean---quiet"><a class="option-anchor" href="#option-payload-clean---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-clean---color"><a class="option-anchor" href="#option-payload-clean---color"></a><code>--color</code> <em>when</em></dt>
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
<dt class="option-term" id="option-payload-clean---manifest-path"><a class="option-anchor" href="#option-payload-clean---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-payload-clean---frozen"><a class="option-anchor" href="#option-payload-clean---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-clean---locked"><a class="option-anchor" href="#option-payload-clean---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-clean---offline"><a class="option-anchor" href="#option-payload-clean---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-payload-clean-+toolchain"><a class="option-anchor" href="#option-payload-clean-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-clean--h"><a class="option-anchor" href="#option-payload-clean--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-clean---help"><a class="option-anchor" href="#option-payload-clean---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-clean--Z"><a class="option-anchor" href="#option-payload-clean--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Remove the entire target directory:

       payload clean

2. Remove only the release artifacts:

       payload clean --release

## SEE ALSO
[payload(1)](payload.html), [payload-build(1)](payload-build.html)
