FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/assets
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
COPY --from=build /usr/local/cargo/bin/assets .
COPY --from=build /usr/src/assets/static /static
CMD ["./assets"]
