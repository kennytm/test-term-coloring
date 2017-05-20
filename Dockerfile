FROM ubuntu:16.04

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl gcc

RUN curl -OL https://github.com/Yelp/dumb-init/releases/download/v1.2.0/dumb-init_1.2.0_amd64.deb && \
    dpkg -i dumb-init_*.deb && \
    rm dumb-init_*.deb

RUN curl -SfLO "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" && \
    chmod u+x rustup-init && \
    ./rustup-init -y --default-toolchain beta

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

