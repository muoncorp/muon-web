#!/bin/sh

#rustup target add x86_64-unknown-linux-gnu --toolchain nightly
cross build --release --target x86_64-unknown-linux-gnu --verbose