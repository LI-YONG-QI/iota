#!/bin/bash

cleanup() {  
  echo "Stopping services..."
  docker compose down
}

trap cleanup EXIT

# Run docker-compose with the updated configuration file
docker compose up