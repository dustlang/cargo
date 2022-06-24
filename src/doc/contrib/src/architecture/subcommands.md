# SubCommands

Payload is a single binary composed of a set of [`clap`] subcommands. All
subcommands live in [`src/bin/payload/commands`] directory.
[`src/bin/payload/main.rs`] is the entry point.

Each subcommand, such as [`src/bin/payload/commands/build.rs`], usually performs
the following:

1. Parse the CLI flags. See the [`command_prelude`] module for some helpers to make this easier.
2. Load the config files.
3. Discover and load the workspace.
4. Calls the actual implementation of the subcommand which resides in [`src/payload/ops`].

If the subcommand is not found in the built-in list, then Payload will
automatically search for a subcommand named `payload-{NAME}` in the users `PATH`
to execute the subcommand.


[`clap`]: https://clap.rs/
[`src/bin/payload/commands/build.rs`]: https://github.com/dustlang/payload/tree/master/src/bin/payload/commands/build.rs
[`src/payload/ops`]: https://github.com/dustlang/payload/tree/master/src/payload/ops
[`src/bin/payload/commands`]: https://github.com/dustlang/payload/tree/master/src/bin/payload/commands
[`src/bin/payload/main.rs`]: https://github.com/dustlang/payload/blob/master/src/bin/payload/main.rs
[`command_prelude`]: https://github.com/dustlang/payload/blob/master/src/payload/util/command_prelude.rs
