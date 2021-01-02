#!/bin/sh

RUST_LOG=info cargo run -- --port 6400 --recipe-dir /home/smores/mega/recipes/ --review-dir /home/smores/mega/reviews/
