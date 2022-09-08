# Frontend Monorepo
This project includes all UI and blockchain integration for Picasso and Pablo.

## What's inside?

This monorepo uses turborepo with [Yarn](https://classic.yarnpkg.com/lang/en/) as a package manager.

### Apps and Packages

- `pablo`: a [Next.js](https://nextjs.org) app
- `picasso`: another [Next.js](https://nextjs.org) app
- `shared`: a utility library that is used by both pablo and picasso applications
- `defi-interfaces`: a collection of types extracted from polkadot chain and defs to use in packages/apps
- `eslint-config-custom`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `tsconfig`: `tsconfig.json`s used throughout the monorepo
- `substrate-react`: a collection of React utilities to be used across Pablo and Picasso

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Utilities

This turborepo has some additional tools already setup for you:

- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

## Setup
`yarn install` is going to be all you need. This project has been setup with yarn v1.
### Build

```
cd frontend
yarn run build
```

### Develop

To develop all apps and packages, run the following command:

```
cd frontend
yarn run dev
```

To develop individual packages, you can use turborepo's filter command:

```bash
yarn --filter=picasso dev
```

### Remote Caching

Turborepo can use a technique known as [Remote Caching](https://turborepo.org/docs/core-concepts/remote-caching) to share cache artifacts across machines, enabling you to share build caches with your team and CI/CD pipelines.

By default, Turborepo will cache locally. To enable Remote Caching you will need an account with Vercel. If you don't have an account you can [create one](https://vercel.com/signup), then enter the following commands:

```
cd my-turborepo
npx turbo login
```

This will authenticate the Turborepo CLI with your [Vercel account](https://vercel.com/docs/concepts/personal-accounts/overview).

Next, you can link your Turborepo to your Remote Cache by running the following command from the root of your turborepo:

```
npx turbo link
```

## Useful Links

Learn more about the power of Turborepo:

- [Pipelines](https://turborepo.org/docs/core-concepts/pipelines)
- [Caching](https://turborepo.org/docs/core-concepts/caching)
- [Remote Caching](https://turborepo.org/docs/core-concepts/remote-caching)
- [Scoped Tasks](https://turborepo.org/docs/core-concepts/scopes)
- [Configuration Options](https://turborepo.org/docs/reference/configuration)
- [CLI Usage](https://turborepo.org/docs/reference/command-line-reference)
