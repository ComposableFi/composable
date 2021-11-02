
<p align="center">

# Composable Node     
  <img alt="Composable Finance" title="Composable Finance" src="composable.png">
</p>


[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/composablefi/composable)](https://github.com/composablefi/composable/tags) [![Twitter](https://img.shields.io/badge/Twitter-gray?logo=twitter)](https://twitter.com/ComposableFin) [![Discord](https://img.shields.io/badge/Discord-gray?logo=discord)](https://discord.gg/pFZn2GCn65) [![Telegram](https://img.shields.io/badge/Telegram-gray?logo=telegram)](https://t.me/ComposableFinanceAnnouncements) [![Medium](https://img.shields.io/badge/Medium-gray?logo=medium)](https://composablefi.medium.com/)


Picasso is our custom built kusama parachain, based on the substrate framework.




## Install   

For linux, FreeBSD, OpenBSD and macOS:

```sh
git clone https://github.com/composableFi/composable
cd composable/
sh scripts/init.sh
cargo build --release
```


## Run  
After compiling the node, you can simply run it with: 
```sh

```


### Pallets
Picasso ships with multiple custom made pallets such as:
[Cubic Vault](frame/vault/README.md)    
[Lending](frame/lending/README.md)    
[Oracle](frame/oracle/README.md)     
[CurveAmm](frame/curve-amm/README.md)
[BribeDAO](https://www.bribe.xyz/)    


## Sudo privileged functions
Thanks to substrate sudo functions we can choose what privileges our function has.
[Read more here](sudo.md)



## Benchmarking   
For all things benchmarking, [Read our benchmarking documentation here](docs/benchmarking.md)
