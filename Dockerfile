# based on https://github.com/jdx/mise/blob/main/packaging/mise/Dockerfile

ARG APP_NAME=meread

FROM rust AS builder
ARG APP_NAME

LABEL maintainer="sermuns"
LABEL org.opencontainers.image.source=https://github.com/sermuns/${APP_NAME}}
LABEL org.opencontainers.image.description="preview github README's locally"
LABEL org.opencontainers.image.licenses=WTFPL

WORKDIR /work
COPY . /work/

RUN cargo build --release

FROM scratch
ARG APP_NAME

COPY --from=builder /work/target/release/${APP_NAME} /bin/${APP_NAME}
ENTRYPOINT ${APP_NAME}
