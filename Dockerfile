FROM rust:1.83 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim as runtime

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/actions-telegram-notifier /actions-telegram-notifier

ENTRYPOINT ["/actions-telegram-notifier"]
