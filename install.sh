#!/bin/sh

root() {
    sudo -V > /dev/null && sudo $@ || doas $@
}

cargo build --release

strip ./target/release/gallium-prompt 2> /dev/null || (rm -rf ./target && cargo build --release && strip ./target/release/gallium-prompt)

echo "\n\nRequiring superuser permissions to move executable to /usr/bin/"

root mv target/release/gallium-prompt /usr/bin/gallium && echo "\nDone\n" || echo "\nFailed to move executable to /usr/bin/\n"
