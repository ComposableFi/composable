# Reading logs

Reading CI logs may be a bit confusing if you are not yet used to how Nix builds packages. Don't worry though, it is very simple once you know the very basics of how Nix works.

One of Nix goals is to **never build the same thing twice**. It works like this for each package:

1. Create the `derivation`, which is a pure function from inputs (the source code + all dependencies) to an output (example: the `composable-node` binary).
2. Get the **hash** of this derivation.
3. Look at our **cache servers**. Have we already built a derivation with this hash?
  - If we have already built this, we **copy the result from the cache server**.
  - If we have not built this yet, we **do the actual building**.

This is why in most of our logs, you will just see **copying path** statments like these:

```
copying path '/nix/store/qrqwd1ji31vmas9gax819j11w5ickgz1-gnugrep-3.7' from 'https://cache.nixos.org'...
copying path '/nix/store/xcs3ns14mddbjsr96cg7mzkqp7ml21qi-centauri' from 'https://composable-community.cachix.org'...
```

The first one is copying `gnugrep-3.7` with hash `qrqwd1ji31vmas9gax819j11w5ickgz1` from the cache server `cache.nixos.org`. This is a system dependency, so it is in the global `cache.nixos.org` cache server.
The second one is copying `centauri` with the hash `xcs3ns14mddbjsr96cg7mzkqp7ml21qi` from the cache server `composable-community.cachix.org`. This is one of our own pacakges, so it is cached in our own `composable-community.cachix.org` cache server.

This is a fantastic Nix feature as it saves us a ton of time and resources.

## The "Build all packages" job

Most of our building happens in the "Build all packages" job, which builts all of our packages. This happens on two machines, `x64-monster` and `arm-monster`. These are beefy machines with a ton of cores, capable of copmiling all of our packages in parallel for both CPU architectures.

Most of the logs will be downloading already built derivations like mentioned above, but for the things that need to be rebuilt (the things that have been altered by your PR) you will see log lines like the following:

```
zombienet> patching sources
zombienet> updateAutotoolsGnuConfigScriptsPhase
zombienet> configuring
zombienet> no configure script, doing nothing
zombienet> building
```

where `zombienet>` is the package being built, and after that you will see the unaltered log lines emitted by the build process of this package.

When all of packages have been downloaded/built, you will see the following log lines at the end:

```
Waiting to finish: 1 pushing, 3 in queue
```

This is the process of Nix uploading our built derivations to our **cache server**, so that they do not need to be rebuilt next time.

## The "List built results" step

Once the "Build all packages" step is completed, we list all of the things that we have built, such as this:

```
acala
bifrost
check-composable-benchmarks-ci
check-dali-benchmarks-ci
check-picasso-benchmarks-ci
cmc-api
composable
composable-2.10005.0
composable-build-2.10005.0
composable-clippy-2.10005.0
composable-fmt-2.10005.0
...
etc
```

## Which packages will be built?

All of the packages that are in the monorepo's `/flake/all.nix` list.

## The "Build all packages" step is failing! Why?

Read through the logs and look for packages that are being built. You will see 

```
a-package-you-worked-on> Some error emitted by the pacakge you have altered in your PR
```

If you want to easily reproduce this error locally you can then run

```
nix build .\#a-package-you-worked-on -L
```

in your local copy of the monorepo.

## I want to build everything that CI is building locally, how do I do this?

Just run the following:

```
nix build .\#all -L
```

on your machine, inside of the monorepo directory. If you have [installed Nix correctly and configred your machine to use our **cache server**](./install), then this should be quick as most derivations will be on our cache server.