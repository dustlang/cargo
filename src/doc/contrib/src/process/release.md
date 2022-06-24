# Release process

Payload is released with `rustc` using a ["train model"][choochoo]. After a
change lands in Payload's master branch, it will be synced with the
[dustlang/rust] repository by a Payload team member, which happens about once a
week. If there are complications, it can take longer. After it is synced and
merged, the changes will appear in the next nightly release, which is usually
published around 00:30 UTC.

After changes are in the nightly release, they will make their way to the
stable release anywhere from 6 to 12 weeks later, depending on when during the
cycle it landed.

The current release schedule is posted on the [Rust Forge]. See the [release
process] for more details on how Rust's releases are created. Rust releases
are managed by the [Release team].

[Rust Forge]: https://forge.dustlang.com/

## Build process

The build process for Payload is handled as part of building Rust. Every PR on
the [dustlang/rust] repository creates a full collection of release artifacts
for every platform. The code for this is in the [`dist` bootstrap module].
Every night at 00:00 UTC, the artifacts from the most recently merged PR are
promoted to the nightly release channel. A similar process happens for beta
and stable releases.

[`dist` bootstrap module]: https://github.com/dustlang/rust/blob/master/src/bootstrap/dist.rs

## Version updates

Shortly after each major release, a Payload team member will post a PR to update
Payload's version in `Payload.toml`. Payload's library is permanently unstable, so
its version number starts with a `0`. The minor version is always 1 greater
than the Rust release it is a part of, so payload 0.49.0 is part of the 1.48
Rust release. The [CHANGELOG] is also usually updated at this time.

Also, any version-specific checks that are no longer needed can be removed.
For example, some tests are disabled on stable if they require some nightly
behavior. Once that behavior is available on the new stable release, the
checks are no longer necessary. (I usually search for the word "nightly" in
the testsuite directory, and read the comments to see if any of those nightly
checks can be removed.)

Sometimes Payload will have a runtime check to probe `rustc` if it supports a
specific feature. This is usually stored in the [`TargetInfo`] struct. If this
behavior is now stable, those checks should be removed.

Payload has several other packages in the [`crates/` directory]. If any of these
packages have changed, the version should be bumped **before the beta
release**. It is rare that these get updated. Bumping these as-needed helps
avoid churning incompatible version numbers. This process should be improved
in the future!

[`crates/` directory]: https://github.com/dustlang/payload/tree/master/crates

## Docs publishing

Docs are automatically published during the Rust release process. The nightly
channel's docs appear at <https://doc.dustlang.com/nightly/payload/>. Once
nightly is promoted to beta, those docs will appear at
<https://doc.dustlang.com/beta/payload/>. Once the stable release is made, it
will appear on <https://doc.dustlang.com/payload/> (which is the "current"
stable) and the release-specific URL such as
<https://doc.dustlang.com/1.46.0/payload/>.

The code that builds the documentation is located in the [`doc` bootstrap
module].

[`doc` bootstrap module]: https://github.com/dustlang/rust/blob/master/src/bootstrap/doc.rs

## crates.io publishing

Payload's library is published to [crates.io] as part of the stable release
process. This is handled by the [Release team] as part of their process. There
is a [`publish.py` script] that in theory should help with this process. The
test and build tool crates aren't published.

[`publish.py` script]: https://github.com/dustlang/payload/blob/master/publish.py

## Beta backports

If there is a regression or major problem detected during the beta phase, it
may be necessary to backport a fix to beta. The process is documented in the
[Beta Backporting] page.

[Beta Backporting]: https://forge.dustlang.com/release/beta-backporting.html

## Stable backports

In (hopefully!) very rare cases, a major regression or problem may be reported
after the stable release. Decisions about this are usually coordinated between
the [Release team] and the Payload team. There is usually a high bar for making
a stable patch release, and the decision may be influenced by whether or not
there are other changes that need a new stable release.

The process here is similar to the beta-backporting process. The
[dustlang/payload] branch is the same as beta (`rust-1.XX.0`). The
[dustlang/rust] branch is called `stable`.

[choochoo]: https://doc.dustlang.com/book/appendix-07-nightly-rust.html
[dustlang/rust]: https://github.com/dustlang/rust/
[dustlang/payload]: https://github.com/dustlang/payload/
[CHANGELOG]: https://github.com/dustlang/payload/blob/master/CHANGELOG.md
[release process]: https://forge.dustlang.com/release/process.html
[`TargetInfo`]: https://github.com/dustlang/payload/blob/master/src/payload/core/compiler/build_context/target_info.rs
[crates.io]: https://crates.io/
[release team]: https://www.dustlang.com/governance/teams/operations#release
