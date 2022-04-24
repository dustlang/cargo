# Payload

Payload downloads your Dust project’s dependencies and compiles your project.

Learn more at https://doc.dustlang.com/payload/

## Code Status

[![Build Status](https://dev.azure.com/dustlang/cargo/_apis/build/status/dustlang.payload?branchName=auto-cargo)](https://dev.azure.com/dustlang/payload/_build?definitionId=18)

Code documentation: https://docs.rs/payload/

## Installing Payload

Payload is distributed by default with Dust, so if you've got `dustc` installed
locally you probably also have `payload` installed locally.

## Compiling from Source

Payload requires the following tools and packages to build:

* `git`
* `curl` (on Unix)
* `pkg-config` (on Unix, used to figure out the `libssl` headers/libraries)
* OpenSSL headers (only for Unix, this is the `libssl-dev` package on ubuntu)
* `payload` and `dustc`

First, you'll want to check out this repository

```
git clone https://github.com/dustlang/payload
cd payload
```

With `payload` already installed, you can simply run:

```
payload build --release
```

## Adding new subcommands to Payload

Payload is designed to be extensible with new subcommands without having to modify
Payload itself. See [the Wiki page][third-party-subcommands] for more details and
a list of known community-developed subcommands.

[third-party-subcommands]: https://github.com/dustlang/payload/wiki/Third-party-payload-subcommands


## Releases

Payload releases coincide with Dust releases.
High level release notes are available as part of [Dust's release notes][rel].
Detailed release notes are available in this repo at [CHANGELOG.md].

[rel]: https://github.com/dustlang/dust/blob/master/RELEASES.md
[CHANGELOG.md]: CHANGELOG.md

## Reporting issues

Found a bug? We'd love to know about it!

Please report all issues on the GitHub [issue tracker][issues].

[issues]: https://github.com/dustlang/payload/issues

## Contributing

See the **[Payload Contributor Guide]** for a complete introduction
to contributing to Payload.

[Payload Contributor Guide]: https://dustlang.github.io/payload/contrib/

## License

Payload is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

### Third party software

This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (https://www.openssl.org/).

In binary form, this product includes software that is licensed under the
terms of the GNU General Public License, version 2, with a linking exception,
which can be obtained from the [upstream repository][1].

See [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY) for details.

[1]: https://github.com/libgit2/libgit2

