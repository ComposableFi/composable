
<p align="center">

# Composable Node     
  <img alt="Composable Finance" title="Composable Finance" src="composable.png">
</p>


[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/composablefi/composable)](https://github.com/composablefi/composable/tags) [![Twitter](https://img.shields.io/badge/Twitter-gray?logo=twitter)](https://twitter.com/ComposableFin) [![Discord](https://img.shields.io/badge/Discord-gray?logo=discord)](https://discord.gg/pFZn2GCn65) [![Telegram](https://img.shields.io/badge/Telegram-gray?logo=telegram)](https://t.me/ComposableFinanceAnnouncements) [![Medium](https://img.shields.io/badge/Medium-gray?logo=medium)](https://composablefi.medium.com/)


Picasso is our custom built kusama parachain, based on the substrate framework.




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


## Run  
After you have compiled the node, you can simply run it with: 

```sh
$ ./target/release/composable --dev --tmp
```


### Pallets
Picasso ships with multiple custom made pallets such as:
[Cubic Vault](frame/vault/README.md)    
[Apollo](frame/oracle/README.md)

and several others you can find in the frame folder.



Read more specific information in [our docs folder](docs/).
