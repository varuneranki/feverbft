#!/bin/bash

while true; do
    docker ps --format '{{json .}}' > /src/peerlist.json
    sleep 5
done
