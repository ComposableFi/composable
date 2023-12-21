# Editing Docs

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

