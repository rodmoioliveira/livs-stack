FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc
COPY --from=build /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD ["backend"]
