# Console Output

All of Payload's output should go through the [`Shell`] struct. You can normally
obtain the `Shell` instance from the [`Config`] struct. Do **not** use the std
`println!` macros.

Most of Payload's output goes to stderr. When running in JSON mode, the output
goes to stdout.

It is important to properly handle errors when writing to the console.
Informational commands, like `payload list`, should ignore any errors writing
the output. There are some [`drop_print`] macros that are intended to make
this easier.

Messages written during compilation should handle errors, and abort the build
if they are unable to be displayed. This is generally automatically handled in
the [`JobQueue`] as it processes each message.

[`Shell`]: https://github.com/dustlang/payload/blob/master/src/payload/core/shell.rs
[`Config`]: https://github.com/dustlang/payload/blob/master/src/payload/util/config/mod.rs
[`drop_print`]: https://github.com/dustlang/payload/blob/e4b65bdc80f2a293447f2f6a808fa7c84bf9a357/src/payload/util/config/mod.rs#L1820-L1848
[`JobQueue`]: https://github.com/dustlang/payload/blob/master/src/payload/core/compiler/job_queue.rs

## Errors

Payload uses [`anyhow`] for managing errors. This makes it convenient to "chain"
errors together, so that Payload can report how an error originated, and what it
was trying to do at the time.

Error helpers are implemented in the [`errors`] module. Use the
`InternalError` error type for errors that are not expected to happen. This
will print a message to the user to file a bug report.

The binary side of Payload uses the `CliError` struct to wrap the process exit
code. Usually Payload exits with 101 for an error, but some commands like `payload
test` will exit with different codes.

[`errors`]: https://github.com/dustlang/payload/blob/master/src/payload/util/errors.rs

## Style

Some guidelines for Payload's output:

* Keep the normal output brief. Payload is already fairly noisy, so try to keep
  the output as brief and clean as possible.
* Good error messages are very important! Try to keep them brief and to the
  point, but good enough that a beginner can understand what is wrong and can
  figure out how to fix. It is a difficult balance to hit! Err on the side of
  providing extra information.
* When using any low-level routines, such as `std::fs`, *always* add error
  context about what it is doing. For example, reading from a file should
  include context about which file is being read if there is an error.
* Payload's error style is usually a phrase, starting with a lowercase letter.
  If there is a longer error message that needs multiple sentences, go ahead
  and use multiple sentences. This should probably be improved sometime in the
  future to be more structured.

## Debug logging

Payload uses the [`env_logger`] crate to display debug log messages. The
`PAYLOAD_LOG` environment variable can be set to enable debug logging, with a
value such as `trace`, `debug`, or `warn`. It also supports filtering for
specific modules. Feel free to use the standard [`log`] macros to help with
diagnosing problems.

```sh
# Outputs all logs with levels debug and higher
PAYLOAD_LOG=debug payload generate-lockfile

# Don't forget that you can filter by module as well
PAYLOAD_LOG=payload::core::resolver=trace payload generate-lockfile

# This will print lots of info about the download process. `trace` prints even more.
PAYLOAD_HTTP_DEBUG=true PAYLOAD_LOG=payload::ops::registry=debug payload fetch

# This is an important command for diagnosing fingerprint issues.
PAYLOAD_LOG=payload::core::compiler::fingerprint=trace payload build
```

[`env_logger`]: https://docs.rs/env_logger
[`log`]: https://docs.rs/log
[`anyhow`]: https://docs.rs/anyhow
