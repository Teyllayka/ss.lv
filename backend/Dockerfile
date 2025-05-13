FROM rust:latest AS build

WORKDIR /usr/src/backend

COPY . .

RUN cargo build --release --jobs 3

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y \
    curl \
    libssl3 \
    libpq-dev \
    postgresql-client \
    bash \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L https://omnitruck.chef.io/install.sh | bash -s -- -P chef -c stable

COPY --from=build /usr/src/backend/target/release/backend /usr/local/bin/backend
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

EXPOSE 80

CMD ["backend"]
