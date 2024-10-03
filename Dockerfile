FROM rust:1.81-alpine3.19 AS build
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev protobuf-dev
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/point-salad

FROM alpine:3.19 AS runtime
RUN adduser -S user
RUN apk add --no-cache libgcc
COPY --from=build --chown=user /app/target/release/point-salad .
USER user
ENTRYPOINT ["/point-salad"]