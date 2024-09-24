# Bitcoin IBC

With a number of IBC Protocol extensions complete, Picasso now turns its focus to Bitcoin. As announced in July 2024, Picasso is in the process of researching an implementation of IBC on Bitcoin. As a result, the world’s first blockchain will become interoperable in a trust-minimized manner with all IBC-enabled chains for the first time. A number of benefits will thus be realized, including that cross-chain DeFi will be positioned to flourish on Bitcoin.

## Background

Bitcoin is the original cryptocurrency, with both the Bitcoin chain and its token being established in 2009. Bitcoin remains to be at the top of crypto rankings; at the time of writing, it has the largest Market Cap, which is over $1.1 trillion (as per [CoinMarketCap](https://coinmarketcap.com/currencies/bitcoin/)). Bitcoin is also the crypto that is arguably the closest to mass adoption, exemplified when Bitcoin ETFs were approved by the US Securities and Exchange Commission in January 2024. 

However, DeFi practices with Bitcoin are limited compared to other blockchains like Ethereum due to its inefficiencies such as significant financial/energy costs to operate, slow transaction speeds and a lack of smart contract infrastructure. Bitcoin’s lack of interoperability [has also been cited](https://metlabs.io/en/defi-bitcoin-possible-and-realistic/) as a main reason for its limited DeFi practices, as decentralized applications on Bitcoin are isolated from other applications and ecosystems. 

Thus, DeFi on Bitcoin represents a huge untapped opportunity, given the massive size and reputation of the network. For instance, [one prediction from Franklin Templeton](https://www.coindesk.com/opinion/2024/06/05/a-more-than-1t-bitcoin-defi-opportunity/#:~:text=The%20primary%20differentiator%20between%20DeFi,underlying%20asset%20(native%20token).) estimates that more than $1 trillion could be generated in the next 5 to 10 years from Bitcoin DeFi practices. 

With Picasso’s Bitcoin IBC connection, users and developers will be able to implement any kind of cross-chain use case imaginable between Bitcoin and the other IBC-enabled chains. The Bitcoin IBC connection will therefore introduce the following benefits:

- Removing barriers to the growth of DeFi on Bitcoin
- Introducing new cross-chain use cases to and from Bitcoin
- Facilitating the flow of Bitcoin-native assets to other ecosystems (and vice versa), resulting in enhanced liquidity across the board
- Streamlining the cross-chain user experience to enhance chain-agnosticism and adoption of DeFi overall

However, there are many challenges faced when trying to implement IBC on Bitcoin. If you are familiar with how IBC works, the first thing you're probably thinking about is how is it even possible due to the fact that Bitcoin is a proof-of-work blockchain, and building a light client based IBC connection for this scenario is impossible.

## How it works

The IBC protocol can be cost- and time-efficient compared to many other bridges, but also it is always trust-minimized. Most popular bridges like Wormhole are trusted, meaning users must put their faith into a third party to uphold the security of the bridge and facilitate asset transfers. In contrast, IBC is trustless, as it uses cryptographic techniques (instead of a trusted third party) to ensure bridge security and reliability. As a result, users only need to trust in publicly visible code when transferring over IBC.

These IBC requirements that Bitcoin lacks are as follows:

### Light Clients

Light clients or light nodes are considered “light” in that they have less data storage than a traditional node. These are required on either side of an IBC connection. Thus, we must create a light client of the Bitcoin chain on each chain connected to Bitcoin over IBC. We must also create a light client of these IBC-enabled chains on Bitcoin.

### A Way for the IBC Protocol to be Understood on Bitcoin

To allow Bitcoin to understand the IBC protocol, we could likely use [BitVM](https://github.com/BitVM/BitVM), or similarly, [ArchVM](https://twitter.com/ArchNtwrk/status/1778113793312034965). BitVM is a novel, work-in-progress computing paradigm enabling Turing-complete smart contracts on Bitcoin via Taproot trees and fraud proofs. This provides an opportunity for light client proofs (such as those from IBC-enabled chains) to be natively verified by Bitcoin. We could use BitVM (or ArchVM) similarly to how [Citrea](https://docs.citrea.xyz/technical-specs/characteristics/bitcoin-settlement-trust-minimized-btc-bridge) uses it; Citrea is a trust-minimized bridge program that consists of an operator and verifier software with ZK circuits of the bridge, built on top of BitVM.

Another option would be using [SNARKnado](https://twitter.com/alpenlabs/status/1785730103122513943?s=46&t=L6WUuf8WDk_t4mjtsDJAQA) by [Alpen Labs](https://www.alpenlabs.io/). This product builds on BitVM to verify SNARKs on Bitcoin. Unlike BitVM2, SNARKnado does not support permissionless challenges. However, SNARKnado avoids unresolved challenges like on-chain costs of BitVM2. Thus, Alpen Labs believes these two protocols can be complementary with shared techniques and optimizations.

### Enabling the IBC Protocol on Both Sides

We also need to enable the IBC Protocol to operate on both Bitcoin and the other chains we are connecting it with. On Bitcoin, we would need the [Pay to Scrypt Hash (P2SH) contract](https://github.com/bitcoin/bips/blob/master/bip-0016.mediawiki) that allows for the storage of Bitcoin, as well as the unlocking based on the ZK proof that is passed over IBC.

:::tip
For a more technical breakdown of how Picasso plans to implement IBC Bitcoin - or to engage in discussion about this innovation - check out our Request for Proposal (RFP) for this initiative on the Composable Research Forum.]
:::