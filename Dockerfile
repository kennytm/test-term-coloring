FROM ubuntu:16.04

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl gcc

RUN curl -vfO https://github.com/Yelp/dumb-init/releases/download/v1.2.0/dumb-init_1.2.0_amd64.deb

RUN curl -vfO "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"

RUN curl -vfO "https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl"


