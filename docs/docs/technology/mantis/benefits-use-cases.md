# Cross-Chain Intent Settlement: Benefits & Use Cases

## Improved Cross-Chain User Experience

At Composable, our focus has always been on improving the cross-chain landscape by providing user-friendly cross-chain infrastructure. By making DeFi more accessible and appealing in this manner, the industry becomes better positioned for mass adoption.

Specifically regarding MANTIS, we believe that a user intent settlement platform (particularly, a cross-chain one) can improve the landscape for blockchain transaction execution. That is because this vastly improves the user experience, carrying out all types of cross-domain transactions and abstracting away the complexity involved in this process. 

Furthermore, users do not have to spend time identifying the best opportunities to satisfy their intents, only to find that these opportunities are no longer available by the time that they have explored all options; instead this is done for them, in short order. 

Here’s the TLDR on MANTIS and how it improves the cross-chain transaction user experience:

- **One Stop**: bring any asset to and from any chain, swap natively, and earn yield instantly
- **Do You**: maximize the potential of your assets with lending, staking, retaking, LP, vaults, lock drops, NFT borrowing, limit orders, and perps
- **Performant**: leveraging IBC, MANTIS can execute simultaneous orders, cross-chain
- **Execution**: get best-price execution with dark pools and cross-chain MEV subsidies (soonTM) 
- **Simplified**: an intuitive, seamless design that lets you have fun on-chain

In this way, MANTIS is the everything dApp for the cross-domain future of DeFi.

The go-to-market plan for MANTIS consists of implementing a layer for gamification, beginning with staking for Solana IBC.

In crypto, we often find ourselves needing to do many steps to hunt for alpha. However, there are lots of people on twitter that know what they’re doing and suggest/curate alpha for the average degen:

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">🚨Easy mode.<br /><br />🚨The next round of Solana airdrop alpha:<br /><br />Take your <a href="https://twitter.com/search?q=%24sol&amp;src=ctag&amp;ref_src=twsrc%5Etfw">$sol</a> and stake it on Marinade<br />(<a href="https://t.co/nszCHsmQxt">https://t.co/nszCHsmQxt</a>)<br /><br />You will receive <a href="https://twitter.com/search?q=%24mSol&amp;src=ctag&amp;ref_src=twsrc%5Etfw">$mSol</a> <br /><br />Take your <a href="https://twitter.com/search?q=%24mSol&amp;src=ctag&amp;ref_src=twsrc%5Etfw">$mSol</a> and lend it on MarginFi <br />(<a href="https://t.co/HwDX81R6c9">https://t.co/HwDX81R6c9</a>)<br /><br />Borrow $JitoSol from MarginFi <br /><br />Swap $JitoSol for <a href="https://twitter.com/search?q=%24Sol&amp;src=ctag&amp;ref_src=twsrc%5Etfw">$Sol</a> <br /><br />Take… <a href="https://t.co/AZngyfZOVD">pic.twitter.com/AZngyfZOVD</a></p>&mdash; Johnny ☠️ (@Cryptilt) <a href="https://twitter.com/Cryptilt/status/1732885620568047832?ref_src=twsrc%5Etfw">December 7, 2023</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

Still, the steps are not necessarily clear and might be daunting. We have thus made a commitment over the past few years to introduce a platform that is natively cross-chain and simplifies the overall crypto experience for people. 

**The best part is, this can be shared with others, introducing a Social Fi feeling that has been lacking from crypto.**

Social Fi has been manifested in many different contexts such as friend.tech, the points system pioneered by MarginFi, and many other platforms such as Blast. However, it has not really been super successful in the realm of copy-trading. We believe that now is the right time to introduce such a product, with a level of gamification to it. This is accomplished by the MANTIS Games, detailed.

## Cross-Domain MEV

A cross-domain intent settlement platform (such as that being developed by Composable) reshapes an emerging type of MEV: cross-domain MEV. As this is a relatively novel form of MEV, and MEV is still a poorly studied and reported phenomenon, a number of questions thus arise. In particular, we at Composable believe that cross-domain MEV could impact the price of intent settlement in a positive way, decreasing cost for users. In fact, this type of MEV is a positive for all levels of the supply chain. 

:::tip [Obadia et al, 2021](https://arxiv.org/pdf/2112.01472.pdf)

Cross-domain MEV can be defined as the extraction of value from cross-chain transactions mathematically defined by Obadia et al, 2021. To summarize, this research found that cross-domain MEV is the maximum of the sum of final balances across all considered domains into a single base asset (canonically the first domain considered), given there is some assortment of transactions across all those domains that are executed together. Importantly, this research also concluded that “We expect bridges to play an extremely important role in such an MEV ecosystem, as the cheaper, more ubiquitous, and faster bridges become, the more competitive these arbitrage transactions naturally become by decreasing the inequality of the action space across players as a function of their capital.” 
:::

In the Composable ecosystem specifically, cross-domain MEV is potentiated from cross-chain intent settlement. Composable’s MANTIS receives user transaction intents, which are then picked up by solvers who compete to find the best solution to execute these intents. Once the optimal solution is chosen via a scoring mechanism, the winning solver must then execute upon their proposed solution. 

A single solution can involve a number of different domains. Searchers can access the orderflow from these solutions not only within each domain but also between domains by accessing the mempool:

![orderflow](../mantis/problem-mempool.png)

This results in cross-domain MEV.

## Free/Reduced Gas

Another exciting potential benefit of a cross-domain intent settlement framework is reducing gas costs for users which is detailed in [this forum post](https://research.composable.finance/t/cross-domain-mev-as-an-influence-on-pricing-of-intent-settlement-can-we-achieve-free-reduced-cost-exchange/47). In brief, the way that gas costs could be kept as low as possible would be to have such gas costs be a dynamic value that is subject to market conditions. This means that users could be able to trade for free, but only in the event that the below incentive equation is positive, and solvers are able to cover user gas fees:

- (+) 0.1% of transfer, like CoW Swap 
- (+) Sale to blockbuilders
- (+) MEV
- (-) Money paid to blockbuilders in the role of searcher
- (-) User’s gas

If not, then users will have a partial gas payment. Solvers can also take out short term loans and use these to cover gas fees, then pay these loans off after the order is executed and they receive their rewards. 

## Future Explorations

There are a number of ways in which Composable envisions expanding and improving our MANTIS intent settlement framework, once deployed. These include:

- Incorporating credible commitment schemes such as MEV-Boost ++ and PEPC-Boost. 
- Building a new relay that would allow for partial block building
- Moving towards a no-builder future where searchers can build blocks collaboratively and send them directly to proposers
- Implementing mempool matching and pre-reserved blockspace

We are incredibly excited about the potential of MANTIS for delivering upon our vision of a user-centric, ecosystem-agnostic future for DeFi. Stay tuned for more updates about this project and our progress towards its deployment.