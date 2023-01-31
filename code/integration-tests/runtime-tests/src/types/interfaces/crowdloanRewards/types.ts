// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { ComposableTraitsDefiCurrencyPairCurrencyId, CurrencyId } from '@composable/types/interfaces/common';
import type { Enum, Null, Struct, bool, u128, u32, u64 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { EthereumAccountId } from '@polkadot/types/interfaces/eth';
import type { EcdsaSignature, MultiSignature } from '@polkadot/types/interfaces/extrinsics';
import type { ParachainInherentData, PersistedValidationData } from '@polkadot/types/interfaces/parachains';
import type { AccountId32, Balance, Permill } from '@polkadot/types/interfaces/runtime';

/** @name CommonMosaicRemoteAssetId */
export interface CommonMosaicRemoteAssetId extends Null {}

/** @name ComposableSupportEthereumAddress */
export interface ComposableSupportEthereumAddress extends Null {}

/** @name ComposableTraitsAccountProxyProxyDefinition */
export interface ComposableTraitsAccountProxyProxyDefinition extends Null {}

/** @name ComposableTraitsAccountProxyProxyType */
export interface ComposableTraitsAccountProxyProxyType extends Null {}

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

/** @name ComposableTraitsCurrencyRational64 */
export interface ComposableTraitsCurrencyRational64 extends Null {}

/** @name ComposableTraitsDefiSell */
export interface ComposableTraitsDefiSell extends Null {}

/** @name ComposableTraitsDefiTake */
export interface ComposableTraitsDefiTake extends Null {}

/** @name ComposableTraitsDexAssetAmount */
export interface ComposableTraitsDexAssetAmount extends Null {}

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
  readonly isIsSigned: boolean;
  readonly asIsSigned: bool;
  readonly isAsSigned: boolean;
  readonly asAsSigned: AccountId32;
  readonly type: 'Root' | 'Signed' | 'IsSigned' | 'AsSigned';
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
export interface ComposableTraitsOraclePrice extends Null {}

/** @name ComposableTraitsOracleRewardTracker */
export interface ComposableTraitsOracleRewardTracker extends Null {}

/** @name ComposableTraitsStakingRewardPool */
export interface ComposableTraitsStakingRewardPool extends Null {}

/** @name ComposableTraitsStakingRewardPoolConfiguration */
export interface ComposableTraitsStakingRewardPoolConfiguration extends Null {}

/** @name ComposableTraitsStakingRewardUpdate */
export interface ComposableTraitsStakingRewardUpdate extends Null {}

/** @name ComposableTraitsStakingStake */
export interface ComposableTraitsStakingStake extends Null {}

/** @name ComposableTraitsTimeTimeReleaseFunction */
export interface ComposableTraitsTimeTimeReleaseFunction extends Null {}

/** @name ComposableTraitsVaultVaultConfig */
export interface ComposableTraitsVaultVaultConfig extends Null {}

/** @name ComposableTraitsXcmAssetsForeignMetadata */
export interface ComposableTraitsXcmAssetsForeignMetadata extends Null {}

/** @name ComposableTraitsXcmAssetsXcmAssetLocation */
export interface ComposableTraitsXcmAssetsXcmAssetLocation extends Null {}

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

/** @name FrameSupportPalletId */
export interface FrameSupportPalletId extends Null {}

/** @name FrameSupportScheduleLookupError */
export interface FrameSupportScheduleLookupError extends Null {}

/** @name FrameSupportScheduleMaybeHashed */
export interface FrameSupportScheduleMaybeHashed extends Null {}

/** @name FrameSupportTokensMiscBalanceStatus */
export interface FrameSupportTokensMiscBalanceStatus extends Null {}

/** @name FrameSupportWeightsDispatchInfo */
export interface FrameSupportWeightsDispatchInfo extends Null {}

/** @name FrameSupportWeightsPerDispatchClassU64 */
export interface FrameSupportWeightsPerDispatchClassU64 extends Null {}

/** @name FrameSupportWeightsRuntimeDbWeight */
export interface FrameSupportWeightsRuntimeDbWeight extends Null {}

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

/** @name PalletAccountProxyAnnouncement */
export interface PalletAccountProxyAnnouncement extends Null {}

/** @name PalletAssetsRegistryCandidateStatus */
export interface PalletAssetsRegistryCandidateStatus extends Null {}

/** @name PalletAssetsRegistryForeignMetadata */
export interface PalletAssetsRegistryForeignMetadata extends Null {}

/** @name PalletAuthorshipUncleEntryItem */
export interface PalletAuthorshipUncleEntryItem extends Null {}

/** @name PalletBalancesAccountData */
export interface PalletBalancesAccountData extends Null {}

/** @name PalletBalancesBalanceLock */
export interface PalletBalancesBalanceLock extends Struct {
  readonly amount: Null;
}

/** @name PalletBalancesReleases */
export interface PalletBalancesReleases extends Null {}

/** @name PalletBalancesReserveData */
export interface PalletBalancesReserveData extends Null {}

/** @name PalletCollatorSelectionCandidateInfo */
export interface PalletCollatorSelectionCandidateInfo extends Null {}

/** @name PalletCollectiveVotes */
export interface PalletCollectiveVotes extends Null {}

/** @name PalletCosmwasmCodeIdentifier */
export interface PalletCosmwasmCodeIdentifier extends Null {}

/** @name PalletCosmwasmCodeInfo */
export interface PalletCosmwasmCodeInfo extends Null {}

/** @name PalletCosmwasmContractInfo */
export interface PalletCosmwasmContractInfo extends Null {}

/** @name PalletCosmwasmEntryPoint */
export interface PalletCosmwasmEntryPoint extends Null {}

/** @name PalletCosmwasmInstrumentCostRules */
export interface PalletCosmwasmInstrumentCostRules extends Null {}

/** @name PalletCrowdloanRewardsModelsEcdsaSignature */
export interface PalletCrowdloanRewardsModelsEcdsaSignature extends EcdsaSignature {}

/** @name PalletCrowdloanRewardsModelsProof */
export interface PalletCrowdloanRewardsModelsProof extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: ITuple<[AccountId32, MultiSignature]>;
  readonly isEthereum: boolean;
  readonly asEthereum: PalletCrowdloanRewardsModelsEcdsaSignature;
  readonly type: 'RelayChain' | 'Ethereum';
}

/** @name PalletCrowdloanRewardsModelsRemoteAccount */
export interface PalletCrowdloanRewardsModelsRemoteAccount extends Enum {
  readonly isRelayChain: boolean;
  readonly asRelayChain: AccountId32;
  readonly isEthereum: boolean;
  readonly asEthereum: EthereumAccountId;
  readonly isRegistry: boolean;
  readonly type: 'RelayChain' | 'Ethereum' | 'Registry';
}

/** @name PalletCrowdloanRewardsModelsReward */
export interface PalletCrowdloanRewardsModelsReward extends Struct {
  readonly total: u128;
  readonly claimed: u128;
  readonly vestingPeriod: u64;
}

/** @name PalletCrowdloanRewardsReward */
export interface PalletCrowdloanRewardsReward extends Null {}

/** @name PalletDemocracyConviction */
export interface PalletDemocracyConviction extends Null {}

/** @name PalletDemocracyVoteAccountVote */
export interface PalletDemocracyVoteAccountVote extends Null {}

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
export interface PalletLiquidationsLiquidationStrategyConfiguration extends Null {}

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

/** @name PalletMultisigMultisig */
export interface PalletMultisigMultisig extends Null {}

/** @name PalletMultisigTimepoint */
export interface PalletMultisigTimepoint extends Null {}

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

/** @name PalletSchedulerReleases */
export interface PalletSchedulerReleases extends Null {}

/** @name PalletSchedulerScheduledV2 */
export interface PalletSchedulerScheduledV2 extends Null {}

/** @name PalletSchedulerScheduledV3 */
export interface PalletSchedulerScheduledV3 extends Null {}

/** @name PalletStakingRewardsRewardAccumulationHookError */
export interface PalletStakingRewardsRewardAccumulationHookError extends Null {}

/** @name PalletTransactionPaymentReleases */
export interface PalletTransactionPaymentReleases extends Null {}

/** @name PalletTreasuryProposal */
export interface PalletTreasuryProposal extends Null {}

/** @name PalletVaultModelsStrategyOverview */
export interface PalletVaultModelsStrategyOverview extends Null {}

/** @name PalletVaultModelsVaultInfo */
export interface PalletVaultModelsVaultInfo extends Null {}

/** @name PalletXcmQueryStatus */
export interface PalletXcmQueryStatus extends Null {}

/** @name PalletXcmVersionMigrationStage */
export interface PalletXcmVersionMigrationStage extends Null {}

/** @name PolkadotCorePrimitivesOutboundHrmpMessage */
export interface PolkadotCorePrimitivesOutboundHrmpMessage extends Null {}

/** @name PolkadotParachainPrimitivesXcmpMessageFormat */
export interface PolkadotParachainPrimitivesXcmpMessageFormat extends Null {}

/** @name PolkadotPrimitivesV1AbridgedHostConfiguration */
export interface PolkadotPrimitivesV1AbridgedHostConfiguration extends Null {}

/** @name PolkadotPrimitivesV1PersistedValidationData */
export interface PolkadotPrimitivesV1PersistedValidationData extends PersistedValidationData {}

/** @name PolkadotPrimitivesV2AbridgedHostConfiguration */
export interface PolkadotPrimitivesV2AbridgedHostConfiguration extends Null {}

/** @name PolkadotPrimitivesV2PersistedValidationData */
export interface PolkadotPrimitivesV2PersistedValidationData extends Null {}

/** @name PolkadotPrimitivesV2UpgradeRestriction */
export interface PolkadotPrimitivesV2UpgradeRestriction extends Null {}

/** @name SpConsensusAuraSr25519AppSr25519Public */
export interface SpConsensusAuraSr25519AppSr25519Public extends Null {}

/** @name SpCoreCryptoKeyTypeId */
export interface SpCoreCryptoKeyTypeId extends Null {}

/** @name SpRuntimeDigest */
export interface SpRuntimeDigest extends Null {}

/** @name SpRuntimeDispatchError */
export interface SpRuntimeDispatchError extends Null {}

/** @name SpRuntimeHeader */
export interface SpRuntimeHeader extends Null {}

/** @name SpTrieStorageProof */
export interface SpTrieStorageProof extends Null {}

/** @name SpVersionRuntimeVersion */
export interface SpVersionRuntimeVersion extends Null {}

/** @name XcmV1MultiAsset */
export interface XcmV1MultiAsset extends Null {}

/** @name XcmV1MultiassetMultiAssets */
export interface XcmV1MultiassetMultiAssets extends Null {}

/** @name XcmV1MultiLocation */
export interface XcmV1MultiLocation extends Null {}

/** @name XcmV2Response */
export interface XcmV2Response extends Null {}

/** @name XcmV2TraitsError */
export interface XcmV2TraitsError extends Null {}

/** @name XcmV2TraitsOutcome */
export interface XcmV2TraitsOutcome extends Null {}

/** @name XcmV2WeightLimit */
export interface XcmV2WeightLimit extends Null {}

/** @name XcmV2Xcm */
export interface XcmV2Xcm extends Null {}

/** @name XcmVersionedMultiAsset */
export interface XcmVersionedMultiAsset extends Null {}

/** @name XcmVersionedMultiAssets */
export interface XcmVersionedMultiAssets extends Null {}

/** @name XcmVersionedMultiLocation */
export interface XcmVersionedMultiLocation extends Null {}

/** @name XcmVersionedXcm */
export interface XcmVersionedXcm extends Null {}

export type PHANTOM_CROWDLOANREWARDS = 'crowdloanRewards';
