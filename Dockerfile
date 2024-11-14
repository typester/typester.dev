FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:x86_64-musl AS builder

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
