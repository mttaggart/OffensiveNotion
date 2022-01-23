FROM rust:latest

RUN mkdir /opt/OffensiveNotion
WORKDIR /opt/OffensiveNotion
COPY agent/ .

RUN cargo build