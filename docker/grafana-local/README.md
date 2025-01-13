# Grafana Local Setup

This repository provides a setup for running Grafana, Prometheus and Tempo locally, either as a single node or as a cluster.

## Single Node (Default)

To run the grafana docker setup for the node, run the following command:

```bash
./run.sh
```

## Cluster

To run the setup for `docker/iota-private-network`, provide the PROMETHEUS_CONFIG_FILE environment variable:
```bash
PROMETHEUS_CONFIG_FILE=prometheus-cluster.yml ./run.sh
```
