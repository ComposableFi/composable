# Composable Data Indexer

## Running the Archiver + Indexer with GraphQL UI

- Archiver
    ```
    docker-compose -f archive/docker-compose.yml up
    ```
- Indexer DB
    ```
    docker compose up -d
    npx squid-typeorm-migration create
    npx squid-typeorm-migration migrate
    ```
- Indexer
  ```
  npm run build && node -r dotenv/config lib/processor.js
  ```
- Run the graphql server
  ```
  npx @subsquid/graphql-server
  ```

## Updating Schema and Adding Migrations

This is the normal development process when trying to support new data types in the db.

- Update the `schema.graphql` file with the necessary entities.
- Run `npx squid-typeorm-codegen` to generate the database entities for the schema.
- Run `npm run build` to compile new files.
- Run `npx squid-typeorm-migration create` to generate a new migration file for the new or updated entities.
- If you would like to reset the db and regenerate the tables run `./scripts/reset-db.sh`
- Now to process the archived and new events run `npm run build && node -r dotenv/config lib/processor.js`

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
