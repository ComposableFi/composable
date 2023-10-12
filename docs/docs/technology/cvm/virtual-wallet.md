# Virtual wallet

Virtual wallet is allows user to manage funds on several chains using only one native account on one chain (on wallet).

Virtual wallet is not separate contract, but possible interactions enabled by CVM and MANTIS which allow to handle user intents across domains using only one signature.

## Inventing virtual wallet

### Problem

`Alice` has only `Ethereum`` wallet and able to pay gas fees here.

`Bob`` has only `Cosmos Hub`` wallet and able to pay gas fees here.

Alice wants some `Atom` for `ETH`.

Bob wants some `ETH` for `Atom`.

Price for Atom and ETH from Bob and Alice are good for them.
Amount of `Atom` Alice wants is little bit above than Bob has.   

**How can we make this exchange happen?**

### General solution

In order to make exchange happen, first we must ensure that when it happens (atomically) both Alice and Bob assets are available at same time on same domain.
How that can be made? 

Alice and Bob escrow their tokens on their source chains, Ethereum and Cosmos Hub, and bridge information about that escrow and its ownership to single domain.

Let use 3rd chain, let this domain Composable.

**We just invented IBC ICS-20 or Polkadot XCM reserve transfer**

So when information about tokens escrow arrived to Composable, token can be atomically exchanged with each other.

After swap users may consider:
- retaining assets on Composable to settle next intention
- move to originating chain (where they have wallet)
- move to source chain of assets (were assets was minted first)
- move to any other chain

In all this cases users want to retain control over the assets.

How he can achieve that?

So here we invent CVM Executor, which works as cross chain account.

### Account creation

When user sends message from his domain to other, 
CVM creates executor per originating `signature + chain` pair.

So if user moved funds from Ethereum to Composable via CVM, 
he can send at same time or as next message later, 
message to put orders or exchange on behalf of user to any CVM executor he owns on any chain.

CVM ensures that only specific signature on specific chain could issue funds management transactions.

**What if users gets his native wallet on Composable or wants to allow other accounts to manage his funds on any chain he send these?**

User sends CVM program to add proxy(delegation) account to CVM Executor account, 
which will allow native account on Composable to manage, with some limits,
funds and issue operation from users's CVM executor.

Any chain were CVM exists and there are standards for proxy(delegation), can manage funds this way.

User also may delegate to other origins of CVM Executors. 

**But users does not sees tokens as owned in Metamask anymore?**

Indeed, until wallets on chains will not support CVM, users must be satisfied with CVM specific wallet and dashboard, aggregating their funds on all chains.


**Is wallet custodial?**

Wallet is non custodial. Instance of CVM executor is created per user signature. Funds always in CVM executor fully owned by user or in flight over bridge.

CVM contracts are managed by cross chain DAO.

## Virtual Wallet value

Virtual wallet allows set of operations happen with less risk, more gas efficient and increase liquidly usage.

- Instead of cross chain swap multiblock swap which may fail, do local atomic exchange and than cross chain transfer (which less likely to fail).

- Replace mutlihop transfer and multihop swap with location operation on assets already accessible in wallets delegated liquidity for that, for example via CoW order.

- Also allow to fund management to increase path prefix or reduce it when it is useful.

- Bundle many small cross chain operation into one.

**Example**

Alice wants to get ATOM Osmosis for ETH on Neutron.

Bob wants to get ETH on Ethereum for Cosmos Hub ATOM.

They both escrow input tokens and make them appear on Composable.

Solvers find that they can:
- they can give Bob ETH on Composable
- they can give Alice ATOM on Composable.

And provide transfer path to settle token on desired chain.

Direct exchange on same and pure transfer are more safe and cheaper,
than allowing someone to transfer and swap on some other chain.