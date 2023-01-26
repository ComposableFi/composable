# Technical Details

Oracles on Composable will utilize our Apollo oracle pallet. Apollo is:

- Built in a highly flexible way
- Able to have its use customized totally determined by the implementer
- Designed to be highly powerful and comes with great responsibility

As for the validity of the data, 
all oracle providers need to put down a stake and if their answers deviate by a threshold from the mean, 
they are slashed. Specific API methods are as follows:

- `Request_price (origin: OriginFor<T>, asset_id: u64)` and `do_request_price(who: &T::AccountId, asset_id: u64)`
  - The former is an extrinsic call, and the latter should be called while writing a pallet
  - If the price for an asset is deemed to be “stale” by your application, 
  based on the amount of blocks back that this is, 
  calling this will trigger a price fetch from the oracle on the given asset ID
  - From the pallet you can pass through an account and pay for your applications oracle updates (see cookbook)
- `prices(asset_id: u64) -> Price<BlockNumber>`
  - Price {price: u64, block: BlockNumber}
  - This will return the price and the block number the price was set at for the asset_id
  - This is replaced when a new price comes in

The oracle is meant to be used in two ways. 
Either you take the price presented at face value (not recommended)
or you check the last time the oracle was updated and schedule for a future price. 
Or, you can use a combination of both; for such “combo” prices:

It is up to the developer to determine their risk factors and set a proper threshold. 
For example, an app that does lending has a longer window that can accept an old price. 
For example, 5 blocks old. A perpetual future platform may want to schedule all calls for a future price.
A simple if statement can do this.
Scheduling a future call should be used with an `on_init` hook.
Note that using a future price should pre-lock funds that a user will take and
probably use some form of user set price banding.

## Technical Requirements for Running Oracles

The oracle is built into the base of the Composable parachain and heavily uses different blockchain hooks to “medianize" 
(calculate and bring towards the median) and update data. 
The goals of the oracle are to be flexible to use and offer integrators differing levels of security.

To become an oracle data provider, one must run a node. Apollo is a decentralized permissionless network. 
This means anyone can run a provider node and share in both the reward and the risk of slashing. 
First, a user needs to download the composable repo and follow the guide to build it in the readme.

There are two keys in the oracle setup: a signing key and a controller key. 
The signing key needs to be injected into the node and the controller key collects rewards. 
To start, first the controller key needs an adequate stake amount. 

It will call `set_signer(signer: T::AccountId)`. 

It is important to keep transaction fee funds in the signer account and manage that (subject to change). 
Next, the key needs to be inserted: 

`Const insert = await api.rpc.author.insertKey(“orac”, seed, publicKey)`. 

The offchain worker will always fallback to query `localhost 3001`.
If you want to change that, you can insert your URL with the following:
- `await api.rpc.offchain.localStorageSet("PERSISTENT", key, value)`
  - Where the key = “0x6f63772d75726c”
  - And the value is the hex form of the desired URL

In addition to running a node, oracle providers need to put down a stake of tokens. 
The stake is slashed if the oracle provider inputs data that is a calculable amount away from the median data value 
(such as price, in the case of a price oracle). Managing stake is accomplished as follows:

- `add_stake(stake: BalanceOf<T>)` to add more stake from any key to signing key
- Removing stake requires a time lock and involves two steps:
  - remove_stake() → sent from controller
  - reclaim_stake() → sent from controller

Querying for relevant staker information involves:

- asset_info(asset_id: u64) → AssetInfo
  - Asset info contains the min answers the max answers for oracles plus how close to the median in percent a provider 
needs to get to be rewarded and not slashed
- signer_to_controller(signer: T::AccountId) → T::AccountId
  - Will return the controller of a signer
- controller_to_signer(controller: T::AccountId) → T::AccountId
  - Will return the signer of a controller
- declared_withdrawls(signer: T::AccountId) → T::AccountId
  - If you have any pending withdrawals

After an asset type is added to an oracle, it can be requested for a price. This will trigger a two-phase update. 
The first will be an HTTP call using an off-chain worker, 
which goes to an application programming interface (API) managed by the node provider. 
The call will fetch the price and add it to a signed transaction (tx) chain. 
Then, on the next block, there is a check to see if a threshold of minimum providers has answered the particular data.

Supposing that the minimum threshold has been met, 
at the top of the block the chain will prune any stale prices (if the price is a certain number of blocks old), 
and if the threshold is still met, 
it will medianize the prices and reward or slash providers if they are within a specific threshold of the median.

After the prices are aggregated, the median price is then stored for that asset ID with a block number. 
Someone who is integrating with this pallet can determine their level of protection by using the block number.

For example, if users are working with significant leverage, 
they can schedule all calls to require a future price with a blockchain hook such as 
`on_init`(to be run at the top of a block). This will mean all users will get a future price that is not frontrunnable. 
Further, where this would typically take two transactions, 
on Substrate users can do this with one transaction. 
Alternatively, if the price requires a less strict risk model, 
an application can use the current price if the price is under a certain number of blocks old.