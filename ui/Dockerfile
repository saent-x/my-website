# Multi-Stage Builds

# 1st Stage for building and project
# get caddy image then copy the dist folder into the caddy path ???
FROM rust:1.83.0-slim AS chef
# install some deps
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# install cargo-chef to help build project dependencies for optimal Docker layer caching
RUN cargo install cargo-chef
# set workdir
WORKDIR /app


FROM chef AS planner
# copy all files to container
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# now we use cargo chef to load cached dependencies and preform the build
FROM planner AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# -y disables interactive mode, --force forces a crate to be installed even if it already exists, --root installs binary with a custom cargo root
RUN cargo install dioxus-cli
# create ui bundle
RUN dx bundle --platform web

# final stage
FROM caddy:2.9.0-beta.3-alpine AS server

COPY --from=builder /app/target/dx/my-website/release/web/public /srv
COPY Caddyfile /etc/caddy/Caddyfile

EXPOSE 8080
