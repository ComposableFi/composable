<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-06-25T22:31:58.560425458Z -->

# Lending Pallet Extrinsics

## Create Market

[`create_market`](https://dali.devnets.composablefinance.ninja/doc/pallet_lending/pallet/enum.Call.html#variant.create_market)

Create a new lending market.

* `origin` : Sender of this extrinsic. Manager for new market to be created. Can pause
  borrow operations.
* `input`   : Borrow & deposits of assets, persentages.

`origin` irreversibly pays `T::OracleMarketCreationStake`.

## Update Market

[`update_market`](https://dali.devnets.composablefinance.ninja/doc/pallet_lending/pallet/enum.Call.html#variant.update_market)

owner must be very careful calling this

## Deposit Collateral

[`deposit_collateral`](https://dali.devnets.composablefinance.ninja/doc/pallet_lending/pallet/enum.Call.html#variant.deposit_collateral)

Deposit collateral to market.

* `origin` : Sender of this extrinsic.
* `market` : Market index to which collateral will be deposited.
* `amount` : Amount of collateral to be deposited.

## Withdraw Collateral

[`withdraw_collateral`](https://dali.devnets.composablefinance.ninja/doc/pallet_lending/pallet/enum.Call.html#variant.withdraw_collateral)

Withdraw collateral from market.

* `origin` : Sender of this extrinsic.
* `market_id` : Market index from which collateral will be withdraw.
* `amount` : Amount of collateral to be withdrawn.

## Borrow

[`borrow`](https://dali.devnets.composablefinance.ninja/doc/pallet_lending/pallet/enum.Call.html#variant.borrow)

Borrow asset against deposited collateral.

* `origin` : Sender of this extrinsic. (Also the user who wants to borrow from market.)
* `market_id` : Market index from which user wants to borrow.
* `amount_to_borrow` : Amount which user wants to borrow.

## Repay Borrow

[`repay_borrow`](https://dali.devnets.composablefinance.ninja/doc/pallet_lending/pallet/enum.Call.html#variant.repay_borrow)

Repay part or all of the borrow in the given market.

### Parameters

* `origin` : Sender of this extrinsic. (Also the user who repays beneficiary's borrow.)
* `market_id` : \[`MarketIndex`\] of the market being repaid.
* `beneficiary` : \[`AccountId`\] of the account who is in debt to (has borrowed assets
  from) the market. This can be same or different from the `origin`, allowing one
  account to pay off another's debts.
* `amount`: The amount to repay. See \[`RepayStrategy`\] for more information.

## Liquidate

[`liquidate`](https://dali.devnets.composablefinance.ninja/doc/pallet_lending/pallet/enum.Call.html#variant.liquidate)

Check if borrows for the `borrowers` accounts are required to be liquidated, initiate
liquidation.

* `origin` : Sender of this extrinsic.
* `market_id` : Market index from which `borrower` has taken borrow.
* `borrowers` : Vector of borrowers accounts' ids.
