######
# Base
FROM rust:1.61.0 AS base

# Install cargo-watch
RUN update-ca-certificates && \
  wget -q https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-unknown-linux-musl.tar.xz && \
  tar -xvf cargo-watch-v8.1.1-x86_64-unknown-linux-musl.tar.xz && \
  mv ./cargo-watch-v8.1.1-x86_64-unknown-linux-musl/cargo-watch /cargo-watch && \
  rm -rf ./cargo-watch-v8.1.1-x86_64-unknown-linux-musl*

ENV BUILD_DIR=/app \
  RUST_BACKTRACE=1
WORKDIR $BUILD_DIR

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src\
  && echo "fn main() {println!(\"dummy main\")}" > src/main.rs
RUN cargo build --release

COPY ./ ./

RUN touch ./src/main.rs

#####
# Dev
FROM base AS dev
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait /wait
RUN chmod +x /wait
CMD ["sh", "-c", "/cargo-watch -w 'src' -- cargo run"]

#########
# Builder
FROM base AS builder
RUN cargo build --release --bin todos_api

######
# Prod
FROM debian:bullseye-slim AS production

RUN useradd -ms /bin/bash app && \
  apt-get update && \
  apt-get install -y --no-install-recommends ca-certificates && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists && \
  update-ca-certificates

WORKDIR /home/app

COPY --from=builder /app/target/release/todos_api app

CMD ["sh", "-c", "chown -R app:app . && runuser -u app ./app"]
