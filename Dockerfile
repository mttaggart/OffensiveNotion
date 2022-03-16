FROM rust:latest AS rustbuilder

# Do the Rust setup, but do it just the once and separate the ON stuff

RUN echo "Installing dependencies"
RUN apt update
RUN apt install -y \ 
    mingw-w64 \ 
    gcc-multilib \ 
    python3-pip
RUN rustup update stable
RUN rustup target add x86_64-pc-windows-gnu


# Now we get to work
# FROM ubuntu:latest as onbuilder

RUN mkdir /OffensiveNotion
RUN mkdir /out
COPY ./ /OffensiveNotion
WORKDIR /OffensiveNotion


# ENV SLEEP=$SLEEP
# ENV JITTER=$JITTER
# ENV PARENT_PAGE_ID=$PARENT_PAGE_ID
# ENV LOG_LEVEL=$LOG_LEVEL
# ENV API_KEY=$API_KEY

# SED all variables into the source code before build
# RUN sed -i "s/<<SLEEP>>/$SLEEP/g" src/config.rs
# RUN sed -i "s/<<JITTER>>/$JITTER/g" src/config.rs
# RUN sed -i "s/<<PARENT_PAGE_ID>>/$PARENT_PAGE_ID/g" src/config.rs
# RUN sed -i "s/<<LOG_LEVEL>>/$LOG_LEVEL/g" src/config.rs
# RUN sed -i "s/<<API_KEY>>/$API_KEY/g" src/config.rs
# RUN cat src/config.rs

#RUN sed -i 's/<<JITTER>>/'"JITTER"'/g' src/config.rs
#RUN sed -i 's/<<PARENT_PAGE_ID>>/'"PARENT_PAGE_ID"'/g' src/config.rs
#RUN sed -i 's/<<LOG_LEVEL>>/'"LOG_LEVEL"'/g' src/config.rs
#RUN sed -i 's/<<API_KEY>>/'"$API_KEY"'/g' src/config.rs

# This Dockerfile gets edited dynamically by main.py. If using main.py, don't touch it. If building the Docker container from source, edit this with your target build and OS
# Litcrypt key as env var
# ENV LITCRYPT_ENCRYPT_KEY=$LITCRYPT_KEY

RUN pip3 install -r requirements.txt
ENTRYPOINT ["/usr/bin/python3", "main.py"]


# ENTRYPOINT cargo build {OS} {RELEASE} --out-dir /out
