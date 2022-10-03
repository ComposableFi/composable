# Editing this Book
_So meta_

---

The editing process of this book has also been nixified.

## Serving a hot-reloading version of this book

To serve a hot-reloading version of this book, simply run:

```bash
nix run ".#serve-book"
```

And open [http://localhost:3000](http://localhost:3000/) in your browser.

Then, edit the sources in `book/src/`, and see them update live in your browser!


## Checking your spelling

After adding new content, you probably want to check your spelling. To do this, run the following:

```bash
nix build ".#spell-check"'
```


## Building a static copy

If you want to build a static copy of this book, run:

```bash
nix build ".#composable-book
```

