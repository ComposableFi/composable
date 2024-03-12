### Installation

```
$ yarn
```

### Local Development

```
$ yarn start
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

### Build

```
$ yarn build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

### Deployment

Using SSH:

```
$ USE_SSH=true yarn deploy
```

Not using SSH:

```
$ GIT_USER=<Your GitHub username> yarn deploy
```

If you are using GitHub pages for hosting, this command is a convenient way to build the website and push to the `gh-pages` branch.


## Via Nix

## Serving a hot-reloading version of these docs

To serve a hot-reloading version of these docs, simply run:

```bash
nix run ".#docs-dev"
```

And open [http://localhost:3000](http://localhost:3000/) in your browser.

Then, edit the sources in `docs/`, and see them update live in your browser!

## Building a static copy

If you want to build a static copy of these docs, run:

```bash
nix build ".#docs-static"
```

