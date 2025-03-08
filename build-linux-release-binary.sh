#!/bin/bash

# Build release binary in Docker container

IMAGE=rust:1.85.0

docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp $IMAGE cargo build --release
