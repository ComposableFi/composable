# Reading logs

Reading CI logs may be a bit confusing if you are not yet used to how Nix build packages. Don't worry though, it is very simple once you know the very basics of how Nix works.

One of Nix goals is to **never build the same thing twice**. It works like this for each package:

1. Create the `derivation`, which is a pure function from inputs (the source code + all dependencies) to an output (example: the `composable-node` binary).
2. Get the **hash** of this derivation.
3. Look at our **cache servers**. Have we already built a derivation with this hash?
  - If we have already built this, we **copy the result from the cache server**
  - If we have not built this yet, we **do the actual building**

This is why in most of our logs, you will just see **copying path** statments like these:

```
copying path '/nix/store/qrqwd1ji31vmas9gax819j11w5ickgz1-gnugrep-3.7' from 'https://cache.nixos.org'...
copying path '/nix/store/xcs3ns14mddbjsr96cg7mzkqp7ml21qi-centauri' from 'https://composable-community.cachix.org'...
```

The first one is copying `gnugrep-3.7` with hash `qrqwd1ji31vmas9gax819j11w5ickgz1` from the cache server `cache.nixos.org`. This is a system dependency, so it is in the global `cache.nixos.org` cache server.
The second one is copying `centauri` with the hash `xcs3ns14mddbjsr96cg7mzkqp7ml21qi` from the cache server `composable-community.cachix.org`. This is one of our own pacakges, so it is cached in our own `composable-community.cachix.org` cache server.

This is a fantastic Nix feature as it saves us a ton of time and resources.


