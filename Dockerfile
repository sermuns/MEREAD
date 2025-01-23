FROM golang:1.23-alpine AS builder
WORKDIR /build
COPY . .
RUN go build -o bin/ -ldflags "-s -w" .

FROM scratch
WORKDIR /app
COPY --from=builder /build/bin/ /bin
ENTRYPOINT ["/bin/MEREAD"]
