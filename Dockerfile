# Builder
FROM rust:1.87 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runner
FROM debian:bookworm
WORKDIR /app

ENV OSVITARIUM_DATABASE_URL=""
ENV OSVITARIUM_SECRET=""
ENV OSVITARIUM_JITSI_KID=""
ENV OSVITARIUM_JITSI_SECRET=""
ENV OSVITARIUM_JITSI_APP_ID=""

COPY --from=builder /app/target/release/osvitarium-backend .

CMD ["./osvitarium-backend"]
