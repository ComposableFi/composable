import { DefinitionRpc } from "@polkadot/types/types";

export default {
  rpc: {
    amountAvailableToClaimFor: {
      description: "The unclaimed amount",
      params: [
        {
          name: "accountId",
          type: "AccountId"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        }
      ],
      type: "Balance"
    },
  },
  types: {
    ComposableTraitsAssetsXcmAssetLocation: "Null",
    PalletCrowdloanRewardsModelsReward: "Null",
    PalletCrowdloanRewardsModelsRemoteAccount: {
      _enum: {
        RelayChain: 'AccountId32',
        Ethereum: 'EthereumAccountId'
      }
    },
    ComposableTraitsCallFilterCallFilterEntry: "Null",
    PalletAssetsRegistryCandidateStatus: "Null",
    SpConsensusAuraSr25519AppSr25519Public: "Null",
    ComposableTraitsBondedFinanceBondOffer: {
      beneficiary: "AccountId32",
      asset: "u128",
      bondPrice: "u128",
      nbOfBonds: "u128",
      maturity: "ComposableTraitsBondedFinanceBondDuration",
      reward: "ComposableTraitsBondedFinanceBondOfferReward",
      keepAlive: "bool"
    },
    ComposableTraitsBondedFinanceBondDuration: {
      Finite: {returnIn: "u32"}
    },
    ComposableTraitsBondedFinanceBondOfferReward: {
      asset: "u128",
      amount: "u128",
      maturity: "u32"
    },
    PalletCollatorSelectionCandidateInfo: "Null",
    PalletCrowdloanRewardsReward: "Null",
    PalletDemocracyVoteThreshold: "Null",
    PalletDemocracyPreimageStatus: "Null",
    PalletDemocracyReferendumInfo: "Null",
    PalletPreimageRequestStatus: "Null",
    PalletDemocracyReleases: "Null",
    PalletDemocracyVoteVoting: "Null",
    CumulusPalletDmpQueueConfigData: "Null",
    PalletDutchAuctionSellOrder: "Null",
    ComposableTraitsVestingVestingSchedule: "Null",
    CumulusPalletDmpQueuePageIndexData: "Null",
    PalletDutchAuctionTakeOrder: "Null",
    ComposableTraitsGovernanceSignedRawOrigin: "Null",
    PalletIdentityRegistration: "Null",
    PalletIdentityRegistrarInfo: "Null",
    PalletOracleAssetInfo: "Null",
    PalletOracleWithdraw: {
      stake: 'u128',
      unlockBlock: 'u32'
    },
    PalletOraclePrePrice: "Null",
    PalletOraclePrice: "Null",
    PolkadotPrimitivesV1AbridgedHostConfiguration: "Null",
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: "Null",
    PolkadotPrimitivesV1PersistedValidationData: "PersistedValidationData",
    PalletSchedulerScheduledV2: "Null",
    PalletSchedulerReleases: "Null",
    PalletSchedulerScheduledV3: "Null",
    DaliRuntimeOpaqueSessionKeys: "Null",
    OrmlTokensAccountData: {
      free: 'u128',
      reserved: 'u128',
      frozen: 'u128'
    },
    OrmlTokensBalanceLock: "Null",
    PalletTreasuryProposal: "Null",
    PalletVaultModelsStrategyOverview: "Null",
    PalletVaultModelsVaultInfo: "Null",
    CumulusPalletXcmpQueueInboundStatus: "Null",
    CumulusPalletXcmpQueueInboundChannelDetails: "Null",
    PolkadotParachainPrimitivesXcmpMessageFormat: "Null",
    CumulusPalletXcmpQueueOutboundStatus: "Null",
    CumulusPalletXcmpQueueQueueConfigData: "Null",
    CumulusPalletXcmpQueueOutboundChannelDetails: "Null",
    PalletCrowdloanRewardsModelsProof: {
      _enum: {
        RelayChain: '(AccountId32, MultiSignature)',
        Ethereum: 'PalletCrowdloanRewardsModelsEcdsaSignature'
      }
    },
    PalletCrowdloanRewardsModelsEcdsaSignature: "EcdsaSignature",
    PalletDemocracyConviction: "Null",
    PalletDemocracyVoteAccountVote: "Null",
    ComposableTraitsDefiSell: "Null",
    ComposableTraitsAuctionAuctionStepFunction: "Null",
    ComposableTraitsDefiTake: "Null",
    ComposableTraitsTimeTimeReleaseFunction: "Null",
    PalletIdentityJudgement: "Null",
    PalletIdentityBitFlags: "Null",
    PalletIdentityIdentityInfo: "Null",
    CumulusPrimitivesParachainInherentParachainInherentData: 'ParachainInherentData',
    DaliRuntimeOriginCaller: "Null",
    ComposableTraitsVaultVaultConfig: "Null",
    XcmVersionedMultiAsset: "Null",
    PalletMosaicNetworkInfo: {
      enabled: "bool",
      maxTransferSize: "Balance"
    },
    PalletMosaicDecayBudgetPenaltyDecayer: "Null",
    PalletAssetsRegistryForeignMetadata: "Null",
    PalletMosaicAssetInfo: "Null",
    PalletMosaicRelayerStaleRelayer: "Null",
    FrameSupportScheduleMaybeHashed: "Null",
    FrameSupportScheduleLookupError: "Null",
    PalletCurrencyFactoryRanges: "Null",
    PalletCurrencyFactoryRangesRange: "Null",
    PalletLiquidationsLiquidationStrategyConfiguration: "Null",
    ComposableTraitsDefiCurrencyPair: {
      base: "u128",
      quote: "u128"
    },
    CommonMosaicRemoteAssetId: "Null",
    ComposableTraitsDexConsantProductPoolInfo: "Null",
    ComposableTraitsLendingMarketConfig: "Null",
    ComposableTraitsLendingCreateInput: "Null",
    ComposableTraitsLendingUpdateInput: "Null",
    ComposableTraitsDexStableSwapPoolInfo: "Null",
    ComposableTraitsOraclePrice: "Null",
    PalletLiquidityBootstrappingPool: "Null",
    ComposableTraitsDexConstantProductPoolInfo: {
      owner: "AccountId32",
      pair: {
        base: "u128",
        quote: "u128"
      },
      lpToken: "u128",
      fee: "Permill",
      ownerFee: "Permill"
    },
  },
};
