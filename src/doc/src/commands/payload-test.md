# payload-test(1)



## NAME

payload-test - Execute unit and integration tests of a package

## SYNOPSIS

`payload test` [_options_] [_testname_] [`--` _test-options_]

## DESCRIPTION

Compile and execute unit and integration tests.

The test filtering argument `TESTNAME` and all the arguments following the two
dashes (`--`) are passed to the test binaries and thus to _libtest_ (rustc's
built in unit-test and micro-benchmarking framework).  If you're passing
arguments to both Payload and the binary, the ones after `--` go to the binary,
the ones before go to Payload.  For details about libtest's arguments see the
output of `payload test -- --help`.

As an example, this will filter for tests with `foo` in their name and run them
on 3 threads in parallel:

    payload test foo -- --test-threads 3

Tests are built with the `--test` option to `rustc` which creates an
executable with a `main` function that automatically runs all functions
annotated with the `#[test]` attribute in multiple threads. `#[bench]`
annotated functions will also be run with one iteration to verify that they
are functional.

The libtest harness may be disabled by setting `harness = false` in the target
manifest settings, in which case your code will need to provide its own `main`
function to handle running tests.

Documentation tests are also run by default, which is handled by `rustdoc`. It
extracts code samples from documentation comments and executes them. See the
[rustdoc book](https://doc.dustlang.com/rustdoc/) for more information on
writing doc tests.

## OPTIONS

### Test Options

<dl>

<dt class="option-term" id="option-payload-test---no-run"><a class="option-anchor" href="#option-payload-test---no-run"></a><code>--no-run</code></dt>
<dd class="option-desc">Compile, but don't run tests.</dd>


<dt class="option-term" id="option-payload-test---no-fail-fast"><a class="option-anchor" href="#option-payload-test---no-fail-fast"></a><code>--no-fail-fast</code></dt>
<dd class="option-desc">Run all tests regardless of failure. Without this flag, Payload will exit
after the first executable fails. The Rust test harness will run all tests
within the executable to completion, this flag only applies to the executable
as a whole.</dd>


</dl>


### Package Selection

By default, when no package selection options are given, the packages selected
depend on the selected manifest file (based on the current working directory if
`--manifest-path` is not given). If the manifest is the root of a workspace then
the workspaces default members are selected, otherwise only the package defined
by the manifest will be selected.

The default members of a workspace can be set explicitly with the
`workspace.default-members` key in the root manifest. If this is not set, a
virtual workspace will include all workspace members (equivalent to passing
`--workspace`), and a non-virtual workspace will include only the root crate itself.

<dl>

<dt class="option-term" id="option-payload-test--p"><a class="option-anchor" href="#option-payload-test--p"></a><code>-p</code> <em>spec</em>...</dt>
<dt class="option-term" id="option-payload-test---package"><a class="option-anchor" href="#option-payload-test---package"></a><code>--package</code> <em>spec</em>...</dt>
<dd class="option-desc">Test only the specified packages. See <a href="payload-pkgid.html">payload-pkgid(1)</a> for the
SPEC format. This flag may be specified multiple times and supports common Unix
glob patterns like <code>*</code>, <code>?</code> and <code>[]</code>. However, to avoid your shell accidentally 
expanding glob patterns before Payload handles them, you must use single quotes or
double quotes around each pattern.</dd>


<dt class="option-term" id="option-payload-test---workspace"><a class="option-anchor" href="#option-payload-test---workspace"></a><code>--workspace</code></dt>
<dd class="option-desc">Test all members in the workspace.</dd>



<dt class="option-term" id="option-payload-test---all"><a class="option-anchor" href="#option-payload-test---all"></a><code>--all</code></dt>
<dd class="option-desc">Deprecated alias for <code>--workspace</code>.</dd>



<dt class="option-term" id="option-payload-test---exclude"><a class="option-anchor" href="#option-payload-test---exclude"></a><code>--exclude</code> <em>SPEC</em>...</dt>
<dd class="option-desc">Exclude the specified packages. Must be used in conjunction with the
<code>--workspace</code> flag. This flag may be specified multiple times and supports
common Unix glob patterns like <code>*</code>, <code>?</code> and <code>[]</code>. However, to avoid your shell
accidentally expanding glob patterns before Payload handles them, you must use
single quotes or double quotes around each pattern.</dd>


</dl>


### Target Selection

When no target selection options are given, `payload test` will build the
following targets of the selected packages:

- lib — used to link with binaries, examples, integration tests, and doc tests
- bins (only if integration tests are built and required features are
  available)
- examples — to ensure they compile
- lib as a unit test
- bins as unit tests
- integration tests
- doc tests for the lib target

The default behavior can be changed by setting the `test` flag for the target
in the manifest settings. Setting examples to `test = true` will build and run
the example as a test. Setting targets to `test = false` will stop them from
being tested by default. Target selection options that take a target by name
ignore the `test` flag and will always test the given target.

Doc tests for libraries may be disabled by setting `doctest = false` for the
library in the manifest.

Binary targets are automatically built if there is an integration test or
benchmark. This allows an integration test to execute the binary to exercise
and test its behavior. The `PAYLOAD_BIN_EXE_<name>`
[environment variable](../reference/environment-variables.html#environment-variables-payload-sets-for-crates)
is set when the integration test is built so that it can use the
[`env` macro](https://doc.dustlang.com/std/macro.env.html) to locate the
executable.

Passing target selection flags will test only the specified
targets. 

Note that `--bin`, `--example`, `--test` and `--bench` flags also 
support common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your 
shell accidentally expanding glob patterns before Payload handles them, you must 
use single quotes or double quotes around each glob pattern.

<dl>

<dt class="option-term" id="option-payload-test---lib"><a class="option-anchor" href="#option-payload-test---lib"></a><code>--lib</code></dt>
<dd class="option-desc">Test the package's library.</dd>


<dt class="option-term" id="option-payload-test---bin"><a class="option-anchor" href="#option-payload-test---bin"></a><code>--bin</code> <em>name</em>...</dt>
<dd class="option-desc">Test the specified binary. This flag may be specified multiple times
and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-test---bins"><a class="option-anchor" href="#option-payload-test---bins"></a><code>--bins</code></dt>
<dd class="option-desc">Test all binary targets.</dd>



<dt class="option-term" id="option-payload-test---example"><a class="option-anchor" href="#option-payload-test---example"></a><code>--example</code> <em>name</em>...</dt>
<dd class="option-desc">Test the specified example. This flag may be specified multiple times
and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-test---examples"><a class="option-anchor" href="#option-payload-test---examples"></a><code>--examples</code></dt>
<dd class="option-desc">Test all example targets.</dd>


<dt class="option-term" id="option-payload-test---test"><a class="option-anchor" href="#option-payload-test---test"></a><code>--test</code> <em>name</em>...</dt>
<dd class="option-desc">Test the specified integration test. This flag may be specified
multiple times and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-test---tests"><a class="option-anchor" href="#option-payload-test---tests"></a><code>--tests</code></dt>
<dd class="option-desc">Test all targets in test mode that have the <code>test = true</code> manifest
flag set. By default this includes the library and binaries built as
unittests, and integration tests. Be aware that this will also build any
required dependencies, so the lib target may be built twice (once as a
unittest, and once as a dependency for binaries, integration tests, etc.).
Targets may be enabled or disabled by setting the <code>test</code> flag in the
manifest settings for the target.</dd>


<dt class="option-term" id="option-payload-test---bench"><a class="option-anchor" href="#option-payload-test---bench"></a><code>--bench</code> <em>name</em>...</dt>
<dd class="option-desc">Test the specified benchmark. This flag may be specified multiple
times and supports common Unix glob patterns.</dd>


<dt class="option-term" id="option-payload-test---benches"><a class="option-anchor" href="#option-payload-test---benches"></a><code>--benches</code></dt>
<dd class="option-desc">Test all targets in benchmark mode that have the <code>bench = true</code>
manifest flag set. By default this includes the library and binaries built
as benchmarks, and bench targets. Be aware that this will also build any
required dependencies, so the lib target may be built twice (once as a
benchmark, and once as a dependency for binaries, benchmarks, etc.).
Targets may be enabled or disabled by setting the <code>bench</code> flag in the
manifest settings for the target.</dd>


<dt class="option-term" id="option-payload-test---all-targets"><a class="option-anchor" href="#option-payload-test---all-targets"></a><code>--all-targets</code></dt>
<dd class="option-desc">Test all targets. This is equivalent to specifying <code>--lib --bins --tests --benches --examples</code>.</dd>


</dl>


<dl>

<dt class="option-term" id="option-payload-test---doc"><a class="option-anchor" href="#option-payload-test---doc"></a><code>--doc</code></dt>
<dd class="option-desc">Test only the library's documentation. This cannot be mixed with other
target options.</dd>


</dl>

### Feature Selection

The feature flags allow you to control which features are enabled. When no
feature options are given, the `default` feature is activated for every
selected package.

See [the features documentation](../reference/features.html#command-line-feature-options)
for more details.

<dl>

<dt class="option-term" id="option-payload-test---features"><a class="option-anchor" href="#option-payload-test---features"></a><code>--features</code> <em>features</em></dt>
<dd class="option-desc">Space or comma separated list of features to activate. Features of workspace
members may be enabled with <code>package-name/feature-name</code> syntax. This flag may
be specified multiple times, which enables all specified features.</dd>


<dt class="option-term" id="option-payload-test---all-features"><a class="option-anchor" href="#option-payload-test---all-features"></a><code>--all-features</code></dt>
<dd class="option-desc">Activate all available features of all selected packages.</dd>


<dt class="option-term" id="option-payload-test---no-default-features"><a class="option-anchor" href="#option-payload-test---no-default-features"></a><code>--no-default-features</code></dt>
<dd class="option-desc">Do not activate the <code>default</code> feature of the selected packages.</dd>


</dl>


### Compilation Options

<dl>

<dt class="option-term" id="option-payload-test---target"><a class="option-anchor" href="#option-payload-test---target"></a><code>--target</code> <em>triple</em></dt>
<dd class="option-desc">Test for the given architecture. The default is the host
architecture. The general format of the triple is
<code>&lt;arch&gt;&lt;sub&gt;-&lt;vendor&gt;-&lt;sys&gt;-&lt;abi&gt;</code>. Run <code>rustc --print target-list</code> for a
list of supported targets.</p>
<p>This may also be specified with the <code>build.target</code>
<a href="../reference/config.html">config value</a>.</p>
<p>Note that specifying this flag makes Payload run in a different mode where the
target artifacts are placed in a separate directory. See the
<a href="../guide/build-cache.html">build cache</a> documentation for more details.</dd>



<dt class="option-term" id="option-payload-test---release"><a class="option-anchor" href="#option-payload-test---release"></a><code>--release</code></dt>
<dd class="option-desc">Test optimized artifacts with the <code>release</code> profile. See the
<a href="#profiles">PROFILES</a> section for details on how this affects profile
selection.</dd>



</dl>

### Output Options

<dl>
<dt class="option-term" id="option-payload-test---target-dir"><a class="option-anchor" href="#option-payload-test---target-dir"></a><code>--target-dir</code> <em>directory</em></dt>
<dd class="option-desc">Directory for all generated artifacts and intermediate files. May also be
specified with the <code>PAYLOAD_TARGET_DIR</code> environment variable, or the
<code>build.target-dir</code> <a href="../reference/config.html">config value</a>.
Defaults to <code>target</code> in the root of the workspace.</dd>


</dl>

### Display Options

By default the Rust test harness hides output from test execution to keep
results readable. Test output can be recovered (e.g., for debugging) by passing
`--nocapture` to the test binaries:

    payload test -- --nocapture

<dl>

<dt class="option-term" id="option-payload-test--v"><a class="option-anchor" href="#option-payload-test--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-payload-test---verbose"><a class="option-anchor" href="#option-payload-test---verbose"></a><code>--verbose</code></dt>
<dd class="option-desc">Use verbose output. May be specified twice for &quot;very verbose&quot; output which
includes extra output such as dependency warnings and build script output.
May also be specified with the <code>term.verbose</code>
<a href="../reference/config.html">config value</a>.</dd>


<dt class="option-term" id="option-payload-test--q"><a class="option-anchor" href="#option-payload-test--q"></a><code>-q</code></dt>
<dt class="option-term" id="option-payload-test---quiet"><a class="option-anchor" href="#option-payload-test---quiet"></a><code>--quiet</code></dt>
<dd class="option-desc">No output printed to stdout.</dd>


<dt class="option-term" id="option-payload-test---color"><a class="option-anchor" href="#option-payload-test---color"></a><code>--color</code> <em>when</em></dt>
<dd class="option-desc">Control when colored output is used. Valid values:</p>
<ul>
<li><code>auto</code> (default): Automatically detect if color support is available on the
terminal.</li>
<li><code>always</code>: Always display colors.</li>
<li><code>never</code>: Never display colors.</li>
</ul>
<p>May also be specified with the <code>term.color</code>
<a href="../reference/config.html">config value</a>.</dd>



<dt class="option-term" id="option-payload-test---message-format"><a class="option-anchor" href="#option-payload-test---message-format"></a><code>--message-format</code> <em>fmt</em></dt>
<dd class="option-desc">The output format for diagnostic messages. Can be specified multiple times
and consists of comma-separated values. Valid values:</p>
<ul>
<li><code>human</code> (default): Display in a human-readable text format. Conflicts with
<code>short</code> and <code>json</code>.</li>
<li><code>short</code>: Emit shorter, human-readable text messages. Conflicts with <code>human</code>
and <code>json</code>.</li>
<li><code>json</code>: Emit JSON messages to stdout. See
<a href="../reference/external-tools.html#json-messages">the reference</a>
for more details. Conflicts with <code>human</code> and <code>short</code>.</li>
<li><code>json-diagnostic-short</code>: Ensure the <code>rendered</code> field of JSON messages contains
the &quot;short&quot; rendering from rustc. Cannot be used with <code>human</code> or <code>short</code>.</li>
<li><code>json-diagnostic-rendered-ansi</code>: Ensure the <code>rendered</code> field of JSON messages
contains embedded ANSI color codes for respecting rustc's default color
scheme. Cannot be used with <code>human</code> or <code>short</code>.</li>
<li><code>json-render-diagnostics</code>: Instruct Payload to not include rustc diagnostics in
in JSON messages printed, but instead Payload itself should render the
JSON diagnostics coming from rustc. Payload's own JSON diagnostics and others
coming from rustc are still emitted. Cannot be used with <code>human</code> or <code>short</code>.</li>
</ul></dd>



</dl>

### Manifest Options

<dl>

<dt class="option-term" id="option-payload-test---manifest-path"><a class="option-anchor" href="#option-payload-test---manifest-path"></a><code>--manifest-path</code> <em>path</em></dt>
<dd class="option-desc">Path to the <code>Payload.toml</code> file. By default, Payload searches for the
<code>Payload.toml</code> file in the current directory or any parent directory.</dd>



<dt class="option-term" id="option-payload-test---frozen"><a class="option-anchor" href="#option-payload-test---frozen"></a><code>--frozen</code></dt>
<dt class="option-term" id="option-payload-test---locked"><a class="option-anchor" href="#option-payload-test---locked"></a><code>--locked</code></dt>
<dd class="option-desc">Either of these flags requires that the <code>Payload.lock</code> file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The <code>--frozen</code> flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.</p>
<p>These may be used in environments where you want to assert that the
<code>Payload.lock</code> file is up-to-date (such as a CI build) or want to avoid network
access.</dd>


<dt class="option-term" id="option-payload-test---offline"><a class="option-anchor" href="#option-payload-test---offline"></a><code>--offline</code></dt>
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

<dt class="option-term" id="option-payload-test-+toolchain"><a class="option-anchor" href="#option-payload-test-+toolchain"></a><code>+</code><em>toolchain</em></dt>
<dd class="option-desc">If Payload has been installed with rustup, and the first argument to <code>payload</code>
begins with <code>+</code>, it will be interpreted as a rustup toolchain name (such
as <code>+stable</code> or <code>+nightly</code>).
See the <a href="https://dustlang.github.io/rustup/overrides.html">rustup documentation</a>
for more information about how toolchain overrides work.</dd>


<dt class="option-term" id="option-payload-test--h"><a class="option-anchor" href="#option-payload-test--h"></a><code>-h</code></dt>
<dt class="option-term" id="option-payload-test---help"><a class="option-anchor" href="#option-payload-test---help"></a><code>--help</code></dt>
<dd class="option-desc">Prints help information.</dd>


<dt class="option-term" id="option-payload-test--Z"><a class="option-anchor" href="#option-payload-test--Z"></a><code>-Z</code> <em>flag</em></dt>
<dd class="option-desc">Unstable (nightly-only) flags to Payload. Run <code>payload -Z help</code> for details.</dd>


</dl>


### Miscellaneous Options

The `--jobs` argument affects the building of the test executable but does not
affect how many threads are used when running the tests. The Rust test harness
includes an option to control the number of threads used:

    payload test -j 2 -- --test-threads=2

<dl>

<dt class="option-term" id="option-payload-test--j"><a class="option-anchor" href="#option-payload-test--j"></a><code>-j</code> <em>N</em></dt>
<dt class="option-term" id="option-payload-test---jobs"><a class="option-anchor" href="#option-payload-test---jobs"></a><code>--jobs</code> <em>N</em></dt>
<dd class="option-desc">Number of parallel jobs to run. May also be specified with the
<code>build.jobs</code> <a href="../reference/config.html">config value</a>. Defaults to
the number of CPUs.</dd>



</dl>

## PROFILES

Profiles may be used to configure compiler options such as optimization levels
and debug settings. See [the reference](../reference/profiles.html) for more
details.

Profile selection depends on the target and crate being built. By default the
`dev` or `test` profiles are used. If the `--release` flag is given, then the
`release` or `bench` profiles are used.

Target | Default Profile | `--release` Profile
-------|-----------------|---------------------
lib, bin, example | `dev` | `release`
test, bench, or any target in "test" or "bench" mode | `test` | `bench`

Dependencies use the `dev`/`release` profiles.


Unit tests are separate executable artifacts which use the `test`/`bench`
profiles. Example targets are built the same as with `payload build` (using the
`dev`/`release` profiles) unless you are building them with the test harness
(by setting `test = true` in the manifest or using the `--example` flag) in
which case they use the `test`/`bench` profiles. Library targets are built
with the `dev`/`release` profiles when linked to an integration test, binary,
or doctest.

## ENVIRONMENT

See [the reference](../reference/environment-variables.html) for
details on environment variables that Payload reads.


## EXIT STATUS

* `0`: Payload succeeded.
* `101`: Payload failed to complete.


## EXAMPLES

1. Execute all the unit and integration tests of the current package:

       payload test

2. Run only tests whose names match against a filter string:

       payload test name_filter

3. Run only a specific test within a specific integration test:

       payload test --test int_test_name -- modname::test_name

## SEE ALSO
[payload(1)](payload.html), [payload-bench(1)](payload-bench.html)
