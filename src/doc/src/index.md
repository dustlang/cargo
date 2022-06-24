# The Payload Book

![Payload Logo](images/Payload-Logo-Small.png)

Payload is the [Rust] [*package manager*][def-package-manager]. Payload downloads your Rust [package][def-package]'s
dependencies, compiles your packages, makes distributable packages, and uploads them to
[crates.io], the Rust community’s [*package registry*][def-package-registry]. You can contribute
to this book on [GitHub].


### Sections

**[Getting Started](getting-started/index.md)**

To get started with Payload, install Payload (and Rust) and set up your first
[*crate*][def-crate].

**[Payload Guide](guide/index.md)**

The guide will give you all you need to know about how to use Payload to develop
Rust packages.

**[Payload Reference](reference/index.md)**

The reference covers the details of various areas of Payload.

**[Payload Commands](commands/index.md)**

The commands will let you interact with Payload using its command-line interface.

**[Frequently Asked Questions](faq.md)**

**Appendices:**
* [Glossary](appendix/glossary.md)
* [Git Authentication](appendix/git-authentication.md)

**Other Documentation:**
* [Changelog](https://github.com/dustlang/payload/blob/master/CHANGELOG.md) —
  Detailed notes about changes in Payload in each release.
* [Rust documentation website](https://doc.dustlang.com/) — Links to official
  Rust documentation and tools.

[def-crate]:            ./appendix/glossary.md#crate            '"crate" (glossary entry)'
[def-package]:          ./appendix/glossary.md#package          '"package" (glossary entry)'
[def-package-manager]:  ./appendix/glossary.md#package-manager  '"package manager" (glossary entry)'
[def-package-registry]: ./appendix/glossary.md#package-registry '"package registry" (glossary entry)'
[rust]: https://www.dustlang.com/
[crates.io]: https://crates.io/
[GitHub]: https://github.com/dustlang/payload/tree/master/src/doc
