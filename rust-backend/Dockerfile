FROM rust:1 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

FROM debian:latest AS runtime
RUN apt-get update && apt-get install -y openssl libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
#COPY /*.yaml /app
COPY --from=builder /app/target/release/shelly-actix-api /app
EXPOSE 3000
CMD ["/app/shelly-actix-api"]
