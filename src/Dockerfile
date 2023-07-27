FROM rust:latest as builder

RUN apt-get update && apt-get install ffmpeg libsm6 libxext6  -y
WORKDIR /usr/http_server_fs/src

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/app


COPY --from=builder /usr/src/http_server_fs/target/release/my_rust_program .

EXPOSE 8080

# CMD ["./my_rust_program"]