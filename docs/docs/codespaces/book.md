# Book

---

This book is part of our [monorepo hosted on GitHub](https://github.com/ComposableFi/composable). 
It is built using [mdBook](https://rust-lang.github.io/mdBook/), and it's contents are located in the `book/` directory.

The structure is simple: there is a `SUMMARY.md` which lists all of the available pages, and defines the structure of the sidebar. 
When you add a link to this file, a matching `.md` file will automatically be generated.

---

## Running the book

Open the integrated terminal in a Codespace and run the following command:

```bash
make run-book
```

A browser will open showing a local copy of this book. Simply edit the contents of the `book/` directory, 
and your changes will be reflected live in the browser. 
(Despite running in the cloud, 
your Codespace will transparently map the ports to your localhost so that you can access the live book at 
`localhost:3000`.)

![Book running locally](./book-running-locally.png)
*So meta*

---

## Getting your changes published

Before publishing your changes, make sure they pass the linter. 
To do this, run the following command inside the `./book/` directory.

```sh
mdbook test
```

After your changes pass the linter without error, simple create a **pull request** on GitHub with your changes to the 
`book/`, and when it gets merged your changes are automatically published by the CI.

---

## Syntax
We use [Commonmark flavored markdown syntax](https://commonmark.org/help/), 
plus [GFM tables](https://github.github.com/gfm/#tables-extension-), 
[GFM task lists](https://github.github.com/gfm/#task-list-items-extension-), 
[GFM strikethrough](https://github.github.com/gfm/#strikethrough-extension-), 
and [footnotes](https://github.com/commonmark/commonmark-spec/wiki/Deployed-Extensions#note). 
In addition to that, we have some special syntax for **image captions** and **H1 emoji**:


### Image captions
If you want to add a caption to an image, you should add an italics line right below it, like this:

```md
![Composable Face](./composable-face.png)
*Composable Finance: The Face of DeFi's Future*
```

![Composable Face](../composable-face.png)
*Composable Finance: The Face of DeFi's Future*

### H1 emoji
Sometimes, you may want to have an emoji in the H1 title (like on the introduction page) in order to do this, 
use the following syntax:

```md
# **👋** Introduction
```

# **👋** Introduction

### Common Errors

By default, mdbook will attempt to lint code blocks with `rustc`. To avoid this, either specify another language 
or add `ignore` to your codeblocks. 
More details [here](https://rust-lang.github.io/mdBook/cli/test.html).

