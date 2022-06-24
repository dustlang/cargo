# payload-package(1)


## NAME

payload-package - Assemble the local package into a distributable tarball

## SYNOPSIS

`payload package` [_options_]

## DESCRIPTION

This command will create a distributable, compressed `.crate` file with the
source code of the package in the current directory. The resulting file will
be stored in the `target/package` directory. This performs the following
steps:

1. Load and check the current workspace, performing some basic checks.
    - Path dependencies are not allowed unless they have a version key. Payload
      will ignore the path key for dependencies in published packages.
      `dev-dependencies` do not have this restriction.
2. Create the compressed `.crate` file.
    - The original `Payload.toml` file is rewritten and normalized.
    - `[patch]`, `[replace]`, and `[workspace]` sections are removed from the
      manifest.
    - `Payload.lock` is automatically included if the package contains an
      executable binary or example target. [payload-install(1)](payload-install.html) will use the
      packaged lock file if the `--locked` flag is used.
    - A `.payload_vcs_info.json` file is included that contains information
      about the current VCS checkout hash if available (not included with
      `--allow-dirty`).
3. Extract the `.crate` file and build it to verify it can build.
    - This will rebuild your package from scratch to ensure that it can be
      built from a pristine state. The `--no-verify` flag can be used to skip
      this step.
4. Check that build scripts did not modify any source files.

The list of files included can be controlled with the `include` and `exclude`
fields in the manifest.

See [the reference](../reference/publishing.html) for more details about
packaging and publishing.

## OPTIONS

### Package Options

<dl>

<dt class="option-term" id="option-payload-package--l"><a class="option-anchor" href="#option-payload-package--l"></a><code>-l</code></dt>
<dt class="option-term" id="option-payload-package---list"><a class="option-anchor" href="#option-payload-package---list"></a><code>--list</code></dt>
<dd class="option-desc">Print files included in a package without making one.</dd>


<dt class="option-term" id="option-payload-package---no-verify"><a class="option-anchor" href="#option-payload-package---no-verify"></a><code>--no-verify</code></dt>
<dd class="option-desc">Don't verify the contents by building them.</dd>


<dt class="option-term" id="option-payload-package---no-metadata"><a class="option-anchor" href="#option-payload-package---no-metadata"></a><code>--no-metadata</code></dt>
<dd class="option-desc">Ignore warnings about a lack of human-usable metadata (such as the description
or the license).</dd>


<dt class="option-term" id="option-payload-package---allow-dirty"><a class="option-anchor" href="#option-payload-package---allow-dirty"></a><code>--allow-dirty</code></dt>
<dd class="option-desc">Allow working directories with uncommitted VCS changes to be packaged.</dd>


</dl>

### Compilation Options

<dl>

<dt class="option-term" id="option-payload-package---target"><a class="option-anchor" href="#option-payload-package---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Package for the given architecture. The default is the host
architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Payload run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-payload-package---target-dir"><a class="option-anchor" href="#option-payload-package---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>PAYLOAD_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to <code>target</code> in the root of the workspace.</dd>



</dl>

### Feature Selection

The feature flags allow you to control which features are enabled. When no
feature options are given, the `default` feature is activated for every
selected package.

See [the features documentation](../reference/features.html#command-line-feature-options)
for more details.

<dl>

<dt class="option-term" id="option-payload-package---features"><a class="option-anchor" href="#option-payload-package---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-payload-package---all-features"><a class="option-anchor" href="#option-payload-package---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-payload-package---no-default-features"><a class="option-anchor" href="#option-payload-package---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Manifest Options

<dl>

<dt class="option-term" id="option-payload-package---manifest-path"><a class="option-anchor" href="#option-payload-package---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-payload-package---frozen"><a class="option-anchor" href="#option-payload-package---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-package---locked"><a class="option-anchor" href="#option-payload-package---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-package---offline"><a class="option-anchor" href="#option-payload-package---offline"></a><code>--offline</code></dt>
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

### Miscellaneous Options

<dl>
<dt class="option-term" id="option-payload-package--j"><a class="option-anchor" href="#option-payload-package--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-payload-package---jobs"><a class="option-anchor" href="#option-payload-package---jobs"></a><code>--jobs</code> <em>N</em></dt>
<dd class="option-desc">Number of parallel jobs to run. May also be specified with the
<code>build.jobs</code> <a href="../reference/config.html">config value</a>. Defaults to
the number of CPUs.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-payload-package--v"><a class="option-anchor" href="#option-payload-package--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-package---verbose"><a class="option-anchor" href="#option-payload-package---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-package--q"><a class="option-anchor" href="#option-payload-package--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-package---quiet"><a class="option-anchor" href="#option-payload-package---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-package---color"><a class="option-anchor" href="#option-payload-package---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-payload-package-+toolchain"><a class="option-anchor" href="#option-payload-package-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-package--h"><a class="option-anchor" href="#option-payload-package--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-package---help"><a class="option-anchor" href="#option-payload-package---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-package--Z"><a class="option-anchor" href="#option-payload-package--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Create a compressed `.crate` file of the current package:

       payload package

## SEE ALSO
[payload(1)](payload.html), [payload-publish(1)](payload-publish.html)
