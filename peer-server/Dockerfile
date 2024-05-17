FROM rust:latest

WORKDIR /usr/src/peer-server

COPY . .

RUN cargo install --path .

CMD ["peer-server"]
