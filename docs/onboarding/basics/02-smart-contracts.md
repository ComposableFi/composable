# Smart contracts

Before we had Ethereum, we had different chains for different use cases; called app chains. Each chain is specifically dedicated to a single application/company. This was also in the early stages of crypto (think 2012-2014), and to be frank, these app chains were not ideal.

The real innovation here was Vitalik realizing that we needed a more general execution environment, which wouldn't just execute logic associated with the specific app chain, but could support many third-party developers and very different applications.

Appchains have gained popularity in the past few years in the Cosmos and Polkadot ecosystems; although these chains usually include smart contract facilities as well.

[![Avoid the Middle-Man (Smart Contracts) - Computerphile](https://img.youtube.com/vi/csS1mZFuNSY/maxresdefault.jpg)](https://youtu.be/csS1mZFuNSY)

### Takeaways

- Understand how smart contracts are just rules written by humans.
- Know smart contracts can only operate on data available onchain and are limited in their capabilities.

## Implementations of VMs

The EVM is the most well-known smart contract engine (or VM). A virtual machine is a program, which can execute other programs. A smart contract is not magic at all, just a regular program; but instead of running on your computer, it is executed on every single node of a blockchain. Each node has to execute the contract to verify the correctness of a proposed block. An alternative to the EVM is webassembly, which was originally developed for the browser as an alternative to Javascript. Your browser executes code when it loads a website, code that has been developed by a third-party developer and thus cannot be trusted. The same problem exists for smart contracts. The blockchain cannot trust the provided code and must ensure a few things.

- The code must be sandboxed. Access to the actual node from the code is very limited.
- It must end in a reasonable amount of time, or otherwise be shut down.
- Execution needs to be deterministic. If 2 nodes run the same contract with the same state and input parameters, the result must be the same.

A subset of webassembly provides these guarantees and is thus a popular runtime.

[![How does WebAssembly work?](https://img.youtube.com/vi/zcADuXro-GQ/maxresdefault.jpg)](https://youtu.be/zcADuXro-GQ)

### Takeaways

- Realize that smart contracts are just programs, but that they can be used as a building block for [dapps](https://www.youtube.com/watch?v=KkZ6iYnSDRw).
- That if we treat smart contracts as regular technology, we can discuss which tradeoffs each technology makes.

### On tradeoffs

Rust is made to build rockets, airplanes, and medical devices. Solidity is based on Javascript to make the barrier of entry as low as possible. Which technology would you prefer to handle billions of USD?

There are projects to bring better languages to the EVM, such as [Vyper](https://vyper.readthedocs.io/en/stable/) and [Fe](https://fe-lang.org/). 

## Further reading

- [ETHEREUM: A SECURE DECENTRALISED GENERALISED TRANSACTION LEDGER](https://ethereum.github.io/yellowpaper/paper.pdf)
- [CosmWasm/wasmvm Specification](https://github.com/CosmWasm/wasmvm/blob/main/spec/Specification.md)

> *Note*
> Why didn't they name it `Wasmos` :-(