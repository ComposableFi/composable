# Squid template project

**Please read [help.md](./help.md) for a composable specific guide**

A starter [Squid](https://subsquid.io) project to demonstrate its structure and conventions.
It accumulates [kusama](https://kusama.network) account balances and serves them via GraphQL API. For a full reference of Subsquid features consult [Docs](https://docs.subsquid.io) and [FAQ](./FAQ.md).

## Summary

- [Squid template project](#squid-template-project)
  - [Summary](#summary)
  - [Prerequisites](#prerequisites)
  - [Quickly running the sample](#quickly-running-the-sample)
  - [Setup for parachains](#setup-for-parachains)
  - [Setup for devnets and testnets](#setup-for-devnets-and-testnets)
  - [Dev flow](#dev-flow)
    - [1. Define database schema](#1-define-database-schema)
      - [Entity Requirements](#entity-requirements)
    - [2. Generate TypeORM classes](#2-generate-typeorm-classes)
    - [3. Generate database migration](#3-generate-database-migration)
    - [4. Generate TypeScript definitions for substrate events and calls](#4-generate-typescript-definitions-for-substrate-events-and-calls)
  - [Deploy the Squid](#deploy-the-squid)
  - [Project conventions](#project-conventions)
  - [Types bundle](#types-bundle)
  - [Differences from polkadot.js](#differences-from-polkadotjs)
  - [Graphql server extensions](#graphql-server-extensions)
  - [Disclaimer](#disclaimer)

## Prerequisites

* [Node 16.x](https://nodejs.org/)
* [Docker](https://docs.docker.com/get-docker/)
* [GNU Make](https://www.gnu.org/software/make/)

## Quickly running the sample

```bash
# 1. Install dependencies
npm ci

# 2. Compile typescript files
npm run build

# 3. Launch postgres and detach
make up

# 4. Create the database schema and run the processor
make process

# 5. Start the GraphQL server
#    Open a new terminal and run:
make serve
```

The GraphQL playground is available at http://localhost:4350/graphql. Open it in a browser and run sample queries by applying filters and data selections in the panel to the left.

[comment]: <> (## Setup for parachains)

[comment]: <> (Subsquid provides Squid Archive data sources for most parachains. Use `lookupArchive&#40;<network name>&#41;` to lookup the archive endpoint by the network name, e.g.)

[comment]: <> (```typescript)

[comment]: <> (processor.setDataSource&#40;{)

[comment]: <> (  archive: lookupArchive&#40;"basilisk"&#41;[0].url,)

[comment]: <> (  //...)

[comment]: <> (}&#41;;)

[comment]: <> (```)

[comment]: <> (To make sure you're indexing the right chain one can additionally filter by genesis hash:)

[comment]: <> (```typescript)

[comment]: <> (processor.setDataSource&#40;{)

[comment]: <> (  archive: lookupArchive&#40;"basilisk", undefined, "0xa85cfb9b9fd4d622a5b28289a02347af987d8f73fa3108450e2b4a11c1ce5755"&#41;[0].url,)

[comment]: <> (  //...)

[comment]: <> (}&#41;;)

[comment]: <> (```)

[comment]: <> (If the chain is not yet supported, please fill the [form]&#40;https://forms.gle/Vhr3exPs4HrF4Zt36&#41; to submit a request.)

[comment]: <> (## Setup for devnets and testnets)

[comment]: <> (Non-production chains, e.g. Devnets and Testnets are not supported by `lookupArchive` and one has to provide a local Squid Archive as a data source.)

[comment]: <> (Inspect `archive/.env` and provide the websocket endpoint for your node. If the network requires custom type bundles &#40;for older versions of Substrate&#41;, mount them as volumes in `archive/docker-compose.yml` and uncomment the relevant sections in `archive/.env`.)

[comment]: <> (Then run &#40;in a separate terminal window&#41;)

[comment]: <> (```bash)

[comment]: <> (docker compose -f archive/docker-compose.yml up)

[comment]: <> (```)

[comment]: <> (Inspect your archive at `http://localhost:4010/console`. Run the processor with)

[comment]: <> (```typescript)

[comment]: <> (processor.setDataSource&#40;{)

[comment]: <> (  archive: `http://localhost:4010/v1/graphql`,)

[comment]: <> (  chain: // your network endpoint here)

[comment]: <> (}&#41;;)

[comment]: <> (```)

[comment]: <> (To drop the archive, run)

[comment]: <> (```bash)

[comment]: <> (docker compose -f archive/docker-compose.yml down -v)

[comment]: <> (```)


## Dev flow

### 1. Define database schema

Start development by defining the schema of the target database via `schema.graphql`.
Schema definition consists of regular graphql type declarations annotated with custom directives.
Full description of `schema.graphql` dialect is available [here](https://docs.subsquid.io/develop-a-squid/schema-file/).

#### Entity Requirements

Entities *must* include the following fields:
- id: Natural key (for example, `ctx.event.id` for `Event` or `accountId` for `Account`) or random UUID. 
- eventId: ID of the event that triggered the creation/update (can be obtained from `ctx.event.id`)

Optionally, they can also include any other id that can be associated with the on-chain entity. When possible, this must come directly from the chain events (like `offerId` for bond offers). Otherwise, it can be created as a unique concatenation of available data separated by a dash (`-`), like `${accountId}-${assetId}` for vesting schedules.

### 2. Generate TypeORM classes

Mapping developers use TypeORM [EntityManager](https://typeorm.io/#/working-with-entity-manager)
to interact with target database during data processing. All necessary entity classes are
generated by the squid framework from `schema.graphql`. This is done by running:

```bash
# Connect to database, analyze its state and generate migration to match the target schema.
# The target schema is derived from entity classes generated earlier.
# Don't forget to compile your entity classes beforehand!
npx squid-typeorm-migration generate

# Create template file for custom database changes
npx squid-typeorm-migration create

# Apply database migrations from `db/migrations`
npx squid-typeorm-migration apply

# Revert the last performed migration
npx squid-typeorm-migration revert 
```

In most cases the simplest way to update the schema is to drop the database and regenerate the migrations from scratch.

1. Update `schema.graphql`
2. Regenerate the model classes and build the squid with `make rebuild`


### 3. Generate TypeScript definitions for substrate events and calls

This is an optional part, but it is very advisable. 

Event, call and runtime storage data comes to mapping handlers as a raw untyped json. 
While it is possible to work with raw untyped json data, it's extremely error-prone and moreover the json structure may change over time due to runtime upgrades.

Squid framework provides tools for generation of type-safe, spec version aware wrappers around events, calls and runtime storage items. Typegen generates type-safe classes in `types/events.ts`, `types/calls.ts` and `types/storage.ts` respectively, with constructors taking `XXXContext` interfaces as the only argument. All historical runtime upgrades are accounted out of the box. A typical usage is as follows (see `src/processor.ts`):

```typescript
function getTransferEvent(ctx: EventHandlerContext): TransferEvent {
  // instantiate the autogenerated type-safe class for Balances.Transfer event
  const event = new BalancesTransferEvent(ctx);
  // for each runtime version, reduce the data to the common interface
  if (event.isV1020) {
    const [from, to, amount] = event.asV1020;
    return { from, to, amount };
  } else if (event.isV1050) {
    const [from, to, amount] = event.asV1050;
    return { from, to, amount };
  } else {
    const { from, to, amount } = event.asV9130;
    return { from, to, amount };
  }
}
``` 

Generation of type-safe wrappers for events, calls and storage items is currently a two-step process.

First, you need to explore the chain to find blocks which introduce new spec version and
fetch corresponding metadata. 

```bash
npx squid-substrate-metadata-explorer \
  --chain wss://dali.devnets.composablefinance.ninja/parachain/alice \
  --archive http://localhost:4010/v1/graphql \
  --out daliDevVersions.jsonl
```

In the above command `--archive` parameter is optional, but it speeds up the process
significantly. From scratch exploration of kusama network without archive takes 20-30 minutes.

You can pass the result of previous exploration to `--out` parameter. In that case exploration will
start from the last known block and thus will take much less time.

After chain exploration is complete you can use `squid-substrate-typegen(1)` to generate 
required wrappers.

```bash
npx @subsquid/substrate-typegen typegen.json
```

Where `typegen.json` config file has the following structure:

```json5
{
  "outDir": "src/types",
  "chainVersions": "daliDevVersions.jsonl",
  "specVersions": "daliDevVersions.jsonl",
  "typesBundle": "kusama",
  "events": [
    "Balances.Transfer",
    "Pablo.PoolCreated",
    "Pablo.PoolDeleted",
    "Pablo.LiquidityAdded",
    "Pablo.LiquidityRemoved",
    "Pablo.Swapped",
    "BondedFinance.NewOffer",
  ],
  "calls": [ // list of calls to generate
    "timestamp.set"
  ],
  "storage": [
    "System.Account" // list of storage items. To generate wrappers for all storage items, set "storage": true
  ]
}
```

## Deploy the Squid

Subsquid offers a free hosted service for deploying your Squid. First, build and run the docker image locally and fix any error or missing files in Dockerfile:

```sh
bash scripts/docker-run.sh # optionally specify DB port as an argument
```

After the local run, follow the [instructions](https://docs.subsquid.io/recipes/deploying-a-squid) for obtaining a deployment key and submitting the Squid to [Aquarium](https://app.subsquid.io).


## Project conventions

Squid tools assume a certain project layout.

* All compiled js files must reside in `lib` and all TypeScript sources in `src`. 
The layout of `lib` must reflect `src`.
* All TypeORM classes must be exported by `src/model/index.ts` (`lib/model` module).
* Database schema must be defined in `schema.graphql`.
* Database migrations must reside in `db/migrations` and must be plain js files.
* `sqd(1)` and `squid-*(1)` executables consult `.env` file for a number of environment variables.

## Types bundle

Substrate chains which have blocks with metadata versions below 14 don't provide enough 
information to decode their data. For those chains external 
[type definitions](https://polkadot.js.org/docs/api/start/types.extend) are required.

Type definitions (`typesBundle`) can be given to squid tools in two forms:

1. as a name of a known chain (currently only `kusama`)
2. as a json file of a structure described below.

```json5
{
  "types": {
    "AccountId": "[u8; 32]"
  },
  "typesAlias": {
    "assets": {
      "Balance": "u64"
    }
  },
  "versions": [
    {
      "minmax": [0, 1000], // block range with inclusive boundaries
      "types": {
        "AccountId": "[u8; 16]"
      },
      "typesAlias": {
        "assets": {
          "Balance": "u32"
        }
      }
    }
  ]
}
```

* `.types` - scale type definitions similar to [polkadot.js types](https://polkadot.js.org/docs/api/start/types.extend#extension)
* `.typesAlias` - similar to [polkadot.js type aliases](https://polkadot.js.org/docs/api/start/types.extend#type-clashes)
* `.versions` - per-block range overrides/patches for above fields.

All fields in types bundle are optional and applied on top of a fixed set of well known
frame types.

## Differences from polkadot.js

Polkadot.js provides lots of [specialized classes](https://polkadot.js.org/docs/api/start/types.basics) for various types of data. 
Even primitives like `u32` are exposed through special classes.
In contrast, squid framework works only with plain js primitives and objects.
This allows to decrease coupling and also simply dictated by the fact, that
there is not enough information in substrate metadata to distinguish between 
interesting cases.

Account addresses is one example where such difference shows up.
From substrate metadata (and squid framework) point of view account address is simply a fixed length
sequence of bytes. On other hand, polkadot.js creates special wrapper for account addresses which 
aware not only of address value, but also of its 
[ss58](https://docs.substrate.io/v3/advanced/ss58/) formatting rules.
Mapping developers should handle such cases themselves.

## Graphql server extensions

It is possible to extend `squid-graphql-server(1)` with custom
[type-graphql](https://typegraphql.com) resolvers and to add request validation.
More details will be added later.

## Disclaimer

This is alpha-quality software. Expect some bugs and incompatible changes in coming weeks.
