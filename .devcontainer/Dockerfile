FROM rust:1.67 as builder

RUN rustup override set nightly
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cd kernel; cargo build

RUN cd simulator; cargo install --path .

FROM mcr.microsoft.com/devcontainers/cpp:1.0-jammy

RUN sudo apt-get update
RUN sudo apt-get -y install qemu-system
