// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Enum, Null, Struct, bool, u128, u32 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { EthereumAccountId } from '@polkadot/types/interfaces/eth';
import type { EcdsaSignature, MultiSignature } from '@polkadot/types/interfaces/extrinsics';
import type { ParachainInherentData, PersistedValidationData } from '@polkadot/types/interfaces/parachains';
import type { AccountId32, Balance } from '@polkadot/types/interfaces/runtime';

/** @name ComposableTraitsAssetsXcmAssetLocation */
export interface ComposableTraitsAssetsXcmAssetLocation extends Null {}

/** @name ComposableTraitsAuctionAuctionStepFunction */
export interface ComposableTraitsAuctionAuctionStepFunction extends Null {}

/** @name ComposableTraitsBondedFinanceBondOffer */
export interface ComposableTraitsBondedFinanceBondOffer extends Null {}

/** @name ComposableTraitsCallFilterCallFilterEntry */
export interface ComposableTraitsCallFilterCallFilterEntry extends Null {}

/** @name ComposableTraitsDefiSell */
export interface ComposableTraitsDefiSell extends Null {}

/** @name ComposableTraitsDefiTake */
export interface ComposableTraitsDefiTake extends Null {}

/** @name ComposableTraitsGovernanceSignedRawOrigin */
export interface ComposableTraitsGovernanceSignedRawOrigin extends Null {}

/** @name ComposableTraitsTimeTimeReleaseFunction */
export interface ComposableTraitsTimeTimeReleaseFunction extends Null {}

/** @name ComposableTraitsVaultVaultConfig */
export interface ComposableTraitsVaultVaultConfig extends Null {}

/** @name ComposableTraitsVestingVestingSchedule */
export interface ComposableTraitsVestingVestingSchedule extends Null {}

/** @name CumulusPalletDmpQueueConfigData */
export interface CumulusPalletDmpQueueConfigData extends Null {}

/** @name CumulusPalletDmpQueuePageIndexData */
export interface CumulusPalletDmpQueuePageIndexData extends Null {}

/** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot */
export interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Null {}

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
export interface CumulusPrimitivesParachainInherentParachainInherentData extends ParachainInherentData {}

/** @name DaliRuntimeOpaqueSessionKeys */
export interface DaliRuntimeOpaqueSessionKeys extends Null {}

/** @name DaliRuntimeOriginCaller */
export interface DaliRuntimeOriginCaller extends Null {}

/** @name FrameSupportScheduleLookupError */
export interface FrameSupportScheduleLookupError extends Null {}

/** @name FrameSupportScheduleMaybeHashed */
export interface FrameSupportScheduleMaybeHashed extends Null {}

/** @name OrmlTokensAccountData */
export interface OrmlTokensAccountData extends Null {}

/** @name OrmlTokensBalanceLock */
export interface OrmlTokensBalanceLock extends Null {}

/** @name PalletAssetsRegistryCandidateStatus */
export interface PalletAssetsRegistryCandidateStatus extends Null {}

/** @name PalletAssetsRegistryForeignMetadata */
export interface PalletAssetsRegistryForeignMetadata extends Null {}

/** @name PalletCollatorSelectionCandidateInfo */
export interface PalletCollatorSelectionCandidateInfo extends Null {}

/** @name PalletCrowdloanRewardsModelsProof */
export interface PalletCrowdloanRewardsModelsProof extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: ITuple<[AccountId32, MultiSignature]>;
  readonly isEthereum: boolean;
  readonly asEthereum: EcdsaSignature;
  readonly type: 'RelayChain' | 'Ethereum';
}

/** @name PalletCrowdloanRewardsModelsRemoteAccount */
export interface PalletCrowdloanRewardsModelsRemoteAccount extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: AccountId32;
  readonly isEthereum: boolean;
  readonly asEthereum: EthereumAccountId;
  readonly type: 'RelayChain' | 'Ethereum';
}

/** @name PalletCrowdloanRewardsModelsReward */
export interface PalletCrowdloanRewardsModelsReward extends Null {}

/** @name PalletCrowdloanRewardsReward */
export interface PalletCrowdloanRewardsReward extends Null {}

/** @name PalletCurrencyFactoryRanges */
export interface PalletCurrencyFactoryRanges extends Null {}

/** @name PalletCurrencyFactoryRangesRange */
export interface PalletCurrencyFactoryRangesRange extends Null {}

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
export interface PalletLiquidationsLiquidationStrategyConfiguration extends Null {}

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
export interface PalletMosaicRelayerStaleRelayer extends Null {}

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
export interface PolkadotPrimitivesV1PersistedValidationData extends PersistedValidationData {}

/** @name SpConsensusAuraSr25519AppSr25519Public */
export interface SpConsensusAuraSr25519AppSr25519Public extends Null {}

/** @name XcmVersionedMultiAsset */
export interface XcmVersionedMultiAsset extends Null {}

export type PHANTOM_CROWDLOANREWARDS = 'crowdloanRewards';
