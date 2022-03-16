FROM rust:latest

RUN apt update -y && apt install mingw-w64 -y && apt install gcc-multilib -y

RUN mkdir /opt/OffensiveNotion
WORKDIR /opt/OffensiveNotion
COPY agent/ .

RUN rustup target add x86_64-pc-windows-gnu && rustup toolchain install stable-x86_64-pc-windows-gnu

ENV SLEEP=$SLEEP
ENV JITTER=$JITTER
ENV PARENT_PAGE_ID=$PARENT_PAGE_ID
ENV LOG_LEVEL=$LOG_LEVEL
ENV API_KEY=$API_KEY

# SED all variables into the source code before build
RUN sed -i "s/<<SLEEP>>/$SLEEP/g" src/config.rs
RUN cat src/config.rs

#RUN sed -i 's/<<JITTER>>/'"JITTER"'/g' src/config.rs
#RUN sed -i 's/<<PARENT_PAGE_ID>>/'"PARENT_PAGE_ID"'/g' src/config.rs
#RUN sed -i 's/<<LOG_LEVEL>>/'"LOG_LEVEL"'/g' src/config.rs
#RUN sed -i 's/<<API_KEY>>/'"$API_KEY"'/g' src/config.rs

# This Dockerfile gets edited dynamically by main.py. If using main.py, don't touch it. If building the Docker container from source, edit this with your target build and OS
# Litcrypt key as env var
ENV LITCRYPT_ENCRYPT_KEY=$LITCRYPT_KEY

RUN cargo build {OS} {RELEASE}
