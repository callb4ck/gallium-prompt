#!/bin/sh

cargo build --release

strip ./target/release/gallium-prompt 2> /dev/null || (rm -rf ./target && cargo build --release && strip ./target/release/gallium-prompt)

mkdir build 2> /dev/null
rm build/gallium 2> /dev/null

mv target/release/gallium-prompt ./build/gallium && echo "\nDone\nYou'll find the executable in the 'build' dir\n"
