#!/bin/sh

cargo build --release && scp target/release/personal-site-api root@cloud.sam-mohr.com:/root/.local/bin/
