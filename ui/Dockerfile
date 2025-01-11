FROM rust:1.83.0 AS chef

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef


FROM chef AS planner

WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM planner AS builder

COPY --from=planner /app/recipe.json recipe.json
COPY . .

RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo install dioxus-cli
RUN dx --version
RUN dx bundle --release --platform web


# final stage
FROM caddy:2.9.0-beta.3-alpine AS client

COPY --from=builder /app/target/dx/my-website/release/web/public /srv
COPY --from=builder /app /app

COPY Caddyfile /etc/caddy/Caddyfile

# EXPOSE 8080


# ---
#
