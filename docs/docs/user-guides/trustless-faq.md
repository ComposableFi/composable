# Trustless FAQ

## All about Composable Chains

❓ **Q1:** What is Picasso, Composable Cosmos and the Composable parachain?

📝 **A1:** Composable Cosmos, Composable Parachain, and Picasso are the cornerstones of our ecosystem, each serving a unique yet interconnected role.

1. **[Composable Cosmos Chain](../networks/composable-cosmos.md)**: This is a native chain within the Cosmos ecosystem, specifically designed for interoperability. It connects to our Picasso parachain in the Kusama network via IBC (Inter-Blockchain Communication), facilitating seamless asset transfers and cross-chain functionalities.
2. **[Composable Parachain](../networks/composable-parachain-overview.md)**: Residing in the Polkadot ecosystem, the Composable Parachain is built for cross-chain functionality and also utilizes IBC to connect with our Picasso parachain in the Kusama network.
3. **[Picasso](../networks/picasso-parachain-overview.md)**: An infrastructure layer on Kusama aiming for DeFi interoperability. Its native token is $PICA, and it connects to Composable Cosmos and Parachain via IBC.

Together, these chains provide a robust, secure, and seamless cross-chain experience for our users.

---

❓ **Q2:** How does multihop work?

**📝 A2:** Multihop is our network routing via multiple intermediaries in Cosmos and Dotsama ecosystems. To properly answer, how does it work, we will dive deep into two things: how are our chains connected between each other and what are the risks of the routing?

### **1. How Chains Are Connected.**

- **Composable Cosmos Chain is connected to Picasso on Kusama**: via IBC (Inter-Blockchain Communication).
- **Composable Parachain on Polkadot is connected to Picasso on Kusama**: also via IBC.
- **Composable Parachain is connected to Polkadot Relay Chain and other Polkadot parachains:** via XCMP (Cross-Consensus Messaging Passing).
- **Picasso is connected to Kusama Relay Chain and other Kusama parachains:** also via XCMP.

Each of these chains has its own relay mechanisms and further connections. For example, the Composable Cosmos Chain is connected to Cosmos Hub, Osmosis and other CometBFT-based blockchains via IBC.

### **Transaction Journey:**

If a user wants to send an asset (e.g., DOT) from Polkadot to Osmosis, the asset would take the following journey:

1. Polkadot → Composable Parachain,
2. Composable Parachain → Picasso,
3. Picasso → Composable Cosmos,
4. Composable Cosmos → Osmosis.

Note: most transactions apart from Polkadot relay and parachains are just one hop away from the Composable Cosmos Chain.

### **2. Risks of Failed Transactions:**

In the unlikely event that a transaction fails during the multihop process, the asset could end up being lodged on one of the intermediate chains. We have automated mechanisms designed to rollback failed transactions; however, there could still be complexities that prevent a smooth return of the asset. In such cases, the relayer won't cover the gas fees, and the funds will remain in the user's wallet on the intermediary chain. From there, users have the flexibility to manage those assets as they see fit.

To get a better understanding of our ecosystem, you might take a look at the following diagram:

![composable.png](../user-guides/composableFAQ.png)

---

❓ **Q3:** What is $PICA?

📝 **A3:** 

**What it is:** PICA is the native token of Picasso, an infrastructure layer built on the Kusama network.

**Use Cases:** As of now, PICA, the native token of Picasso and the Composable Cosmos chain, can be used for participating in governance decisions and staking, and LP staking on Pablo.

**Tokenomics:** You can find more details on the **[Picasso Tokenomics Page](../networks/picasso/tokenomics.md)**.

---

❓ **Q4:** What are all of the official Composable products?

📝 **A4:** Composable Finance offers a range of products designed to enhance the interoperable DeFi landscape. Here's a rundown of the official products:

1. **[Trustless Zone](https://trustless.zone/)**: Trustless Zone is designed to offer secure, automated, and trustless asset transfers between different blockchains.
2. **[Pablo DEX](https://app.pablo.finance/)**: Pablo serves as a Decentralized Exchange (DEX) and a cross-chain liquidity hub that is part of the Picasso platform. It facilitates secure and efficient trading between various assets.

---

❓ **Q5:** What is meant by trustless transfers, and how is it different from other transfer protocols?

📝 **A5:** Trustless transfers refer to the automated and secure process of moving assets across different blockchains without requiring a trusted intermediary or third party such as a multisig or an oracle. This is achieved through the use of transport layers like IBC and XCMP.

Our Trustless transfer system leverages the well-established IBC (Inter-Blockchain Communication) transfer protocol, which has been in the industry for several years and has facilitated a significant number of secure transfers. For a more in-depth understanding, you're encouraged to read our **[existing blog posts](https://blog.cosmos.network/how-composable-finance-overcame-technical-barriers-to-architect-a-trust-minimized-cosmodot-cde782fda8c4)** on this topic.

This approach stands in contrast to some other transfer protocols that may require a layer of trust or manual validation, introducing centralization risks.

By opting for trustless transfers, you benefit from both enhanced security and decentralization.

---

❓ **Q6:** What are canonical tokens?

📝 **A6:** In the context of Trustless Zone, canonical tokens refer to assets that maintain their native properties when transferred across different blockchains. They don't fragment liquidity and remain native throughout the transfer process.

Our system adopts the secure and industry-tested IBC (Inter-Blockchain Communication) standard, which allows for the seamless transfer of native assets between different blockchain ecosystems. This approach ensures deeper liquidity and enhanced security for cross-chain transactions.

---

## Trustless transfers and fees

❓ **Q7:** What are the max transfer limits of Trustless?

📝 **A7:** We have implemented a dynamic rate limit based on a 30% in-and-out cap within a 24-hour period of the total asset supply in escrow. This floating rate limit allows for flexibility while maintaining network security. If you attempt to exceed these limits, you will be prompted to enter a smaller amount. Please note that these limitations are in place to ensure the stability and security of the network.

---

❓ **Q8:** What is the min transfer limit of Trustless?

📝 **A8:** The minimum transfer limit on Trustless is a dynamic value that depends on four primary factors:

1. **Gas token on the source chain**: The type and amount of gas required to initiate the transaction.
2. **Existential Deposit (ED) on the source chain**: The minimum amount of tokens that must be kept in an account on the originating chain.
3. **ED on the destination chain**: Similar to the source chain, the destination chain also has an Existential Deposit requirement.
4. **Gas consumption on the hop chain**: The gas used on any intermediary chains that the assets cross through.

Here's a scenario to illustrate how these factors interact:

- Assume a multi-hop transfer from Polkadot to Picasso.
- The user has 0 DOT on Composable/Picasso.

What we check:

1. If the user has enough DOT left for the Existential Deposit on Polkadot after the transfer.
2. Whether the amount of gas used on Polkadot and Composable does not exceed the amount being transferred.
3. If the amount sent by the user is above the Existential Deposit on Picasso.

The sum of these values will be the minimum transfer limit for the specific scenario on Trustless.

---

❓ **Q9:** What is our transfer fee?

**📝 A9:** Our standard transfer fee is 0.4% for using any of our transfers, applicable in both directions. However, it's important to note that for multi-hop transfers, such as those from Polkadot to Osmosis, fees apply to every connection. In this specific example:

- From Composable parachain to Picasso: 0.4% fee
- From Picasso to Composable Cosmos Chain: 0.4% fee

That means for a multi-hop transfer, you'll incur an around 0.8% fee in total, plus a nominal amount for gas fees associated with XCM and IBC transfers. This structure ensures the sustainability of our services while keeping fees transparent and straightforward for our users.

---

❓ **Q10:** Why is my transfer stuck?

📝 **A10:** Encountering a "stuck" transfer can understandably be worrying. The term "stuck" could mean various things: from a transaction that's still loading and hasn't been confirmed, to a failed transaction where funds seem to be missing or held up. Here are some typical scenarios and what they mean:

1. **Temporary Network Delays**: Occasionally, network or system lags may cause short-term interruptions in the transfer process.
2. **Chain-Specific Issues**: Technical problems on any of the blockchain networks involved can contribute to delays or failed transactions.
3. **Transfer has timed out:** Can happen when an Inter-Blockchain Communication (IBC) packet, sent from Chain A to Chain B, doesn't receive an acknowledgment of successful transfer within a time frame of 1 hour. This timeout could occur due to network delays, chain congestion, or other issues.

For more information on IBC and light clients, you may refer to this **[Cosmos IBC Tutorial](https://tutorials.cosmos.network/academy/3-ibc/5-light-client-dev.html)**.

### **What Happens Next?**

- **Automatic Reversal**: In cases where the transfer issue is temporary, such as network delays, your funds should automatically revert to your original account, typically within an hour.
- **Reach Out for Support**: If your transfer remains stuck or if you encounter any issues that are unclear, please don't hesitate to reach out to us on our **[Telegram](https://t.me/composable_chat)** or 
**[Discord](https://discord.com/invite/composable)** channels for additional support. We're actively working to resolve any issues as quickly as possible.

This approach ensures that you have multiple avenues for resolving your concerns while we work to make the transfer process as smooth as possible.

---

## All about staking

❓ **Q11:** Why am I staking $PICA on a Cosmos chain instead of parachain?

📝 **A11:** Staking $PICA on our Cosmos chain is part of our broader strategy to secure the Composable Cosmos Chain. Although $PICA started as a native token of the Picasso chain, we have also designated it as the native token of the Cosmos chain. To facilitate seamless token flow, we've set up IBC middleware between the two chains, ensuring that the tokens remain native on both platforms. Over 1B $PICA tokens have been transferred from our treasury to validators on the Composable Cosmos Chain to bolster its security and robustness. It's worth noting that staking is not currently available on the Picasso chain. For more details, you can refer to our **[full tokenomics documentation](https://docs.composable.finance/networks/picasso/pica-use-cases)**.

---

❓ **Q12:** What is the $PICA staking APR% on the Composable Cosmos chain?

**📝 A12:** Understanding the APR for $PICA staking on our Composable Cosmos chain, is a bit different from what you might be used to on other Cosmos chains. This is because $PICA is a non-inflationary token, meaning it's not subject to newly minted tokens for rewards. All PICA tokens have already been minted, and only those are used for rewards.

As a result, you won't be able to fetch APR percentages in the usual way through the Cosmos inflation endpoint, as it is floating. Instead, you can calculate the APR manually using certain metrics and a formula we've provided:

1. **Annual Provision** ($AP$) is the total number of tokens allocated for staking rewards per year. It can be calculated as `Annual Provision = GET /cosmos/mint/v1beta1/annual_provisions` or check **[this 
link](https://api-composable-ia.cosmosia.notional.ventures/cosmos/mint/v1beta1/annual_provisions)** for the API call.
2. **Bonded Tokens** ($BT$) refer to the total number of tokens currently staked in the network. It can be calculated as `Bonded Tokens = GET /cosmos/staking/v1beta1/pool` or check **[this 
link](https://api-composable-ia.cosmosia.notional.ventures/cosmos/staking/v1beta1/pool**) for the API call.

Using these, you can calculate the **Nominal APR** ($APR_N$) as follows:

$$
APR_N=\frac{AP\times(1−CT)}{BT},
$$

Where $CT$ is **Community Tax** and it can be found **[here](https://api-composable-ia.cosmosia.notional.ventures//cosmos/distribution/v1beta1/params)**.

To adjust this for real-world conditions, you also have to take into account the actual number of blocks minted during the year **Real Annual Provision** ($AP_R$). To calculate that we need to get **Blocks Per Year** ($BPY$) and **Real Blocks Per Year** ($BPY_R$):

$$
AP_R=AP\times\frac{BPY_R}{BPY},
$$

$BPY$ can be found **[here](https://api-composable-ia.cosmosia.notional.ventures/cosmos/mint/v1beta1/params)** and $BPY_R$ is calculated **[here](https://github.com/giansalex/cosmos-staking-apr/blob/master/src/index.js)** in a function 
`getBlocksPerYearReal,` which is an estimate of how many blocks are likely to be generated in one year, based on recent block timings.

The **Real Staking APR** ($APR_{RS}$) is then calculated as:

$$
APR_{RS}=APR_N\times\frac{AP_R}{AP}=APR_N\times\frac{BPY_R}{BPY},
$$

and it is the one you can see on **[trustless.zone](https://app.trustless.zone)**.

Finally, the **Final Staking APR** ($APR_{FS}$) takes into account the **Validator's Commission** ($VC$), which can be found **[here](https://explorer.nodestake.top/composable/staking)**:

$$
APR_{FS}=APR_{RS}\times\left(1-VC\right).
$$

At present, the estimated APR stands around 13%.

---

❓ **Q13:** How to choose a validator on Composable Cosmos Chain to stake with, is there any risk involved in staking?

📝 **A13:** Choosing a validator on Composable Cosmos Chain requires thoughtful consideration of multiple factors:

1. **Uptime**: Aim to select a validator with near-perfect uptime. A high uptime is a hallmark of reliability, and it's crucial for earning consistent staking rewards. Keep in mind that if a validator is jailed or tombstoned due to downtime or double signing, you won't receive rewards during that period.
2. **Voting Power**: Opting for a validator with a lower percentage of total voting power can contribute to the decentralization of the network. By doing so, you're distributing the voting power and reducing the risk of centralization.
3. **Commission Rate**: While a lower commission rate is preferable as it allows you to keep a higher proportion of the staking rewards, it shouldn't be the sole criterion for selection.
4. **Slashing Policy**: It's important to note that slashing parameters are set at the chain level and apply uniformly to all validators. Therefore, if a validator misbehaves, the slashing consequences will be the same regardless of which validator you've chosen.
5. **Community Presence**: Validators who are vocal, active, and have a positive reputation within the community are generally considered more trustworthy and transparent.
6. **Blockchain Contributions**: Consider selecting validators who actively contribute to the blockchain ecosystem through the development of tools, provision of services, and participation in governance.

By taking these factors into account, you can make a more informed decision about which validator best aligns with your risk tolerance and staking goals.

---

## Support and assistance

❓ **Q14:** I need help with a product, where do I go?

📝 **A14:** If you're experiencing issues or need assistance with any of our products, the best way to get help is through our Discord channel by opening a support ticket. You can join our Discord community **[here](https://discord.com/invite/composable)**.

Alternatively, you can also reach out to us on Telegram at our **[Composable Chat](https://t.me/composable_chat)**.

Either way, our team is readily available to assist you with your queries and concerns.

---
