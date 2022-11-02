// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/events';

import type { ComposableTraitsDefiCurrencyPairCurrencyId } from '@composable/types/interfaces/common';
import type { CommonMosaicRemoteAssetId, ComposableSupportEthereumAddress, ComposableTraitsAccountProxyProxyType, ComposableTraitsCallFilterCallFilterEntry, ComposableTraitsLendingUpdateInput, ComposableTraitsTimeTimeReleaseFunction, ComposableTraitsVestingVestingSchedule, ComposableTraitsVestingVestingScheduleIdSet, ComposableTraitsXcmAssetsXcmAssetLocation, FrameSupportScheduleLookupError, PalletCosmwasmContractInfo, PalletCosmwasmEntryPoint, PalletCrowdloanRewardsModelsRemoteAccount, PalletDemocracyVoteAccountVote, PalletDutchAuctionSellOrder, PalletIbcErrorsIbcError, PalletIbcEventsIbcEvent, PalletMosaicAmmSwapInfo, PalletMosaicDecayBudgetPenaltyDecayer, PalletMosaicNetworkInfo, PalletStakingRewardsRewardAccumulationHookError } from '@composable/types/interfaces/crowdloanRewards';
import type { PalletDemocracyVoteThreshold } from '@composable/types/interfaces/democracy';
import type { ComposableTraitsDexFee } from '@composable/types/interfaces/pablo';
import type { ApiTypes, AugmentedEvent } from '@polkadot/api-base/types';
import type { BTreeMap, Bytes, Null, Option, Result, Struct, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256, Percent } from '@polkadot/types/interfaces/runtime';
import type { FrameSupportTokensMiscBalanceStatus, FrameSupportWeightsDispatchInfo, PalletMultisigTimepoint, SpRuntimeDispatchError, XcmV1MultiAsset, XcmV1MultiLocation, XcmV1MultiassetMultiAssets, XcmV2Response, XcmV2TraitsError, XcmV2TraitsOutcome, XcmV2Xcm, XcmVersionedMultiAssets, XcmVersionedMultiLocation } from '@polkadot/types/lookup';

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module '@polkadot/api-base/types/events' {
  interface AugmentedEvents<ApiType extends ApiTypes> {
    assetsRegistry: {
      AssetRegistered: AugmentedEvent<ApiType, [assetId: u128, location: ComposableTraitsXcmAssetsXcmAssetLocation], { assetId: u128, location: ComposableTraitsXcmAssetsXcmAssetLocation }>;
      AssetUpdated: AugmentedEvent<ApiType, [assetId: u128, location: ComposableTraitsXcmAssetsXcmAssetLocation], { assetId: u128, location: ComposableTraitsXcmAssetsXcmAssetLocation }>;
      MinFeeUpdated: AugmentedEvent<ApiType, [targetParachainId: u32, foreignAssetId: ComposableTraitsXcmAssetsXcmAssetLocation, amount: Option<u128>], { targetParachainId: u32, foreignAssetId: ComposableTraitsXcmAssetsXcmAssetLocation, amount: Option<u128> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    balances: {
      /**
       * A balance was set by root.
       **/
      BalanceSet: AugmentedEvent<ApiType, [who: AccountId32, free: u128, reserved: u128], { who: AccountId32, free: u128, reserved: u128 }>;
      /**
       * Some amount was deposited (e.g. for transaction fees).
       **/
      Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * An account was removed whose balance was non-zero but below ExistentialDeposit,
       * resulting in an outright loss.
       **/
      DustLost: AugmentedEvent<ApiType, [account: AccountId32, amount: u128], { account: AccountId32, amount: u128 }>;
      /**
       * An account was created with some free balance.
       **/
      Endowed: AugmentedEvent<ApiType, [account: AccountId32, freeBalance: u128], { account: AccountId32, freeBalance: u128 }>;
      /**
       * Some balance was reserved (moved from free to reserved).
       **/
      Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some balance was moved from the reserve of the first account to the second account.
       * Final argument indicates the destination balance type.
       **/
      ReserveRepatriated: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus], { from: AccountId32, to: AccountId32, amount: u128, destinationStatus: FrameSupportTokensMiscBalanceStatus }>;
      /**
       * Some amount was removed from the account (e.g. for misbehavior).
       **/
      Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Transfer succeeded.
       **/
      Transfer: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128], { from: AccountId32, to: AccountId32, amount: u128 }>;
      /**
       * Some balance was unreserved (moved from reserved to free).
       **/
      Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Some amount was withdrawn from the account (e.g. for transaction fees).
       **/
      Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    bondedFinance: {
      /**
       * A new bond has been registered.
       **/
      NewBond: AugmentedEvent<ApiType, [offerId: u128, who: AccountId32, nbOfBonds: u128], { offerId: u128, who: AccountId32, nbOfBonds: u128 }>;
      /**
       * A new offer has been created.
       **/
      NewOffer: AugmentedEvent<ApiType, [offerId: u128, beneficiary: AccountId32], { offerId: u128, beneficiary: AccountId32 }>;
      /**
       * An offer has been cancelled by the `AdminOrigin`.
       **/
      OfferCancelled: AugmentedEvent<ApiType, [offerId: u128], { offerId: u128 }>;
      /**
       * An offer has been completed.
       **/
      OfferCompleted: AugmentedEvent<ApiType, [offerId: u128], { offerId: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    callFilter: {
      /**
       * Paused transaction
       **/
      Disabled: AugmentedEvent<ApiType, [entry: ComposableTraitsCallFilterCallFilterEntry], { entry: ComposableTraitsCallFilterCallFilterEntry }>;
      /**
       * Unpaused transaction
       **/
      Enabled: AugmentedEvent<ApiType, [entry: ComposableTraitsCallFilterCallFilterEntry], { entry: ComposableTraitsCallFilterCallFilterEntry }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    collatorSelection: {
      CandidateAdded: AugmentedEvent<ApiType, [accountId: AccountId32, deposit: u128], { accountId: AccountId32, deposit: u128 }>;
      CandidateRemoved: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
      NewCandidacyBond: AugmentedEvent<ApiType, [bondAmount: u128], { bondAmount: u128 }>;
      NewDesiredCandidates: AugmentedEvent<ApiType, [desiredCandidates: u32], { desiredCandidates: u32 }>;
      NewInvulnerables: AugmentedEvent<ApiType, [invulnerables: Vec<AccountId32>], { invulnerables: Vec<AccountId32> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    cosmwasm: {
      AdminCleared: AugmentedEvent<ApiType, [contract: AccountId32, oldAdmin: Option<AccountId32>], { contract: AccountId32, oldAdmin: Option<AccountId32> }>;
      AdminUpdated: AugmentedEvent<ApiType, [contract: AccountId32, oldAdmin: Option<AccountId32>, newAdmin: AccountId32], { contract: AccountId32, oldAdmin: Option<AccountId32>, newAdmin: AccountId32 }>;
      Emitted: AugmentedEvent<ApiType, [contract: AccountId32, ty: Bytes, attributes: Vec<ITuple<[Bytes, Bytes]>>], { contract: AccountId32, ty: Bytes, attributes: Vec<ITuple<[Bytes, Bytes]>> }>;
      Executed: AugmentedEvent<ApiType, [contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, data: Option<Bytes>], { contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, data: Option<Bytes> }>;
      ExecutionFailed: AugmentedEvent<ApiType, [contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, error: Bytes], { contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, error: Bytes }>;
      Instantiated: AugmentedEvent<ApiType, [contract: AccountId32, info: PalletCosmwasmContractInfo], { contract: AccountId32, info: PalletCosmwasmContractInfo }>;
      Migrated: AugmentedEvent<ApiType, [contract: AccountId32, to: u64], { contract: AccountId32, to: u64 }>;
      Uploaded: AugmentedEvent<ApiType, [codeHash: H256, codeId: u64], { codeHash: H256, codeId: u64 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    council: {
      /**
       * A motion was approved by the required threshold.
       **/
      Approved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
      /**
       * A proposal was closed because its threshold was reached or after its duration was up.
       **/
      Closed: AugmentedEvent<ApiType, [proposalHash: H256, yes: u32, no: u32], { proposalHash: H256, yes: u32, no: u32 }>;
      /**
       * A motion was not approved by the required threshold.
       **/
      Disapproved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
      /**
       * A motion was executed; result will be `Ok` if it returned without error.
       **/
      Executed: AugmentedEvent<ApiType, [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>], { proposalHash: H256, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A single member did some action; result will be `Ok` if it returned without error.
       **/
      MemberExecuted: AugmentedEvent<ApiType, [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>], { proposalHash: H256, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A motion (given hash) has been proposed (by given account) with a threshold (given
       * `MemberCount`).
       **/
      Proposed: AugmentedEvent<ApiType, [account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32], { account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32 }>;
      /**
       * A motion (given hash) has been voted on by given account, leaving
       * a tally (yes votes and no votes given respectively as `MemberCount`).
       **/
      Voted: AugmentedEvent<ApiType, [account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32], { account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    councilMembership: {
      /**
       * Phantom member, never used.
       **/
      Dummy: AugmentedEvent<ApiType, []>;
      /**
       * One of the members' keys changed.
       **/
      KeyChanged: AugmentedEvent<ApiType, []>;
      /**
       * The given member was added; see the transaction for who.
       **/
      MemberAdded: AugmentedEvent<ApiType, []>;
      /**
       * The given member was removed; see the transaction for who.
       **/
      MemberRemoved: AugmentedEvent<ApiType, []>;
      /**
       * The membership was reset; see the transaction for who the new set is.
       **/
      MembersReset: AugmentedEvent<ApiType, []>;
      /**
       * Two members were swapped; see the transaction for who.
       **/
      MembersSwapped: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    crowdloanRewards: {
      /**
       * A remote account has been associated with a reward account.
       **/
      Associated: AugmentedEvent<ApiType, [remoteAccount: PalletCrowdloanRewardsModelsRemoteAccount, rewardAccount: AccountId32], { remoteAccount: PalletCrowdloanRewardsModelsRemoteAccount, rewardAccount: AccountId32 }>;
      /**
       * A claim has been made.
       **/
      Claimed: AugmentedEvent<ApiType, [remoteAccount: PalletCrowdloanRewardsModelsRemoteAccount, rewardAccount: AccountId32, amount: u128], { remoteAccount: PalletCrowdloanRewardsModelsRemoteAccount, rewardAccount: AccountId32, amount: u128 }>;
      /**
       * The crowdloan has been initialized or set to initialize at some time.
       **/
      Initialized: AugmentedEvent<ApiType, [at: u64], { at: u64 }>;
      /**
       * The crowdloan was successfully initialized, but with excess funds that won't be
       * claimed.
       **/
      OverFunded: AugmentedEvent<ApiType, [excessFunds: u128], { excessFunds: u128 }>;
      /**
       * A portion of rewards have been unlocked and future claims will not have locks
       **/
      RewardsUnlocked: AugmentedEvent<ApiType, [at: u64], { at: u64 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    cumulusXcm: {
      /**
       * Downward message executed with the given outcome.
       * \[ id, outcome \]
       **/
      ExecutedDownward: AugmentedEvent<ApiType, [U8aFixed, XcmV2TraitsOutcome]>;
      /**
       * Downward message is invalid XCM.
       * \[ id \]
       **/
      InvalidFormat: AugmentedEvent<ApiType, [U8aFixed]>;
      /**
       * Downward message is unsupported version of XCM.
       * \[ id \]
       **/
      UnsupportedVersion: AugmentedEvent<ApiType, [U8aFixed]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    currencyFactory: {
      RangeCreated: AugmentedEvent<ApiType, [range: {
    readonly current: u128;
    readonly end: u128;
  } & Struct], { range: {
    readonly current: u128;
    readonly end: u128;
  } & Struct }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    democracy: {
      /**
       * A proposal_hash has been blacklisted permanently.
       **/
      Blacklisted: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
      /**
       * A referendum has been cancelled.
       **/
      Cancelled: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
      /**
       * An account has delegated their vote to another account.
       **/
      Delegated: AugmentedEvent<ApiType, [who: AccountId32, target: AccountId32], { who: AccountId32, target: AccountId32 }>;
      /**
       * A proposal has been enacted.
       **/
      Executed: AugmentedEvent<ApiType, [refIndex: u32, result: Result<Null, SpRuntimeDispatchError>], { refIndex: u32, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * An external proposal has been tabled.
       **/
      ExternalTabled: AugmentedEvent<ApiType, []>;
      /**
       * A proposal has been rejected by referendum.
       **/
      NotPassed: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
      /**
       * A proposal has been approved by referendum.
       **/
      Passed: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
      /**
       * A proposal could not be executed because its preimage was invalid.
       **/
      PreimageInvalid: AugmentedEvent<ApiType, [proposalHash: H256, refIndex: u32], { proposalHash: H256, refIndex: u32 }>;
      /**
       * A proposal could not be executed because its preimage was missing.
       **/
      PreimageMissing: AugmentedEvent<ApiType, [proposalHash: H256, refIndex: u32], { proposalHash: H256, refIndex: u32 }>;
      /**
       * A proposal's preimage was noted, and the deposit taken.
       **/
      PreimageNoted: AugmentedEvent<ApiType, [proposalHash: H256, who: AccountId32, deposit: u128], { proposalHash: H256, who: AccountId32, deposit: u128 }>;
      /**
       * A registered preimage was removed and the deposit collected by the reaper.
       **/
      PreimageReaped: AugmentedEvent<ApiType, [proposalHash: H256, provider: AccountId32, deposit: u128, reaper: AccountId32], { proposalHash: H256, provider: AccountId32, deposit: u128, reaper: AccountId32 }>;
      /**
       * A proposal preimage was removed and used (the deposit was returned).
       **/
      PreimageUsed: AugmentedEvent<ApiType, [proposalHash: H256, provider: AccountId32, deposit: u128], { proposalHash: H256, provider: AccountId32, deposit: u128 }>;
      /**
       * A proposal got canceled.
       **/
      ProposalCanceled: AugmentedEvent<ApiType, [propIndex: u32], { propIndex: u32 }>;
      /**
       * A motion has been proposed by a public account.
       **/
      Proposed: AugmentedEvent<ApiType, [proposalIndex: u32, deposit: u128], { proposalIndex: u32, deposit: u128 }>;
      /**
       * An account has secconded a proposal
       **/
      Seconded: AugmentedEvent<ApiType, [seconder: AccountId32, propIndex: u32], { seconder: AccountId32, propIndex: u32 }>;
      /**
       * A referendum has begun.
       **/
      Started: AugmentedEvent<ApiType, [refIndex: u32, threshold: PalletDemocracyVoteThreshold], { refIndex: u32, threshold: PalletDemocracyVoteThreshold }>;
      /**
       * A public proposal has been tabled for referendum vote.
       **/
      Tabled: AugmentedEvent<ApiType, [proposalIndex: u32, deposit: u128, depositors: Vec<AccountId32>], { proposalIndex: u32, deposit: u128, depositors: Vec<AccountId32> }>;
      /**
       * An account has cancelled a previous delegation operation.
       **/
      Undelegated: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      /**
       * An external proposal has been vetoed.
       **/
      Vetoed: AugmentedEvent<ApiType, [who: AccountId32, proposalHash: H256, until: u32], { who: AccountId32, proposalHash: H256, until: u32 }>;
      /**
       * An account has voted in a referendum
       **/
      Voted: AugmentedEvent<ApiType, [voter: AccountId32, refIndex: u32, vote: PalletDemocracyVoteAccountVote], { voter: AccountId32, refIndex: u32, vote: PalletDemocracyVoteAccountVote }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    dexRouter: {
      RouteAdded: AugmentedEvent<ApiType, [xAssetId: u128, yAssetId: u128, route: Vec<u128>], { xAssetId: u128, yAssetId: u128, route: Vec<u128> }>;
      RouteDeleted: AugmentedEvent<ApiType, [xAssetId: u128, yAssetId: u128, route: Vec<u128>], { xAssetId: u128, yAssetId: u128, route: Vec<u128> }>;
      RouteUpdated: AugmentedEvent<ApiType, [xAssetId: u128, yAssetId: u128, oldRoute: Vec<u128>, updatedRoute: Vec<u128>], { xAssetId: u128, yAssetId: u128, oldRoute: Vec<u128>, updatedRoute: Vec<u128> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    dmpQueue: {
      /**
       * Downward message executed with the given outcome.
       **/
      ExecutedDownward: AugmentedEvent<ApiType, [messageId: U8aFixed, outcome: XcmV2TraitsOutcome], { messageId: U8aFixed, outcome: XcmV2TraitsOutcome }>;
      /**
       * Downward message is invalid XCM.
       **/
      InvalidFormat: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
      /**
       * Downward message is overweight and was placed in the overweight queue.
       **/
      OverweightEnqueued: AugmentedEvent<ApiType, [messageId: U8aFixed, overweightIndex: u64, requiredWeight: u64], { messageId: U8aFixed, overweightIndex: u64, requiredWeight: u64 }>;
      /**
       * Downward message from the overweight queue was executed.
       **/
      OverweightServiced: AugmentedEvent<ApiType, [overweightIndex: u64, weightUsed: u64], { overweightIndex: u64, weightUsed: u64 }>;
      /**
       * Downward message is unsupported version of XCM.
       **/
      UnsupportedVersion: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
      /**
       * The weight limit for handling downward messages was reached.
       **/
      WeightExhausted: AugmentedEvent<ApiType, [messageId: U8aFixed, remainingWeight: u64, requiredWeight: u64], { messageId: U8aFixed, remainingWeight: u64, requiredWeight: u64 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    dutchAuction: {
      ConfigurationAdded: AugmentedEvent<ApiType, [configurationId: u128, configuration: ComposableTraitsTimeTimeReleaseFunction], { configurationId: u128, configuration: ComposableTraitsTimeTimeReleaseFunction }>;
      OrderAdded: AugmentedEvent<ApiType, [orderId: u128, order: PalletDutchAuctionSellOrder], { orderId: u128, order: PalletDutchAuctionSellOrder }>;
      OrderRemoved: AugmentedEvent<ApiType, [orderId: u128], { orderId: u128 }>;
      /**
       * raised when part or whole order was taken with mentioned balance
       **/
      OrderTaken: AugmentedEvent<ApiType, [orderId: u128, taken: u128], { orderId: u128, taken: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    fnft: {
      FinancialNftBurned: AugmentedEvent<ApiType, [collectionId: u128, instanceId: u64], { collectionId: u128, instanceId: u64 }>;
      FinancialNftCollectionCreated: AugmentedEvent<ApiType, [collectionId: u128, who: AccountId32, admin: AccountId32], { collectionId: u128, who: AccountId32, admin: AccountId32 }>;
      FinancialNftCreated: AugmentedEvent<ApiType, [collectionId: u128, instanceId: u64], { collectionId: u128, instanceId: u64 }>;
      FinancialNftTransferred: AugmentedEvent<ApiType, [collectionId: u128, instanceId: u64, to: AccountId32], { collectionId: u128, instanceId: u64, to: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    governanceRegistry: {
      GrantRoot: AugmentedEvent<ApiType, [assetId: u128], { assetId: u128 }>;
      Remove: AugmentedEvent<ApiType, [assetId: u128], { assetId: u128 }>;
      Set: AugmentedEvent<ApiType, [assetId: u128, value: AccountId32], { assetId: u128, value: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    ibc: {
      /**
       * Ibc errors
       **/
      IbcErrors: AugmentedEvent<ApiType, [errors: Vec<PalletIbcErrorsIbcError>], { errors: Vec<PalletIbcErrorsIbcError> }>;
      /**
       * Raw Ibc events
       **/
      IbcEvents: AugmentedEvent<ApiType, [events: Vec<PalletIbcEventsIbcEvent>], { events: Vec<PalletIbcEventsIbcEvent> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    ibcPing: {
      /**
       * A channel has been opened
       **/
      ChannelOpened: AugmentedEvent<ApiType, [channelId: Bytes, portId: Bytes], { channelId: Bytes, portId: Bytes }>;
      /**
       * A send packet has been registered
       **/
      PacketSent: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    identity: {
      /**
       * A name was cleared, and the given balance returned.
       **/
      IdentityCleared: AugmentedEvent<ApiType, [who: AccountId32, deposit: u128], { who: AccountId32, deposit: u128 }>;
      /**
       * A name was removed and the given balance slashed.
       **/
      IdentityKilled: AugmentedEvent<ApiType, [who: AccountId32, deposit: u128], { who: AccountId32, deposit: u128 }>;
      /**
       * A name was set or reset (which will remove all judgements).
       **/
      IdentitySet: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
      /**
       * A judgement was given by a registrar.
       **/
      JudgementGiven: AugmentedEvent<ApiType, [target: AccountId32, registrarIndex: u32], { target: AccountId32, registrarIndex: u32 }>;
      /**
       * A judgement was asked from a registrar.
       **/
      JudgementRequested: AugmentedEvent<ApiType, [who: AccountId32, registrarIndex: u32], { who: AccountId32, registrarIndex: u32 }>;
      /**
       * A judgement request was retracted.
       **/
      JudgementUnrequested: AugmentedEvent<ApiType, [who: AccountId32, registrarIndex: u32], { who: AccountId32, registrarIndex: u32 }>;
      /**
       * A registrar was added.
       **/
      RegistrarAdded: AugmentedEvent<ApiType, [registrarIndex: u32], { registrarIndex: u32 }>;
      /**
       * A sub-identity was added to an identity and the deposit paid.
       **/
      SubIdentityAdded: AugmentedEvent<ApiType, [sub: AccountId32, main: AccountId32, deposit: u128], { sub: AccountId32, main: AccountId32, deposit: u128 }>;
      /**
       * A sub-identity was removed from an identity and the deposit freed.
       **/
      SubIdentityRemoved: AugmentedEvent<ApiType, [sub: AccountId32, main: AccountId32, deposit: u128], { sub: AccountId32, main: AccountId32, deposit: u128 }>;
      /**
       * A sub-identity was cleared, and the given deposit repatriated from the
       * main identity account to the sub-identity account.
       **/
      SubIdentityRevoked: AugmentedEvent<ApiType, [sub: AccountId32, main: AccountId32, deposit: u128], { sub: AccountId32, main: AccountId32, deposit: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    indices: {
      /**
       * A account index was assigned.
       **/
      IndexAssigned: AugmentedEvent<ApiType, [who: AccountId32, index: u32], { who: AccountId32, index: u32 }>;
      /**
       * A account index has been freed up (unassigned).
       **/
      IndexFreed: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
      /**
       * A account index has been frozen to its current account ID.
       **/
      IndexFrozen: AugmentedEvent<ApiType, [index: u32, who: AccountId32], { index: u32, who: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    lending: {
      /**
       * Event emitted when user borrows from given market.
       **/
      Borrowed: AugmentedEvent<ApiType, [sender: AccountId32, marketId: u32, amount: u128], { sender: AccountId32, marketId: u32, amount: u128 }>;
      /**
       * Event emitted when user repays borrow of beneficiary in given market.
       **/
      BorrowRepaid: AugmentedEvent<ApiType, [sender: AccountId32, marketId: u32, beneficiary: AccountId32, amount: u128], { sender: AccountId32, marketId: u32, beneficiary: AccountId32, amount: u128 }>;
      /**
       * Event emitted when collateral is deposited.
       **/
      CollateralDeposited: AugmentedEvent<ApiType, [sender: AccountId32, marketId: u32, amount: u128], { sender: AccountId32, marketId: u32, amount: u128 }>;
      /**
       * Event emitted when collateral is withdrawn.
       **/
      CollateralWithdrawn: AugmentedEvent<ApiType, [sender: AccountId32, marketId: u32, amount: u128], { sender: AccountId32, marketId: u32, amount: u128 }>;
      /**
       * Event emitted when a liquidation is initiated for a loan.
       **/
      LiquidationInitiated: AugmentedEvent<ApiType, [marketId: u32, borrowers: Vec<AccountId32>], { marketId: u32, borrowers: Vec<AccountId32> }>;
      /**
       * Event emitted when new lending market is created.
       **/
      MarketCreated: AugmentedEvent<ApiType, [marketId: u32, vaultId: u64, manager: AccountId32, currencyPair: ComposableTraitsDefiCurrencyPairCurrencyId], { marketId: u32, vaultId: u64, manager: AccountId32, currencyPair: ComposableTraitsDefiCurrencyPairCurrencyId }>;
      MarketUpdated: AugmentedEvent<ApiType, [marketId: u32, input: ComposableTraitsLendingUpdateInput], { marketId: u32, input: ComposableTraitsLendingUpdateInput }>;
      /**
       * Event emitted to warn that loan may go under collateralize soon.
       **/
      MayGoUnderCollateralizedSoon: AugmentedEvent<ApiType, [marketId: u32, account: AccountId32], { marketId: u32, account: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    liquidations: {
      PositionWasSentToLiquidation: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    mosaic: {
      /**
       * An asset mapping has been created.
       **/
      AssetMappingCreated: AugmentedEvent<ApiType, [assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId], { assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId }>;
      /**
       * An existing asset mapping has been deleted.
       **/
      AssetMappingDeleted: AugmentedEvent<ApiType, [assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId], { assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId }>;
      /**
       * An existing asset mapping has been updated.
       **/
      AssetMappingUpdated: AugmentedEvent<ApiType, [assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId], { assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId }>;
      BudgetUpdated: AugmentedEvent<ApiType, [assetId: u128, amount: u128, decay: PalletMosaicDecayBudgetPenaltyDecayer], { assetId: u128, amount: u128, decay: PalletMosaicDecayBudgetPenaltyDecayer }>;
      /**
       * The `NetworkInfos` `network_info` was updated for `network_id`.
       **/
      NetworksUpdated: AugmentedEvent<ApiType, [networkId: u32, networkInfo: PalletMosaicNetworkInfo], { networkId: u32, networkInfo: PalletMosaicNetworkInfo }>;
      /**
       * The relayer partially accepted the user's `OutgoingTransaction`.
       **/
      PartialTransferAccepted: AugmentedEvent<ApiType, [from: AccountId32, assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, amount: u128], { from: AccountId32, assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, amount: u128 }>;
      /**
       * The relayer has been rotated to `account_id`.
       **/
      RelayerRotated: AugmentedEvent<ApiType, [ttl: u32, accountId: AccountId32], { ttl: u32, accountId: AccountId32 }>;
      /**
       * The account of the relayer has been set.
       **/
      RelayerSet: AugmentedEvent<ApiType, [relayer: AccountId32], { relayer: AccountId32 }>;
      /**
       * User claimed outgoing tx that was not (yet) picked up by the relayer
       **/
      StaleTxClaimed: AugmentedEvent<ApiType, [to: AccountId32, by: AccountId32, assetId: u128, amount: u128], { to: AccountId32, by: AccountId32, assetId: u128, amount: u128 }>;
      /**
       * The relayer accepted the user's `OutgoingTransaction`.
       **/
      TransferAccepted: AugmentedEvent<ApiType, [from: AccountId32, assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, amount: u128], { from: AccountId32, assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, amount: u128 }>;
      /**
       * The user claims his `IncomingTransaction` and unlocks the locked amount.
       **/
      TransferClaimed: AugmentedEvent<ApiType, [by: AccountId32, to: AccountId32, assetId: u128, amount: u128], { by: AccountId32, to: AccountId32, assetId: u128, amount: u128 }>;
      /**
       * An incoming tx is created and waiting for the user to claim.
       **/
      TransferInto: AugmentedEvent<ApiType, [id: H256, to: AccountId32, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, assetId: u128, amount: u128], { id: H256, to: AccountId32, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, assetId: u128, amount: u128 }>;
      /**
       * When we have finality issues occur on the Ethereum chain,
       * we burn the locked `IncomingTransaction` for which we know that it is invalid.
       **/
      TransferIntoRescined: AugmentedEvent<ApiType, [account: AccountId32, amount: u128, assetId: u128], { account: AccountId32, amount: u128, assetId: u128 }>;
      /**
       * An outgoing tx is created, and locked in the outgoing tx pool.
       **/
      TransferOut: AugmentedEvent<ApiType, [id: H256, to: ComposableSupportEthereumAddress, assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, amount: u128, swapToNative: bool, sourceUserAccount: AccountId32, ammSwapInfo: Option<PalletMosaicAmmSwapInfo>, minimumAmountOut: u128], { id: H256, to: ComposableSupportEthereumAddress, assetId: u128, networkId: u32, remoteAssetId: CommonMosaicRemoteAssetId, amount: u128, swapToNative: bool, sourceUserAccount: AccountId32, ammSwapInfo: Option<PalletMosaicAmmSwapInfo>, minimumAmountOut: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    multisig: {
      /**
       * A multisig operation has been approved by someone.
       **/
      MultisigApproval: AugmentedEvent<ApiType, [approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed], { approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed }>;
      /**
       * A multisig operation has been cancelled.
       **/
      MultisigCancelled: AugmentedEvent<ApiType, [cancelling: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed], { cancelling: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed }>;
      /**
       * A multisig operation has been executed.
       **/
      MultisigExecuted: AugmentedEvent<ApiType, [approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed, result: Result<Null, SpRuntimeDispatchError>], { approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A new multisig operation has begun.
       **/
      NewMultisig: AugmentedEvent<ApiType, [approving: AccountId32, multisig: AccountId32, callHash: U8aFixed], { approving: AccountId32, multisig: AccountId32, callHash: U8aFixed }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    oracle: {
      /**
       * Answer from oracle removed for staleness. \[oracle_address, price\]
       **/
      AnswerPruned: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /**
       * Asset info created or changed. \[asset_id, threshold, min_answers, max_answers,
       * block_interval, reward, slash\]
       **/
      AssetInfoChange: AugmentedEvent<ApiType, [u128, Percent, u32, u32, u32, u128, u128]>;
      /**
       * Oracle rewarded. \[oracle_address, asset_id, price\]
       **/
      OracleRewarded: AugmentedEvent<ApiType, [AccountId32, u128, u128]>;
      /**
       * Price changed by oracle \[asset_id, price\]
       **/
      PriceChanged: AugmentedEvent<ApiType, [u128, u128]>;
      /**
       * Price submitted by oracle. \[oracle_address, asset_id, price\]
       **/
      PriceSubmitted: AugmentedEvent<ApiType, [AccountId32, u128, u128]>;
      /**
       * Rewarding Started \[rewarding start timestamp]
       **/
      RewardingAdjustment: AugmentedEvent<ApiType, [u64]>;
      /**
       * Signer was set. \[signer, controller\]
       **/
      SignerSet: AugmentedEvent<ApiType, [AccountId32, AccountId32]>;
      /**
       * Stake was added. \[added_by, amount_added, total_amount\]
       **/
      StakeAdded: AugmentedEvent<ApiType, [AccountId32, u128, u128]>;
      /**
       * Stake reclaimed. \[reclaimed_by, amount\]
       **/
      StakeReclaimed: AugmentedEvent<ApiType, [AccountId32, u128]>;
      /**
       * Stake removed. \[removed_by, amount, block_number\]
       **/
      StakeRemoved: AugmentedEvent<ApiType, [AccountId32, u128, u32]>;
      /**
       * Oracle slashed. \[oracle_address, asset_id, amount\]
       **/
      UserSlashed: AugmentedEvent<ApiType, [AccountId32, u128, u128]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    pablo: {
      /**
       * Liquidity added into the pool `T::PoolId`.
       **/
      LiquidityAdded: AugmentedEvent<ApiType, [who: AccountId32, poolId: u128, baseAmount: u128, quoteAmount: u128, mintedLp: u128], { who: AccountId32, poolId: u128, baseAmount: u128, quoteAmount: u128, mintedLp: u128 }>;
      /**
       * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
       **/
      LiquidityRemoved: AugmentedEvent<ApiType, [who: AccountId32, poolId: u128, baseAmount: u128, quoteAmount: u128, totalIssuance: u128], { who: AccountId32, poolId: u128, baseAmount: u128, quoteAmount: u128, totalIssuance: u128 }>;
      /**
       * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
       **/
      PoolCreated: AugmentedEvent<ApiType, [poolId: u128, owner: AccountId32, assets: ComposableTraitsDefiCurrencyPairCurrencyId], { poolId: u128, owner: AccountId32, assets: ComposableTraitsDefiCurrencyPairCurrencyId }>;
      /**
       * The sale ended, the funds repatriated and the pool deleted.
       **/
      PoolDeleted: AugmentedEvent<ApiType, [poolId: u128, baseAmount: u128, quoteAmount: u128], { poolId: u128, baseAmount: u128, quoteAmount: u128 }>;
      /**
       * Token exchange happened.
       **/
      Swapped: AugmentedEvent<ApiType, [poolId: u128, who: AccountId32, baseAsset: u128, quoteAsset: u128, baseAmount: u128, quoteAmount: u128, fee: ComposableTraitsDexFee], { poolId: u128, who: AccountId32, baseAsset: u128, quoteAsset: u128, baseAmount: u128, quoteAmount: u128, fee: ComposableTraitsDexFee }>;
      /**
       * TWAP updated.
       **/
      TwapUpdated: AugmentedEvent<ApiType, [poolId: u128, timestamp: u64, twaps: BTreeMap<u128, u128>], { poolId: u128, timestamp: u64, twaps: BTreeMap<u128, u128> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    parachainSystem: {
      /**
       * Downward messages were processed using the given weight.
       **/
      DownwardMessagesProcessed: AugmentedEvent<ApiType, [weightUsed: u64, dmqHead: H256], { weightUsed: u64, dmqHead: H256 }>;
      /**
       * Some downward messages have been received and will be processed.
       **/
      DownwardMessagesReceived: AugmentedEvent<ApiType, [count: u32], { count: u32 }>;
      /**
       * An upgrade has been authorized.
       **/
      UpgradeAuthorized: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
      /**
       * The validation function was applied as of the contained relay chain block number.
       **/
      ValidationFunctionApplied: AugmentedEvent<ApiType, [relayChainBlockNum: u32], { relayChainBlockNum: u32 }>;
      /**
       * The relay-chain aborted the upgrade process.
       **/
      ValidationFunctionDiscarded: AugmentedEvent<ApiType, []>;
      /**
       * The validation function has been scheduled to apply.
       **/
      ValidationFunctionStored: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    preimage: {
      /**
       * A preimage has ben cleared.
       **/
      Cleared: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
      /**
       * A preimage has been noted.
       **/
      Noted: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
      /**
       * A preimage has been requested.
       **/
      Requested: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    proxy: {
      /**
       * An announcement was placed to make a call in the future.
       **/
      Announced: AugmentedEvent<ApiType, [real: AccountId32, proxy: AccountId32, callHash: H256], { real: AccountId32, proxy: AccountId32, callHash: H256 }>;
      /**
       * Anonymous account has been created by new proxy with given
       * disambiguation index and proxy type.
       **/
      AnonymousCreated: AugmentedEvent<ApiType, [anonymous: AccountId32, who: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, disambiguationIndex: u16], { anonymous: AccountId32, who: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, disambiguationIndex: u16 }>;
      /**
       * A proxy was added.
       **/
      ProxyAdded: AugmentedEvent<ApiType, [delegator: AccountId32, delegatee: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, delay: u32], { delegator: AccountId32, delegatee: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, delay: u32 }>;
      /**
       * A proxy was executed correctly, with the given.
       **/
      ProxyExecuted: AugmentedEvent<ApiType, [result: Result<Null, SpRuntimeDispatchError>], { result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A proxy was removed.
       **/
      ProxyRemoved: AugmentedEvent<ApiType, [delegator: AccountId32, delegatee: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, delay: u32], { delegator: AccountId32, delegatee: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, delay: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    relayerXcm: {
      /**
       * Some assets have been placed in an asset trap.
       * 
       * \[ hash, origin, assets \]
       **/
      AssetsTrapped: AugmentedEvent<ApiType, [H256, XcmV1MultiLocation, XcmVersionedMultiAssets]>;
      /**
       * Execution of an XCM message was attempted.
       * 
       * \[ outcome \]
       **/
      Attempted: AugmentedEvent<ApiType, [XcmV2TraitsOutcome]>;
      /**
       * Expected query response has been received but the origin location of the response does
       * not match that expected. The query remains registered for a later, valid, response to
       * be received and acted upon.
       * 
       * \[ origin location, id, expected location \]
       **/
      InvalidResponder: AugmentedEvent<ApiType, [XcmV1MultiLocation, u64, Option<XcmV1MultiLocation>]>;
      /**
       * Expected query response has been received but the expected origin location placed in
       * storage by this runtime previously cannot be decoded. The query remains registered.
       * 
       * This is unexpected (since a location placed in storage in a previously executing
       * runtime should be readable prior to query timeout) and dangerous since the possibly
       * valid response will be dropped. Manual governance intervention is probably going to be
       * needed.
       * 
       * \[ origin location, id \]
       **/
      InvalidResponderVersion: AugmentedEvent<ApiType, [XcmV1MultiLocation, u64]>;
      /**
       * Query response has been received and query is removed. The registered notification has
       * been dispatched and executed successfully.
       * 
       * \[ id, pallet index, call index \]
       **/
      Notified: AugmentedEvent<ApiType, [u64, u8, u8]>;
      /**
       * Query response has been received and query is removed. The dispatch was unable to be
       * decoded into a `Call`; this might be due to dispatch function having a signature which
       * is not `(origin, QueryId, Response)`.
       * 
       * \[ id, pallet index, call index \]
       **/
      NotifyDecodeFailed: AugmentedEvent<ApiType, [u64, u8, u8]>;
      /**
       * Query response has been received and query is removed. There was a general error with
       * dispatching the notification call.
       * 
       * \[ id, pallet index, call index \]
       **/
      NotifyDispatchError: AugmentedEvent<ApiType, [u64, u8, u8]>;
      /**
       * Query response has been received and query is removed. The registered notification could
       * not be dispatched because the dispatch weight is greater than the maximum weight
       * originally budgeted by this runtime for the query result.
       * 
       * \[ id, pallet index, call index, actual weight, max budgeted weight \]
       **/
      NotifyOverweight: AugmentedEvent<ApiType, [u64, u8, u8, u64, u64]>;
      /**
       * A given location which had a version change subscription was dropped owing to an error
       * migrating the location to our new XCM format.
       * 
       * \[ location, query ID \]
       **/
      NotifyTargetMigrationFail: AugmentedEvent<ApiType, [XcmVersionedMultiLocation, u64]>;
      /**
       * A given location which had a version change subscription was dropped owing to an error
       * sending the notification to it.
       * 
       * \[ location, query ID, error \]
       **/
      NotifyTargetSendFail: AugmentedEvent<ApiType, [XcmV1MultiLocation, u64, XcmV2TraitsError]>;
      /**
       * Query response has been received and is ready for taking with `take_response`. There is
       * no registered notification call.
       * 
       * \[ id, response \]
       **/
      ResponseReady: AugmentedEvent<ApiType, [u64, XcmV2Response]>;
      /**
       * Received query response has been read and removed.
       * 
       * \[ id \]
       **/
      ResponseTaken: AugmentedEvent<ApiType, [u64]>;
      /**
       * A XCM message was sent.
       * 
       * \[ origin, destination, message \]
       **/
      Sent: AugmentedEvent<ApiType, [XcmV1MultiLocation, XcmV1MultiLocation, XcmV2Xcm]>;
      /**
       * The supported version of a location has been changed. This might be through an
       * automatic notification or a manual intervention.
       * 
       * \[ location, XCM version \]
       **/
      SupportedVersionChanged: AugmentedEvent<ApiType, [XcmV1MultiLocation, u32]>;
      /**
       * Query response received which does not match a registered query. This may be because a
       * matching query was never registered, it may be because it is a duplicate response, or
       * because the query timed out.
       * 
       * \[ origin location, id \]
       **/
      UnexpectedResponse: AugmentedEvent<ApiType, [XcmV1MultiLocation, u64]>;
      /**
       * An XCM version change notification message has been attempted to be sent.
       * 
       * \[ destination, result \]
       **/
      VersionChangeNotified: AugmentedEvent<ApiType, [XcmV1MultiLocation, u32]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    scheduler: {
      /**
       * The call for the provided hash was not found so the task has been aborted.
       **/
      CallLookupFailed: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<Bytes>, error: FrameSupportScheduleLookupError], { task: ITuple<[u32, u32]>, id: Option<Bytes>, error: FrameSupportScheduleLookupError }>;
      /**
       * Canceled some task.
       **/
      Canceled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32, index: u32 }>;
      /**
       * Dispatched some task.
       **/
      Dispatched: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<Bytes>, result: Result<Null, SpRuntimeDispatchError>], { task: ITuple<[u32, u32]>, id: Option<Bytes>, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * Scheduled some task.
       **/
      Scheduled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32, index: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    session: {
      /**
       * New session has happened. Note that the argument is the session index, not the
       * block number as the type might suggest.
       **/
      NewSession: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    stakingRewards: {
      Claimed: AugmentedEvent<ApiType, [owner: AccountId32, fnftCollectionId: u128, fnftInstanceId: u64], { owner: AccountId32, fnftCollectionId: u128, fnftInstanceId: u64 }>;
      MaxRewardsAccumulated: AugmentedEvent<ApiType, [poolId: u128, assetId: u128], { poolId: u128, assetId: u128 }>;
      RewardAccumulationHookError: AugmentedEvent<ApiType, [poolId: u128, assetId: u128, error: PalletStakingRewardsRewardAccumulationHookError], { poolId: u128, assetId: u128, error: PalletStakingRewardsRewardAccumulationHookError }>;
      /**
       * Pool with specified id `T::AssetId` was created successfully by `T::AccountId`.
       **/
      RewardPoolCreated: AugmentedEvent<ApiType, [poolId: u128, owner: AccountId32, endBlock: u32], { poolId: u128, owner: AccountId32, endBlock: u32 }>;
      RewardPoolUpdated: AugmentedEvent<ApiType, [poolId: u128], { poolId: u128 }>;
      RewardsPotIncreased: AugmentedEvent<ApiType, [poolId: u128, assetId: u128, amount: u128], { poolId: u128, assetId: u128, amount: u128 }>;
      /**
       * Reward transfer event.
       **/
      RewardTransferred: AugmentedEvent<ApiType, [from: AccountId32, poolId: u128, rewardCurrency: u128, rewardIncrement: u128], { from: AccountId32, poolId: u128, rewardCurrency: u128, rewardIncrement: u128 }>;
      /**
       * A staking position was split.
       **/
      SplitPosition: AugmentedEvent<ApiType, [positions: Vec<ITuple<[u128, u64, u128]>>], { positions: Vec<ITuple<[u128, u64, u128]>> }>;
      StakeAmountExtended: AugmentedEvent<ApiType, [fnftCollectionId: u128, fnftInstanceId: u64, amount: u128], { fnftCollectionId: u128, fnftInstanceId: u64, amount: u128 }>;
      Staked: AugmentedEvent<ApiType, [poolId: u128, owner: AccountId32, amount: u128, durationPreset: u64, fnftCollectionId: u128, fnftInstanceId: u64, rewardMultiplier: u64, keepAlive: bool], { poolId: u128, owner: AccountId32, amount: u128, durationPreset: u64, fnftCollectionId: u128, fnftInstanceId: u64, rewardMultiplier: u64, keepAlive: bool }>;
      Unstaked: AugmentedEvent<ApiType, [owner: AccountId32, fnftCollectionId: u128, fnftInstanceId: u64, slash: Option<u128>], { owner: AccountId32, fnftCollectionId: u128, fnftInstanceId: u64, slash: Option<u128> }>;
      UnstakeRewardSlashed: AugmentedEvent<ApiType, [poolId: u128, owner: AccountId32, fnftInstanceId: u64, rewardAssetId: u128, amountSlashed: u128], { poolId: u128, owner: AccountId32, fnftInstanceId: u64, rewardAssetId: u128, amountSlashed: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /**
       * The \[sudoer\] just switched identity; the old key is supplied if one existed.
       **/
      KeyChanged: AugmentedEvent<ApiType, [oldSudoer: Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      Sudid: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A sudo just took place. \[result\]
       **/
      SudoAsDone: AugmentedEvent<ApiType, [sudoResult: Result<Null, SpRuntimeDispatchError>], { sudoResult: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    system: {
      /**
       * `:code` was updated.
       **/
      CodeUpdated: AugmentedEvent<ApiType, []>;
      /**
       * An extrinsic failed.
       **/
      ExtrinsicFailed: AugmentedEvent<ApiType, [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportWeightsDispatchInfo], { dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportWeightsDispatchInfo }>;
      /**
       * An extrinsic completed successfully.
       **/
      ExtrinsicSuccess: AugmentedEvent<ApiType, [dispatchInfo: FrameSupportWeightsDispatchInfo], { dispatchInfo: FrameSupportWeightsDispatchInfo }>;
      /**
       * An account was reaped.
       **/
      KilledAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      /**
       * A new account was created.
       **/
      NewAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
      /**
       * On on-chain remark happened.
       **/
      Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32, hash_: H256 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    technicalCollective: {
      /**
       * A motion was approved by the required threshold.
       **/
      Approved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
      /**
       * A proposal was closed because its threshold was reached or after its duration was up.
       **/
      Closed: AugmentedEvent<ApiType, [proposalHash: H256, yes: u32, no: u32], { proposalHash: H256, yes: u32, no: u32 }>;
      /**
       * A motion was not approved by the required threshold.
       **/
      Disapproved: AugmentedEvent<ApiType, [proposalHash: H256], { proposalHash: H256 }>;
      /**
       * A motion was executed; result will be `Ok` if it returned without error.
       **/
      Executed: AugmentedEvent<ApiType, [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>], { proposalHash: H256, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A single member did some action; result will be `Ok` if it returned without error.
       **/
      MemberExecuted: AugmentedEvent<ApiType, [proposalHash: H256, result: Result<Null, SpRuntimeDispatchError>], { proposalHash: H256, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A motion (given hash) has been proposed (by given account) with a threshold (given
       * `MemberCount`).
       **/
      Proposed: AugmentedEvent<ApiType, [account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32], { account: AccountId32, proposalIndex: u32, proposalHash: H256, threshold: u32 }>;
      /**
       * A motion (given hash) has been voted on by given account, leaving
       * a tally (yes votes and no votes given respectively as `MemberCount`).
       **/
      Voted: AugmentedEvent<ApiType, [account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32], { account: AccountId32, proposalHash: H256, voted: bool, yes: u32, no: u32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    technicalMembership: {
      /**
       * Phantom member, never used.
       **/
      Dummy: AugmentedEvent<ApiType, []>;
      /**
       * One of the members' keys changed.
       **/
      KeyChanged: AugmentedEvent<ApiType, []>;
      /**
       * The given member was added; see the transaction for who.
       **/
      MemberAdded: AugmentedEvent<ApiType, []>;
      /**
       * The given member was removed; see the transaction for who.
       **/
      MemberRemoved: AugmentedEvent<ApiType, []>;
      /**
       * The membership was reset; see the transaction for who the new set is.
       **/
      MembersReset: AugmentedEvent<ApiType, []>;
      /**
       * Two members were swapped; see the transaction for who.
       **/
      MembersSwapped: AugmentedEvent<ApiType, []>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    tokens: {
      /**
       * A balance was set by root.
       **/
      BalanceSet: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, free: u128, reserved: u128], { currencyId: u128, who: AccountId32, free: u128, reserved: u128 }>;
      /**
       * Deposited some balance into an account
       **/
      Deposited: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
      /**
       * An account was removed whose balance was non-zero but below
       * ExistentialDeposit, resulting in an outright loss.
       **/
      DustLost: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
      /**
       * An account was created with some free balance.
       **/
      Endowed: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
      /**
       * Some locked funds were unlocked
       **/
      LockRemoved: AugmentedEvent<ApiType, [lockId: U8aFixed, currencyId: u128, who: AccountId32], { lockId: U8aFixed, currencyId: u128, who: AccountId32 }>;
      /**
       * Some funds are locked
       **/
      LockSet: AugmentedEvent<ApiType, [lockId: U8aFixed, currencyId: u128, who: AccountId32, amount: u128], { lockId: U8aFixed, currencyId: u128, who: AccountId32, amount: u128 }>;
      /**
       * Some balance was reserved (moved from free to reserved).
       **/
      Reserved: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
      /**
       * Some reserved balance was repatriated (moved from reserved to
       * another account).
       **/
      ReserveRepatriated: AugmentedEvent<ApiType, [currencyId: u128, from: AccountId32, to: AccountId32, amount: u128, status: FrameSupportTokensMiscBalanceStatus], { currencyId: u128, from: AccountId32, to: AccountId32, amount: u128, status: FrameSupportTokensMiscBalanceStatus }>;
      /**
       * Some balances were slashed (e.g. due to mis-behavior)
       **/
      Slashed: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, freeAmount: u128, reservedAmount: u128], { currencyId: u128, who: AccountId32, freeAmount: u128, reservedAmount: u128 }>;
      /**
       * The total issuance of an currency has been set
       **/
      TotalIssuanceSet: AugmentedEvent<ApiType, [currencyId: u128, amount: u128], { currencyId: u128, amount: u128 }>;
      /**
       * Transfer succeeded.
       **/
      Transfer: AugmentedEvent<ApiType, [currencyId: u128, from: AccountId32, to: AccountId32, amount: u128], { currencyId: u128, from: AccountId32, to: AccountId32, amount: u128 }>;
      /**
       * Some balance was unreserved (moved from reserved to free).
       **/
      Unreserved: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
      /**
       * Some balances were withdrawn (e.g. pay for transaction fee)
       **/
      Withdrawn: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    transactionPayment: {
      /**
       * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
       * has been paid by `who`.
       **/
      TransactionFeePaid: AugmentedEvent<ApiType, [who: AccountId32, actualFee: u128, tip: u128], { who: AccountId32, actualFee: u128, tip: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    transfer: {
      /**
       * A channel has been opened
       **/
      ChannelOpened: AugmentedEvent<ApiType, [channelId: Bytes, portId: Bytes], { channelId: Bytes, portId: Bytes }>;
      /**
       * Pallet params updated
       **/
      PalletParamsUpdated: AugmentedEvent<ApiType, [sendEnabled: bool, receiveEnabled: bool], { sendEnabled: bool, receiveEnabled: bool }>;
      TokenTransferInitiated: AugmentedEvent<ApiType, [from: AccountId32, to: Bytes, amount: u128], { from: AccountId32, to: Bytes, amount: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    treasury: {
      /**
       * Some funds have been allocated.
       **/
      Awarded: AugmentedEvent<ApiType, [proposalIndex: u32, award: u128, account: AccountId32], { proposalIndex: u32, award: u128, account: AccountId32 }>;
      /**
       * Some of our funds have been burnt.
       **/
      Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
      /**
       * Some funds have been deposited.
       **/
      Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
      /**
       * New proposal.
       **/
      Proposed: AugmentedEvent<ApiType, [proposalIndex: u32], { proposalIndex: u32 }>;
      /**
       * A proposal was rejected; funds were slashed.
       **/
      Rejected: AugmentedEvent<ApiType, [proposalIndex: u32, slashed: u128], { proposalIndex: u32, slashed: u128 }>;
      /**
       * Spending has finished; this is the amount that rolls over until next spend.
       **/
      Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
      /**
       * A new spend proposal has been approved.
       **/
      SpendApproved: AugmentedEvent<ApiType, [proposalIndex: u32, amount: u128, beneficiary: AccountId32], { proposalIndex: u32, amount: u128, beneficiary: AccountId32 }>;
      /**
       * We have ended a spend period and will now allocate funds.
       **/
      Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    unknownTokens: {
      /**
       * Deposit success.
       **/
      Deposited: AugmentedEvent<ApiType, [asset: XcmV1MultiAsset, who: XcmV1MultiLocation], { asset: XcmV1MultiAsset, who: XcmV1MultiLocation }>;
      /**
       * Withdraw success.
       **/
      Withdrawn: AugmentedEvent<ApiType, [asset: XcmV1MultiAsset, who: XcmV1MultiLocation], { asset: XcmV1MultiAsset, who: XcmV1MultiLocation }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    utility: {
      /**
       * Batch of dispatches completed fully with no error.
       **/
      BatchCompleted: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches completed but has errors.
       **/
      BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
       * well as the error.
       **/
      BatchInterrupted: AugmentedEvent<ApiType, [index: u32, error: SpRuntimeDispatchError], { index: u32, error: SpRuntimeDispatchError }>;
      /**
       * A call was dispatched.
       **/
      DispatchedAs: AugmentedEvent<ApiType, [result: Result<Null, SpRuntimeDispatchError>], { result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * A single item within a Batch of dispatches has completed with no error.
       **/
      ItemCompleted: AugmentedEvent<ApiType, []>;
      /**
       * A single item within a Batch of dispatches has completed with error.
       **/
      ItemFailed: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    vault: {
      /**
       * Emitted after a user deposits funds into the vault.
       **/
      Deposited: AugmentedEvent<ApiType, [account: AccountId32, assetAmount: u128, lpAmount: u128], { account: AccountId32, assetAmount: u128, lpAmount: u128 }>;
      /**
       * Emitted after a successful emergency shutdown.
       **/
      EmergencyShutdown: AugmentedEvent<ApiType, [vault: u64], { vault: u64 }>;
      LiquidateStrategy: AugmentedEvent<ApiType, [account: AccountId32, amount: u128], { account: AccountId32, amount: u128 }>;
      /**
       * Emitted after a vault has been successfully created.
       **/
      VaultCreated: AugmentedEvent<ApiType, [id: u64], { id: u64 }>;
      /**
       * Emitted after a vault is restarted.
       **/
      VaultStarted: AugmentedEvent<ApiType, [vault: u64], { vault: u64 }>;
      /**
       * Emitted after a user exchanges LP tokens back for underlying assets
       **/
      Withdrawn: AugmentedEvent<ApiType, [account: AccountId32, lpAmount: u128, assetAmount: u128], { account: AccountId32, lpAmount: u128, assetAmount: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    vesting: {
      /**
       * Claimed vesting.
       **/
      Claimed: AugmentedEvent<ApiType, [who: AccountId32, asset: u128, vestingScheduleIds: ComposableTraitsVestingVestingScheduleIdSet, lockedAmount: u128, claimedAmountPerSchedule: BTreeMap<u128, u128>], { who: AccountId32, asset: u128, vestingScheduleIds: ComposableTraitsVestingVestingScheduleIdSet, lockedAmount: u128, claimedAmountPerSchedule: BTreeMap<u128, u128> }>;
      /**
       * Added new vesting schedule.
       **/
      VestingScheduleAdded: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, asset: u128, vestingScheduleId: u128, schedule: ComposableTraitsVestingVestingSchedule, scheduleAmount: u128], { from: AccountId32, to: AccountId32, asset: u128, vestingScheduleId: u128, schedule: ComposableTraitsVestingVestingSchedule, scheduleAmount: u128 }>;
      /**
       * Updated vesting schedules.
       **/
      VestingSchedulesUpdated: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    xcmpQueue: {
      /**
       * Bad XCM format used.
       **/
      BadFormat: AugmentedEvent<ApiType, [messageHash: Option<H256>], { messageHash: Option<H256> }>;
      /**
       * Bad XCM version used.
       **/
      BadVersion: AugmentedEvent<ApiType, [messageHash: Option<H256>], { messageHash: Option<H256> }>;
      /**
       * Some XCM failed.
       **/
      Fail: AugmentedEvent<ApiType, [messageHash: Option<H256>, error: XcmV2TraitsError, weight: u64], { messageHash: Option<H256>, error: XcmV2TraitsError, weight: u64 }>;
      /**
       * An XCM exceeded the individual message weight budget.
       **/
      OverweightEnqueued: AugmentedEvent<ApiType, [sender: u32, sentAt: u32, index: u64, required: u64], { sender: u32, sentAt: u32, index: u64, required: u64 }>;
      /**
       * An XCM from the overweight queue was executed with the given actual weight used.
       **/
      OverweightServiced: AugmentedEvent<ApiType, [index: u64, used: u64], { index: u64, used: u64 }>;
      /**
       * Some XCM was executed ok.
       **/
      Success: AugmentedEvent<ApiType, [messageHash: Option<H256>, weight: u64], { messageHash: Option<H256>, weight: u64 }>;
      /**
       * An upward message was sent to the relay chain.
       **/
      UpwardMessageSent: AugmentedEvent<ApiType, [messageHash: Option<H256>], { messageHash: Option<H256> }>;
      /**
       * An HRMP message was sent to a sibling parachain.
       **/
      XcmpMessageSent: AugmentedEvent<ApiType, [messageHash: Option<H256>], { messageHash: Option<H256> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    xTokens: {
      /**
       * Transferred `MultiAsset` with fee.
       **/
      TransferredMultiAssets: AugmentedEvent<ApiType, [sender: AccountId32, assets: XcmV1MultiassetMultiAssets, fee: XcmV1MultiAsset, dest: XcmV1MultiLocation], { sender: AccountId32, assets: XcmV1MultiassetMultiAssets, fee: XcmV1MultiAsset, dest: XcmV1MultiLocation }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
