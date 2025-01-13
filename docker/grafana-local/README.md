# Grafana Local Setup

This repository provides a setup for running Grafana, Prometheus and Tempo locally, either as a single node or as a cluster.

## Node (Default)

To run the grafana docker setup for the node, run the following command:

```bash
./run.sh
```

For linux user, update your host address in prometheus.yaml, as 'host.docker.internal' is not supported.
```
scrape_configs:
 - targets: 
```
## Node run with docker
To run the grafana docker setup for the node started with docker, run the following command:

```bash
./run.sh --network-override
```

## Cluster
To run the setup for `docker/iota-private-network`, run:
```bash
 ./run.sh --cluster
```

