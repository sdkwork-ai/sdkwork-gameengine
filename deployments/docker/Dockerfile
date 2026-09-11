FROM rust:1-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release -p sdkwork-api-gameengine-standalone-gateway

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /src/target/release/sdkwork-api-gameengine-standalone-gateway /usr/local/bin/sdkwork-api-gameengine-standalone-gateway
ENV SDKWORK_GAMES_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:8095
ENV GAMES_API_BIND=0.0.0.0:8095
EXPOSE 8095
CMD ["sdkwork-api-gameengine-standalone-gateway"]
