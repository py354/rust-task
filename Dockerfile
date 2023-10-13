FROM rust:slim-buster

WORKDIR /app
COPY . /app/

RUN cargo build --release
EXPOSE 8080
ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]