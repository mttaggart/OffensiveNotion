FROM rust:latest

RUN apt update -y && apt install mingw-w64 -y && apt install gcc-multilib -y

RUN mkdir /opt/OffensiveNotion
WORKDIR /opt/OffensiveNotion
COPY agent/ .

RUN rustup target add x86_64-pc-windows-gnu && rustup toolchain install stable-x86_64-pc-windows-gnu

# This Dockerfile gets edited dynamically by main.py. If using main.py, don't touch it. If building the Docker container from source, edit this with your target build and OS

RUN cargo build {OS} {RELEASE}
