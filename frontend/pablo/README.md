# UI-Template

# Description

This template is going to be used on future projects initiated within the company. 

The purpose of this project is to streamline and remove the time and hassle needed to create new projects, as well as provide a minimum standard of features and guidelines when developing projects. 

By creating a template, we are going to make it easy for development just by cloning this repository. The plan is to be able to quickly bootstrap a new project and spend time on the logic itself instead of configuration.

# Scope

- What is in scope and out of scope?
- Are there some initial project [requirements](https://thedigitalprojectmanager.com/requirements-gathering-guide/) that are already defined?
- What are the project boundaries that the team shall not cross?

# Breakdown

Under the hood, there are multiple components configured to work together. 

- [Next.js](https://nextjs.org/)
- [Typescript](https://www.typescriptlang.org/)
- [MUI](https://mui.com/)
- [Jest](https://jestjs.io/)
- [Linter](https://eslint.org/)
- [Prettier](https://prettier.io/)
- [SVG inline import](https://github.com/gregberge/svgr)
- [PWA](https://github.com/shadowwalker/next-pwa)
- [Redux](https://redux.js.org/)

# Maintenance

To start developing, run the following command:

```bash
$ cp .env.example .env.local
```

Edit `.env.local` with the valid credentials.

Then, run the following command:

```bash
$ yarn start
```

# Sync

To sync the cloned repository with the remote repository, run the following command:
```bash
$ yarn sync
```

# Development

## Quickstart

To create a next.js project using this template, run the following command:

```bash
$ yarn create next-app --example "https://github.com/advancedblockchain/ui-template"
```
## Storybook

There are two ways to run your storybook:

- `yarn storybook` to run the storybook

- `yarn storybook:build` to build the storybook

## Testing

There are two ways to test your application:

- `yarn test` to run the tests

- `yarn test:watch` to run the tests in watch mode

<aside>
💡 Note: Wiring between storybook and Jest is done, check out the example `tests/pages/home.test.tsx` for more information.

</aside>

## Running the app

There are multiple scripts ready to use on your first tryout.

- `yarn start` to launch the development server
- `yarn build` to build production
- `yarn lint` to run the linter

<aside>
💡 `yarn install` will trigger `.scripts/postinstall.js` script. Which contains important checks on whether you need to update certain configurations. It is strongly advised to make sure that script doesn’t throw any warnings, or in case of the warning, resolve them ASAP before going live.

</aside>