# simple-metrics-server

A simple HTTP server for testing Flagger, Prometheus etc.

## Usage

This assumes you are using Istio and the Prometheus Operator.

Deploy the components.

```bash
kubectl apply -k deploy/kustomize
```

Once deployed, you can try and bump the image version.

```bash
kubectl set image deployment/simple-metrics-endpoint simple-metrics-endpoint=joedborg/simple-metrics-endpoint:0.1.2 -n test
```
