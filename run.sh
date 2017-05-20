#!/bin/bash

export PATH=$HOME/.cargo/bin:$HOME/.local/bin:$PATH

cd /x
cargo clean --color always
cargo run --color always

export TERM=xterm
cargo clean --color always
cargo run --color always

