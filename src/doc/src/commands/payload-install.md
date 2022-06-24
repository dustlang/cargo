# payload-install(1)



## NAME

payload-install - Build and install a Rust binary

## SYNOPSIS

`payload install` [_options_] _crate_...\
`payload install` [_options_] `--path` _path_\
`payload install` [_options_] `--git` _url_ [_crate_...]\
`payload install` [_options_] `--list`

## DESCRIPTION

This command manages Payload's local set of installed binary crates. Only
packages which have executable `[[bin]]` or `[[example]]` targets can be
installed, and all executables are installed into the installation root's
`bin` folder.

The installation root is determined, in order of precedence:

- `--root` option
- `PAYLOAD_INSTALL_ROOT` environment variable
- `install.root` Payload [config value](../reference/config.html)
- `PAYLOAD_HOME` environment variable
- `$HOME/.payload`


There are multiple sources from which a crate can be installed. The default
location is crates.io but the `--git`, `--path`, and `--registry` flags can
change this source. If the source contains more than one package (such as
crates.io or a git repository with multiple crates) the _crate_ argument is
required to indicate which crate should be installed.

Crates from crates.io can optionally specify the version they wish to install
via the `--version` flags, and similarly packages from git repositories can
optionally specify the branch, tag, or revision that should be installed. If a
crate has multiple binaries, the `--bin` argument can selectively install only
one of them, and if you'd rather install examples the `--example` argument can
be used as well.

If the package is already installed, Payload will reinstall it if the installed
version does not appear to be up-to-date. If any of the following values
change, then Payload will reinstall the package:

- The package version and source.
- The set of binary names installed.
- The chosen features.
- The release mode (`--debug`).
- The target (`--target`).

Installing with `--path` will always build and install, unless there are
conflicting binaries from another package. The `--force` flag may be used to
force Payload to always reinstall the package.

If the source is crates.io or `--git` then by default the crate will be built
in a temporary target directory. To avoid this, the target directory can be
specified by setting the `PAYLOAD_TARGET_DIR` environment variable to a relative
path. In particular, this can be useful for caching build artifacts on
continuous integration systems.

By default, the `Payload.lock` file that is included with the package will be
ignored. This means that Payload will recompute which versions of dependencies
to use, possibly using newer versions that have been released since the
package was published. The `--locked` flag can be used to force Payload to use
the packaged `Payload.lock` file if it is available. This may be useful for
ensuring reproducible builds, to use the exact same set of dependencies that
were available when the package was published. It may also be useful if a
newer version of a dependency is published that no longer builds on your
system, or has other problems. The downside to using `--locked` is that you
will not receive any fixes or updates to any dependency. Note that Payload did
not start publishing `Payload.lock` files until version 1.37, which means
packages published with prior versions will not have a `Payload.lock` file
available.

## OPTIONS

### Install Options

<dl>

<dt class="option-term" id="option-payload-install---vers"><a class="option-anchor" href="#option-payload-install---vers"></a><code>--vers</code> <em>version</em></dt>
<dt class="option-term" id="option-payload-install---version"><a class="option-anchor" href="#option-payload-install---version"></a><code>--version</code> <em>version</em></dt>
<dd class="option-desc">Specify a version to install. This may be a <a href="../reference/specifying-dependencies.md">version
requirement</a>, like <code>~1.2</code>, to have Payload
select the newest version from the given requirement. If the version does not
have a requirement operator (such as <code>^</code> or <code>~</code>), then it must be in the form
<em>MAJOR.MINOR.PATCH</em>, and will install exactly that version; it is <em>not</em>
treated as a caret requirement like Payload dependencies are.</dd>


<dt class="option-term" id="option-payload-install---git"><a class="option-anchor" href="#option-payload-install---git"></a><code>--git</code> <em>url</em></dt>
<dd class="option-desc">Git URL to install the specified crate from.</dd>


<dt class="option-term" id="option-payload-install---branch"><a class="option-anchor" href="#option-payload-install---branch"></a><code>--branch</code> <em>branch</em></dt>
<dd class="option-desc">Branch to use when installing from git.</dd>


<dt class="option-term" id="option-payload-install---tag"><a class="option-anchor" href="#option-payload-install---tag"></a><code>--tag</code> <em>tag</em></dt>
<dd class="option-desc">Tag to use when installing from git.</dd>


<dt class="option-term" id="option-payload-install---rev"><a class="option-anchor" href="#option-payload-install---rev"></a><code>--rev</code> <em>sha</em></dt>
<dd class="option-desc">Specific commit to use when installing from git.</dd>


<dt class="option-term" id="option-payload-install---path"><a class="option-anchor" href="#option-payload-install---path"></a><code>--path</code> <em>path</em></dt>
<dd class="option-desc">Filesystem path to local crate to install.</dd>


<dt class="option-term" id="option-payload-install---list"><a class="option-anchor" href="#option-payload-install---list"></a><code>--list</code></dt>
<dd class="option-desc">List all installed packages and their versions.</dd>


<dt class="option-term" id="option-payload-install--f"><a class="option-anchor" href="#option-payload-install--f"></a><code>-f</code></dt>
<dt class="option-term" id="option-payload-install---force"><a class="option-anchor" href="#option-payload-install---force"></a><code>--force</code></dt>
<dd class="option-desc">Force overwriting existing crates or binaries. This can be used if a package
has installed a binary with the same name as another package. This is also
useful if something has changed on the system that you want to rebuild with,
such as a newer version of <code>rustc</code>.</dd>


<dt class="option-term" id="option-payload-install---no-track"><a class="option-anchor" href="#option-payload-install---no-track"></a><code>--no-track</code></dt>
<dd class="option-desc">By default, Payload keeps track of the installed packages with a metadata file
stored in the installation root directory. This flag tells Payload not to use or
create that file. With this flag, Payload will refuse to overwrite any existing
files unless the <code>--force</code> flag is used. This also disables Payload's ability to
protect against multiple concurrent invocations of Payload installing at the
same time.</dd>


<dt class="option-term" id="option-payload-install---bin"><a class="option-anchor" href="#option-payload-install---bin"></a><code>--bin</code> <em>name</em>...</dt>
<dd class="option-desc">Install only the specified binary.</dd>


<dt class="option-term" id="option-payload-install---bins"><a class="option-anchor" href="#option-payload-install---bins"></a><code>--bins</code></dt>
<dd class="option-desc">Install all binaries.</dd>


<dt class="option-term" id="option-payload-install---example"><a class="option-anchor" href="#option-payload-install---example"></a><code>--example</code> <em>name</em>...</dt>
<dd class="option-desc">Install only the specified example.</dd>


<dt class="option-term" id="option-payload-install---examples"><a class="option-anchor" href="#option-payload-install---examples"></a><code>--examples</code></dt>
<dd class="option-desc">Install all examples.</dd>


<dt class="option-term" id="option-payload-install---root"><a class="option-anchor" href="#option-payload-install---root"></a><code>--root</code> <em>dir</em></dt>
<dd class="option-desc">Directory to install packages into.</dd>


<dt class="option-term" id="option-payload-install---registry"><a class="option-anchor" href="#option-payload-install---registry"></a><code>--registry</code> <em>registry</em></dt>
<dd class="option-desc">Name of the registry to use. Registry names are defined in <a href="../reference/config.html">Payload config
files</a>. If not specified, the default registry is used,
which is defined by the <code>registry.default</code> config key which defaults to
<code>crates-io</code>.</dd>



<dt class="option-term" id="option-payload-install---index"><a class="option-anchor" href="#option-payload-install---index"></a><code>--index</code> <em>index</em></dt>
<dd class="option-desc">The URL of the registry index to use.</dd>



</dl>

### Feature Selection

The feature flags allow you to control which features are enabled. When no
feature options are given, the `default` feature is activated for every
selected package.

See [the features documentation](../reference/features.html#command-line-feature-options)
for more details.

<dl>

<dt class="option-term" id="option-payload-install---features"><a class="option-anchor" href="#option-payload-install---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-payload-install---all-features"><a class="option-anchor" href="#option-payload-install---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-payload-install---no-default-features"><a class="option-anchor" href="#option-payload-install---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Compilation Options

<dl>

<dt class="option-term" id="option-payload-install---target"><a class="option-anchor" href="#option-payload-install---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Install for the given architecture. The default is the host
architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Payload run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-payload-install---target-dir"><a class="option-anchor" href="#option-payload-install---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>PAYLOAD_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to a new temporary folder located in the
temporary directory of the platform. </p>
<p>When using <code>--path</code>, by default it will use <code>target</code> directory in the workspace
of the local crate unless <code>--target-dir</code>
is specified.</dd>



<dt class="option-term" id="option-payload-install---debug"><a class="option-anchor" href="#option-payload-install---debug"></a><code>--debug</code></dt>
<dd class="option-desc">Build with the <code>dev</code> profile instead the <code>release</code> profile.</dd>


</dl>

### Manifest Options

<dl>
<dt class="option-term" id="option-payload-install---frozen"><a class="option-anchor" href="#option-payload-install---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-install---locked"><a class="option-anchor" href="#option-payload-install---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-install---offline"><a class="option-anchor" href="#option-payload-install---offline"></a><code>--offline</code></dt>
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
<dt class="option-term" id="option-payload-install--j"><a class="option-anchor" href="#option-payload-install--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-payload-install---jobs"><a class="option-anchor" href="#option-payload-install---jobs"></a><code>--jobs</code> <em>N</em></dt>
<dd class="option-desc">Number of parallel jobs to run. May also be specified with the
<code>build.jobs</code> <a href="../reference/config.html">config value</a>. Defaults to
the number of CPUs.</dd>


</dl>

### Display Options

<dl>
<dt class="option-term" id="option-payload-install--v"><a class="option-anchor" href="#option-payload-install--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-install---verbose"><a class="option-anchor" href="#option-payload-install---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-install--q"><a class="option-anchor" href="#option-payload-install--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-install---quiet"><a class="option-anchor" href="#option-payload-install---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-install---color"><a class="option-anchor" href="#option-payload-install---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-payload-install-+toolchain"><a class="option-anchor" href="#option-payload-install-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-install--h"><a class="option-anchor" href="#option-payload-install--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-install---help"><a class="option-anchor" href="#option-payload-install---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-install--Z"><a class="option-anchor" href="#option-payload-install--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Install or upgrade a package from crates.io:

       payload install ripgrep

2. Install or reinstall the package in the current directory:

       payload install --path .

3. View the list of installed packages:

       payload install --list

## SEE ALSO
[payload(1)](payload.html), [payload-uninstall(1)](payload-uninstall.html), [payload-search(1)](payload-search.html), [payload-publish(1)](payload-publish.html)
