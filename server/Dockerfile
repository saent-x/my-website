FROM rust:1.83.0-slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS server

WORKDIR /app
COPY --from=builder /app/target/release/server .

EXPOSE 8000
CMD [ "./server" ]