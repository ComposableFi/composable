<!-- AUTOMATICALLY GENERATED -->
<!-- Generated at 2022-09-05T18:35:35.110223Z -->

# Airdrop Pallet Extrinsics

## Create Airdrop

[`create_airdrop`](https://dali.devnets.composablefinance.ninja/doc/pallet_airdrop/pallet/enum.Call.html#variant.create_airdrop)

Create a new Airdrop. This requires that the user puts down a stake in PICA.

If `start_at` is `Some(MomentOf<T>)` and the `MomentOf<T>` is greater than the current
block, the Airdrop will be scheduled to start automatically.

Can be called by any signed origin.

### Parameter Sources

* `start_at` - user provided, optional
* `vesting_schedule` - user provided

### Emits

* `AirdropCreated`
* `AirdropStarted`

### Errors

* `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
* `AirdropAlreadyStarted` - The Airdrop has already started or has been scheduled to
  start
* `BackToTheFuture` - The provided `start` has already passed

## Add Recipient

[`add_recipient`](https://dali.devnets.composablefinance.ninja/doc/pallet_airdrop/pallet/enum.Call.html#variant.add_recipient)

Add one or more recipients to the Airdrop, specifying the token amount that each
provided address will receive.

Only callable by the origin that created the Airdrop.

### Parameter Sources

* `airdrop_id` - user selected, provided by the system
* `recipients` - user provided

### Emits

* `RecipientsAdded`

### Errors

* `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
* `NotAirdropCreator` - Signer of the origin is not the creator of the Airdrop

## Remove Recipient

[`remove_recipient`](https://dali.devnets.composablefinance.ninja/doc/pallet_airdrop/pallet/enum.Call.html#variant.remove_recipient)

Remove a recipient from an Airdrop.

Only callable by the origin that created the Airdrop.

### Parameter Sources

* `airdrop_id` - user selected, provided by the system
* `recipient` - user selected, provided by the system

### Emits

* `RecipientRemoved`
* `AirdropEnded`

### Errors

* `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
* `NotAirdropCreator` - Signer of the origin is not the creator of the Airdrop
* `RecipientAlreadyClaimed` - The recipient has already began claiming their funds.
* `RecipientNotFound` - No recipient associated with the `identity` could be found.

## Enable Airdrop

[`enable_airdrop`](https://dali.devnets.composablefinance.ninja/doc/pallet_airdrop/pallet/enum.Call.html#variant.enable_airdrop)

Start an Airdrop.

Only callable by the origin that created the Airdrop.

### Parameter Sources

* `airdrop_id` - user selected, provided by the system

### Emits

* `AirdropStarted`

### Errors

* `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
* `AirdropAlreadyStarted` - The Airdrop has already started or has been scheduled to
  start
* `BackToTheFuture` - The provided `start` has already passed
* `NotAirdropCreator` - Signer of the origin is not the creator of the Airdrop

## Disable Airdrop

[`disable_airdrop`](https://dali.devnets.composablefinance.ninja/doc/pallet_airdrop/pallet/enum.Call.html#variant.disable_airdrop)

Stop an Airdrop.

Only callable by the origin that created the Airdrop.

### Parameter Sources

* `airdrop_id` - user selected, provided by the system

### Emits

* `AirdropEnded`

### Errors

* `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
* `NotAirdropCreator` - Signer of the origin is not the creator of the Airdrop

## Claim

[`claim`](https://dali.devnets.composablefinance.ninja/doc/pallet_airdrop/pallet/enum.Call.html#variant.claim)

Claim recipient funds from an Airdrop.

If no more funds are left to claim, the Airdrop will be removed.

Callable by any unsigned origin.

### Parameter Sources

* `airdrop_id` - user selected, provided by the system
* `reward_account` - user provided
* `proof` - calculated by the system (requires applicable signing)

### Emits

* `AirdropEnded`

### Errors

* `AirdropDoesNotExist` - No Airdrop exist that is associated 'airdrop_id'
* `AirdropIsNotEnabled` - The Airdrop has not been enabled
* `AssociatedWithAnotherAccount` - Associated with a different account
* `ArithmeticError` - Overflow while totaling claimed funds
* `InvalidProof`
* `RecipientNotFound` - No recipient associated with the `identity` could be found.
