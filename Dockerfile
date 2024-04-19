FROM rust:1.77.2-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release
RUN strip target/release/markdown_catalogue_server

FROM debian:buster-slim

COPY --from=builder /app/target/release/markdown_catalogue_server /usr/local/bin/markdown_catalogue_server
EXPOSE 8080
VOLUME [ "/collection" ]

CMD ["markdown_catalogue_server"]