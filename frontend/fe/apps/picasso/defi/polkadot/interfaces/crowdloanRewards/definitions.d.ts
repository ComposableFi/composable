declare const _default: {
    rpc: {
        amountAvailableToClaimFor: {
            description: string;
            params: ({
                name: string;
                type: string;
                isOptional?: undefined;
            } | {
                name: string;
                type: string;
                isOptional: boolean;
            })[];
            type: string;
        };
    };
    types: {
        ComposableTraitsAssetsXcmAssetLocation: string;
        PalletCrowdloanRewardsModelsReward: string;
        PalletCrowdloanRewardsModelsRemoteAccount: {
            _enum: {
                RelayChain: string;
                Ethereum: string;
            };
        };
        ComposableTraitsCallFilterCallFilterEntry: string;
        PalletAssetsRegistryCandidateStatus: string;
        SpConsensusAuraSr25519AppSr25519Public: string;
        ComposableTraitsBondedFinanceBondOffer: {
            beneficiary: string;
            asset: string;
            bondPrice: string;
            nbOfBonds: string;
            maturity: string;
            reward: string;
            keepAlive: string;
        };
        ComposableTraitsBondedFinanceBondDuration: {
            Finite: {
                returnIn: string;
            };
        };
        ComposableTraitsBondedFinanceBondOfferReward: {
            asset: string;
            amount: string;
            maturity: string;
        };
        PalletCollatorSelectionCandidateInfo: string;
        PalletCrowdloanRewardsReward: string;
        PalletDemocracyVoteThreshold: string;
        PalletDemocracyPreimageStatus: string;
        PalletDemocracyReferendumInfo: string;
        PalletPreimageRequestStatus: string;
        PalletDemocracyReleases: string;
        PalletDemocracyVoteVoting: string;
        CumulusPalletDmpQueueConfigData: string;
        PalletDutchAuctionSellOrder: string;
        ComposableTraitsVestingVestingSchedule: string;
        CumulusPalletDmpQueuePageIndexData: string;
        PalletDutchAuctionTakeOrder: string;
        ComposableTraitsGovernanceSignedRawOrigin: string;
        PalletIdentityRegistration: string;
        PalletIdentityRegistrarInfo: string;
        PalletOracleAssetInfo: string;
        PalletOracleWithdraw: {
            stake: string;
            unlockBlock: string;
        };
        PalletOraclePrePrice: string;
        PalletOraclePrice: string;
        PolkadotPrimitivesV1AbridgedHostConfiguration: string;
        CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: string;
        PolkadotPrimitivesV1PersistedValidationData: string;
        PalletSchedulerScheduledV2: string;
        PalletSchedulerReleases: string;
        PalletSchedulerScheduledV3: string;
        DaliRuntimeOpaqueSessionKeys: string;
        OrmlTokensAccountData: {
            free: string;
            reserved: string;
            frozen: string;
        };
        OrmlTokensBalanceLock: string;
        PalletTreasuryProposal: string;
        PalletVaultModelsStrategyOverview: string;
        PalletVaultModelsVaultInfo: string;
        CumulusPalletXcmpQueueInboundStatus: string;
        CumulusPalletXcmpQueueInboundChannelDetails: string;
        PolkadotParachainPrimitivesXcmpMessageFormat: string;
        CumulusPalletXcmpQueueOutboundStatus: string;
        CumulusPalletXcmpQueueQueueConfigData: string;
        CumulusPalletXcmpQueueOutboundChannelDetails: string;
        PalletCrowdloanRewardsModelsProof: {
            _enum: {
                RelayChain: string;
                Ethereum: string;
            };
        };
        PalletCrowdloanRewardsModelsEcdsaSignature: string;
        PalletDemocracyConviction: string;
        PalletDemocracyVoteAccountVote: string;
        ComposableTraitsDefiSell: string;
        ComposableTraitsAuctionAuctionStepFunction: string;
        ComposableTraitsDefiTake: string;
        ComposableTraitsTimeTimeReleaseFunction: string;
        PalletIdentityJudgement: string;
        PalletIdentityBitFlags: string;
        PalletIdentityIdentityInfo: string;
        CumulusPrimitivesParachainInherentParachainInherentData: string;
        DaliRuntimeOriginCaller: string;
        ComposableTraitsVaultVaultConfig: string;
        XcmVersionedMultiAsset: string;
        PalletMosaicNetworkInfo: {
            enabled: string;
            maxTransferSize: string;
        };
        PalletMosaicDecayBudgetPenaltyDecayer: string;
        PalletAssetsRegistryForeignMetadata: string;
        PalletMosaicAssetInfo: string;
        PalletMosaicRelayerStaleRelayer: {
            relayer: {
                current: string;
                next: {
                    ttl: string;
                    account: string;
                };
            };
        };
        FrameSupportScheduleMaybeHashed: string;
        FrameSupportScheduleLookupError: string;
        PalletCurrencyFactoryRanges: string;
        PalletCurrencyFactoryRangesRange: string;
        PalletLiquidationsLiquidationStrategyConfiguration: string;
        CommonMosaicRemoteAssetId: string;
        ComposableTraitsDexConsantProductPoolInfo: string;
        ComposableTraitsLendingMarketConfig: string;
        ComposableTraitsLendingCreateInput: string;
        ComposableTraitsLendingUpdateInput: string;
        ComposableTraitsDexStableSwapPoolInfo: string;
        ComposableTraitsOraclePrice: string;
        PalletLiquidityBootstrappingPool: string;
        ComposableTraitsDexConstantProductPoolInfo: {
            owner: string;
            pair: string;
            lpToken: string;
            fee: string;
            ownerFee: string;
        };
        ComposableSupportEthereumAddress: string;
        ComposableTraitsAssetsBasicAssetMetadata: string;
        ComposableTraitsDexDexRoute: string;
        ComposableTraitsLendingRepayStrategy: string;
        ComposableTraitsXcmAssetsXcmAssetLocation: string;
        SpTrieStorageProof: string;
        ComposableTraitsXcmAssetsForeignMetadata: string;
    };
};
export default _default;
