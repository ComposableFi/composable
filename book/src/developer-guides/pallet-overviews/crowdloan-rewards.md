# Crowdloan Rewards

## Overveiw

The Crowdloan Rewards pallet allows contributors to claim their rewards.

### Unsigned Transactions

Before associating their account, users will not have the funds necessary to pay 
transaction fees. To get around this, unsigned transactions are allowed but must 
be validated.

For a transaction to be validated, these conditions must be met:

* Transaction contains a reward ID and valid proof

* The pallet has been initailized

* The reward account contained in the call has not been associated

* The remote account (from ETH or relay chain) is retrievable from the proof

* The reward account has a positive reward balance availiable to claim

## Rewards Workflow

After reward accounts have been populated and the pallet has been initialized, 
contributors can start to claim their rewards. A percentage of the reward is 
paid at the first claim, and the remaining percentage is vested over an 
arbitrary period of time split into Vesting Partitions.

**Rewards are claimed by the following steps**

1. An [`AdminOrigin`](#adminorigin) sets up and populates the reward accounts, 
  consisting of a vector of (PublicKey, Amount, VestingPeriod). The PublicKey is 
  either coming from the relay chain (Kusama in this case) or from ETH.

2. Since the users don't own any funds, the first claim has to be made using our 
  service so that we can [`associate`](#associate) and fund the account. The 
  association results in the remote account being associated with the reward 
  account. This association automatically triggers the first claim, the claim 
  results in the first payment being distributed to the newly associated Picasso 
  account.

3. Once the first claim has been made, the user has to wait until the next 
  [`VestingStep`](#vestingstep). After having waited for the vesting partition. 
  The user is able to either [`associate`](#associate) a new account or directly 
  [`claim`](#claim) using their already associated Picasso account. This can be 
  repeated until the contributor has claimed all of their reward.

## Pallet Extrinsics

### Initialize

`initailize`

Initializes the pallet at the current transaction block.

Must come from an [`AdminOrigin`](#adminorigin)

### Initialize At

`initailize_at`

Initializes the pallet at the given transaction block.

Must come from an [`AdminOrigin`](#adminorigin)

### Populate 

`populate`

Populates pallet by adding more rewards to an account.

Must come from an [`AdminOrigin`](#adminorigin)

Can be called multible times. Idempotent.

Can only be called before the pallet has been initialized with a call to 
`initailize` or `initailize_at`

### Associate

`associate`

Associates a reward account and makes the first claim.

Valid proof must be provided for the reward account.

If the proof is valid and the transaction is succussful, no fees are applied.

This extrinsic expects unsigned transactions.

### Claim

`claim`

Claim a reward from the associated reward account.

A previous call to `associate` should have been made.

If the transaction is succussful, no fees are applied.

## Pallet Configuration

### Event

### Balance

### RewardAsset

The reward asset used to transfer the rewards.

### AdminOrigin

The origin that is allowed to initailize the pallet.

### Convert

A function for converting block numbers to [`Balance`](#balace)

### RelayChainAccountId

The relay chain account identifier.

### InitialPayment

The upfront liquidity that is unlocked at the first claim.

### VestingStep

The number of blocks a fragment of the reward is vested.

### Prefix

The arbitrary prefix that is used for the proof.

### WeightInfo 

The implementation of extrinsic weights.

### PalletId

The unique identifier of this pallet.
