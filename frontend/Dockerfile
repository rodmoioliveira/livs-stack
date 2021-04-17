FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/frontend
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
COPY --from=build /usr/local/cargo/bin/frontend .
COPY --from=build /usr/src/frontend/static /static
CMD ["./frontend"]
