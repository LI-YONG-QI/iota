#!/bin/bash

# Load env  variables
source .env

# Check if GRAFANA_DIR is provided in the .env file
if  [ -z "${GRAFANA_DIR}" ]; then
  echo "To run please provide path to the grafana setup as GRAFANA_DIR environment variable"

  exit
fi

# Check if tracing is enabled
if  [ -z "${TRACE_FILTER}" ]; then
  echo "Opentelemetry not enabled, tracing not possible, to enable set TRACE_FILTER env variable"
else
  echo "TRACE_FILTER set to ${TRACE_FILTER}, opentelemetry enabled"
fi

MONITORING_NETWORK=iota-network-node
# Network
if ! docker network ls | grep -q $MONITORING_NETWORK; then
  echo "Creating ${MONITORING_NETWORK} network..."
  docker network create "${MONITORING_NETWORK}"
else
  echo "Monitoring network already exists."
fi

cd "${GRAFANA_DIR}" || exit
# Start grafana or restart if it's already running
#echo "No grafana network found,  starting grafana network in ${GRAFANA_DIR}"
./run.sh --network-override

cd - || exit


echo "Starting the node with monitoring network in a detached mode..."
docker compose -f docker-compose.yaml -f "${GRAFANA_DIR}/docker-compose-node-override.yaml" up -d