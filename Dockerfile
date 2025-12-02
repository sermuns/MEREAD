# based on https://github.com/orhun/rustypaste/blob/8329095c7585142a4f9e36e1ab74bbcbbeae73d9/Dockerfile


FROM rust:1.91.0-alpine3.22 AS builder

WORKDIR /app
RUN apk update
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.toml
RUN mkdir -p src/
RUN echo "fn main() {println!(\"failed to build\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/meread*
COPY . .
RUN cargo build --locked --release


FROM scratch

COPY --from=builder /app/target/release/meread /bin/
WORKDIR /app
USER 1000:1000
ENTRYPOINT ["meread"]
