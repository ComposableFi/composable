# Liquid Crowdloan Module

### Overview

* The goal of this pallet is to allow for an extra promise of future tx fees from the chain to crowdloan participants.

### Usage

* A percent of tx fees need to go to the pallet's account
* Call initalize from sudo or from a desiered origin.
  * This will mint the desiered tokens specified. Best to make it 1:1 for the ksm in the crowdloan
* Fees will accumilate into the pallet and users can use these receipt tokens as over the usage time
* When the agreed time period has ran out, upgrade the chain to remove sending a % of the fees to the pallet, then call make_claimable.
* users will now be able to burn their receipt tokens and claim the tx fees promised through the claim function.
