# payload-init(1)

## NAME

payload-init - Create a new Payload package in an existing directory

## SYNOPSIS

`payload init` [_options_] [_path_]

## DESCRIPTION

This command will create a new Payload manifest in the current directory. Give a
path as an argument to create in the given directory.

If there are typically-named Rust source files already in the directory, those
will be used. If not, then a sample `src/main.rs` file will be created, or
`src/lib.rs` if `--lib` is passed.

If the directory is not already in a VCS repository, then a new repository
is created (see `--vcs` below).

The "authors" field in the manifest is determined from the environment or
configuration settings. A name is required and is determined from (first match
wins):

- `payload-new.name` Payload config value
- `PAYLOAD_NAME` environment variable
- `GIT_AUTHOR_NAME` environment variable
- `GIT_COMMITTER_NAME` environment variable
- `user.name` git configuration value
- `USER` environment variable
- `USERNAME` environment variable
- `NAME` environment variable

The email address is optional and is determined from:

- `payload-new.email` Payload config value
- `PAYLOAD_EMAIL` environment variable
- `GIT_AUTHOR_EMAIL` environment variable
- `GIT_COMMITTER_EMAIL` environment variable
- `user.email` git configuration value
- `EMAIL` environment variable

See [the reference](../reference/config.html) for more information about
configuration files.


See [payload-new(1)](payload-new.html) for a similar command which will create a new package in
a new directory.

## OPTIONS

### Init Options

<dl>

<dt class="option-term" id="option-payload-init---bin"><a class="option-anchor" href="#option-payload-init---bin"></a><code>--bin</code></dt>
<dd class="option-desc">Create a package with a binary target (<code>src/main.rs</code>).
This is the default behavior.</dd>


<dt class="option-term" id="option-payload-init---lib"><a class="option-anchor" href="#option-payload-init---lib"></a><code>--lib</code></dt>
<dd class="option-desc">Create a package with a library target (<code>src/lib.rs</code>).</dd>


<dt class="option-term" id="option-payload-init---edition"><a class="option-anchor" href="#option-payload-init---edition"></a><code>--edition</code> <em>edition</em></dt>
<dd class="option-desc">Specify the Rust edition to use. Default is 2018.
Possible values: 2015, 2018, 2021</dd>


<dt class="option-term" id="option-payload-init---name"><a class="option-anchor" href="#option-payload-init---name"></a><code>--name</code> <em>name</em></dt>
<dd class="option-desc">Set the package name. Defaults to the directory name.</dd>


<dt class="option-term" id="option-payload-init---vcs"><a class="option-anchor" href="#option-payload-init---vcs"></a><code>--vcs</code> <em>vcs</em></dt>
<dd class="option-desc">Initialize a new VCS repository for the given version control system (git,
hg, pijul, or fossil) or do not initialize any version control at all
(none). If not specified, defaults to <code>git</code> or the configuration value
<code>payload-new.vcs</code>, or <code>none</code> if already inside a VCS repository.</dd>


<dt class="option-term" id="option-payload-init---registry"><a class="option-anchor" href="#option-payload-init---registry"></a><code>--registry</code> <em>registry</em></dt>
<dd class="option-desc">This sets the <code>publish</code> field in <code>Payload.toml</code> to the given registry name
which will restrict publishing only to that registry.</p>
<p>Registry names are defined in <a href="../reference/config.html">Payload config files</a>.
If not specified, the default registry defined by the <code>registry.default</code>
config key is used. If the default registry is not set and <code>--registry</code> is not
used, the <code>publish</code> field will not be set which means that publishing will not
be restricted.</dd>


</dl>


### Display Options

<dl>
<dt class="option-term" id="option-payload-init--v"><a class="option-anchor" href="#option-payload-init--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-init---verbose"><a class="option-anchor" href="#option-payload-init---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-init--q"><a class="option-anchor" href="#option-payload-init--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-init---quiet"><a class="option-anchor" href="#option-payload-init---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-init---color"><a class="option-anchor" href="#option-payload-init---color"></a><code>--color</code> <em>when</em></dt>
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

<dt class="option-term" id="option-payload-init-+toolchain"><a class="option-anchor" href="#option-payload-init-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-init--h"><a class="option-anchor" href="#option-payload-init--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-init---help"><a class="option-anchor" href="#option-payload-init---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-init--Z"><a class="option-anchor" href="#option-payload-init--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Create a binary Payload package in the current directory:

       payload init

## SEE ALSO
[payload(1)](payload.html), [payload-new(1)](payload-new.html)
