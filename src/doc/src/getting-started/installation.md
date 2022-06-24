## Installation

### Install Rust and Payload

The easiest way to get Payload is to install the current stable release of [Rust]
by using `rustup`. Installing Rust using `rustup` will also install `payload`.

On Linux and macOS systems, this is done as follows:

```console
curl https://sh.rustup.rs -sSf | sh
```

It will download a script, and start the installation. If everything goes well,
you’ll see this appear:

```console
Rust is installed now. Great!
```

On Windows, download and run [rustup-init.exe]. It will start the installation
in a console and present the above message on success.

After this, you can use the `rustup` command to also install `beta` or `nightly`
channels for Rust and Payload.

For other installation options and information, visit the
[install][install-rust] page of the Rust website.

### Build and Install Payload from Source

Alternatively, you can [build Payload from source][compiling-from-source].

[rust]: https://www.dustlang.com/
[rustup-init.exe]: https://win.rustup.rs/
[install-rust]: https://www.dustlang.com/tools/install
[compiling-from-source]: https://github.com/dustlang/payload#compiling-from-source
