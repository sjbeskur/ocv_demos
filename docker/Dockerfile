FROM ubuntu:16.04
#FROM debian:stretch

ENV USER root
ENV RUST_VERSION=1.63.0

RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  curl \
  git \
  openssh-client \
  libssl-dev \
  pkg-config
RUN curl -sO https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init 
RUN chmod +x rustup-init
RUN ./rustup-init -y --default-toolchain $RUST_VERSION --no-modify-path

ENV PATH $PATH:/root/.cargo/bin

# prepare for arm7 Build
RUN dpkg --add-architecture armhf
RUN apt install -qq gcc-arm-linux-gnueabihf -y

RUN rustup target add armv7-unknown-linux-gnueabihf

COPY /docker/fs/cargo-config /root/.cargo/config

#  DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
#  DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
RUN rm -rf \
  rustup-init \
  /var/lib/apt/lists/* \
  /tmp/* \
  /var/tmp/* && \
  mkdir /source

RUN cargo install cargo-deb

WORKDIR /source
CMD ["/bin/bash"]