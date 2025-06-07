FROM rust:1.87.0

LABEL authors="norris1z"

WORKDIR /bahn

COPY . .

RUN cargo install --path .

ENTRYPOINT [ "./target/release/bahn"]