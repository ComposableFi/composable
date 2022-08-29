# Bridging and Cross Chain

If you check out [coinmarketcap](https://coinmarketcap.com/), you'll see many different coins. Some are assets on a chain, so 1 blockchain may have many assets. Others represent the chain itself and are used for fees. You might realize then that there are a lot of chains out there, each with similar assets, functionality, and objective.

The logical successor to multiple, independent ecosystems, is linking them up with bridges. This allows existing token holders on Ethereum to start using dapps on Fantom. 

These token bridges operate in two ways:

- They burn tokens on the sender side, and mint tokens on the receiving side.
- They hold tokens on the sender and receiver sides, and when a deposit (lock) is made on the sender side, the receiver side releases.

How we orchestrate this synchronization will be discussed later. For now, it is enough to know that there are two types of protocols:

- `Trusted/Optimistic`: The synchronization protocol can commit fraud (release tokens without locking them), but may have some mechanism to punish fraud.
- `Trustless/Deterministic`: The synchronization mechanism uses fancy mathematics to make it impossible to commit fraud.

- [What Are Blockchain Bridges And How Can We Classify Them?
](https://blog.li.fi/what-are-blockchain-bridges-and-how-can-we-classify-them-560dc6ec05fa)

### Takeaways

The above article makes clear distinctions, although we consider atomic swaps to be a synchronization primitive, and not a way to manage liquidity. Atomic swaps can enable burn & mint or lock & release style liquidity management.

## Further Reading

- [BEEFY](https://github.com/paritytech/grandpa-bridge-gadget/blob/master/docs/beefy.md)
- [Nomad](https://docs.nomad.xyz/)
- [Polkadot - Learn Bridges (good for beginners)](https://wiki.polkadot.network/docs/learn-bridges)