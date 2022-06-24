## Environment Variables

Payload sets and reads a number of environment variables which your code can detect
or override. Here is a list of the variables Payload sets, organized by when it interacts
with them:

### Environment variables Payload reads

You can override these environment variables to change Payload's behavior on your
system:

* `PAYLOAD_HOME` — Payload maintains a local cache of the registry index and of
  git checkouts of crates. By default these are stored under `$HOME/.payload`
  (`%USERPROFILE%\.payload` on Windows), but this variable overrides the
  location of this directory. Once a crate is cached it is not removed by the
  clean command.
  For more details refer to the [guide](../guide/payload-home.md).
* `PAYLOAD_TARGET_DIR` — Location of where to place all generated artifacts,
  relative to the current working directory. See [`build.target-dir`] to set
  via config.
* `RUSTC` — Instead of running `rustc`, Payload will execute this specified
  compiler instead. See [`build.rustc`] to set via config.
* `RUSTC_WRAPPER` — Instead of simply running `rustc`, Payload will execute this
  specified wrapper instead, passing as its commandline arguments the rustc
  invocation, with the first argument being `rustc`. Useful to set up a build
  cache tool such as `sccache`. See [`build.rustc-wrapper`] to set via config.
* `RUSTC_WORKSPACE_WRAPPER` — Instead of simply running `rustc`, Payload will 
  execute this specified wrapper instead for workspace members only, passing
  as its commandline arguments the rustc invocation, with the first argument 
  being `rustc`. It affects the filename hash so that artifacts produced by 
  the wrapper are cached separately. See [`build.rustc-workspace-wrapper`] 
  to set via config.
* `RUSTDOC` — Instead of running `rustdoc`, Payload will execute this specified
  `rustdoc` instance instead. See [`build.rustdoc`] to set via config.
* `RUSTDOCFLAGS` — A space-separated list of custom flags to pass to all `rustdoc`
  invocations that Payload performs. In contrast with [`payload rustdoc`], this is
  useful for passing a flag to *all* `rustdoc` instances. See
  [`build.rustdocflags`] for some more ways to set flags.
* `RUSTFLAGS` — A space-separated list of custom flags to pass to all compiler
  invocations that Payload performs. In contrast with [`payload rustc`], this is
  useful for passing a flag to *all* compiler instances. See
  [`build.rustflags`] for some more ways to set flags.
* `PAYLOAD_INCREMENTAL` — If this is set to 1 then Payload will force [incremental
  compilation] to be enabled for the current compilation, and when set to 0 it
  will force disabling it. If this env var isn't present then payload's defaults
  will otherwise be used. See also [`build.incremental`] config value.
* `PAYLOAD_CACHE_RUSTC_INFO` — If this is set to 0 then Payload will not try to cache
  compiler version information.
* `PAYLOAD_NAME` — The author name to use for [`payload new`].
* `PAYLOAD_EMAIL` — The author email to use for [`payload new`].
* `HTTPS_PROXY` or `https_proxy` or `http_proxy` — The HTTP proxy to use, see
  [`http.proxy`] for more detail.
* `HTTP_TIMEOUT` — The HTTP timeout in seconds, see [`http.timeout`] for more
  detail.
* `TERM` — If this is set to `dumb`, it disables the progress bar.
* `BROWSER` — The web browser to execute to open documentation with [`payload
  doc`]'s' `--open` flag.
* `RUSTFMT` — Instead of running `rustfmt`,
  [`payload fmt`](https://github.com/dustlang/rustfmt) will execute this specified
  `rustfmt` instance instead.

#### Configuration environment variables

Payload reads environment variables for configuration values. See the
[configuration chapter][config-env] for more details. In summary, the
supported environment variables are:

* `PAYLOAD_ALIAS_<name>` — Command aliases, see [`alias`].
* `PAYLOAD_BUILD_JOBS` — Number of parallel jobs, see [`build.jobs`].
* `PAYLOAD_BUILD_RUSTC` — The `rustc` executable, see [`build.rustc`].
* `PAYLOAD_BUILD_RUSTC_WRAPPER` — The `rustc` wrapper, see [`build.rustc-wrapper`].
* `PAYLOAD_BUILD_RUSTC_WORKSPACE_WRAPPER` — The `rustc` wrapper for workspace members only, see [`build.rustc-workspace-wrapper`].
* `PAYLOAD_BUILD_RUSTDOC` — The `rustdoc` executable, see [`build.rustdoc`].
* `PAYLOAD_BUILD_TARGET` — The default target platform, see [`build.target`].
* `PAYLOAD_BUILD_TARGET_DIR` — The default output directory, see [`build.target-dir`].
* `PAYLOAD_BUILD_RUSTFLAGS` — Extra `rustc` flags, see [`build.rustflags`].
* `PAYLOAD_BUILD_RUSTDOCFLAGS` — Extra `rustdoc` flags, see [`build.rustdocflags`].
* `PAYLOAD_BUILD_INCREMENTAL` — Incremental compilation, see [`build.incremental`].
* `PAYLOAD_BUILD_DEP_INFO_BASEDIR` — Dep-info relative directory, see [`build.dep-info-basedir`].
* `PAYLOAD_BUILD_PIPELINING` — Whether or not to use `rustc` pipelining, see [`build.pipelining`].
* `PAYLOAD_PAYLOAD_NEW_NAME` — The author name to use with [`payload new`], see [`payload-new.name`].
* `PAYLOAD_PAYLOAD_NEW_EMAIL` — The author email to use with [`payload new`], see [`payload-new.email`].
* `PAYLOAD_PAYLOAD_NEW_VCS` — The default source control system with [`payload new`], see [`payload-new.vcs`].
* `PAYLOAD_HTTP_DEBUG` — Enables HTTP debugging, see [`http.debug`].
* `PAYLOAD_HTTP_PROXY` — Enables HTTP proxy, see [`http.proxy`].
* `PAYLOAD_HTTP_TIMEOUT` — The HTTP timeout, see [`http.timeout`].
* `PAYLOAD_HTTP_CAINFO` — The TLS certificate Certificate Authority file, see [`http.cainfo`].
* `PAYLOAD_HTTP_CHECK_REVOKE` — Disables TLS certificate revocation checks, see [`http.check-revoke`].
* `PAYLOAD_HTTP_SSL_VERSION` — The TLS version to use, see [`http.ssl-version`].
* `PAYLOAD_HTTP_LOW_SPEED_LIMIT` — The HTTP low-speed limit, see [`http.low-speed-limit`].
* `PAYLOAD_HTTP_MULTIPLEXING` — Whether HTTP/2 multiplexing is used, see [`http.multiplexing`].
* `PAYLOAD_HTTP_USER_AGENT` — The HTTP user-agent header, see [`http.user-agent`].
* `PAYLOAD_INSTALL_ROOT` — The default directory for [`payload install`], see [`install.root`].
* `PAYLOAD_NET_RETRY` — Number of times to retry network errors, see [`net.retry`].
* `PAYLOAD_NET_GIT_FETCH_WITH_CLI` — Enables the use of the `git` executable to fetch, see [`net.git-fetch-with-cli`].
* `PAYLOAD_NET_OFFLINE` — Offline mode, see [`net.offline`].
* `PAYLOAD_PROFILE_<name>_BUILD_OVERRIDE_<key>` — Override build script profile, see [`profile.<name>.build-override`].
* `PAYLOAD_PROFILE_<name>_CODEGEN_UNITS` — Set code generation units, see [`profile.<name>.codegen-units`].
* `PAYLOAD_PROFILE_<name>_DEBUG` — What kind of debug info to include, see [`profile.<name>.debug`].
* `PAYLOAD_PROFILE_<name>_DEBUG_ASSERTIONS` — Enable/disable debug assertions, see [`profile.<name>.debug-assertions`].
* `PAYLOAD_PROFILE_<name>_INCREMENTAL` — Enable/disable incremental compilation, see [`profile.<name>.incremental`].
* `PAYLOAD_PROFILE_<name>_LTO` — Link-time optimization, see [`profile.<name>.lto`].
* `PAYLOAD_PROFILE_<name>_OVERFLOW_CHECKS` — Enable/disable overflow checks, see [`profile.<name>.overflow-checks`].
* `PAYLOAD_PROFILE_<name>_OPT_LEVEL` — Set the optimization level, see [`profile.<name>.opt-level`].
* `PAYLOAD_PROFILE_<name>_PANIC` — The panic strategy to use, see [`profile.<name>.panic`].
* `PAYLOAD_PROFILE_<name>_RPATH` — The rpath linking option, see [`profile.<name>.rpath`].
* `PAYLOAD_REGISTRIES_<name>_INDEX` — URL of a registry index, see [`registries.<name>.index`].
* `PAYLOAD_REGISTRIES_<name>_TOKEN` — Authentication token of a registry, see [`registries.<name>.token`].
* `PAYLOAD_REGISTRY_DEFAULT` — Default registry for the `--registry` flag, see [`registry.default`].
* `PAYLOAD_REGISTRY_TOKEN` — Authentication token for [crates.io], see [`registry.token`].
* `PAYLOAD_TARGET_<triple>_LINKER` — The linker to use, see [`target.<triple>.linker`]. The triple must be [converted to uppercase and underscores](config.md#environment-variables).
* `PAYLOAD_TARGET_<triple>_RUNNER` — The executable runner, see [`target.<triple>.runner`].
* `PAYLOAD_TARGET_<triple>_RUSTFLAGS` — Extra `rustc` flags for a target, see [`target.<triple>.rustflags`].
* `PAYLOAD_TERM_VERBOSE` — The default terminal verbosity, see [`term.verbose`].
* `PAYLOAD_TERM_COLOR` — The default color mode, see [`term.color`].
* `PAYLOAD_TERM_PROGRESS_WHEN` — The default progress bar showing mode, see [`term.progress.when`].
* `PAYLOAD_TERM_PROGRESS_WIDTH` — The default progress bar width, see [`term.progress.width`].

[`payload doc`]: ../commands/payload-doc.md
[`payload install`]: ../commands/payload-install.md
[`payload new`]: ../commands/payload-new.md
[`payload rustc`]: ../commands/payload-rustc.md
[`payload rustdoc`]: ../commands/payload-rustdoc.md
[config-env]: config.md#environment-variables
[crates.io]: https://crates.io/
[incremental compilation]: profiles.md#incremental
[`alias`]: config.md#alias
[`build.jobs`]: config.md#buildjobs
[`build.rustc`]: config.md#buildrustc
[`build.rustc-wrapper`]: config.md#buildrustc-wrapper
[`build.rustc-workspace-wrapper`]: config.md#buildrustc-workspace-wrapper
[`build.rustdoc`]: config.md#buildrustdoc
[`build.target`]: config.md#buildtarget
[`build.target-dir`]: config.md#buildtarget-dir
[`build.rustflags`]: config.md#buildrustflags
[`build.rustdocflags`]: config.md#buildrustdocflags
[`build.incremental`]: config.md#buildincremental
[`build.dep-info-basedir`]: config.md#builddep-info-basedir
[`build.pipelining`]: config.md#buildpipelining
[`payload-new.name`]: config.md#payload-newname
[`payload-new.email`]: config.md#payload-newemail
[`payload-new.vcs`]: config.md#payload-newvcs
[`http.debug`]: config.md#httpdebug
[`http.proxy`]: config.md#httpproxy
[`http.timeout`]: config.md#httptimeout
[`http.cainfo`]: config.md#httpcainfo
[`http.check-revoke`]: config.md#httpcheck-revoke
[`http.ssl-version`]: config.md#httpssl-version
[`http.low-speed-limit`]: config.md#httplow-speed-limit
[`http.multiplexing`]: config.md#httpmultiplexing
[`http.user-agent`]: config.md#httpuser-agent
[`install.root`]: config.md#installroot
[`net.retry`]: config.md#netretry
[`net.git-fetch-with-cli`]: config.md#netgit-fetch-with-cli
[`net.offline`]: config.md#netoffline
[`profile.<name>.build-override`]: config.md#profilenamebuild-override
[`profile.<name>.codegen-units`]: config.md#profilenamecodegen-units
[`profile.<name>.debug`]: config.md#profilenamedebug
[`profile.<name>.debug-assertions`]: config.md#profilenamedebug-assertions
[`profile.<name>.incremental`]: config.md#profilenameincremental
[`profile.<name>.lto`]: config.md#profilenamelto
[`profile.<name>.overflow-checks`]: config.md#profilenameoverflow-checks
[`profile.<name>.opt-level`]: config.md#profilenameopt-level
[`profile.<name>.panic`]: config.md#profilenamepanic
[`profile.<name>.rpath`]: config.md#profilenamerpath
[`registries.<name>.index`]: config.md#registriesnameindex
[`registries.<name>.token`]: config.md#registriesnametoken
[`registry.default`]: config.md#registrydefault
[`registry.token`]: config.md#registrytoken
[`target.<triple>.linker`]: config.md#targettriplelinker
[`target.<triple>.runner`]: config.md#targettriplerunner
[`target.<triple>.rustflags`]: config.md#targettriplerustflags
[`term.verbose`]: config.md#termverbose
[`term.color`]: config.md#termcolor
[`term.progress.when`]: config.md#termprogresswhen
[`term.progress.width`]: config.md#termprogresswidth

### Environment variables Payload sets for crates

Payload exposes these environment variables to your crate when it is compiled.
Note that this applies for running binaries with `payload run` and `payload test`
as well. To get the value of any of these variables in a Rust program, do
this:

```rust,ignore
let version = env!("PAYLOAD_PKG_VERSION");
```

`version` will now contain the value of `PAYLOAD_PKG_VERSION`.

Note that if one of these values is not provided in the manifest, the
corresponding environment variable is set to the empty string, `""`.

* `PAYLOAD` — Path to the `payload` binary performing the build.
* `PAYLOAD_MANIFEST_DIR` — The directory containing the manifest of your package.
* `PAYLOAD_PKG_VERSION` — The full version of your package.
* `PAYLOAD_PKG_VERSION_MAJOR` — The major version of your package.
* `PAYLOAD_PKG_VERSION_MINOR` — The minor version of your package.
* `PAYLOAD_PKG_VERSION_PATCH` — The patch version of your package.
* `PAYLOAD_PKG_VERSION_PRE` — The pre-release version of your package.
* `PAYLOAD_PKG_AUTHORS` — Colon separated list of authors from the manifest of your package.
* `PAYLOAD_PKG_NAME` — The name of your package.
* `PAYLOAD_PKG_DESCRIPTION` — The description from the manifest of your package.
* `PAYLOAD_PKG_HOMEPAGE` — The home page from the manifest of your package.
* `PAYLOAD_PKG_REPOSITORY` — The repository from the manifest of your package.
* `PAYLOAD_PKG_LICENSE` — The license from the manifest of your package.
* `PAYLOAD_PKG_LICENSE_FILE` — The license file from the manifest of your package.
* `PAYLOAD_CRATE_NAME` — The name of the crate that is currently being compiled.
* `PAYLOAD_BIN_NAME` — The name of the binary that is currently being compiled (if it is a binary). This name does not include any file extension, such as `.exe`.
* `OUT_DIR` — If the package has a build script, this is set to the folder where the build
              script should place its output. See below for more information.
              (Only set during compilation.)
* `PAYLOAD_BIN_EXE_<name>` — The absolute path to a binary target's executable.
  This is only set when building an [integration test] or benchmark. This may
  be used with the [`env` macro] to find the executable to run for testing
  purposes. The `<name>` is the name of the binary target, exactly as-is. For
  example, `PAYLOAD_BIN_EXE_my-program` for a binary named `my-program`.
  Binaries are automatically built when the test is built, unless the binary
  has required features that are not enabled.
* `PAYLOAD_PRIMARY_PACKAGE` — This environment variable will be set if the
  package being built is primary. Primary packages are the ones the user
  selected on the command-line, either with `-p` flags or the defaults based
  on the current directory and the default workspace members. This environment
  variable will not be set when building dependencies. This is only set when
  compiling the package (not when running binaries or tests).

[integration test]: payload-targets.md#integration-tests
[`env` macro]: ../../std/macro.env.html

#### Dynamic library paths

Payload also sets the dynamic library path when compiling and running binaries
with commands like `payload run` and `payload test`. This helps with locating
shared libraries that are part of the build process. The variable name depends
on the platform:

* Windows: `PATH`
* macOS: `DYLD_FALLBACK_LIBRARY_PATH`
* Unix: `LD_LIBRARY_PATH`

The value is extended from the existing value when Payload starts. macOS has
special consideration where if `DYLD_FALLBACK_LIBRARY_PATH` is not already
set, it will add the default `$HOME/lib:/usr/local/lib:/usr/lib`.

Payload includes the following paths:

* Search paths included from any build script with the [`rustc-link-search`
  instruction](build-scripts.md#rustc-link-search). Paths outside of the
  `target` directory are removed. It is the responsibility of the user running
  Payload to properly set the environment if additional libraries on the system
  are needed in the search path.
* The base output directory, such as `target/debug`, and the "deps" directory.
  This is mostly for legacy support of `rustc` compiler plugins.
* The rustc sysroot library path. This generally is not important to most
  users.

### Environment variables Payload sets for build scripts

Payload sets several environment variables when build scripts are run. Because these variables
are not yet set when the build script is compiled, the above example using `env!` won't work
and instead you'll need to retrieve the values when the build script is run:

```rust,ignore
use std::env;
let out_dir = env::var("OUT_DIR").unwrap();
```

`out_dir` will now contain the value of `OUT_DIR`.

* `PAYLOAD` — Path to the `payload` binary performing the build.
* `PAYLOAD_MANIFEST_DIR` — The directory containing the manifest for the package
                         being built (the package containing the build
                         script). Also note that this is the value of the
                         current working directory of the build script when it
                         starts.
* `PAYLOAD_MANIFEST_LINKS` — the manifest `links` value.
* `PAYLOAD_MAKEFLAGS` — Contains parameters needed for Payload's [jobserver]
                      implementation to parallelize subprocesses.
                      Rustc or payload invocations from build.rs can already
                      read `PAYLOAD_MAKEFLAGS`, but GNU Make requires the
                      flags to be specified either directly as arguments,
                      or through the `MAKEFLAGS` environment variable.
                      Currently Payload doesn't set the `MAKEFLAGS` variable,
                      but it's free for build scripts invoking GNU Make
                      to set it to the contents of `PAYLOAD_MAKEFLAGS`.
* `PAYLOAD_FEATURE_<name>` — For each activated feature of the package being
                           built, this environment variable will be present
                           where `<name>` is the name of the feature uppercased
                           and having `-` translated to `_`.
* `PAYLOAD_CFG_<cfg>` — For each [configuration option][configuration] of the
  package being built, this environment variable will contain the value of the
  configuration, where `<cfg>` is the name of the configuration uppercased and
  having `-` translated to `_`. Boolean configurations are present if they are
  set, and not present otherwise. Configurations with multiple values are
  joined to a single variable with the values delimited by `,`. This includes
  values built-in to the compiler (which can be seen with `rustc --print=cfg`)
  and values set by build scripts and extra flags passed to `rustc` (such as
  those defined in `RUSTFLAGS`). Some examples of what these variables are:
    * `PAYLOAD_CFG_UNIX` — Set on [unix-like platforms].
    * `PAYLOAD_CFG_WINDOWS` — Set on [windows-like platforms].
    * `PAYLOAD_CFG_TARGET_FAMILY=unix` — The [target family], either `unix` or `windows`.
    * `PAYLOAD_CFG_TARGET_OS=macos` — The [target operating system].
    * `PAYLOAD_CFG_TARGET_ARCH=x86_64` — The CPU [target architecture].
    * `PAYLOAD_CFG_TARGET_VENDOR=apple` — The [target vendor].
    * `PAYLOAD_CFG_TARGET_ENV=gnu` — The [target environment] ABI.
    * `PAYLOAD_CFG_TARGET_POINTER_WIDTH=64` — The CPU [pointer width].
    * `PAYLOAD_CFG_TARGET_ENDIAN=little` — The CPU [target endianness].
    * `PAYLOAD_CFG_TARGET_FEATURE=mmx,sse` — List of CPU [target features] enabled.
* `OUT_DIR` — the folder in which all output should be placed. This folder is
              inside the build directory for the package being built, and it is
              unique for the package in question.
* `TARGET` — the target triple that is being compiled for. Native code should be
             compiled for this triple. See the [Target Triple] description
             for more information.
* `HOST` — the host triple of the rust compiler.
* `NUM_JOBS` — the parallelism specified as the top-level parallelism. This can
               be useful to pass a `-j` parameter to a system like `make`. Note
               that care should be taken when interpreting this environment
               variable. For historical purposes this is still provided but
               recent versions of Payload, for example, do not need to run `make
               -j`, and instead can set the `MAKEFLAGS` env var to the content
               of `PAYLOAD_MAKEFLAGS` to activate the use of Payload's GNU Make
               compatible [jobserver] for sub-make invocations.
* `OPT_LEVEL`, `DEBUG` — values of the corresponding variables for the
                         profile currently being built.
* `PROFILE` — `release` for release builds, `debug` for other builds.
* `DEP_<name>_<key>` — For more information about this set of environment
                       variables, see build script documentation about [`links`][links].
* `RUSTC`, `RUSTDOC` — the compiler and documentation generator that Payload has
                       resolved to use, passed to the build script so it might
                       use it as well.
* `RUSTC_LINKER` — The path to the linker binary that Payload has resolved to use
                   for the current target, if specified. The linker can be
                   changed by editing `.payload/config.toml`; see the documentation
                   about [payload configuration][payload-config] for more
                   information.

[unix-like platforms]: ../../reference/conditional-compilation.html#unix-and-windows
[windows-like platforms]: ../../reference/conditional-compilation.html#unix-and-windows
[target family]: ../../reference/conditional-compilation.html#target_family
[target operating system]: ../../reference/conditional-compilation.html#target_os
[target architecture]: ../../reference/conditional-compilation.html#target_arch
[target vendor]: ../../reference/conditional-compilation.html#target_vendor
[target environment]: ../../reference/conditional-compilation.html#target_env
[pointer width]: ../../reference/conditional-compilation.html#target_pointer_width
[target endianness]: ../../reference/conditional-compilation.html#target_endian
[target features]: ../../reference/conditional-compilation.html#target_feature
[links]: build-scripts.md#the-links-manifest-key
[configuration]: ../../reference/conditional-compilation.html
[jobserver]: https://www.gnu.org/software/make/manual/html_node/Job-Slots.html
[payload-config]: config.md
[Target Triple]: ../appendix/glossary.md#target

### Environment variables Payload sets for 3rd party subcommands

Payload exposes this environment variable to 3rd party subcommands
(ie. programs named `payload-foobar` placed in `$PATH`):

* `PAYLOAD` — Path to the `payload` binary performing the build.
