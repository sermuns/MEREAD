# based on https://github.com/jdx/mise/blob/main/packaging/mise/Dockerfile

FROM rust AS builder
LABEL maintainer="sermuns"
LABEL org.opencontainers.image.source=https://github.com/sermuns/meread
LABEL org.opencontainers.image.description="preview github README's locally "
LABEL org.opencontainers.image.licenses=WTFPL

WORKDIR /usr/src/meread
COPY . /usr/src/meread/

RUN cargo build --release

FROM scratch
COPY --from=builder /usr/src/mise/target/release/meread /bin/meread
ENTRYPOINT ["meread"]
