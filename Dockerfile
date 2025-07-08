# based on https://github.com/jdx/mise/blob/main/packaging/mise/Dockerfile and https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev gc
RUN rustup target add x86_64-unknown-linux-musl

LABEL maintainer="sermuns"
LABEL org.opencontainers.image.source=https://github.com/sermuns/meread
LABEL org.opencontainers.image.description="preview github README's locally "
LABEL org.opencontainers.image.licenses=WTFPL

WORKDIR /build

# fake layer, just for caching
COPY Cargo.toml Cargo.lock .
RUN mkdir src/ && \
    echo 'fn main() {println!("Dummy build for caching dependencies")}' > src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/meread*


COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/meread /bin/
ENTRYPOINT ["meread"]
