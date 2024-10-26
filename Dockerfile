FROM denoland/deno:latest AS builder

WORKDIR /app

COPY . .

RUN deno install

# Compile the Deno app to a single binary
RUN deno compile --allow-read --allow-net --output /app/output/mcs src/main.ts

FROM debian:buster-slim

COPY --from=builder /app/output/mcs /usr/local/bin/mcs
EXPOSE 8080
VOLUME [ "/collection" ]

CMD ["mcs"]