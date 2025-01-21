FROM rust:1.83.0-slim AS chef

RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /app


FROM chef AS builder

COPY . .
RUN cargo install cargo-binstall --locked
RUN cargo binstall dioxus-cli
ARG API_URL
RUN dx bundle --release --platform web


FROM caddy:2.9.0-beta.3-alpine AS client

RUN ls
RUN mkdir /my-website_logs
COPY --from=builder /app/target/dx/my-website/release/web/public /srv
COPY Caddyfile /etc/caddy/Caddyfile
