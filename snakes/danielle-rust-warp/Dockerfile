FROM rust:1.70-bookworm as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bookworm
COPY --from=builder /usr/local/cargo/bin/battlesnake /usr/local/bin/battlesnake
EXPOSE 8080
CMD /usr/local/bin/battlesnake
