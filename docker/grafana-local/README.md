# Grafana Local Setup

This repository provides a setup for running Grafana, Prometheus and Tempo locally, either as a single node or as a cluster.

## Single Node (Default)

By default, it runs as a single node:

```bash
./run.sh
```

## Cluster

To run the setup for `docker/iota-private-network`, make an `.env` file with the following content:

```bash
PROMETHEUS_CONFIG_FILE=prometheus-cluster.yml
```

Then run the setup:

```bash
./run.sh
```