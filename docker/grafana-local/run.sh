#!/bin/bash

# Default configuration file
CONFIG_FILE="prometheus.yaml"

# Check if the CLUSTER flag is passed as an argument
if [ "$1" = "--cluster" ]; then
  CONFIG_FILE="prometheus_cluster.yaml"
fi

# Update the docker-compose.yaml file with the correct config file
sed -i "s|./prometheus.yaml:/etc/.*|./${CONFIG_FILE}:/etc/${CONFIG_FILE}|" docker-compose.yaml

cleanup() {
  echo "Stopping services..."
  docker compose down
}

trap cleanup EXIT

# Run docker-compose with the updated configuration file
docker compose up