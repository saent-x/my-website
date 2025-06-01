FROM rust:1.83.0-slim-bullseye AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS server

WORKDIR /app
COPY --from=builder /app/target/release/server .
COPY my-website-info.toml .

EXPOSE 8000
CMD [ "./server" ]