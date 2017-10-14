#!/bin/bash

set -ex

SRC=$(pwd)

cp config.toml ~/rust/
cd ~/rust
ls -a

if [ ! -d ./dot-git ]; then
    cd ..
    git clone https://github.com/rust-lang/rust.git
    cd rust
else
    mv dot-git .git
    git fetch origin master
    git reset --hard origin/master
fi
git remote add self https://github.com/kennytm/rust.git || true
git fetch self travis-color-color-conflict
git reset --hard self/travis-color-color-conflict
git submodule update --recursive --init
mv .git dot-git

export PATH=$PATH:/home/travis/cmake-3.9.4-Linux-x86_64/bin
cmake /V

./x.py build --stage 0 src/librustc -j31
