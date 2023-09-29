// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/events';

import type { ApiTypes, AugmentedEvent } from '@polkadot/api-base/types';
import type { BTreeMap, Bytes, Null, Option, Result, Struct, Text, U8aFixed, Vec, bool, i128, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256, Percent, Permill } from '@polkadot/types/interfaces/runtime';
import {
  ComposableTraitsAccountProxyProxyType,
  ComposableTraitsAssetsAssetInfo,
  ComposableTraitsAssetsAssetInfoUpdate,
  ComposableTraitsDexFee,
  FrameSupportDispatchDispatchInfo, FrameSupportDispatchPostDispatchInfo,
  FrameSupportPreimagesBounded,
  FrameSupportTokensMiscBalanceStatus,
  IbcCoreIcs02ClientHeight,
  PalletCallFilterCallFilterEntry,
  PalletConvictionVotingTally,
  PalletCosmwasmContractInfo,
  PalletCosmwasmEntryPoint,
  PalletCrowdloanRewardsModelsRemoteAccount,
  PalletDemocracyMetadataOwner,
  PalletDemocracyVoteAccountVote,
  PalletDemocracyVoteThreshold,
  PalletIbcErrorsIbcError,
  PalletIbcEventsIbcEvent,
  PalletMultihopXcmIbcMultihopEventReason,
  PalletMultisigTimepoint, PalletVestingVestingSchedule, PalletVestingVestingScheduleIdSet,
  PrimitivesCurrencyForeignAssetId,
  SpRuntimeDispatchError, SpRuntimeDispatchErrorWithPostInfo,
  SpWeightsWeightV2Weight,
  XcmV3MultiAsset,
  XcmV3MultiassetMultiAssets,
  XcmV3MultiLocation,
  XcmV3Response,
  XcmV3TraitsError,
  XcmV3TraitsOutcome,
  XcmV3Xcm,
  XcmVersionedMultiAssets,
  XcmVersionedMultiLocation
} from "@polkadot/types/lookup";

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module '@polkadot/api-base/types/events' {
  interface AugmentedEvents<ApiType extends ApiTypes> {
    assetsRegistry: {
      AssetLocationRemoved: AugmentedEvent<ApiType, [assetId: u128], { assetId: u128 }>;
      AssetLocationUpdated: AugmentedEvent<ApiType, [assetId: u128, location: PrimitivesCurrencyForeignAssetId], { assetId: u128, location: PrimitivesCurrencyForeignAssetId }>;
      AssetRegistered: AugmentedEvent<ApiType, [assetId: u128, location: Option<PrimitivesCurrencyForeignAssetId>, assetInfo: ComposableTraitsAssetsAssetInfo], { assetId: u128, location: Option<PrimitivesCurrencyForeignAssetId>, assetInfo: ComposableTraitsAssetsAssetInfo }>;
      AssetUpdated: AugmentedEvent<ApiType, [assetId: u128, assetInfo: ComposableTraitsAssetsAssetInfoUpdate], { assetId: u128, assetInfo: ComposableTraitsAssetsAssetInfoUpdate }>;
      MinFeeUpdated: AugmentedEvent<ApiType, [targetParachainId: u32, foreignAssetId: PrimitivesCurrencyForeignAssetId, amount: Option<u128>], { targetParachainId: u32, foreignAssetId: PrimitivesCurrencyForeignAssetId, amount: Option<u128> }>;
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
      Disabled: AugmentedEvent<ApiType, [entry: PalletCallFilterCallFilterEntry], { entry: PalletCallFilterCallFilterEntry }>;
      /**
       * Unpaused transaction
       **/
      Enabled: AugmentedEvent<ApiType, [entry: PalletCallFilterCallFilterEntry], { entry: PalletCallFilterCallFilterEntry }>;
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
    convictionVoting: {
      /**
       * An account has delegated their vote to another account. \[who, target\]
       **/
      Delegated: AugmentedEvent<ApiType, [AccountId32, AccountId32]>;
      /**
       * An \[account\] has cancelled a previous delegation operation.
       **/
      Undelegated: AugmentedEvent<ApiType, [AccountId32]>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    cosmwasm: {
      AdminUpdated: AugmentedEvent<ApiType, [contract: AccountId32, newAdmin: Option<AccountId32>], { contract: AccountId32, newAdmin: Option<AccountId32> }>;
      Emitted: AugmentedEvent<ApiType, [contract: AccountId32, ty: Bytes, attributes: Vec<ITuple<[Bytes, Bytes]>>], { contract: AccountId32, ty: Bytes, attributes: Vec<ITuple<[Bytes, Bytes]>> }>;
      Executed: AugmentedEvent<ApiType, [contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, data: Option<Bytes>], { contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, data: Option<Bytes> }>;
      ExecutionFailed: AugmentedEvent<ApiType, [contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, error: Bytes], { contract: AccountId32, entrypoint: PalletCosmwasmEntryPoint, error: Bytes }>;
      Instantiated: AugmentedEvent<ApiType, [contract: AccountId32, info: PalletCosmwasmContractInfo], { contract: AccountId32, info: PalletCosmwasmContractInfo }>;
      Migrated: AugmentedEvent<ApiType, [contract: AccountId32, to: u64], { contract: AccountId32, to: u64 }>;
      Uploaded: AugmentedEvent<ApiType, [codeHash: U8aFixed, codeId: u64], { codeHash: U8aFixed, codeId: u64 }>;
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
       * Called after rewards have been added through the `add` extrinsic.
       **/
      RewardsAdded: AugmentedEvent<ApiType, [additions: Vec<ITuple<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>>], { additions: Vec<ITuple<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>> }>;
      /**
       * Called after rewards have been deleted through the `delete` extrinsic.
       **/
      RewardsDeleted: AugmentedEvent<ApiType, [deletions: Vec<PalletCrowdloanRewardsModelsRemoteAccount>], { deletions: Vec<PalletCrowdloanRewardsModelsRemoteAccount> }>;
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
      ExecutedDownward: AugmentedEvent<ApiType, [U8aFixed, XcmV3TraitsOutcome]>;
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
       * An external proposal has been tabled.
       **/
      ExternalTabled: AugmentedEvent<ApiType, []>;
      /**
       * Metadata for a proposal or a referendum has been cleared.
       **/
      MetadataCleared: AugmentedEvent<ApiType, [owner: PalletDemocracyMetadataOwner, hash_: H256], { owner: PalletDemocracyMetadataOwner, hash_: H256 }>;
      /**
       * Metadata for a proposal or a referendum has been set.
       **/
      MetadataSet: AugmentedEvent<ApiType, [owner: PalletDemocracyMetadataOwner, hash_: H256], { owner: PalletDemocracyMetadataOwner, hash_: H256 }>;
      /**
       * Metadata has been transferred to new owner.
       **/
      MetadataTransferred: AugmentedEvent<ApiType, [prevOwner: PalletDemocracyMetadataOwner, owner: PalletDemocracyMetadataOwner, hash_: H256], { prevOwner: PalletDemocracyMetadataOwner, owner: PalletDemocracyMetadataOwner, hash_: H256 }>;
      /**
       * A proposal has been rejected by referendum.
       **/
      NotPassed: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
      /**
       * A proposal has been approved by referendum.
       **/
      Passed: AugmentedEvent<ApiType, [refIndex: u32], { refIndex: u32 }>;
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
      Tabled: AugmentedEvent<ApiType, [proposalIndex: u32, deposit: u128], { proposalIndex: u32, deposit: u128 }>;
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
    dmpQueue: {
      /**
       * Downward message executed with the given outcome.
       **/
      ExecutedDownward: AugmentedEvent<ApiType, [messageId: U8aFixed, outcome: XcmV3TraitsOutcome], { messageId: U8aFixed, outcome: XcmV3TraitsOutcome }>;
      /**
       * Downward message is invalid XCM.
       **/
      InvalidFormat: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
      /**
       * The maximum number of downward messages was.
       **/
      MaxMessagesExhausted: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
      /**
       * Downward message is overweight and was placed in the overweight queue.
       **/
      OverweightEnqueued: AugmentedEvent<ApiType, [messageId: U8aFixed, overweightIndex: u64, requiredWeight: SpWeightsWeightV2Weight], { messageId: U8aFixed, overweightIndex: u64, requiredWeight: SpWeightsWeightV2Weight }>;
      /**
       * Downward message from the overweight queue was executed.
       **/
      OverweightServiced: AugmentedEvent<ApiType, [overweightIndex: u64, weightUsed: SpWeightsWeightV2Weight], { overweightIndex: u64, weightUsed: SpWeightsWeightV2Weight }>;
      /**
       * Downward message is unsupported version of XCM.
       **/
      UnsupportedVersion: AugmentedEvent<ApiType, [messageId: U8aFixed], { messageId: U8aFixed }>;
      /**
       * The weight limit for handling downward messages was reached.
       **/
      WeightExhausted: AugmentedEvent<ApiType, [messageId: U8aFixed, remainingWeight: SpWeightsWeightV2Weight, requiredWeight: SpWeightsWeightV2Weight], { messageId: U8aFixed, remainingWeight: SpWeightsWeightV2Weight, requiredWeight: SpWeightsWeightV2Weight }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    farming: {
      RewardClaimed: AugmentedEvent<ApiType, [accountId: AccountId32, poolCurrencyId: u128, rewardCurrencyId: u128, amount: u128], { accountId: AccountId32, poolCurrencyId: u128, rewardCurrencyId: u128, amount: u128 }>;
      RewardDistributed: AugmentedEvent<ApiType, [poolCurrencyId: u128, rewardCurrencyId: u128, amount: u128], { poolCurrencyId: u128, rewardCurrencyId: u128, amount: u128 }>;
      RewardScheduleUpdated: AugmentedEvent<ApiType, [poolCurrencyId: u128, rewardCurrencyId: u128, periodCount: u32, perPeriod: u128], { poolCurrencyId: u128, rewardCurrencyId: u128, periodCount: u32, perPeriod: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    farmingRewards: {
      DepositStake: AugmentedEvent<ApiType, [poolId: u128, stakeId: AccountId32, amount: i128], { poolId: u128, stakeId: AccountId32, amount: i128 }>;
      DistributeReward: AugmentedEvent<ApiType, [currencyId: u128, amount: i128], { currencyId: u128, amount: i128 }>;
      WithdrawReward: AugmentedEvent<ApiType, [poolId: u128, stakeId: AccountId32, currencyId: u128, amount: i128], { poolId: u128, stakeId: AccountId32, currencyId: u128, amount: i128 }>;
      WithdrawStake: AugmentedEvent<ApiType, [poolId: u128, stakeId: AccountId32, amount: i128], { poolId: u128, stakeId: AccountId32, amount: i128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    ibc: {
      /**
       * Asset Admin Account Updated
       **/
      AssetAdminUpdated: AugmentedEvent<ApiType, [adminAccount: AccountId32], { adminAccount: AccountId32 }>;
      /**
       * A channel has been opened
       **/
      ChannelOpened: AugmentedEvent<ApiType, [channelId: Bytes, portId: Bytes], { channelId: Bytes, portId: Bytes }>;
      ChargingFeeConfirmed: AugmentedEvent<ApiType, [sequence: u64], { sequence: u64 }>;
      ChargingFeeFailedAcknowledgement: AugmentedEvent<ApiType, [sequence: u64], { sequence: u64 }>;
      ChargingFeeOnTransferInitiated: AugmentedEvent<ApiType, [sequence: u64, from: Bytes, to: Bytes, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isFlatFee: bool, sourceChannel: Bytes, destinationChannel: Bytes], { sequence: u64, from: Bytes, to: Bytes, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isFlatFee: bool, sourceChannel: Bytes, destinationChannel: Bytes }>;
      ChargingFeeTimeout: AugmentedEvent<ApiType, [sequence: u64], { sequence: u64 }>;
      ChildStateUpdated: AugmentedEvent<ApiType, []>;
      /**
       * Client has been frozen
       **/
      ClientFrozen: AugmentedEvent<ApiType, [clientId: Bytes, height: u64, revisionNumber: u64], { clientId: Bytes, height: u64, revisionNumber: u64 }>;
      ClientStateSubstituted: AugmentedEvent<ApiType, [clientId: Text, height: IbcCoreIcs02ClientHeight], { clientId: Text, height: IbcCoreIcs02ClientHeight }>;
      /**
       * Client upgrade path has been set
       **/
      ClientUpgradeSet: AugmentedEvent<ApiType, []>;
      /**
       * Events emitted by the ibc subsystem
       **/
      Events: AugmentedEvent<ApiType, [events: Vec<Result<PalletIbcEventsIbcEvent, PalletIbcErrorsIbcError>>], { events: Vec<Result<PalletIbcEventsIbcEvent, PalletIbcErrorsIbcError>> }>;
      ExecuteMemoIbcTokenTransferFailed: AugmentedEvent<ApiType, [from: AccountId32, to: Bytes, assetId: u128, amount: u128, channel: u64, nextMemo: Option<Text>], { from: AccountId32, to: Bytes, assetId: u128, amount: u128, channel: u64, nextMemo: Option<Text> }>;
      ExecuteMemoIbcTokenTransferFailedWithReason: AugmentedEvent<ApiType, [from: AccountId32, memo: Text, reason: u8], { from: AccountId32, memo: Text, reason: u8 }>;
      ExecuteMemoIbcTokenTransferSuccess: AugmentedEvent<ApiType, [from: AccountId32, to: Bytes, assetId: u128, amount: u128, channel: u64, nextMemo: Option<Text>], { from: AccountId32, to: Bytes, assetId: u128, amount: u128, channel: u64, nextMemo: Option<Text> }>;
      ExecuteMemoStarted: AugmentedEvent<ApiType, [accountId: AccountId32, memo: Option<Text>], { accountId: AccountId32, memo: Option<Text> }>;
      ExecuteMemoXcmFailed: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128, assetId: u128, paraId: Option<u32>], { from: AccountId32, to: AccountId32, amount: u128, assetId: u128, paraId: Option<u32> }>;
      ExecuteMemoXcmSuccess: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, amount: u128, assetId: u128, paraId: Option<u32>], { from: AccountId32, to: AccountId32, amount: u128, assetId: u128, paraId: Option<u32> }>;
      FeeLessChannelIdsAdded: AugmentedEvent<ApiType, [sourceChannel: u64, destinationChannel: u64], { sourceChannel: u64, destinationChannel: u64 }>;
      FeeLessChannelIdsRemoved: AugmentedEvent<ApiType, [sourceChannel: u64, destinationChannel: u64], { sourceChannel: u64, destinationChannel: u64 }>;
      /**
       * On recv packet was not processed successfully processes
       **/
      OnRecvPacketError: AugmentedEvent<ApiType, [msg: Bytes], { msg: Bytes }>;
      /**
       * Pallet params updated
       **/
      ParamsUpdated: AugmentedEvent<ApiType, [sendEnabled: bool, receiveEnabled: bool], { sendEnabled: bool, receiveEnabled: bool }>;
      /**
       * Ibc tokens have been received and minted
       **/
      TokenReceived: AugmentedEvent<ApiType, [from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isReceiverSource: bool, sourceChannel: Bytes, destinationChannel: Bytes], { from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isReceiverSource: bool, sourceChannel: Bytes, destinationChannel: Bytes }>;
      /**
       * An outgoing Ibc token transfer has been completed and burnt
       **/
      TokenTransferCompleted: AugmentedEvent<ApiType, [from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes], { from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes }>;
      /**
       * Ibc transfer failed, received an acknowledgement error, tokens have been refunded
       **/
      TokenTransferFailed: AugmentedEvent<ApiType, [from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes], { from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes }>;
      /**
       * An Ibc token transfer has been started
       **/
      TokenTransferInitiated: AugmentedEvent<ApiType, [from: Bytes, to: Bytes, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes], { from: Bytes, to: Bytes, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes }>;
      /**
       * Happens when token transfer timeouts, tokens have been refunded. expected
       * `TokenTransferFailed` does not happen in this case.
       **/
      TokenTransferTimeout: AugmentedEvent<ApiType, [from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes], { from: Text, to: Text, ibcDenom: Bytes, localAssetId: Option<u128>, amount: u128, isSenderSource: bool, sourceChannel: Bytes, destinationChannel: Bytes }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    ics20Fee: {
      FeeLessChannelIdsAdded: AugmentedEvent<ApiType, [sourceChannel: u64, destinationChannel: u64], { sourceChannel: u64, destinationChannel: u64 }>;
      FeeLessChannelIdsRemoved: AugmentedEvent<ApiType, [sourceChannel: u64, destinationChannel: u64], { sourceChannel: u64, destinationChannel: u64 }>;
      IbcTransferFeeCollected: AugmentedEvent<ApiType, [amount: u128, assetId: u128], { amount: u128, assetId: u128 }>;
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
    openGovBalances: {
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
       * Signer removed
       **/
      SignerRemoved: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128]>;
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
      LiquidityAdded: AugmentedEvent<ApiType, [who: AccountId32, poolId: u128, assetAmounts: BTreeMap<u128, u128>, mintedLp: u128], { who: AccountId32, poolId: u128, assetAmounts: BTreeMap<u128, u128>, mintedLp: u128 }>;
      /**
       * Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
       **/
      LiquidityRemoved: AugmentedEvent<ApiType, [who: AccountId32, poolId: u128, assetAmounts: BTreeMap<u128, u128>], { who: AccountId32, poolId: u128, assetAmounts: BTreeMap<u128, u128> }>;
      /**
       * Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
       **/
      PoolCreated: AugmentedEvent<ApiType, [poolId: u128, owner: AccountId32, assetWeights: BTreeMap<u128, Permill>, lpTokenId: u128], { poolId: u128, owner: AccountId32, assetWeights: BTreeMap<u128, Permill>, lpTokenId: u128 }>;
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
    palletMultihopXcmIbc: {
      FailedCallback: AugmentedEvent<ApiType, [originAddress: U8aFixed, routeId: u128, reason: PalletMultihopXcmIbcMultihopEventReason], { originAddress: U8aFixed, routeId: u128, reason: PalletMultihopXcmIbcMultihopEventReason }>;
      FailedMatchLocation: AugmentedEvent<ApiType, []>;
      FailedXcmToIbc: AugmentedEvent<ApiType, [originAddress: AccountId32, to: U8aFixed, amount: u128, assetId: u128, memo: Option<Text>], { originAddress: AccountId32, to: U8aFixed, amount: u128, assetId: u128, memo: Option<Text> }>;
      MultihopXcmMemo: AugmentedEvent<ApiType, [reason: PalletMultihopXcmIbcMultihopEventReason, from: AccountId32, to: AccountId32, amount: u128, assetId: u128, isError: bool], { reason: PalletMultihopXcmIbcMultihopEventReason, from: AccountId32, to: AccountId32, amount: u128, assetId: u128, isError: bool }>;
      SuccessXcmToIbc: AugmentedEvent<ApiType, [originAddress: AccountId32, to: U8aFixed, amount: u128, assetId: u128, memo: Option<Text>], { originAddress: AccountId32, to: U8aFixed, amount: u128, assetId: u128, memo: Option<Text> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    parachainSystem: {
      /**
       * Downward messages were processed using the given weight.
       **/
      DownwardMessagesProcessed: AugmentedEvent<ApiType, [weightUsed: SpWeightsWeightV2Weight, dmqHead: H256], { weightUsed: SpWeightsWeightV2Weight, dmqHead: H256 }>;
      /**
       * Some downward messages have been received and will be processed.
       **/
      DownwardMessagesReceived: AugmentedEvent<ApiType, [count: u32], { count: u32 }>;
      /**
       * An upgrade has been authorized.
       **/
      UpgradeAuthorized: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
      /**
       * An upward message was sent to the relay chain.
       **/
      UpwardMessageSent: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>], { messageHash: Option<U8aFixed> }>;
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
    polkadotXcm: {
      /**
       * Some assets have been claimed from an asset trap
       * 
       * \[ hash, origin, assets \]
       **/
      AssetsClaimed: AugmentedEvent<ApiType, [H256, XcmV3MultiLocation, XcmVersionedMultiAssets]>;
      /**
       * Some assets have been placed in an asset trap.
       * 
       * \[ hash, origin, assets \]
       **/
      AssetsTrapped: AugmentedEvent<ApiType, [H256, XcmV3MultiLocation, XcmVersionedMultiAssets]>;
      /**
       * Execution of an XCM message was attempted.
       * 
       * \[ outcome \]
       **/
      Attempted: AugmentedEvent<ApiType, [XcmV3TraitsOutcome]>;
      /**
       * Fees were paid from a location for an operation (often for using `SendXcm`).
       * 
       * \[ paying location, fees \]
       **/
      FeesPaid: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
      /**
       * Expected query response has been received but the querier location of the response does
       * not match the expected. The query remains registered for a later, valid, response to
       * be received and acted upon.
       * 
       * \[ origin location, id, expected querier, maybe actual querier \]
       **/
      InvalidQuerier: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64, XcmV3MultiLocation, Option<XcmV3MultiLocation>]>;
      /**
       * Expected query response has been received but the expected querier location placed in
       * storage by this runtime previously cannot be decoded. The query remains registered.
       * 
       * This is unexpected (since a location placed in storage in a previously executing
       * runtime should be readable prior to query timeout) and dangerous since the possibly
       * valid response will be dropped. Manual governance intervention is probably going to be
       * needed.
       * 
       * \[ origin location, id \]
       **/
      InvalidQuerierVersion: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64]>;
      /**
       * Expected query response has been received but the origin location of the response does
       * not match that expected. The query remains registered for a later, valid, response to
       * be received and acted upon.
       * 
       * \[ origin location, id, expected location \]
       **/
      InvalidResponder: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64, Option<XcmV3MultiLocation>]>;
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
      InvalidResponderVersion: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64]>;
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
      NotifyOverweight: AugmentedEvent<ApiType, [u64, u8, u8, SpWeightsWeightV2Weight, SpWeightsWeightV2Weight]>;
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
      NotifyTargetSendFail: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64, XcmV3TraitsError]>;
      /**
       * Query response has been received and is ready for taking with `take_response`. There is
       * no registered notification call.
       * 
       * \[ id, response \]
       **/
      ResponseReady: AugmentedEvent<ApiType, [u64, XcmV3Response]>;
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
      Sent: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiLocation, XcmV3Xcm]>;
      /**
       * The supported version of a location has been changed. This might be through an
       * automatic notification or a manual intervention.
       * 
       * \[ location, XCM version \]
       **/
      SupportedVersionChanged: AugmentedEvent<ApiType, [XcmV3MultiLocation, u32]>;
      /**
       * Query response received which does not match a registered query. This may be because a
       * matching query was never registered, it may be because it is a duplicate response, or
       * because the query timed out.
       * 
       * \[ origin location, id \]
       **/
      UnexpectedResponse: AugmentedEvent<ApiType, [XcmV3MultiLocation, u64]>;
      /**
       * An XCM version change notification message has been attempted to be sent.
       * 
       * The cost of sending it (borne by the chain) is included.
       * 
       * \[ destination, result, cost \]
       **/
      VersionChangeNotified: AugmentedEvent<ApiType, [XcmV3MultiLocation, u32, XcmV3MultiassetMultiAssets]>;
      /**
       * We have requested that a remote chain sends us XCM version change notifications.
       * 
       * \[ destination location, cost \]
       **/
      VersionNotifyRequested: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
      /**
       * A remote has requested XCM version change notification from us and we have honored it.
       * A version information message is sent to them and its cost is included.
       * 
       * \[ destination location, cost \]
       **/
      VersionNotifyStarted: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
      /**
       * We have requested that a remote chain stops sending us XCM version change notifications.
       * 
       * \[ destination location, cost \]
       **/
      VersionNotifyUnrequested: AugmentedEvent<ApiType, [XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
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
       * A pure account has been created by new proxy with given
       * disambiguation index and proxy type.
       **/
      PureCreated: AugmentedEvent<ApiType, [pure: AccountId32, who: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, disambiguationIndex: u16], { pure: AccountId32, who: AccountId32, proxyType: ComposableTraitsAccountProxyProxyType, disambiguationIndex: u16 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    referenda: {
      /**
       * A referendum has been approved and its proposal has been scheduled.
       **/
      Approved: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
      /**
       * A referendum has been cancelled.
       **/
      Cancelled: AugmentedEvent<ApiType, [index: u32, tally: PalletConvictionVotingTally], { index: u32, tally: PalletConvictionVotingTally }>;
      ConfirmAborted: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
      /**
       * A referendum has ended its confirmation phase and is ready for approval.
       **/
      Confirmed: AugmentedEvent<ApiType, [index: u32, tally: PalletConvictionVotingTally], { index: u32, tally: PalletConvictionVotingTally }>;
      ConfirmStarted: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
      /**
       * The decision deposit has been placed.
       **/
      DecisionDepositPlaced: AugmentedEvent<ApiType, [index: u32, who: AccountId32, amount: u128], { index: u32, who: AccountId32, amount: u128 }>;
      /**
       * The decision deposit has been refunded.
       **/
      DecisionDepositRefunded: AugmentedEvent<ApiType, [index: u32, who: AccountId32, amount: u128], { index: u32, who: AccountId32, amount: u128 }>;
      /**
       * A referendum has moved into the deciding phase.
       **/
      DecisionStarted: AugmentedEvent<ApiType, [index: u32, track: u16, proposal: FrameSupportPreimagesBounded, tally: PalletConvictionVotingTally], { index: u32, track: u16, proposal: FrameSupportPreimagesBounded, tally: PalletConvictionVotingTally }>;
      /**
       * A deposit has been slashaed.
       **/
      DepositSlashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32, amount: u128 }>;
      /**
       * A referendum has been killed.
       **/
      Killed: AugmentedEvent<ApiType, [index: u32, tally: PalletConvictionVotingTally], { index: u32, tally: PalletConvictionVotingTally }>;
      /**
       * Metadata for a referendum has been cleared.
       **/
      MetadataCleared: AugmentedEvent<ApiType, [index: u32, hash_: H256], { index: u32, hash_: H256 }>;
      /**
       * Metadata for a referendum has been set.
       **/
      MetadataSet: AugmentedEvent<ApiType, [index: u32, hash_: H256], { index: u32, hash_: H256 }>;
      /**
       * A proposal has been rejected by referendum.
       **/
      Rejected: AugmentedEvent<ApiType, [index: u32, tally: PalletConvictionVotingTally], { index: u32, tally: PalletConvictionVotingTally }>;
      /**
       * The submission deposit has been refunded.
       **/
      SubmissionDepositRefunded: AugmentedEvent<ApiType, [index: u32, who: AccountId32, amount: u128], { index: u32, who: AccountId32, amount: u128 }>;
      /**
       * A referendum has been submitted.
       **/
      Submitted: AugmentedEvent<ApiType, [index: u32, track: u16, proposal: FrameSupportPreimagesBounded], { index: u32, track: u16, proposal: FrameSupportPreimagesBounded }>;
      /**
       * A referendum has been timed out without being decided.
       **/
      TimedOut: AugmentedEvent<ApiType, [index: u32, tally: PalletConvictionVotingTally], { index: u32, tally: PalletConvictionVotingTally }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    releaseCommittee: {
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
    releaseMembership: {
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
    scheduler: {
      /**
       * The call for the provided hash was not found so the task has been aborted.
       **/
      CallUnavailable: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * Canceled some task.
       **/
      Canceled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32, index: u32 }>;
      /**
       * Dispatched some task.
       **/
      Dispatched: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError> }>;
      /**
       * The given task was unable to be renewed since the agenda is full at that block.
       **/
      PeriodicFailed: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
      /**
       * The given task can never be executed since it is overweight.
       **/
      PermanentlyOverweight: AugmentedEvent<ApiType, [task: ITuple<[u32, u32]>, id: Option<U8aFixed>], { task: ITuple<[u32, u32]>, id: Option<U8aFixed> }>;
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
      ExtrinsicFailed: AugmentedEvent<ApiType, [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo }>;
      /**
       * An extrinsic completed successfully.
       **/
      ExtrinsicSuccess: AugmentedEvent<ApiType, [dispatchInfo: FrameSupportDispatchDispatchInfo], { dispatchInfo: FrameSupportDispatchDispatchInfo }>;
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
    technicalCommittee: {
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
    technicalCommitteeMembership: {
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
       * Some free balance was locked.
       **/
      Locked: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
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
       * Some locked balance was freed.
       **/
      Unlocked: AugmentedEvent<ApiType, [currencyId: u128, who: AccountId32, amount: u128], { currencyId: u128, who: AccountId32, amount: u128 }>;
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
       * The inactive funds of the pallet have been updated.
       **/
      UpdatedInactive: AugmentedEvent<ApiType, [reactivated: u128, deactivated: u128], { reactivated: u128, deactivated: u128 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    unknownTokens: {
      /**
       * Deposit success.
       **/
      Deposited: AugmentedEvent<ApiType, [asset: XcmV3MultiAsset, who: XcmV3MultiLocation], { asset: XcmV3MultiAsset, who: XcmV3MultiLocation }>;
      /**
       * Withdraw success.
       **/
      Withdrawn: AugmentedEvent<ApiType, [asset: XcmV3MultiAsset, who: XcmV3MultiLocation], { asset: XcmV3MultiAsset, who: XcmV3MultiLocation }>;
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
    vesting: {
      /**
       * Claimed vesting.
       **/
      Claimed: AugmentedEvent<ApiType, [who: AccountId32, asset: u128, vestingScheduleIds: PalletVestingVestingScheduleIdSet, lockedAmount: u128, claimedAmountPerSchedule: BTreeMap<u128, u128>], { who: AccountId32, asset: u128, vestingScheduleIds: PalletVestingVestingScheduleIdSet, lockedAmount: u128, claimedAmountPerSchedule: BTreeMap<u128, u128> }>;
      /**
       * Added new vesting schedule.
       **/
      VestingScheduleAdded: AugmentedEvent<ApiType, [from: AccountId32, to: AccountId32, asset: u128, vestingScheduleId: u128, schedule: PalletVestingVestingSchedule, scheduleAmount: u128], { from: AccountId32, to: AccountId32, asset: u128, vestingScheduleId: u128, schedule: PalletVestingVestingSchedule, scheduleAmount: u128 }>;
      /**
       * Updated vesting schedules.
       **/
      VestingSchedulesUpdated: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    whitelist: {
      CallWhitelisted: AugmentedEvent<ApiType, [callHash: H256], { callHash: H256 }>;
      WhitelistedCallDispatched: AugmentedEvent<ApiType, [callHash: H256, result: Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo>], { callHash: H256, result: Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo> }>;
      WhitelistedCallRemoved: AugmentedEvent<ApiType, [callHash: H256], { callHash: H256 }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    xcmpQueue: {
      /**
       * Bad XCM format used.
       **/
      BadFormat: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>], { messageHash: Option<U8aFixed> }>;
      /**
       * Bad XCM version used.
       **/
      BadVersion: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>], { messageHash: Option<U8aFixed> }>;
      /**
       * Some XCM failed.
       **/
      Fail: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>, error: XcmV3TraitsError, weight: SpWeightsWeightV2Weight], { messageHash: Option<U8aFixed>, error: XcmV3TraitsError, weight: SpWeightsWeightV2Weight }>;
      /**
       * An XCM exceeded the individual message weight budget.
       **/
      OverweightEnqueued: AugmentedEvent<ApiType, [sender: u32, sentAt: u32, index: u64, required: SpWeightsWeightV2Weight], { sender: u32, sentAt: u32, index: u64, required: SpWeightsWeightV2Weight }>;
      /**
       * An XCM from the overweight queue was executed with the given actual weight used.
       **/
      OverweightServiced: AugmentedEvent<ApiType, [index: u64, used: SpWeightsWeightV2Weight], { index: u64, used: SpWeightsWeightV2Weight }>;
      /**
       * Some XCM was executed ok.
       **/
      Success: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>, weight: SpWeightsWeightV2Weight], { messageHash: Option<U8aFixed>, weight: SpWeightsWeightV2Weight }>;
      /**
       * An HRMP message was sent to a sibling parachain.
       **/
      XcmpMessageSent: AugmentedEvent<ApiType, [messageHash: Option<U8aFixed>], { messageHash: Option<U8aFixed> }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
    xTokens: {
      /**
       * Transferred `MultiAsset` with fee.
       **/
      TransferredMultiAssets: AugmentedEvent<ApiType, [sender: AccountId32, assets: XcmV3MultiassetMultiAssets, fee: XcmV3MultiAsset, dest: XcmV3MultiLocation], { sender: AccountId32, assets: XcmV3MultiassetMultiAssets, fee: XcmV3MultiAsset, dest: XcmV3MultiLocation }>;
      /**
       * Generic event
       **/
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
