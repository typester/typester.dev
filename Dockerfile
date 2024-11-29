FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:latest AS builder-base

# Determine the appropriate rust-musl-cross tag
ARG TARGETPLATFORM
RUN case "${TARGETPLATFORM}" in \
        "linux/amd64") export CROSS_TARGET="x86_64-musl" ;; \
        "linux/arm64") export CROSS_TARGET="aarch64-musl" ;; \
        *) echo "Unsupported TARGETPLATFORM: ${TARGETPLATFORM}" && exit 1 ;; \
    esac && \
    echo "Using CROSS_TARGET=${CROSS_TARGET}"

FROM messense/rust-musl-cross:${CROSS_TARGET} AS builder

COPY . .
RUN cargo build --release --target ${RUST_MUSL_CROSS_TARGET}

# Runtime
FROM alpine

WORKDIR /app

# Timezone PST
RUN apk add --no-cache tzdata
ENV TZ=America/Los_Angeles

# Copy application binary from builder image
COPY --from=builder /home/rust/src/target/*/release/blog .
# Copy static files
COPY ./public /app/public

ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/blog"]
