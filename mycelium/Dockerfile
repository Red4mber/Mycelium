
# This is the Dockerfile for Mycelium C2
# 
# We use multistage builds, as preconised by docker's documentation
# This is to keep the container size as small as possible 

################################################################################
#                               BUILDER STAGE
################################################################################

ARG RUST_VERSION=1.80.0
ARG APP_NAME=mycelium-api 

FROM rust:${RUST_VERSION}-alpine AS builder
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

# Build the application.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/mycelium-api
EOF

# FIXME > Broken due to inconsistent linebreaks - Fix: Make a entrypoint.sh
# GOD I HATE WINDOWS - WHY IS CRLF EVEN A THING

################################################################################
#                               RUNNER STAGE
################################################################################

FROM alpine:3.18 AS runner
# Create a non-privileged user that the app will run under.
ARG UID=10001
WORKDIR /mycelium

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=builder /bin/mycelium-api /mycelium/
COPY settings.toml /mycelium/

# Expose the port that the application listens on.
EXPOSE 3000

CMD ["/mycelium/mycelium-api"]
