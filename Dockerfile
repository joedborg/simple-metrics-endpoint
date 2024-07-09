FROM rust:1.79 as builder
WORKDIR /usr/src/simple-metrics-endpoint
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/simple-metrics-endpoint /usr/local/bin/simple-metrics-endpoint
RUN apt-get update && apt-get upgrade -y
EXPOSE 8080
# CMD ["sh", "-c", "sleep 10 && simple-metrics-endpoint"]  Enable to fail liveness.
CMD ["simple-metrics-endpoint"]
