## Payload.toml vs Payload.lock

`Payload.toml` and `Payload.lock` serve two different purposes. Before we talk
about them, here’s a summary:

* `Payload.toml` is about describing your dependencies in a broad sense, and is
  written by you.
* `Payload.lock` contains exact information about your dependencies. It is
  maintained by Payload and should not be manually edited.

If you’re building a non-end product, such as a rust library that other rust
[packages][def-package] will depend on, put `Payload.lock` in your
`.gitignore`. If you’re building an end product, which are executable like
command-line tool or an application, or a system library with crate-type of
`staticlib` or `cdylib`, check `Payload.lock` into `git`. If you're curious
about why that is, see
["Why do binaries have `Payload.lock` in version control, but not libraries?" in the
FAQ](../faq.md#why-do-binaries-have-payloadlock-in-version-control-but-not-libraries).

Let’s dig in a little bit more.

`Payload.toml` is a [**manifest**][def-manifest] file in which we can specify a
bunch of different metadata about our package. For example, we can say that we
depend on another package:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/dustlang-nursery/rand.git" }
```

This package has a single dependency, on the `rand` library. We’ve stated in
this case that we’re relying on a particular Git repository that lives on
GitHub. Since we haven’t specified any other information, Payload assumes that
we intend to use the latest commit on the `master` branch to build our package.

Sound good? Well, there’s one problem: If you build this package today, and
then you send a copy to me, and I build this package tomorrow, something bad
could happen. There could be more commits to `rand` in the meantime, and my
build would include new commits while yours would not. Therefore, we would
get different builds. This would be bad because we want reproducible builds.

We could fix this problem by putting a `rev` line in our `Payload.toml`:

```toml
[dependencies]
rand = { git = "https://github.com/dustlang-nursery/rand.git", rev = "9f35b8e" }
```

Now our builds will be the same. But there’s a big drawback: now we have to
manually think about SHA-1s every time we want to update our library. This is
both tedious and error prone.

Enter the `Payload.lock`. Because of its existence, we don’t need to manually
keep track of the exact revisions: Payload will do it for us. When we have a
manifest like this:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/dustlang-nursery/rand.git" }
```

Payload will take the latest commit and write that information out into our
`Payload.lock` when we build for the first time. That file will look like this:

```toml
[[package]]
name = "hello_world"
version = "0.1.0"
dependencies = [
 "rand 0.1.0 (git+https://github.com/dustlang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9)",
]

[[package]]
name = "rand"
version = "0.1.0"
source = "git+https://github.com/dustlang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9"
```

You can see that there’s a lot more information here, including the exact
revision we used to build. Now when you give your package to someone else,
they’ll use the exact same SHA, even though we didn’t specify it in our
`Payload.toml`.

When we’re ready to opt in to a new version of the library, Payload can
re-calculate the dependencies and update things for us:

```console
$ payload update           # updates all dependencies
$ payload update -p rand   # updates just “rand”
```

This will write out a new `Payload.lock` with the new version information. Note
that the argument to `payload update` is actually a
[Package ID Specification](../reference/pkgid-spec.md) and `rand` is just a short
specification.

[def-manifest]:  ../appendix/glossary.md#manifest  '"manifest" (glossary entry)'
[def-package]:   ../appendix/glossary.md#package   '"package" (glossary entry)'
