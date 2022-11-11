# Runtime upgrades

This tool is used by SRE for runtime upgrades. It provides two modes of runtime upgrading: `sudo` based and `democracy` based.

## Protocol

The tool performs the following steps in sequence:

1. Initialize a local chain using [`fork-of-substrate`](https://github.com/maxsam4/fork-off-substrate).

2. Performs a `sudo`-based runtime upgrade against the local chain to verify that the upgrade will succeed.

3. If 2 is succesful, perform either a `sudo`-based upgrade or `democracy` proposal, depending on the provided flags.