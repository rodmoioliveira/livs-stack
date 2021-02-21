FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM alpine:latest as security
RUN apk update && apk add --no-cache ca-certificates tzdata openssh

FROM gcr.io/distroless/cc-debian10
COPY --from=security /etc/ssl/certs /etc/ssl/certs
COPY --from=security /etc/passwd /etc/passwd
COPY --from=security /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=build /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD ["backend"]
