#!/bin/bash

set -e

payload run -- -t md -o doc/out doc/*.md
payload run -- -t txt -o doc/out doc/*.md
payload run -- -t man -o doc/out doc/*.md
