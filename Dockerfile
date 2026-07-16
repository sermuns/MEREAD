# based on https://github.com/orhun/rustypaste/blob/8329095c7585142a4f9e36e1ab74bbcbbeae73d9/Dockerfile


FROM rust:alpine AS builder

WORKDIR /app
RUN apk update
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --package meread --locked --release


FROM scratch

COPY --from=builder /app/target/release/meread /bin/
WORKDIR /app
USER 1000:1000
ENTRYPOINT ["meread"]
