FROM rust:latest AS build

WORKDIR /usr/src/backend


COPY . .

RUN cargo build --release --jobs 3

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y libssl3 libpq-dev postgresql-client bash \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/src/backend/target/release/backend /usr/local/bin/backend

COPY entrypoint.sh /usr/local/bin/entrypoint.sh
COPY docker.env .env
RUN chmod +x /usr/local/bin/entrypoint.sh

EXPOSE 80

#ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["backend"]
