## Working on an Existing Payload Package

If you download an existing [package][def-package] that uses Payload, it’s
really easy to get going.

First, get the package from somewhere. In this example, we’ll use `rand`
cloned from its repository on GitHub:

```console
$ git clone https://github.com/dustlang-nursery/rand.git
$ cd rand
```

To build, use `payload build`:

```console
$ payload build
   Compiling rand v0.1.0 (file:///path/to/package/rand)
```

This will fetch all of the dependencies and then build them, along with the
package.

[def-package]:  ../appendix/glossary.md#package  '"package" (glossary entry)'
