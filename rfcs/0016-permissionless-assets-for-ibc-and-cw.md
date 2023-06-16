# Overview

To transfer any asset from any chain connected via Centauri to DotSama, this asset needs to be defined in Picasso or Composable assets registry. This operation needs to be permissionless to allow free flow of assets between chains. Moreover, permissionless asset creation needs to support CosmWasm contracts which have functionality of creating, minting and burning tokens. There 2 types of permissionless assets: mintable assets(needed for cosmwasm) and nonmintable(for bridging). This document is an approach how to design permissionless assets and avoid potential risks.

## What we currently have

Currently we support asset creation in AssetsRegistry pallet via Root(Governance). During the permissioned asset creation process, user needs to specify asset's `protocol_id, nonce, name, symbol, decimals, location, ED, ratio`. 

## What we want to achieve

Permissionless assets need to follow the same structure as Permissioned assets and be part of Assets Registry. We want new assets to abide and not break interfaces we have via AssetsRegistry pallet and AssetsTransactorRouter pallet. Permissionless assets will also be modifiable via Root. For permissionless assets' creation a separate pallet is used to become a wrapper for AssetsRegistry: `permissionless_assets`. To create a new asset, user will need to pay registration fee in PICA to register a new asset.

This wrapper will track the creator of the asset and will allow the admin to modify the asset's metadata. The CW assets will also use the admin to allow the account to mint and burn token's assets.

## Potential Risks of Permissionless assets

* **ID manipulation**: Users should not be able to manipulate `protocol_id` and `nonce` values to create ids that conflict with other pallets' ids.
* **Ratio Manipulation**: Users should not be able to set any ratio value for any asset to be used for BYOG and making XCM transfers with self-set fees. 
* **ED Manipulation**: Users should not be able to set any Existential Deposits for their assets to avoid bloating the chain's storage


## Solutions to the above risks

### ID manipulation

permissionless_assets will have a predefined `protocol_id` and `StorageValue` to increment `nonce` on a new asset's creation to avoid users manipulating new assets' ids and making them conflicted with other pallets' protocol ids.

### Ratio Manipulation

Ratio will be set to `None` for all permissionless assets. It will then be set to some value via governance if asset is decided to be good for BYOG and paying for XCM transfers. Until then, 2 assets will be required to be sent via XCM: permissionless asset + asset to pay for transaction.

### ED Manipulation

Existential Deposit(ED) is a safeguard against bloating storage of the chain. If a user transfers a nonvaluable asset and sets its ED to 0 or 1, this user will be able to affect performance of the parachain by transferring this asset to many accounts. Thus, to avoid this potential attack vector, we will use Asset Hub (formerly Statemine) approach of non-sufficient assets: a user to be able to recieve permissionless asset needs to hold ED in PICA on Picasso and LAYR on Composable parachains. This approach will limit the growth of the storage. 

The chain needs to make sure that permissionless assets can be transferred to accounts that dont have ED in parachain's native token. It is also required to fail transfers that will result in drop of native token below ED if this account has nonsufficient tokens. [Reference](https://substrate.stackexchange.com/questions/2447/influence-of-existential-deposits-on-account-assets)

#### Non-sufficient assets implementation concerns

#### Recieving Account doesnt hold pica during bridging
**Potential solution**: relayer will send PICA to the reciever in case reciever doesnt have it. In case bridged asset is FlatFee asset, additional fee equal to ED in PICA will be charged from the transferred amount. Otherwise, percentage fee will be applied(To be configured)

#### orml tokens pallet EDs
Currently assets are being stored in orml tokens pallet which require Existential Deposits for stored assets. 

**Potential Solution**: for non sufficient assets still provide Existential Deposit parameter which is similar to minimum balance in Asset Hub implementation. Then the Exitstential logic for orml tokens pallet will stay the same because account wont be able to hold a non sufficient asset without ED in parachain's native token. This will require a new field in AssetsRegistry: is_sufficient. All permissionless assets will have this field to be false by default. Assets created via permissioned extrinsic will allow to choose if asset is sufficient or not. Asset's sufficiency can be changed by governance.


## Updated extrinsics

Extrinsic to create asset via Root  `assets_registry.register_asset(name, symbol, decimals, location, ED, Ratio, is_sufficient)`.

Extrinsic to create asset for bridging is `permissionless_assets.create_bridge_asset(name, symbol, decimals, location, ED)`.

Extrinsic to create asset for CosmWasm is `permissionless_assets.create_cw_asset(name, symbol, decimals, ED)`.

## permissionless_assets Storage Items

#### For updating assets metadata

`pub type Admins<T:Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::AccountId, OptionQuery>;`

extrinsic `permissionless_assets.update_asset`  will check that caller of the extrinsic is an assets' admin. This storage item will be populated on `create_bridge_asset` extrinsic call.

#### For minting/buring

`pub type MintAdmin<T:Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::AccountId, OptionQuery>;`

extrinsics `permissionless_assets.mint` and `permissionless_assets.burn` will check that caller of the extrinsic is an assets' admin and if so will use `AssetsTransactorRouter`'s `fungibles::Mutate` trait's `mint_into` or `burn_from` methods. This storage item will be populated on `create_cw_asset` extrinsic call.

# References

* [Statemine Ed](https://substrate.stackexchange.com/questions/5917/do-assets-from-the-asset-pallet-on-statemine-mint-have-an-existential-deposi/5923#5923)
* [Control sufficiency via system pallet](https://substrate.stackexchange.com/questions/2447/influence-of-existential-deposits-on-account-assets)
* [Account Structure to Implement nonsufficient assets](https://docs.substrate.io/reference/account-data-structures/)
* [More Statemine ED details](https://substrate.stackexchange.com/questions/6522/does-holding-only-sufficient-asset-in-statemint-imply-there-is-no-ed-for-the-acc/6524#6524)
* [ED dust cleanup](https://substrate.stackexchange.com/questions/3482/how-does-substrate-clean-up-accounts-whose-balance-is-below-the-existential-depo)