#!/bin/bash

# shut down running services
if docker compose ps | grep -q "Up"; then
  echo "Grafana Docker Compose already running, restarting..."

  docker compose down
fi

if [[ "$1" == "--cluster" ]]; then
  PROMETHEUS_CONFIG_FILE="prometheus-cluster.yml"
  echo "Running for cluster in a detached mode"
  docker compose up -d -f docker-compose.yaml -f docker-compose-grafana-cluster-override.yaml
# adds monitoring network to work with node started with docker
elif [[ "$1" == "--network-override" ]]; then
  echo "Running grafana for a fullnode in a detached mode"
  PROMETHEUS_CONFIG_FILE="prometheus.yaml"
  docker compose -f docker-compose.yaml -f docker-compose-grafana-node-override.yaml up -d
else
  echo "Running in a detached mode"
  PROMETHEUS_CONFIG_FILE="prometheus.yaml"
  docker compose up -d
fi

echo "Use 'docker compose down' to stop services"

