# Use the official Rust image as a parent image
FROM rust:latest as builder

WORKDIR /usr/src/my-app

COPY . .

#RUN cargo install sea-orm-cli

#RUN cargo build

#RUN sea-orm-cli migrate


CMD ["cargo", "run", "dev"]


