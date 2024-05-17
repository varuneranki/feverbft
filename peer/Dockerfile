FROM rust:latest

WORKDIR /usr/src/peer

COPY . .

RUN cargo install --path .

CMD ["peer"]
