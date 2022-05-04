# Hooks

This folder contains scripts that are designed to be run as [git hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).

Each file name should follow the format of `<hook name>.<descriptive name>`; for example `pre-commit.style` for a pre-commit hook that runs `make style`. This allows for multiple scripts for the same hook to exist.

To install a script, simply copy the contents of the script you want to install to `.git/hooks/<hook name>` (or append it if you're already using said hook). There will probably be various `<hook name>.sample` files in `.git/hooks/`; they are populated by git and serve as examples for each of the possible hooks.

***NOTE:*** **Hooks will only be run if the file name matches the hook name exactly!**

## `pre-commit.style`

The style `pre-commit` hook runs `make style` to format all files and re-stage all originally changed files. This can be useful to help avoid "rustfmt" commits to get CI to pass.

**NOTE**: You probably want to disable or turn this off when merging and/ or rebasing.
