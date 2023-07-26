# just to test the build in a recent debian environment
FROM rust:1.70.0-bookworm

RUN apt-get update -y
RUN apt-get install -y build-essential meson ninja-build clang libclang-dev libfribidi-dev libthai-dev libpcre2-dev

COPY /vendor /pango2-alpha/vendor

WORKDIR /pango2-alpha

RUN cd vendor/ && make clean && make install

# RUN cargo build
