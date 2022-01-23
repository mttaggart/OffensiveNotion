FROM rust:latest

RUN mkdir /opt/OffensiveNotion
WORKDIR /opt/OffensiveNotion
COPY . .

RUN cargo build