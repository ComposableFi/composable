### docker compose 

Save following content to docker-compose.yml and run docker-compose up -d

```yaml
version: "3.7"

services:
  composable_node:
    image: composablefi/composable:latest
    container_name: composable_node
    volumes:
      - ./chain-data:/chain-data
    ports:
      - 9833:9833
      - 9844:9844
      - 40333:40333
      - 30333:30333
    restart: unless-stopped
    command: >
      /usr/local/bin/сomposable
      --collator 
      --chain=picasso
      --pruning=archive
      --base-path /chain-data
      --port 30333
      --unsafe-ws-external
      --unsafe-rpc-external
      --listen-addr=/ip4/0.0.0.0/tcp/30334
      --execution wasm
      --
      --execution=wasm 
      --listen-addr=/ip4/0.0.0.0/tcp/30333
```
