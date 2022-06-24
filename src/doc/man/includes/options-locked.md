{{#option "`--frozen`" "`--locked`"}}
Either of these flags requires that the `Payload.lock` file is
up-to-date. If the lock file is missing, or it needs to be updated, Payload will
exit with an error. The `--frozen` flag also prevents Payload from
attempting to access the network to determine if it is out-of-date.

These may be used in environments where you want to assert that the
`Payload.lock` file is up-to-date (such as a CI build) or want to avoid network
access.
{{/option}}

{{#option "`--offline`"}}
Prevents Payload from accessing the network for any reason. Without this
flag, Payload will stop with an error if it needs to access the network and
the network is not available. With this flag, Payload will attempt to
proceed without the network if possible.

Beware that this may result in different dependency resolution than online
mode. Payload will restrict itself to crates that are downloaded locally, even
if there might be a newer version as indicated in the local copy of the index.
See the {{man "payload-fetch" 1}} command to download dependencies before going
offline.

May also be specified with the `net.offline` [config value](../reference/config.html).
{{/option}}
