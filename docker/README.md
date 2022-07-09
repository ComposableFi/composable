# docker-compose setups

For every development process

## Parachain

Work within the codespace and use
`cd ./scripts/polkadot-launch && yarn && yarn composable`

## Frontend

`docker-compose up` should run:

- composable-sandbox.dockerfile. compiled and cached image from docker hub reflecting main
  - Polkadot relaychain (5 nodes)
  - Composable parachian (3 nodes)
- Subsquid. cached version of docker image from docker hub reflecting main

in order to develop, edit contents of `frontend/`, and have a local frontend instance running in the codespace which contains to this docker-compose file

## Integration tests

Same setup as frontend (?)

## QA

`docker-compose up` should run:

- composable-sandbox.dockerfile. compiled and cached image from docker hub reflecting main
  - Polkadot relaychain (5 nodes)
  - Composable parachian (3 nodes)
- Subsquid. cached version of docker image from docker hub reflecting main
- Frontend. cached version of docker image from docker hub reflecting main

