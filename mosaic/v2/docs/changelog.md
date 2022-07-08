# Changelog for Phase1 to Phase2

Phase2 has more components than phase1. Changes in existing contracts and new contracts are explained below.

In phase2 users can provide liquidity on [MosaicVault](/docs/MosaicVault.md) on each layer it is deployed on. Phase2 has many more layers connected and has the flexibility to add a new layer without changes in the contracts.

## Mosaic Holding [NEW]

This is a new contract which is used to hold the funds in the vaults. The vault contract does not hold the funds. Mosaic holding has the vault contract set as the `Mosaic_VAULT` role. This is done to have better control over the funds.

Mosaic holding has the feature to invest the funds via Compound/Aave protocol using the `invest` method which only the admin of the holding can call.

Read more about the holding contract [here](/docs/MosaicHolding.md).

## MosaicVault

There is only 1 vault in phase2 called the [MosaicVault](/docs/MosaicVault.md). This vault has all the features needed to transfer tokens and provide liquidity.

**Changes in methods for L1Vault**

```
moveFunds -> removed
```

```
deposit ->  provideActiveLiquidity
            provideActiveEthLiquidity
            providePassiveLiquidity
            providePassiveEthLiquidity
```

```
withdraw -> withdrawLiquidityRequest
```

The user calls `withdrawLiquidityRequest` then the relayer calls `withdrawLiquidity`

```
sendAssetPolygon -> removed
sendAssetArbitrum -> removed
```

Liquidity rebalancing happens off chain.

**Changes in methods for l2vault**

```
depositERC20 -> transferERC20ToLayer
                transferETHToLayer
```

```
withdrawTo -> withdrawTo
```

Same method but with added params

```
saveFunds -> removed
```

new methods added to MosaicHolding `startSaveFunds` and `executeSaveFunds`

```
unlockInTransferFunds -> removed
unlockFunds -> removed
```

The funds locking mechanism has been refactored.

```
redeemTransferFunds
redeemLiquidityFunds
```

Now the relayer calls `addValidClaimRoots` to add valid claims on the vault.

user can then call new methods `redeemTransferFunds` and `redeemLiquidityFunds` to claim the funds back.

**Changes in events on L1Vault**

```
FundsMoved -> LiquidityMoved
```

`LiquidityMoved` is emitted in the Holding contract

```
Deposit ->  DepositPassiveLiquidity
            DepositActiveLiquidity
```

```
Withdrawal ->   WithdrawRequest
                LiquidityWithdrawn
```

```
AssetSend -> removed.
```

**Changes in events on L2Vault**

```
DepositCompleted -> TransferInitiated
```

```
WithdrawalCompleted -> WithdrawalCompleted
```

```
TransferFundsUnlocked -> removed
```

```
FundsUnlocked ->  ClaimRootAdded
                  TransferFundsRefunded
```

```
FeeTaken -> FeeTaken
```

**New events**

```
FundsDigested
LiquidityRefunded
TokenCreated  // emitted when a ReceiptToken is deployed
```

## IOU and Receipt Tokens [NEW]

Phase2 introduces the novel concept of claimable receipt tokens for the liquidity provided. User can provides liquidity in the MosaicVault and they can get back either
Receipt tokens for passive liquidity or get IOU tokens for active liquidity. Read more about it [here](/docs/tokens.md).

## NFT Vault [NEW]

A contract called `Summoner` is the NFT vault. Summoner will be deployed on each layer and is used to transfer NFTs between layers. The user will deposit NFT in the source vault. The relayer will then pass on the information to the destination vault. On the destination vault the relayer calls `summonNFT` to mint a new `Mosaic NFT` and transfer it to the user. Read more about the Summoner [here](/docs/nfts.md).

[home](/readme.md)
