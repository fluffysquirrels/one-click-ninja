FROM ubuntu:20.04

RUN apt-get update -y
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get -y install tzdata
RUN apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev \
                       clang lld curl && \
    apt-get clean -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
