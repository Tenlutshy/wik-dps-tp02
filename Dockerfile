FROM rust:slim-bullseye

WORKDIR /app

RUN useradd -r -s /usr/sbin/nologin user
RUN chown user .
RUN chmod 544 .

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY ./src ./src

RUN cargo build --locked --release

USER user

CMD ["./target/release/api-rust"]