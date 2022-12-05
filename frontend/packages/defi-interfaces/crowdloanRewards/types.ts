// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { ComposableTraitsDefiCurrencyPairCurrencyId, CurrencyId, } from "../common";
import type { bool, Enum, Null, Struct, u128, u32, u64, } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { EthereumAccountId } from "@polkadot/types/interfaces/eth";
import type { EcdsaSignature, MultiSignature, } from "@polkadot/types/interfaces/extrinsics";
import type { ParachainInherentData, PersistedValidationData, } from "@polkadot/types/interfaces/parachains";
import type { AccountId32, Balance, Permill, } from "@polkadot/types/interfaces/runtime";

/** @name CommonMosaicRemoteAssetId */
export interface CommonMosaicRemoteAssetId extends Null {}

/** @name ComposableSupportEthereumAddress */
export interface ComposableSupportEthereumAddress extends Null {}

/** @name ComposableTraitsAssetsBasicAssetMetadata */
export interface ComposableTraitsAssetsBasicAssetMetadata extends Struct {
  readonly symbol: {
    readonly inner: Null;
  } & Struct;
  readonly name: {
    readonly inner: Null;
  } & Struct;
}

/** @name ComposableTraitsAssetsXcmAssetLocation */
export interface ComposableTraitsAssetsXcmAssetLocation extends Null {}

/** @name ComposableTraitsAuctionAuctionStepFunction */
export interface ComposableTraitsAuctionAuctionStepFunction extends Null {}

/** @name ComposableTraitsBondedFinanceBondDuration */
export interface ComposableTraitsBondedFinanceBondDuration extends Struct {
  readonly Finite: {
    readonly returnIn: u32;
  } & Struct;
}

/** @name ComposableTraitsBondedFinanceBondOffer */
export interface ComposableTraitsBondedFinanceBondOffer extends Struct {
  readonly beneficiary: AccountId32;
  readonly asset: CurrencyId;
  readonly bondPrice: u128;
  readonly nbOfBonds: u128;
  readonly maturity: ComposableTraitsBondedFinanceBondDuration;
  readonly reward: ComposableTraitsBondedFinanceBondOfferReward;
  readonly keepAlive: bool;
}

/** @name ComposableTraitsBondedFinanceBondOfferReward */
export interface ComposableTraitsBondedFinanceBondOfferReward extends Struct {
  readonly asset: CurrencyId;
  readonly amount: u128;
  readonly maturity: u32;
}

/** @name ComposableTraitsCallFilterCallFilterEntry */
export interface ComposableTraitsCallFilterCallFilterEntry extends Null {}

/** @name ComposableTraitsDefiSell */
export interface ComposableTraitsDefiSell extends Null {}

/** @name ComposableTraitsDefiTake */
export interface ComposableTraitsDefiTake extends Null {}

/** @name ComposableTraitsDexConstantProductPoolInfo */
export interface ComposableTraitsDexConstantProductPoolInfo {}

/** @name ComposableTraitsDexConstantProductPoolInfo */
export interface ComposableTraitsDexConstantProductPoolInfo extends Struct {
  readonly owner: AccountId32;
  readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
  readonly lpToken: u128;
  readonly fee: Permill;
  readonly ownerFee: Permill;
}

/** @name ComposableTraitsDexDexRoute */
export interface ComposableTraitsDexDexRoute extends Null {}

/** @name ComposableTraitsDexStableSwapPoolInfo */
export interface ComposableTraitsDexStableSwapPoolInfo extends Null {}

/** @name ComposableTraitsGovernanceSignedRawOrigin */
export interface ComposableTraitsGovernanceSignedRawOrigin extends Enum {
  readonly isRoot: boolean;
  readonly isSigned: boolean;
  readonly asSigned: AccountId32;
  readonly isIsSigned: boolean;
  readonly asIsSigned: bool;
  readonly isAsSigned: boolean;
  readonly asAsSigned: AccountId32;
  readonly type: "Root" | "Signed" | "IsSigned" | "AsSigned";
}

/** @name ComposableTraitsLendingCreateInput */
export interface ComposableTraitsLendingCreateInput extends Null {}

/** @name ComposableTraitsLendingMarketConfig */
export interface ComposableTraitsLendingMarketConfig extends Null {}

/** @name ComposableTraitsLendingRepayStrategy */
export interface ComposableTraitsLendingRepayStrategy extends Null {}

/** @name ComposableTraitsLendingUpdateInput */
export interface ComposableTraitsLendingUpdateInput extends Null {}

/** @name ComposableTraitsOraclePrice */
export interface ComposableTraitsOraclePrice extends Null {
  price: u128;
  block: u64;
}

/** @name ComposableTraitsOracleRewardTracker */
export interface ComposableTraitsOracleRewardTracker extends Null {}

/** @name ComposableTraitsStakingRewardPool */
export interface ComposableTraitsStakingRewardPool extends Null {}

/** @name ComposableTraitsStakingRewardPoolConfiguration */
export interface ComposableTraitsStakingRewardPoolConfiguration extends Null {}

/** @name ComposableTraitsStakingStake */
export interface ComposableTraitsStakingStake extends Null {}

/** @name ComposableTraitsTimeTimeReleaseFunction */
export interface ComposableTraitsTimeTimeReleaseFunction extends Null {}

/** @name ComposableTraitsVaultVaultConfig */
export interface ComposableTraitsVaultVaultConfig extends Null {}

/** @name ComposableTraitsVestingVestingSchedule */
export interface ComposableTraitsVestingVestingSchedule extends Null {}

/** @name ComposableTraitsXcmAssetsForeignMetadata */
export interface ComposableTraitsXcmAssetsForeignMetadata extends Null {}

/** @name ComposableTraitsXcmAssetsXcmAssetLocation */
export interface ComposableTraitsXcmAssetsXcmAssetLocation extends Null {}

/** @name CumulusPalletDmpQueueConfigData */
export interface CumulusPalletDmpQueueConfigData extends Null {}

/** @name CumulusPalletDmpQueuePageIndexData */
export interface CumulusPalletDmpQueuePageIndexData extends Null {}

/** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot */
export interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot
  extends Null {}

/** @name CumulusPalletXcmpQueueInboundChannelDetails */
export interface CumulusPalletXcmpQueueInboundChannelDetails extends Null {}

/** @name CumulusPalletXcmpQueueInboundStatus */
export interface CumulusPalletXcmpQueueInboundStatus extends Null {}

/** @name CumulusPalletXcmpQueueOutboundChannelDetails */
export interface CumulusPalletXcmpQueueOutboundChannelDetails extends Null {}

/** @name CumulusPalletXcmpQueueOutboundStatus */
export interface CumulusPalletXcmpQueueOutboundStatus extends Null {}

/** @name CumulusPalletXcmpQueueQueueConfigData */
export interface CumulusPalletXcmpQueueQueueConfigData extends Null {}

/** @name CumulusPrimitivesParachainInherentParachainInherentData */
export interface CumulusPrimitivesParachainInherentParachainInherentData
  extends ParachainInherentData {}

/** @name DaliRuntimeOpaqueSessionKeys */
export interface DaliRuntimeOpaqueSessionKeys extends Null {}

/** @name DaliRuntimeOriginCaller */
export interface DaliRuntimeOriginCaller extends Null {}

/** @name FrameSupportScheduleLookupError */
export interface FrameSupportScheduleLookupError extends Null {}

/** @name FrameSupportScheduleMaybeHashed */
export interface FrameSupportScheduleMaybeHashed extends Null {}

/** @name FrameSystemAccountInfo */
export interface FrameSystemAccountInfo extends Struct {
  readonly nonce: Null;
  readonly consumers: Null;
  readonly providers: Null;
  readonly sufficients: Null;
  readonly data: {
    readonly free: u128;
    readonly reserved: u128;
    readonly miscFrozen: u128;
    readonly feeFrozen: u128;
  } & Struct;
}

/** @name IbcTraitOpenChannelParams */
export interface IbcTraitOpenChannelParams extends Null {}

/** @name IbcTransferPalletParams */
export interface IbcTransferPalletParams extends Null {}

/** @name IbcTransferTransferParams */
export interface IbcTransferTransferParams extends Null {}

/** @name OrmlTokensAccountData */
export interface OrmlTokensAccountData extends Struct {
  readonly free: u128;
  readonly reserved: u128;
  readonly frozen: u128;
}

/** @name OrmlTokensBalanceLock */
export interface OrmlTokensBalanceLock extends Null {}

/** @name OrmlTokensReserveData */
export interface OrmlTokensReserveData extends Null {}

/** @name PalletAssetsRegistryCandidateStatus */
export interface PalletAssetsRegistryCandidateStatus extends Null {}

/** @name PalletAssetsRegistryForeignMetadata */
export interface PalletAssetsRegistryForeignMetadata extends Null {}

/** @name PalletCollatorSelectionCandidateInfo */
export interface PalletCollatorSelectionCandidateInfo extends Null {}

/** @name PalletCrowdloanRewardsModelsEcdsaSignature */
export interface PalletCrowdloanRewardsModelsEcdsaSignature
  extends EcdsaSignature {}

/** @name PalletCrowdloanRewardsModelsProof */
export interface PalletCrowdloanRewardsModelsProof extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: ITuple<[AccountId32, MultiSignature]>;
  readonly isEthereum: boolean;
  readonly asEthereum: PalletCrowdloanRewardsModelsEcdsaSignature;
  readonly type: "RelayChain" | "Ethereum";
}

/** @name PalletCrowdloanRewardsModelsRemoteAccount */
export interface PalletCrowdloanRewardsModelsRemoteAccount extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: AccountId32;
  readonly isEthereum: boolean;
  readonly asEthereum: EthereumAccountId;
  readonly type: "RelayChain" | "Ethereum";
}

/** @name PalletCrowdloanRewardsModelsReward */
export interface PalletCrowdloanRewardsModelsReward extends Null {}

/** @name PalletCrowdloanRewardsReward */
export interface PalletCrowdloanRewardsReward extends Null {}

/** @name PalletDemocracyConviction */
export interface PalletDemocracyConviction extends Null {}

/** @name PalletDemocracyPreimageStatus */
export interface PalletDemocracyPreimageStatus extends Null {}

/** @name PalletDemocracyReferendumInfo */
export interface PalletDemocracyReferendumInfo extends Null {}

/** @name PalletDemocracyReleases */
export interface PalletDemocracyReleases extends Null {}

/** @name PalletDemocracyVoteAccountVote */
export interface PalletDemocracyVoteAccountVote extends Null {}

/** @name PalletDemocracyVoteThreshold */
export interface PalletDemocracyVoteThreshold extends Null {}

/** @name PalletDemocracyVoteVoting */
export interface PalletDemocracyVoteVoting extends Null {}

/** @name PalletDutchAuctionSellOrder */
export interface PalletDutchAuctionSellOrder extends Null {}

/** @name PalletDutchAuctionTakeOrder */
export interface PalletDutchAuctionTakeOrder extends Null {}

/** @name PalletIbcAny */
export interface PalletIbcAny extends Null {}

/** @name PalletIbcConnectionParams */
export interface PalletIbcConnectionParams extends Null {}

/** @name PalletIbcErrorsIbcError */
export interface PalletIbcErrorsIbcError extends Null {}

/** @name PalletIbcEventsIbcEvent */
export interface PalletIbcEventsIbcEvent extends Null {}

/** @name PalletIbcIbcConsensusState */
export interface PalletIbcIbcConsensusState extends Null {}

/** @name PalletIbcPingSendPingParams */
export interface PalletIbcPingSendPingParams extends Null {}

/** @name PalletIdentityBitFlags */
export interface PalletIdentityBitFlags extends Null {}

/** @name PalletIdentityIdentityInfo */
export interface PalletIdentityIdentityInfo extends Null {}

/** @name PalletIdentityJudgement */
export interface PalletIdentityJudgement extends Null {}

/** @name PalletIdentityRegistrarInfo */
export interface PalletIdentityRegistrarInfo extends Null {}

/** @name PalletIdentityRegistration */
export interface PalletIdentityRegistration extends Null {}

/** @name PalletLiquidationsLiquidationStrategyConfiguration */
export interface PalletLiquidationsLiquidationStrategyConfiguration
  extends Null {}

/** @name PalletLiquidityBootstrappingPool */
export interface PalletLiquidityBootstrappingPool extends Null {}

/** @name PalletMosaicAmmSwapInfo */
export interface PalletMosaicAmmSwapInfo extends Null {}

/** @name PalletMosaicAssetInfo */
export interface PalletMosaicAssetInfo extends Null {}

/** @name PalletMosaicDecayBudgetPenaltyDecayer */
export interface PalletMosaicDecayBudgetPenaltyDecayer extends Null {}

/** @name PalletMosaicNetworkInfo */
export interface PalletMosaicNetworkInfo extends Struct {
  readonly enabled: bool;
  readonly maxTransferSize: Balance;
}

/** @name PalletMosaicRelayerStaleRelayer */
export interface PalletMosaicRelayerStaleRelayer extends Struct {
  readonly relayer: {
    readonly current: AccountId32;
    readonly next: {
      readonly ttl: u32;
      readonly account: AccountId32;
    } & Struct;
  } & Struct;
}

/** @name PalletOracleAssetInfo */
export interface PalletOracleAssetInfo extends Null {}

/** @name PalletOraclePrePrice */
export interface PalletOraclePrePrice extends Null {}

/** @name PalletOraclePrice */
export interface PalletOraclePrice extends Null {}

/** @name PalletOracleWithdraw */
export interface PalletOracleWithdraw extends Struct {
  readonly stake: u128;
  readonly unlockBlock: u32;
}

/** @name PalletPreimageRequestStatus */
export interface PalletPreimageRequestStatus extends Null {}

/** @name PalletSchedulerReleases */
export interface PalletSchedulerReleases extends Null {}

/** @name PalletSchedulerScheduledV2 */
export interface PalletSchedulerScheduledV2 extends Null {}

/** @name PalletSchedulerScheduledV3 */
export interface PalletSchedulerScheduledV3 extends Null {}

/** @name PalletTreasuryProposal */
export interface PalletTreasuryProposal extends Null {}

/** @name PalletVaultModelsStrategyOverview */
export interface PalletVaultModelsStrategyOverview extends Null {}

/** @name PalletVaultModelsVaultInfo */
export interface PalletVaultModelsVaultInfo extends Null {}

/** @name PolkadotParachainPrimitivesXcmpMessageFormat */
export interface PolkadotParachainPrimitivesXcmpMessageFormat extends Null {}

/** @name PolkadotPrimitivesV1AbridgedHostConfiguration */
export interface PolkadotPrimitivesV1AbridgedHostConfiguration extends Null {}

/** @name PolkadotPrimitivesV1PersistedValidationData */
export interface PolkadotPrimitivesV1PersistedValidationData
  extends PersistedValidationData {}

/** @name PolkadotPrimitivesV2AbridgedHostConfiguration */
export interface PolkadotPrimitivesV2AbridgedHostConfiguration extends Null {}

/** @name PolkadotPrimitivesV2PersistedValidationData */
export interface PolkadotPrimitivesV2PersistedValidationData extends Null {}

/** @name PolkadotPrimitivesV2UpgradeRestriction */
export interface PolkadotPrimitivesV2UpgradeRestriction extends Null {}

/** @name SpConsensusAuraSr25519AppSr25519Public */
export interface SpConsensusAuraSr25519AppSr25519Public extends Null {}

/** @name SpTrieStorageProof */
export interface SpTrieStorageProof extends Null {}

/** @name XcmVersionedMultiAsset */
export interface XcmVersionedMultiAsset extends Null {}

/** @name ComposableTraitsVestingVestingSchedule */
export interface ComposableTraitsVestingVestingSchedule extends Null {}

/** @name ComposableTraitsVestingVestingScheduleIdSet */
export interface ComposableTraitsVestingVestingScheduleIdSet extends Null {}

/** @name ComposableTraitsVestingVestingScheduleInfo */
export interface ComposableTraitsVestingVestingScheduleInfo extends Null {}

export type PHANTOM_CROWDLOANREWARDS = "crowdloanRewards";
