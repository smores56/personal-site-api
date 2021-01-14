#!/bin/sh

cargo build --release && scp target/release/mohr-codes-api root@dev.mohr.codes:/root/.local/bin/
