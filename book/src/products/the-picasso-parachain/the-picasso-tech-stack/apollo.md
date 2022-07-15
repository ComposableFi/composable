# Apollo: The Oracle Pallet

---

The first pallet being built on the Picasso parachain is an MEV-resistant oracle 
pallet, Apollo. Composable is constructing this pallet as a means of kicking off 
development on Picasso, while delivering essential oracle functionalities and 
enabling more advanced secondary and tertiary DeFi functions later on. 
Ultimately, Apollo will join our suite of other parachain pallet offerings, and 
even other oracle pallets.

By offering an oracle pallet in our initial chain release, we intend to 
kickstart the DeFi ecosystem on Picasso.

## The Importance of Oracles in DeFi's Infrastructure

Taking a broader view of the industry, oracles fit in as a core piece of 
infrastructure, referred to as DeFi primitives (or, primary infrastructure). 
DeFi primitives are “...the essential features of the underlying blockchain 
layer which have particular relevance to the security of DeFi protocols,” 
[(Werner et al., 2021)](https://arxiv.org/pdf/2101.08778.pdf).

DApps then utilize secondary infrastructure to generate tertiary functionality 
and infrastructure, which provides more complex use cases. As an example, yield 
aggregators provide a means of profit maximization by moving user funds around 
to earn the highest yield. These require various different basic DeFi uses that 
generate yield (i.e. secondary infrastructure), making use of various tools to 
determine which secondary infrastructure to put which token type into at any 
given time.

## The Vision for Apollo in the Picasso Ecosystem

We thus see Apollo as the stepping stone to the creation of secondary and 
tertiary applications on Picasso that will rely on price feeds for 
functionality, which are currently under development by teams being incubated by 
Composable Labs.

However, we see Apollo as being one of many oracle solutions in the Polkadot 
ecosystem. As an example, Acala is currently building its Open Oracle Gateway to 
foster a more open, inclusive, and decentralized oracle infrastructure. The 
Gateway enables multiple oracles to be deployed on Acala, where they can serve 
any dApps on Acala, Polkadot, Kusama, and even other chains/networks. Hence, we 
believe Apollo will become one oracle of a well-developed oracle stack, and 
intend to work with partners that share this vision.

## Technical Details

The oracle is built into the base of the Picasso parachain and heavily uses 
different blockchain hooks to medianize (i.e. calculate and bring towards the 
median) and update data. The goals of the oracle are to be flexible to use and 
to offer integrators differing levels of security.

To become an oracle data provider, one must just run a node and put down a stake 
of tokens. The stake is slashed if the oracle provider inputs data that is a 
calculable amount away from the median data value (such as price, in the case of 
a price oracle).

After an asset type is added to an oracle, it can be requested for a price. This 
will trigger a two-phase update. The first will be an http call using an 
offchain worker, which goes out to an application programming interface (API) 
managed by the node provider. This will fetch the price and add it to the chain 
with a signed transaction (tx). Then, on the next block, there is a check to see 
if a threshold of minimum providers have given an answer for the particular 
data.

If the minimum threshold has been met then at the top of the block, the chain 
will prune any stale prices (if the price is a certain number of blocks old), 
and if the threshold is still met, it will medianize the prices and reward or 
slash providers if they are within a specific threshold of the median.  

After the prices are aggregated, the median price is then stored for that asset 
ID with a block number. Someone who is integrating with this pallet can 
determine their level of protection by using the block number.

For example, if a user is working with significant leverage, they can schedule 
all calls to require a future price with a blockchain hook like `on_init` (to be 
run at the top of a block). This will mean all users will get a future price 
that is not frontrunable. Further, where this would normally take 2 
transactions, a user can do this with one transaction on Substrate.

Alternatively, if the price requires a less strict risk model, an application 
can opt to use the current price if the price is under a certain number of 
blocks old.

**We believe that Apollo is a key step to kicking off innovation on Picasso, and 
look forward to seeing what projects are created therein.**
