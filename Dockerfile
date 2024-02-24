
FROM rust:latest

WORKDIR /usr/src/api

COPY Cargo.toml ./

#RUN cargo build

COPY src ./src

RUN cargo build --release

#EXPOSE 8080

CMD ["./target/release/api"]
