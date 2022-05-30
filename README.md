
<p align="center">

# Composable Node
  <img alt="Composable Finance" title="Composable Finance" src="composable.png">
</p>


[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/composablefi/composable)](https://github.com/composablefi/composable/tags) [![Twitter](https://img.shields.io/badge/Twitter-gray?logo=twitter)](https://twitter.com/ComposableFin) [![Discord](https://img.shields.io/badge/Discord-gray?logo=discord)](https://discord.gg/pFZn2GCn65) [![Telegram](https://img.shields.io/badge/Telegram-gray?logo=telegram)](https://t.me/ComposableFinanceAnnouncements) [![Medium](https://img.shields.io/badge/Medium-gray?logo=medium)](https://composablefi.medium.com/)
[![Bors enabled](https://bors.tech/images/badge_small.svg)](https://app.bors.tech/repositories/45659)


Picasso is our custom built Kusama parachain, based on the substrate framework.

## Documentation

To learn more about our ecosystem, vision, and product specifics - visit our 
[mdbook](https://dali.devnets.composablefinance.ninja).

You can also find our cargo docs [here](https://dali.devnets.composablefinance.ninja/doc).

## Install

For linux, FreeBSD, OpenBSD and macOS:

```sh
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
git clone https://github.com/composableFi/composable
cd composable/
sh scripts/init.sh
cargo build --release
```

or you can simply install it with this one liner:    

```sh
curl https://get.composable.finance | sh
```     


## Run  

Since Composable is now a parachain, you need to run a full relaychain in addition to running Composable.
In order to do so, install [yarn](https://classic.yarnpkg.com/lang/en/docs/install/#mac-stable) and use the [polkadot-launch](scripts/polkadot-launch) script.



## Pallets
Picasso ships with multiple custom made pallets such as:
[Cubic Vault](frame/vault/README.md)
[Apollo](frame/oracle/README.md)

and several others you can find in the frame folder.


Read more specific information in [our docs folder](docs/).