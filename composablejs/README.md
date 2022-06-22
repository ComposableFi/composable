# ComposableJS

ComposableJS is our all-in-one JavaScript library for building applications utilizing the Composable Blockchain.

## Index

- [(@composable) Core Package](./README.md)
-
    - [(@composable/types) Composable Types](./packages/types/README.md)
-
    - [(@composable/utils) Composable Utils](./packages/utils/README.md)
-
    - [(@composable/oracle_setup) Oracle Initializer](./packages/oracle_setup/README.md)

## Developer Notes

### Building from source

#### Core Package

To build the core package:

```bash
make build
```

#### All packages independently

If you want to build each package by itself:

```bash
make build-all
```

#### Regenerate & build types

If you only want to regenerate types & compile them:

```bash
make build-types
```

#### Utilities Package

If you only want to compile the utilities package:

```bash
make build-utils
```

#### Oracle Initializer Package

If you only want to compile the oracle setup script:

```bash
make build-oracle_setup
```

#### Starting the Oracle initializer

If you want to directly run the oracle setup script without compiling first:

```bash
make oracle_setup
```

### How to add a new package

To add a new package, move it into the `packages/` folder and make the following changes with the packages you require:

**tsconfig.json:**

```json
{
  "extends": "../../tsconfig.json",
  "...",
  "references": [
    {
      "path": "../types/tsconfig.json"
    },
    {
      "path": "../utils/tsconfig.json"
    }
  ],
  "..."
}
```

**package.json:**

```json
{
  "...",
  "dependencies": {
    "@composable/types": "*",
    "@composable/utils": "*",
    "..."
  }
```
