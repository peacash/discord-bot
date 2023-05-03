FROM rust:latest as builder
WORKDIR /a
COPY . .
RUN cargo build --bin tofuri-bot --release
FROM debian:stable-slim
COPY --from=builder /a/target/release/tofuri-bot /usr/local/bin/
EXPOSE 2023
ENTRYPOINT ["tofuri-bot"]