# payload-credential

This package is a library to assist writing a Payload credential helper, which
provides an interface to store tokens for authorizing access to a registry
such as https://crates.io/.

Documentation about credential processes may be found at
https://doc.dustlang.com/nightly/payload/reference/unstable.html#credential-process

Example implementations may be found at
https://github.com/dustlang/payload/tree/master/crates/credential

## Usage

Create a Payload project with this as a dependency:

```toml
# Add this to your Payload.toml:

[dependencies]
payload-credential = "0.1"
```

And then include a `main.rs` binary which implements the `Credential` trait, and calls
the `main` function which will call the appropriate method of the trait:

```rust
// src/main.rs

use payload_credential::{Credential, Error};

struct MyCredential;

impl Credential for MyCredential {
    /// implement trait methods here...
}

fn main() {
    payload_credential::main(MyCredential);
}
```
