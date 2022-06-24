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

[payload-bench(1)](payload-bench.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Execute benchmarks of a package.

[payload-build(1)](payload-build.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Compile a package.

[payload-check(1)](payload-check.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Check a local package and all of its dependencies for errors.

[payload-clean(1)](payload-clean.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Remove artifacts that Payload has generated in the past.

[payload-doc(1)](payload-doc.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Build a package's documentation.

[payload-fetch(1)](payload-fetch.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Fetch dependencies of a package from the network.

[payload-fix(1)](payload-fix.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Automatically fix lint warnings reported by rustc.

[payload-run(1)](payload-run.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Run a binary or example of the local package.

[payload-rustc(1)](payload-rustc.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Compile a package, and pass extra options to the compiler.

[payload-rustdoc(1)](payload-rustdoc.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Build a package's documentation, using specified custom flags.

[payload-test(1)](payload-test.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Execute unit and integration tests of a package.

### Manifest Commands

[payload-generate-lockfile(1)](payload-generate-lockfile.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Generate `Payload.lock` for a project.

[payload-locate-project(1)](payload-locate-project.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Print a JSON representation of a `Payload.toml` file's location.

[payload-metadata(1)](payload-metadata.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Output the resolved dependencies of a package in machine-readable format.

[payload-pkgid(1)](payload-pkgid.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Print a fully qualified package specification.

[payload-tree(1)](payload-tree.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Display a tree visualization of a dependency graph.

[payload-update(1)](payload-update.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Update dependencies as recorded in the local lock file.

[payload-vendor(1)](payload-vendor.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Vendor all dependencies locally.

[payload-verify-project(1)](payload-verify-project.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Check correctness of crate manifest.

### Package Commands

[payload-init(1)](payload-init.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Create a new Payload package in an existing directory.

[payload-install(1)](payload-install.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Build and install a Rust binary.

[payload-new(1)](payload-new.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Create a new Payload package.

[payload-search(1)](payload-search.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Search packages in crates.io.

[payload-uninstall(1)](payload-uninstall.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Remove a Rust binary.

### Publishing Commands

[payload-login(1)](payload-login.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Save an API token from the registry locally.

[payload-owner(1)](payload-owner.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Manage the owners of a crate on the registry.

[payload-package(1)](payload-package.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Assemble the local package into a distributable tarball.

[payload-publish(1)](payload-publish.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Upload a package to the registry.

[payload-yank(1)](payload-yank.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Remove a pushed crate from the index.

### General Commands

[payload-help(1)](payload-help.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Display help information about Payload.

[payload-version(1)](payload-version.html)\
&nbsp;&nbsp;&nbsp;&nbsp;Show version information.

## OPTIONS

### Special Options

<dl>

<dt class="option-term" id="option-payload--V"><a class="option-anchor" href="#option-payload--V"></a><code>-V</code></dt>
<dt class="option-term" id="option-payload---version"><a class="option-anchor" href="#option-payload---version"></a><code>--version</code></dt>
<dd class="option-desc">Print version info and exit. If used with <code>--verbose</code>, prints extra
information.</dd>


<dt class="option-term" id="option-payload---list"><a class="option-anchor" href="#option-payload---list"></a><code>--list</code></dt>
<dd class="option-desc">List all installed Payload subcommands. If used with <code>--verbose</code>, prints extra
information.</dd>


<dt class="option-term" id="option-payload---explain"><a class="option-anchor" href="#option-payload---explain"></a><code>--explain</code> <em>code</em></dt>
<dd class="option-desc">Run <code>rustc --explain CODE</code> which will print out a detailed explanation of an
error message (for example, <code>E0004</code>).</dd>


</dl>

### Display Options

<dl>

<dt class="option-term" id="option-payload--v"><a class="option-anchor" href="#option-payload--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload---verbose"><a class="option-anchor" href="#option-payload---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload--q"><a class="option-anchor" href="#option-payload--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload---quiet"><a class="option-anchor" href="#option-payload---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload---color"><a class="option-anchor" href="#option-payload---color"></a><code>--color</code> <em>when</em></dt>
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
<dt class="option-term" id="option-payload---frozen"><a class="option-anchor" href="#option-payload---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload---locked"><a class="option-anchor" href="#option-payload---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload---offline"><a class="option-anchor" href="#option-payload---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-payload-+toolchain"><a class="option-anchor" href="#option-payload-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload--h"><a class="option-anchor" href="#option-payload--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload---help"><a class="option-anchor" href="#option-payload---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload--Z"><a class="option-anchor" href="#option-payload--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## FILES

`~/.payload/`\
&nbsp;&nbsp;&nbsp;&nbsp;Default location for Payload's "home" directory where it
stores various files. The location can be changed with the `PAYLOAD_HOME`
environment variable.

`$PAYLOAD_HOME/bin/`\
&nbsp;&nbsp;&nbsp;&nbsp;Binaries installed by [payload-install(1)](payload-install.html) will be located here. If using
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
[rustc(1)](https://doc.dustlang.com/rustc/index.html), [rustdoc(1)](https://doc.dustlang.com/rustdoc/index.html)
