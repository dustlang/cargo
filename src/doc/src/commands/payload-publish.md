# payload-publish(1)


## NAME

payload-publish - Upload a package to the registry

## SYNOPSIS

`payload publish` [_options_]

## DESCRIPTION

This command will create a distributable, compressed `.crate` file with the
source code of the package in the current directory and upload it to a
registry. The default registry is <https://crates.io>. This performs the
following steps:

1. Performs a few checks, including:
   - Checks the `package.publish` key in the manifest for restrictions on
     which registries you are allowed to publish to.
2. Create a `.crate` file by following the steps in [payload-package(1)](payload-package.html).
3. Upload the crate to the registry. Note that the server will perform
   additional checks on the crate.

This command requires you to be authenticated with either the `--token` option
or using [payload-login(1)](payload-login.html).

See [the reference](../reference/publishing.html) for more details about
packaging and publishing.

## OPTIONS

### Publish Options

<dl>

<dt class="option-term" id="option-payload-publish---dry-run"><a class="option-anchor" href="#option-payload-publish---dry-run"></a><code>--dry-run</code></dt>
<dd class="option-desc">Perform all checks without uploading.</dd>


<dt class="option-term" id="option-payload-publish---token"><a class="option-anchor" href="#option-payload-publish---token"></a><code>--token</code> <em>token</em></dt>
<dd class="option-desc">API token to use when authenticating. This overrides the token stored in
the credentials file (which is created by <a href="payload-login.html">payload-login(1)</a>).</p>
<p><a href="../reference/config.html">Payload config</a> environment variables can be
used to override the tokens stored in the credentials file. The token for
crates.io may be specified with the <code>PAYLOAD_REGISTRY_TOKEN</code> environment
variable. Tokens for other registries may be specified with environment
variables of the form <code>PAYLOAD_REGISTRIES_NAME_TOKEN</code> where <code>NAME</code> is the name
of the registry in all capital letters.</dd>



<dt class="option-term" id="option-payload-publish---no-verify"><a class="option-anchor" href="#option-payload-publish---no-verify"></a><code>--no-verify</code></dt>
<dd class="option-desc">Don't verify the contents by building them.</dd>


<dt class="option-term" id="option-payload-publish---allow-dirty"><a class="option-anchor" href="#option-payload-publish---allow-dirty"></a><code>--allow-dirty</code></dt>
<dd class="option-desc">Allow working directories with uncommitted VCS changes to be packaged.</dd>


<dt class="option-term" id="option-payload-publish---index"><a class="option-anchor" href="#option-payload-publish---index"></a><code>--index</code> <em>index</em></dt>
<dd class="option-desc">The URL of the registry index to use.</dd>



<dt class="option-term" id="option-payload-publish---registry"><a class="option-anchor" href="#option-payload-publish---registry"></a><code>--registry</code> <em>registry</em></dt>
<dd class="option-desc">Name of the registry to publish to. Registry names are defined in <a href="../reference/config.html">Payload
config files</a>. If not specified, and there is a
<a href="../reference/manifest.html#the-publish-field"><code>package.publish</code></a> field in
<code>Payload.toml</code> with a single registry, then it will publish to that registry.
Otherwise it will use the default registry, which is defined by the
<a href="../reference/config.html#registrydefault"><code>registry.default</code></a> config key
which defaults to <code>crates-io</code>.</dd>


</dl>

### Compilation Options

<dl>

<dt class="option-term" id="option-payload-publish---target"><a class="option-anchor" href="#option-payload-publish---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Publish for the given architecture. The default is the host
architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Payload run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-payload-publish---target-dir"><a class="option-anchor" href="#option-payload-publish---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
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

<dt class="option-term" id="option-payload-publish---features"><a class="option-anchor" href="#option-payload-publish---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-payload-publish---all-features"><a class="option-anchor" href="#option-payload-publish---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-payload-publish---no-default-features"><a class="option-anchor" href="#option-payload-publish---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Manifest Options

<dl>

<dt class="option-term" id="option-payload-publish---manifest-path"><a class="option-anchor" href="#option-payload-publish---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-payload-publish---frozen"><a class="option-anchor" href="#option-payload-publish---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-publish---locked"><a class="option-anchor" href="#option-payload-publish---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-publish---offline"><a class="option-anchor" href="#option-payload-publish---offline"></a><code>--offline</code></dt>
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
<dt class="option-term" id="option-payload-publish--j"><a class="option-anchor" href="#option-payload-publish--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-payload-publish---jobs"><a class="option-anchor" href="#option-payload-publish---jobs"></a><code>--jobs</code> <em>N</em></dt>
<dd class="option-desc">Number of parallel jobs to run. May also be specified with the
<code>build.jobs</code> <a href="../reference/config.html">config value</a>. Defaults to
the number of CPUs.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-payload-publish--v"><a class="option-anchor" href="#option-payload-publish--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-publish---verbose"><a class="option-anchor" href="#option-payload-publish---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-publish--q"><a class="option-anchor" href="#option-payload-publish--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-publish---quiet"><a class="option-anchor" href="#option-payload-publish---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-publish---color"><a class="option-anchor" href="#option-payload-publish---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-payload-publish-+toolchain"><a class="option-anchor" href="#option-payload-publish-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-publish--h"><a class="option-anchor" href="#option-payload-publish--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-publish---help"><a class="option-anchor" href="#option-payload-publish---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-publish--Z"><a class="option-anchor" href="#option-payload-publish--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Publish the current package:

       payload publish

## SEE ALSO
[payload(1)](payload.html), [payload-package(1)](payload-package.html), [payload-login(1)](payload-login.html)
