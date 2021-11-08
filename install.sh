#!/bin/sh

cargo build --release

strip target/release/gallium-prompt

echo "Requiring super user to move executable to /usr/bin/"

sudo mv target/release/gallium-prompt /usr/bin/gallium
