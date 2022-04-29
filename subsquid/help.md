# Composable Data Indexer

## Running the Archiver + Indexer with GraphQL UI

- Archiver
    ```
    docker-compose -f archive/docker-compose.yml up
    ```
- Indexer DB
    ```
    docker compose up -d
    npx sqd db create
    npx sqd db migrate
    ```
- Indexer
  ```
  npm run build && node -r dotenv/config lib/processor.js
  ```
- Run the graphql server
  ```
  npx squid-graphql-server
  ```

## Updating Types

This is used for example to add support for new events

- Run the meta data explorer
  ```
   npx squid-substrate-metadata-explorer \ 
  --chain wss://dali.devnets.composablefinance.ninja/parachain/alice \
  --archive http://localhost:4010/v1/graphql \
  --out daliDevVersions.json
  ```
- Run the type generator
  ```
  npx squid-substrate-typegen typegen.json 
  ```
- Reset db (local)
  ```
  ./scripts/reset-db.sh
  ```
