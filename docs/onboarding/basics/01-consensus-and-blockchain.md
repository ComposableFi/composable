# Blockchain

The term is quite overloaded and can refer to several different things, such as:

- The data structure represents blocks of data connected by hashes through [header data](https://www.oreilly.com/library/view/mastering-bitcoin/9781491902639/ch07.html). We consider this the most naive definition, as the data structure is not novel, and does not encapsulate any of the core concepts. Unknowledgable developers might misconstrue it with a fancy, persistent [linked list](https://www.geeksforgeeks.org/data-structures/linked-list/).
- A decentralized network of nodes, using a blockchain data structure to maintain a shared database. This is the more sophisticated definition, and better expresses the amount of technological innovation. Other examples of decentralized networks not based on a blockchain data structure include torrent networks, which still use [peer-to-peer networks](https://www.geeksforgeeks.org/what-is-p2ppeer-to-peer-process/). When we talk about blockchain within Composable, we usually refer to this definition.
- A collection of different decentralized technologies, such as wallets, zero-knowledge proofs, and smart contracts. Often when we speak about **'the blockchain revolution"**, this is what we mean.

## Introduction Video

[![The Blockchain & Bitcoin - Computerphile](https://img.youtube.com/vi/qcuc3rgwZAE/maxresdefault.jpg)](https://youtu.be/qcuc3rgwZAE)

### Takeaways

- [hashing](https://www.educative.io/answers/what-is-hashing)
- [signatures](https://www.coinbase.com/cloud/discover/dev-foundations/digital-signatures#:~:text=Digital%20signatures%20are%20a%20fundamental,other%20users%20from%20spending%20them)
- [longest chain rule](https://learnmeabitcoin.com/technical/longest-chain#:~:text=The%20longest%20chain%20is%20what,on%20the%20same%20transaction%20history.)

One thing that you should realize by now, is that processing transactions is not inherently expensive in Bitcoin. Oftentimes, people compare the total energy required by the Bitcoin network to smaller countries, as an example of how the technology is inefficient. Mining is just the process of choosing the next author of a block, and the energy required to become the next author is irrespective of the transactions being processed. The security of the Bitcoin network is proportional to the total energy expenditure and the cost of energy, however, this mining process plays an incredibly important part in securing (large) transactions.

## Finality

One question that every merchant asks is: when do I know that a payment will no longer be reverted, because of a network delay, a bug, or some other reason. Only after they know that they have their money, can they give the customer their product, as otherwise the payment might be reverted. We refer to this concept as [finality](https://smithandcrown.com/glossary/transaction-finality-probabilisticdeterministic/). The actual goal of a consensus protocol is to provide finality.

> *Note*
> Is Bitcoin's finality probabilistic or deterministic?

[![Finality in Blockchain Consensus - by Alexis Gauba - Mechanism Labs & She256](https://img.youtube.com/vi/efyiPhZvqOA/maxresdefault.jpg)](https://youtu.be/efyiPhZvqOA)

### Takeaways

- Probabilistic finality is strictly worse than deterministic finality.
- There are still tradeoffs, we pay a price for deterministic finality.
- We live in an imperfect world. Blockchains need to be resilient toward network failures as well as malicious actors.

## Proof of Stake (PoS)

An alternative method to providing finality is using proof of stake, which does not select the next leader based on expensive hashing, but instead uses a function that selects the next block author based on how much they are staking. The more you stake, the more often you are selected. The obvious advantage is the lower energy wastage, as well as the buy-in from block producers. With Proof of Work (mining), miners are not necessarily invested in the success of the underlying network. If the network loses value, is attacked, or is abandoned, they (the miners) can still use their mining equipment to make a profit on another PoW network, while with PoS their future profit is tied to the success of the network.

[![Ethereum 2.0: Proof of Stake vs Proof of Work | Vitalik Buterin and Lex Fridman](https://img.youtube.com/vi/3yrqBG-7EVE/maxresdefault.jpg)](https://youtu.be/3yrqBG-7EVE)

## Nominated/Delegated Proof of Stake (DPoS)

One problem that we find with PoS is that small token holders are not incentivized to participate in block production, as the cost of running a node (server costs, electricity, etc.) outweighs the reward. A solution to this problem is allowing token holders to pool their funds, and select one of them to run the node and produce blocks. This way, the token holder can earn a portion of the block reward. DPoS is often combined with setting a limit on the maximum number of block producers so that only the top X producers are earning a fee.

Although the above scheme seems very advantageous, there are a few downsides:

- Cartel forming. Block producers with large holdings usually work together to remain in the top `X`.
- Centralization of block production: often investors and professionals run validators. These parties are registered in their respective countries and vulnerable to regulations and government-enforced censorship.

That is why it is important to have a large number of validators, as well as to ensure that malicious validators can be booted from the network if we can prove that they are censoring transactions or missing blocks. Proving that a party is acting maliciously is incredibly difficult, as a network failure can be the cause of missing a block or not including a transaction (because it never arrived in the validator's mempool).

A big upside of DPoS is that, since we are running fewer staking nodes, and do not expect the regular user to run one, we can increase the minimum hardware requirements for a node. This increases the throughput of the network. A 64-core machine with 2 TB of RAM is more powerful than grandma's Raspberry PI.

[![Cardano's Proof of Stake Consensus Algorithm Explained | Charles Hoskinson and Lex Fridman](https://img.youtube.com/vi/Cj4dhHSJqDQ/maxresdefault.jpg)](https://youtu.be/Cj4dhHSJqDQ)

### Takeaways

- DPoS is a consensus protocol, but not necessarily better, just different tradeoffs again.
- We need to be extra vigilant with DPoS to guarantee censorship resistance.
- Less decentralized networks can still be censorship resistant, as long as new validators can freely join if old ones are taken down.

## Further reading

If you're interested in actually learning this stuff, get started with:

- [Byzantine Finality Gadgets](https://research.web3.foundation/en/latest/polkadot/finality.html#grandpa-full-paper)
- [BABE](https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html)
- [Bitcoin: A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf)

Note that this is not expected for your onboarding.
