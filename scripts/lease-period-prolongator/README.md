# lease-period-prolongator

This script allows updating a parachain lease period.

## Build

```bash
yarn build
```

## Running

```bash
yarn start
```

### Environment variables

| Name         | Default             | Notes |
| ------------ | ------------------- | ----- |
| NODE_URL     | ws://localhost:9944 |       |
| PARA_ID      | 2000                |       |
| LEASE_PERIOD | 365                 | days  |


## Docker

### Build a Docker image

```bash
docker build . -t lease-period-prolongator
```

### Running using the Docker image

```bash
docker run --rm -ti -u$(id -u):$(id -g) -eNODE_URL=ws://node-url.example.com:9944 lease-period-prolongator
```
