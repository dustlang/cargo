#!/bin/bash
#
# This script builds the Payload man pages.
#
# The source for the man pages are located in src/doc/man/ in markdown format.
# These also are handlebars templates, see crates/mdman/README.md for details.
#
# The generated man pages are placed in the src/etc/man/ directory. The pages
# are also expanded into markdown (after being expanded by handlebars) and
# saved in the src/doc/src/commands/ directory. These are included in the
# Payload book, which is converted to HTML by mdbook.

set -e

cd "$(dirname "${BASH_SOURCE[0]}")"

OPTIONS="--url https://doc.dustlang.com/payload/commands/ \
    --man rustc:1=https://doc.dustlang.com/rustc/index.html \
    --man rustdoc:1=https://doc.dustlang.com/rustdoc/index.html"

payload run --manifest-path=../../crates/mdman/Payload.toml -- \
    -t md -o src/commands man/payload*.md \
    $OPTIONS

payload run --manifest-path=../../crates/mdman/Payload.toml -- \
    -t txt -o man/generated_txt man/payload*.md \
    $OPTIONS

payload run --manifest-path=../../crates/mdman/Payload.toml -- \
    -t man -o ../etc/man man/payload*.md \
    $OPTIONS
