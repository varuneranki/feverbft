#use the main official rust docker image as our builder
FROM rust:latest as builder


#set the work directory
WORKDIR /usr/src/peerb

#copy the app into the docker image
COPY . .

RUN cargo install --path .

#CMD ["peer"]

#FROM alpine:latest
#COPY --from=builder /usr/src/peer/target/debug/peer /usr/src/

#COPY --from=builder /usr/local/cargo/target/debug/peer /usr/local/target/debug/peer
#WORKDIR /usr/src

FROM rust:slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/peerb /usr/local/bin/peerb

CMD ["peerb"]
