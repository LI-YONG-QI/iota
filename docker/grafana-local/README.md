# Grafana Local Setup

This repository provides a setup for running Grafana, Prometheus and Tempo locally, either as a single node or as a cluster.

## Single Node (Default)

By default, it runs as a single node:

```bash
./run.sh
```

## Cluster

To run the setup for `docker/iota-private-network`:

```bash
./run.sh --cluster
```