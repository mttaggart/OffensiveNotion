FROM rust:latest

RUN apt update -y && apt install mingw-w64 -y && apt install gcc-multilib -y

RUN mkdir /opt/OffensiveNotion
WORKDIR /opt/OffensiveNotion
COPY agent/ .

RUN rustup target add x86_64-pc-windows-gnu && rustup toolchain install stable-x86_64-pc-windows-gnu

# Set env vars
ENV API_KEY=$API_KEY

# SED all variables into the source code before build
WORKDIR agent/
#RUN $sed 's/<<API_KEY>>/$API_KEY/' src/config.rs

# This Dockerfile gets edited dynamically by main.py. If using main.py, don't touch it. If building the Docker container from source, edit this with your target build and OS
# Litcrypt key as env var
ENV LITCRYPT_ENCRYPT_KEY=$LITCRYPT_KEY

RUN cargo build {OS} {RELEASE}
