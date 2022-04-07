# Contributing
*Thank you for investing your time in contributing to our book!*

---

This book is part of our [monorepo hosted on GitHub](https://github.com/ComposableFi/composable). It is built using [mdBook](https://rust-lang.github.io/mdBook/), and it's contents are located in the `book/` directory.

The structure is simple: there is a `SUMMARY.md` which lists all of the available pages, and defines the structure of the sidebar. When you add a link to this file, a matching `.md` file will automatically be generated.

---

## Serving a local version

### Nix

If you are using [nix](https://nixos.org/), you can run the following without installing anything:

```bash
nix develop ".#book" --command mdbook serve ./book --open
```

*(If you're running Visual Studio Code, you can also simply open the repository and press `ctrl + shift + B`)*



### macOS / Linux

Follow the [mdBook installation instructions](https://rust-lang.github.io/mdBook/guide/installation.html), and run the following:

```bash
mdbook serve ./book --open
```

A browser will open showing a local copy of this book. Simply edit the contents of the `book/` directory, and your changes will be reflected live in the browser.

---

## Getting your changes published

Simple create a **pull request** on GitHub with your changes to the `book/`, and when it gets merged your changes are automatically published by the CI.

---

## Syntax
We use [Commonmark flavored markdown syntax](https://commonmark.org/help/), plus [GFM tables](https://github.github.com/gfm/#tables-extension-), [GFM task lists](https://github.github.com/gfm/#task-list-items-extension-), [GFM strikethrough](https://github.github.com/gfm/#strikethrough-extension-), and [footnotes](https://github.com/commonmark/commonmark-spec/wiki/Deployed-Extensions#note). In addition to that, we have some special syntax for **image captions** and **H1 emoji**:


### Image captions
If you want to add a caption to an image, you should add an italics line right below it, like this:

```md
![Composable Face](./composable-face.png)
*Composable Finance: The Face of DeFi's Future*
```

![Composable Face](../composable-face.png)
*Composable Finance: The Face of DeFi's Future*

### H1 emoji
Sometimes, you may want to have an emoji in the H1 title (like on the introduction page) in order to do this, use the following syntax:

```md
# **👋** Introduction
```

# **👋** Introduction



