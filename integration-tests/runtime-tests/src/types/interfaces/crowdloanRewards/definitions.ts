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
    PalletCrowdloanRewardsModelsRemoteAccount: {
      _enum: {
        RelayChain: 'AccountId32',
        Ethereum: 'EthereumAccountId'
      }
    },
    PalletCrowdloanRewardsModelsProof: {
      _enum: {
        RelayChain: '(AccountId32, MultiSignature)',
        Ethereum: 'EcdsaSignature'
      }
    },
    PalletCrowdloanRewardsReward: "Null",
    PalletAssetsRegistryCandidateStatus: "Null",
    SpConsensusAuraSr25519AppSr25519Public: "Null",
    PalletCollatorSelectionCandidateInfo: "Null",
    PalletDemocracyVoteThreshold: "Null",
    PalletDemocracyPreimageStatus: "Null",
    PalletDemocracyReferendumInfo: "Null",
    PalletDemocracyReleases: "Null",
    PalletDemocracyVoteVoting: "Null",
    CumulusPalletDmpQueueConfigData: "Null",
    CumulusPalletDmpQueuePageIndexData: "Null",
    PalletAssetsRegistryForeignMetadata: {
      decimals: 'u32'
    },
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
    DaliRuntimeOpaqueSessionKeys: "Null",
    OrmlTokensAccountData: "Null",
    OrmlTokensBalanceLock: "Null",
    PalletTreasuryProposal: "Null",
    PalletVaultModelsStrategyOverview: "Null",
    PalletVaultModelsVaultInfo: "Null",
    CumulusPalletXcmpQueueInboundStatus: "Null",
    PolkadotParachainPrimitivesXcmpMessageFormat: "Null",
    CumulusPalletXcmpQueueOutboundStatus: "Null",
    CumulusPalletXcmpQueueQueueConfigData: "Null",
    PalletDemocracyConviction: "Null",
    PalletDemocracyVoteAccountVote: "Null",
    PalletIdentityJudgement: "Null",
    PalletIdentityBitFlags: "Null",
    PalletIdentityIdentityInfo: "Null",
    CumulusPrimitivesParachainInherentParachainInherentData: 'ParachainInherentData',
    DaliRuntimeOriginCaller: "Null",
    XcmVersionedMultiAsset: "Null",

    ComposableTraitsAssetsXcmAssetLocation: "Null",
    ComposableTraitsCallFilterCallFilterEntry: "Null",
    ComposableTraitsBondedFinanceBondOffer: "Null",
    ComposableTraitsVestingVestingSchedule: "Null",
    ComposableTraitsGovernanceSignedRawOrigin: "Null",
    ComposableTraitsVaultVaultConfig: "Null",
    ComposableTraitsDefiSell: "Null",
    ComposableTraitsAuctionAuctionStepFunction: "Null",
    ComposableTraitsDefiTake: "Null",
    ComposableTraitsTimeTimeReleaseFunction: "Null",

    PalletDutchAuctionSellOrder: "Null",
    PalletDutchAuctionTakeOrder: "Null",
  },
};
