FROM rust:1.59.0

WORKDIR /app
RUN apt update && apt install ld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]