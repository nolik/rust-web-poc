ARG BASE_IMAGE=ekidd/rust-musl-builder

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
COPY --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Build final image.
FROM alpine
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-web-poc \
    /usr/local/bin/
EXPOSE 8090
CMD /usr/local/bin/rust-web-poc
