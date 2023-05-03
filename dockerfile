FROM rust:latest as build
WORKDIR /usr/src/tofuri-bot
COPY . .
RUN cargo build --bin tofuri-bot --release
FROM debian:stable-slim
COPY --from=build /usr/src/tofuri-bot/target/release/tofuri-bot /usr/local/bin/
EXPOSE 2023
ENTRYPOINT ["tofuri-bot"]