// eslint-disable-next-line @typescript-eslint/no-unused-vars

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
          isOptional: true
        }
      ],
      type: "Balance"
    }
  },
  types: {
    ComposableTraitsAssetsXcmAssetLocation: "Null",
    PalletCrowdloanRewardsModelsReward: {
      total: "u128",
      claimed: "u128",
      vestingPeriod: "u64"
    },
    PalletCrowdloanRewardsModelsRemoteAccount: {
      _enum: {
        RelayChain: "AccountId32",
        Ethereum: "EthereumAccountId",
        Registry: "Null"
      }
    },
    ComposableTraitsCallFilterCallFilterEntry: "Null",
    PalletAssetsRegistryCandidateStatus: "Null",
    SpConsensusAuraSr25519AppSr25519Public: "Null",
    ComposableTraitsBondedFinanceBondOffer: {
      beneficiary: "AccountId32",
      asset: "CurrencyId",
      bondPrice: "u128",
      nbOfBonds: "u128",
      maturity: "ComposableTraitsBondedFinanceBondDuration",
      reward: "ComposableTraitsBondedFinanceBondOfferReward",
      keepAlive: "bool"
    },
    ComposableTraitsBondedFinanceBondDuration: {
      Finite: { returnIn: "u32" }
    },
    ComposableTraitsBondedFinanceBondOfferReward: {
      asset: "CurrencyId",
      amount: "u128",
      maturity: "u32"
    },
    PalletCollatorSelectionCandidateInfo: "Null",
    PalletCrowdloanRewardsReward: "Null",
    CumulusPalletDmpQueueConfigData: "Null",
    PalletDutchAuctionSellOrder: "Null",
    CumulusPalletDmpQueuePageIndexData: "Null",
    PalletDutchAuctionTakeOrder: "Null",
    ComposableTraitsGovernanceSignedRawOrigin: {
      _enum: {
        Root: "Null",
        Signed: "",
        isSigned: "bool",
        asSigned: "AccountId32"
      }
    },
    PalletIdentityRegistration: "Null",
    PalletIdentityRegistrarInfo: "Null",
    PalletOracleAssetInfo: "Null",
    PalletOracleWithdraw: {
      stake: "u128",
      unlockBlock: "u32"
    },
    PalletOraclePrePrice: "Null",
    PalletOraclePrice: "Null",
    PolkadotPrimitivesV1AbridgedHostConfiguration: "Null",
    PolkadotPrimitivesV2PersistedValidationData: "Null",
    PolkadotPrimitivesV2UpgradeRestriction: "Null",
    PolkadotPrimitivesV2AbridgedHostConfiguration: "Null",
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: "Null",
    PolkadotPrimitivesV1PersistedValidationData: "PersistedValidationData",
    PalletSchedulerScheduledV2: "Null",
    PalletSchedulerReleases: "Null",
    PalletSchedulerScheduledV3: "Null",
    DaliRuntimeOpaqueSessionKeys: "Null",
    OrmlTokensAccountData: {
      free: "u128",
      reserved: "u128",
      frozen: "u128"
    },
    OrmlTokensBalanceLock: "Null",
    OrmlTokensReserveData: "Null",
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
        RelayChain: "(AccountId32, MultiSignature)",
        Ethereum: "PalletCrowdloanRewardsModelsEcdsaSignature"
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
    CumulusPrimitivesParachainInherentParachainInherentData: "ParachainInherentData",
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
    PalletMosaicRelayerStaleRelayer: {
      relayer: {
        current: "AccountId32",
        next: {
          ttl: "u32",
          account: "AccountId32"
        }
      }
    },
    FrameSupportScheduleMaybeHashed: "Null",
    FrameSupportScheduleLookupError: "Null",
    PalletLiquidationsLiquidationStrategyConfiguration: "Null",
    CommonMosaicRemoteAssetId: "Null",
    ComposableTraitsLendingMarketConfig: "Null",
    ComposableTraitsLendingCreateInput: "Null",
    ComposableTraitsLendingUpdateInput: "Null",
    ComposableTraitsDexStableSwapPoolInfo: "Null",
    ComposableTraitsOraclePrice: "Null",
    PalletLiquidityBootstrappingPool: "Null",
    ComposableTraitsDexConstantProductPoolInfo: {
      owner: "AccountId32",
      pair: "ComposableTraitsDefiCurrencyPairCurrencyId",
      lpToken: "u128",
      fee: "Permill",
      ownerFee: "Permill"
    },
    ComposableSupportEthereumAddress: "Null",
    ComposableTraitsAssetsBasicAssetMetadata: {
      symbol: {
        inner: "Null"
      },
      name: {
        inner: "Null"
      }
    },
    ComposableTraitsDexDexRoute: "Null",
    ComposableTraitsLendingRepayStrategy: "Null",
    ComposableTraitsXcmAssetsXcmAssetLocation: "Null",
    SpTrieStorageProof: "Null",
    ComposableTraitsXcmAssetsForeignMetadata: "Null",
    PalletIbcPingSendPingParams: "Null",
    IbcTraitOpenChannelParams: "Null",
    PalletIbcConnectionParams: "Null",
    PalletIbcAny: "Null",
    PalletIbcIbcConsensusState: "Null",
    PalletIbcEventsIbcEvent: "Null",
    PalletIbcErrorsIbcError: "Null",
    PalletMosaicAmmSwapInfo: "Null",
    ComposableTraitsStakingRewardPool: "Null",
    ComposableTraitsStakingRewardPoolConfiguration: "Null",
    IbcTransferPalletParams: "Null",
    IbcTransferTransferParams: "Null",
    ComposableTraitsOracleRewardTracker: "Null",
    ComposableTraitsStakingStake: "Null",
    ComposableTraitsStakingRewardUpdate: "Null",
    ComposableTraitsAccountProxyProxyType: "Null",
    ComposableTraitsAccountProxyProxyDefinition: "Null",
    PalletAccountProxyAnnouncement: "Null",
    PalletCosmwasmContractInfo: "Null",
    PalletCosmwasmCodeInfo: "Null",
    PalletCosmwasmEntryPoint: "Null",
    PalletStakingRewardsRewardAccumulationHookError: "Null",
    XcmVersionedMultiAssets: "Null",
    XcmVersionedMultiLocation: "Null",
    XcmVersionedXcm: "Null",
    PalletMultisigTimepoint: "Null",
    XcmV2WeightLimit: "Null",
    ComposableTraitsDexAssetAmount: "Null",
    PalletCosmwasmCodeIdentifier: "Null",
    XcmV1MultiLocation: "Null",
    XcmV1MultiAsset: "Null",
    XcmV1MultiassetMultiAssets: "Null",
    XcmV2TraitsOutcome: "Null",
    XcmV2Xcm: "Null",
    SpRuntimeDispatchError: "Null",
    SpRuntimeHeader: "Null",
    SpVersionRuntimeVersion: "Null",
    FrameSupportWeightsRuntimeDbWeight: "Null",
    PalletCollectiveVotes: "Null",
    SpRuntimeDigest: "Null",
    FrameSupportWeightsPerDispatchClassU64: "Null",
    SpCoreCryptoKeyTypeId: "Null",
    PalletXcmQueryStatus: "Null",
    PalletXcmVersionMigrationStage: "Null",
    PolkadotCorePrimitivesOutboundHrmpMessage: "Null",
    PalletBalancesReleases: "Null",
    PalletBalancesReserveData: "Null",
    PalletBalancesBalanceLock: {
      amount: "Null",
    },
    PalletBalancesAccountData: "Null",
    PalletAuthorshipUncleEntryItem: "Null",
    PalletMultisigMultisig: "Null",
    PalletTransactionPaymentReleases: "Null",
    XcmV2TraitsError: "Null",
    XcmV2Response: "Null",
    FrameSupportTokensMiscBalanceStatus: "Null",
    FrameSupportWeightsDispatchInfo: "Null",
    FrameSupportPalletId: "Null",
    ComposableTraitsCurrencyRational64: "Null",
    PalletCosmwasmInstrumentCostRules: "Null",
  }
};
