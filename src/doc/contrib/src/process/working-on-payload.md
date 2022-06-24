# Working on Payload

This chapter gives an overview of how to build Payload, make a change, and
submit a Pull Request.

1. [Check out the Payload source.](#checkout-out-the-source)
2. [Building Payload.](#building-payload)
3. [Making a change.](#making-a-change)
4. [Writing and running tests.](../tests/index.md)
5. [Submitting a Pull Request.](#submitting-a-pull-request)
6. [The merging process.](#the-merging-process)

## Checkout out the source

We use the "fork and pull" model [described here][development-models], where
contributors push changes to their personal fork and [create pull requests] to
bring those changes into the source repository. Payload uses [git] and [GitHub]
for all development.

1. Fork the [`dustlang/payload`] repository on GitHub to your personal account
   (see [GitHub docs][how-to-fork]).
2. Clone your fork to your local machine using `git clone` (see [GitHub
   docs][how-to-clone])
3. It is recommended to start a new branch for the change you want to make.
   All Pull Requests are made against the master branch.

## Building Payload

Payload is built by...running `payload`! There are a few prerequisites that you
need to have installed:

* `rustc` and `payload` need to be installed. Payload is expected to build and
  test with the current stable, beta, and nightly releases. It is your choice
  which to use. Nightly is recommended, since some nightly-specific tests are
  disabled when using the stable release. But using stable is fine if you
  aren't working on those.
* A C compiler (typically gcc, clang, or MSVC).
* [git]
* Unix:
    * pkg-config
    * OpenSSL (`libssl-dev` on Ubuntu, `openssl-devel` on Fedora)
* macOS:
    * OpenSSL ([homebrew] is recommended to install the `openssl` package)

If you can successfully run `payload build`, you should be good to go!

[homebrew]: https://brew.sh/

## Running Payload

You can use `payload run` to run payload itself, or you can use the path directly
to the payload binary, such as `target/debug/payload`.

If you are using [`rustup`], beware that running the binary directly can cause
issues with rustup overrides. Usually, when `payload` is executed as part of
rustup, the toolchain becomes sticky (via an environment variable), and all
calls to `rustc` will use the same toolchain. But when `payload` is not run via
rustup, the toolchain may change based on the directory. Since Payload changes
the directory for each compilation, this can cause different calls to `rustc`
to use different versions. There are a few workarounds:

* Don't use rustup overrides.
* Use `rustup run target/debug/payload` to execute `payload`.
* Set the `RUSTC` environment variable to a specific `rustc` executable (not
  the rustup wrapper).
* Create a [custom toolchain]. This is a bit of a hack, but you can create a
  directory in the rustup `toolchains` directory, and create symlinks for all
  the files and directories in there to your toolchain of choice (such as
  nightly), except for the `payload` binary, which you can symlink to your
  `target/debug/payload` binary in your project directory.

*Normally*, all development is done by running Payload's test suite, so running
it directly usually isn't required. But it can be useful for testing Payload on
more complex projects.

[`rustup`]: https://dustlang.github.io/rustup/
[custom toolchain]: https://dustlang.github.io/rustup/concepts/toolchains.html#custom-toolchains

## Making a change

Some guidelines on working on a change:

* All code changes are expected to comply with the formatting suggested by
  `rustfmt`. You can use `rustup component add rustfmt` to install `rustfmt`
  and use `payload fmt` to automatically format your code.
* [Commit as you go][githelp].
* Include tests that cover all non-trivial code. See the [Testing chapter] for
  more about writing and running tests.
* All code should be warning-free. This is checked during tests.

## Submitting a Pull Request

After you have committed your work, and pushed it to GitHub, you can
open a Pull Request

* Push your commits to GitHub and create a pull request against Payload's
  `master` branch.
* Include a clear description of what the change is and why it is being made.
* Use [GitHub's keywords] in the description to automatically link to an issue
  if the PR resolves the issue. For example `Closes #1234` will link issue
  #1234 to the PR. When the PR is merged, GitHub will automatically close the
  issue.

The [rust-highfive] bot will automatically assign a reviewer for the PR. It
may take at least a few days for someone to respond. If you don't get a
response in over a week, feel free to ping the assigned reviewer.

When your PR is submitted, GitHub automatically runs all tests. The GitHub
interface will show a green checkmark if it passes, or a red X if it fails.
There are links to the logs on the PR page to diagnose any issues. The tests
typically finish in under 30 minutes.

The reviewer might point out changes deemed necessary. Large or tricky changes
may require several passes of review and changes.

## The merging process

After a reviewer has approved your PR, they will issue a command to the [bors]
bot (also known as "Homu", the software that powers [`@bors`]). Bors will
create a temporary branch with your PR, and run all tests. Only if all tests
pass will it merge the PR to master. If it fails, the bot will leave a comment
on the PR. This system ensures that the master branch is always in a good
state, and that merges are processed one at a time. The [Homu queue
dashboard][homu-payload] shows the current merge queue. Payload's queue is rarely
busy, but a busy project like the [rust repo][homu-rust] is constantly full.

Assuming everything works, congratulations! It may take at least a week for
the changes to arrive on the nightly channel. See the [release chapter] for
more information on how Payload releases are made.


[development-models]: https://help.github.com/articles/about-collaborative-development-models/
[create pull requests]: https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request
[how-to-fork]: https://docs.github.com/en/github/getting-started-with-github/fork-a-repo
[`dustlang/payload`]: https://github.com/dustlang/payload/
[git]: https://git-scm.com/
[GitHub]: https://github.com/
[how-to-clone]: https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/cloning-a-repository
[githelp]: https://dont-be-afraid-to-commit.readthedocs.io/en/latest/git/commandlinegit.html
[Testing chapter]: ../tests/index.md
[GitHub's keywords]: https://docs.github.com/en/github/managing-your-work-on-github/linking-a-pull-request-to-an-issue
[rust-highfive]: https://github.com/rust-highfive
[bors]: https://buildbot2.dustlang.com/homu/
[`@bors`]: https://github.com/bors
[homu-payload]: https://buildbot2.dustlang.com/homu/queue/payload
[homu-rust]: https://buildbot2.dustlang.com/homu/queue/rust
[release chapter]: release.md
