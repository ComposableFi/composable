declare const _default: {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: string;
        consumers: string;
        providers: string;
        sufficients: string;
        data: string;
    };
    /**
     * Lookup5: pallet_balances::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: string;
        reserved: string;
        miscFrozen: string;
        feeFrozen: string;
    };
    /**
     * Lookup7: frame_support::weights::PerDispatchClass<T>
     **/
    FrameSupportWeightsPerDispatchClassU64: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup11: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: string;
    };
    /**
     * Lookup13: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: string;
            __Unused1: string;
            __Unused2: string;
            __Unused3: string;
            Consensus: string;
            Seal: string;
            PreRuntime: string;
            __Unused7: string;
            RuntimeEnvironmentUpdated: string;
        };
    };
    /**
     * Lookup16: frame_system::EventRecord<rococo_runtime::Event, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: string;
        event: string;
        topics: string;
    };
    /**
     * Lookup18: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: string;
            ExtrinsicFailed: string;
            CodeUpdated: string;
            NewAccount: string;
            KilledAccount: string;
            Remarked: string;
        };
    };
    /**
     * Lookup19: frame_support::weights::DispatchInfo
     **/
    FrameSupportWeightsDispatchInfo: {
        weight: string;
        class: string;
        paysFee: string;
    };
    /**
     * Lookup20: frame_support::weights::DispatchClass
     **/
    FrameSupportWeightsDispatchClass: {
        _enum: string[];
    };
    /**
     * Lookup21: frame_support::weights::Pays
     **/
    FrameSupportWeightsPays: {
        _enum: string[];
    };
    /**
     * Lookup22: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: string;
            CannotLookup: string;
            BadOrigin: string;
            Module: {
                index: string;
                error: string;
            };
            ConsumerRemaining: string;
            NoProviders: string;
            Token: string;
            Arithmetic: string;
        };
    };
    /**
     * Lookup23: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: string[];
    };
    /**
     * Lookup24: sp_runtime::ArithmeticError
     **/
    SpRuntimeArithmeticError: {
        _enum: string[];
    };
    /**
     * Lookup25: pallet_indices::pallet::Event<T>
     **/
    PalletIndicesEvent: {
        _enum: {
            IndexAssigned: {
                who: string;
                index: string;
            };
            IndexFreed: {
                index: string;
            };
            IndexFrozen: {
                index: string;
                who: string;
            };
        };
    };
    /**
     * Lookup26: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: string;
                freeBalance: string;
            };
            DustLost: {
                account: string;
                amount: string;
            };
            Transfer: {
                from: string;
                to: string;
                amount: string;
            };
            BalanceSet: {
                who: string;
                free: string;
                reserved: string;
            };
            Reserved: {
                who: string;
                amount: string;
            };
            Unreserved: {
                who: string;
                amount: string;
            };
            ReserveRepatriated: {
                from: string;
                to: string;
                amount: string;
                destinationStatus: string;
            };
            Deposit: {
                who: string;
                amount: string;
            };
            Withdraw: {
                who: string;
                amount: string;
            };
            Slashed: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup27: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: string[];
    };
    /**
     * Lookup28: pallet_offences::pallet::Event
     **/
    PalletOffencesEvent: {
        _enum: {
            Offence: {
                kind: string;
                timeslot: string;
            };
        };
    };
    /**
     * Lookup30: pallet_session::pallet::Event
     **/
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: string;
            };
        };
    };
    /**
     * Lookup31: pallet_grandpa::pallet::Event
     **/
    PalletGrandpaEvent: {
        _enum: {
            NewAuthorities: {
                authoritySet: string;
            };
            Paused: string;
            Resumed: string;
        };
    };
    /**
     * Lookup34: sp_finality_grandpa::app::Public
     **/
    SpFinalityGrandpaAppPublic: string;
    /**
     * Lookup35: sp_core::ed25519::Public
     **/
    SpCoreEd25519Public: string;
    /**
     * Lookup36: pallet_im_online::pallet::Event<T>
     **/
    PalletImOnlineEvent: {
        _enum: {
            HeartbeatReceived: {
                authorityId: string;
            };
            AllGood: string;
            SomeOffline: {
                offline: string;
            };
        };
    };
    /**
     * Lookup37: pallet_im_online::sr25519::app_sr25519::Public
     **/
    PalletImOnlineSr25519AppSr25519Public: string;
    /**
     * Lookup38: sp_core::sr25519::Public
     **/
    SpCoreSr25519Public: string;
    /**
     * Lookup42: polkadot_runtime_parachains::inclusion::pallet::Event<T>
     **/
    PolkadotRuntimeParachainsInclusionPalletEvent: {
        _enum: {
            CandidateBacked: string;
            CandidateIncluded: string;
            CandidateTimedOut: string;
        };
    };
    /**
     * Lookup43: polkadot_primitives::v1::CandidateReceipt<primitive_types::H256>
     **/
    PolkadotPrimitivesV1CandidateReceipt: {
        descriptor: string;
        commitmentsHash: string;
    };
    /**
     * Lookup44: polkadot_primitives::v1::CandidateDescriptor<primitive_types::H256>
     **/
    PolkadotPrimitivesV1CandidateDescriptor: {
        paraId: string;
        relayParent: string;
        collator: string;
        persistedValidationDataHash: string;
        povHash: string;
        erasureRoot: string;
        signature: string;
        paraHead: string;
        validationCodeHash: string;
    };
    /**
     * Lookup46: polkadot_primitives::v0::collator_app::Public
     **/
    PolkadotPrimitivesV0CollatorAppPublic: string;
    /**
     * Lookup47: polkadot_primitives::v0::collator_app::Signature
     **/
    PolkadotPrimitivesV0CollatorAppSignature: string;
    /**
     * Lookup48: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: string;
    /**
     * Lookup54: polkadot_runtime_parachains::paras::pallet::Event
     **/
    PolkadotRuntimeParachainsParasPalletEvent: {
        _enum: {
            CurrentCodeUpdated: string;
            CurrentHeadUpdated: string;
            CodeUpgradeScheduled: string;
            NewHeadNoted: string;
            ActionQueued: string;
        };
    };
    /**
     * Lookup55: polkadot_runtime_parachains::ump::pallet::Event
     **/
    PolkadotRuntimeParachainsUmpPalletEvent: {
        _enum: {
            InvalidFormat: string;
            UnsupportedVersion: string;
            ExecutedUpward: string;
            WeightExhausted: string;
            UpwardMessagesReceived: string;
            OverweightEnqueued: string;
            OverweightServiced: string;
        };
    };
    /**
     * Lookup56: xcm::v2::traits::Outcome
     **/
    XcmV2TraitsOutcome: {
        _enum: {
            Complete: string;
            Incomplete: string;
            Error: string;
        };
    };
    /**
     * Lookup57: xcm::v2::traits::Error
     **/
    XcmV2TraitsError: {
        _enum: {
            Overflow: string;
            Unimplemented: string;
            UntrustedReserveLocation: string;
            UntrustedTeleportLocation: string;
            MultiLocationFull: string;
            MultiLocationNotInvertible: string;
            BadOrigin: string;
            InvalidLocation: string;
            AssetNotFound: string;
            FailedToTransactAsset: string;
            NotWithdrawable: string;
            LocationCannotHold: string;
            ExceedsMaxMessageSize: string;
            DestinationUnsupported: string;
            Transport: string;
            Unroutable: string;
            UnknownClaim: string;
            FailedToDecode: string;
            TooMuchWeightRequired: string;
            NotHoldingFees: string;
            TooExpensive: string;
            Trap: string;
            UnhandledXcmVersion: string;
            WeightLimitReached: string;
            Barrier: string;
            WeightNotComputable: string;
        };
    };
    /**
     * Lookup58: polkadot_runtime_parachains::hrmp::pallet::Event<T>
     **/
    PolkadotRuntimeParachainsHrmpPalletEvent: {
        _enum: {
            OpenChannelRequested: string;
            OpenChannelCanceled: string;
            OpenChannelAccepted: string;
            ChannelClosed: string;
        };
    };
    /**
     * Lookup59: polkadot_parachain::primitives::HrmpChannelId
     **/
    PolkadotParachainPrimitivesHrmpChannelId: {
        sender: string;
        recipient: string;
    };
    /**
     * Lookup60: polkadot_runtime_parachains::disputes::pallet::Event<T>
     **/
    PolkadotRuntimeParachainsDisputesPalletEvent: {
        _enum: {
            DisputeInitiated: string;
            DisputeConcluded: string;
            DisputeTimedOut: string;
            Revert: string;
        };
    };
    /**
     * Lookup62: polkadot_runtime_parachains::disputes::DisputeLocation
     **/
    PolkadotRuntimeParachainsDisputesDisputeLocation: {
        _enum: string[];
    };
    /**
     * Lookup63: polkadot_runtime_parachains::disputes::DisputeResult
     **/
    PolkadotRuntimeParachainsDisputesDisputeResult: {
        _enum: string[];
    };
    /**
     * Lookup64: polkadot_runtime_common::paras_registrar::pallet::Event<T>
     **/
    PolkadotRuntimeCommonParasRegistrarPalletEvent: {
        _enum: {
            Registered: string;
            Deregistered: string;
            Reserved: string;
        };
    };
    /**
     * Lookup65: polkadot_runtime_common::auctions::pallet::Event<T>
     **/
    PolkadotRuntimeCommonAuctionsPalletEvent: {
        _enum: {
            AuctionStarted: string;
            AuctionClosed: string;
            Reserved: string;
            Unreserved: string;
            ReserveConfiscated: string;
            BidAccepted: string;
            WinningOffset: string;
        };
    };
    /**
     * Lookup66: polkadot_runtime_common::crowdloan::pallet::Event<T>
     **/
    PolkadotRuntimeCommonCrowdloanPalletEvent: {
        _enum: {
            Created: string;
            Contributed: string;
            Withdrew: string;
            PartiallyRefunded: string;
            AllRefunded: string;
            Dissolved: string;
            HandleBidResult: string;
            Edited: string;
            MemoUpdated: string;
            AddedToNewRaise: string;
        };
    };
    /**
     * Lookup68: polkadot_runtime_common::slots::pallet::Event<T>
     **/
    PolkadotRuntimeCommonSlotsPalletEvent: {
        _enum: {
            NewLeasePeriod: string;
            Leased: string;
        };
    };
    /**
     * Lookup69: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: string;
            };
            KeyChanged: {
                newSudoer: string;
            };
            SudoAsDone: {
                sudoResult: string;
            };
        };
    };
    /**
     * Lookup70: rococo_runtime::validator_manager::RawEvent<sp_core::crypto::AccountId32>
     **/
    RococoRuntimeValidatorManagerRawEvent: {
        _enum: {
            ValidatorsRegistered: string;
            ValidatorsDeregistered: string;
        };
    };
    /**
     * Lookup72: pallet_collective::pallet::Event<T, I>
     **/
    PalletCollectiveEvent: {
        _enum: {
            Proposed: {
                account: string;
                proposalIndex: string;
                proposalHash: string;
                threshold: string;
            };
            Voted: {
                account: string;
                proposalHash: string;
                voted: string;
                yes: string;
                no: string;
            };
            Approved: {
                proposalHash: string;
            };
            Disapproved: {
                proposalHash: string;
            };
            Executed: {
                proposalHash: string;
                result: string;
            };
            MemberExecuted: {
                proposalHash: string;
                result: string;
            };
            Closed: {
                proposalHash: string;
                yes: string;
                no: string;
            };
        };
    };
    /**
     * Lookup74: pallet_membership::pallet::Event<T, I>
     **/
    PalletMembershipEvent: {
        _enum: string[];
    };
    /**
     * Lookup75: pallet_utility::pallet::Event
     **/
    PalletUtilityEvent: {
        _enum: {
            BatchInterrupted: {
                index: string;
                error: string;
            };
            BatchCompleted: string;
            ItemCompleted: string;
            DispatchedAs: string;
        };
    };
    /**
     * Lookup76: pallet_proxy::pallet::Event<T>
     **/
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: string;
            };
            AnonymousCreated: {
                anonymous: string;
                who: string;
                proxyType: string;
                disambiguationIndex: string;
            };
            Announced: {
                real: string;
                proxy: string;
                callHash: string;
            };
            ProxyAdded: {
                delegator: string;
                delegatee: string;
                proxyType: string;
                delay: string;
            };
        };
    };
    /**
     * Lookup77: rococo_runtime::ProxyType
     **/
    RococoRuntimeProxyType: {
        _enum: string[];
    };
    /**
     * Lookup79: pallet_multisig::pallet::Event<T>
     **/
    PalletMultisigEvent: {
        _enum: {
            NewMultisig: {
                approving: string;
                multisig: string;
                callHash: string;
            };
            MultisigApproval: {
                approving: string;
                timepoint: string;
                multisig: string;
                callHash: string;
            };
            MultisigExecuted: {
                approving: string;
                timepoint: string;
                multisig: string;
                callHash: string;
                result: string;
            };
            MultisigCancelled: {
                cancelling: string;
                timepoint: string;
                multisig: string;
                callHash: string;
            };
        };
    };
    /**
     * Lookup80: pallet_multisig::Timepoint<BlockNumber>
     **/
    PalletMultisigTimepoint: {
        height: string;
        index: string;
    };
    /**
     * Lookup81: pallet_xcm::pallet::Event<T>
     **/
    PalletXcmEvent: {
        _enum: {
            Attempted: string;
            Sent: string;
            UnexpectedResponse: string;
            ResponseReady: string;
            Notified: string;
            NotifyOverweight: string;
            NotifyDispatchError: string;
            NotifyDecodeFailed: string;
            InvalidResponder: string;
            InvalidResponderVersion: string;
            ResponseTaken: string;
            AssetsTrapped: string;
            VersionChangeNotified: string;
            SupportedVersionChanged: string;
            NotifyTargetSendFail: string;
            NotifyTargetMigrationFail: string;
        };
    };
    /**
     * Lookup82: xcm::v1::multilocation::MultiLocation
     **/
    XcmV1MultiLocation: {
        parents: string;
        interior: string;
    };
    /**
     * Lookup83: xcm::v1::multilocation::Junctions
     **/
    XcmV1MultilocationJunctions: {
        _enum: {
            Here: string;
            X1: string;
            X2: string;
            X3: string;
            X4: string;
            X5: string;
            X6: string;
            X7: string;
            X8: string;
        };
    };
    /**
     * Lookup84: xcm::v1::junction::Junction
     **/
    XcmV1Junction: {
        _enum: {
            Parachain: string;
            AccountId32: {
                network: string;
                id: string;
            };
            AccountIndex64: {
                network: string;
                index: string;
            };
            AccountKey20: {
                network: string;
                key: string;
            };
            PalletInstance: string;
            GeneralIndex: string;
            GeneralKey: string;
            OnlyChild: string;
            Plurality: {
                id: string;
                part: string;
            };
        };
    };
    /**
     * Lookup86: xcm::v0::junction::NetworkId
     **/
    XcmV0JunctionNetworkId: {
        _enum: {
            Any: string;
            Named: string;
            Polkadot: string;
            Kusama: string;
        };
    };
    /**
     * Lookup90: xcm::v0::junction::BodyId
     **/
    XcmV0JunctionBodyId: {
        _enum: {
            Unit: string;
            Named: string;
            Index: string;
            Executive: string;
            Technical: string;
            Legislative: string;
            Judicial: string;
        };
    };
    /**
     * Lookup91: xcm::v0::junction::BodyPart
     **/
    XcmV0JunctionBodyPart: {
        _enum: {
            Voice: string;
            Members: {
                count: string;
            };
            Fraction: {
                nom: string;
                denom: string;
            };
            AtLeastProportion: {
                nom: string;
                denom: string;
            };
            MoreThanProportion: {
                nom: string;
                denom: string;
            };
        };
    };
    /**
     * Lookup92: xcm::v2::Xcm<Call>
     **/
    XcmV2Xcm: string;
    /**
     * Lookup94: xcm::v2::Instruction<Call>
     **/
    XcmV2Instruction: {
        _enum: {
            WithdrawAsset: string;
            ReserveAssetDeposited: string;
            ReceiveTeleportedAsset: string;
            QueryResponse: {
                queryId: string;
                response: string;
                maxWeight: string;
            };
            TransferAsset: {
                assets: string;
                beneficiary: string;
            };
            TransferReserveAsset: {
                assets: string;
                dest: string;
                xcm: string;
            };
            Transact: {
                originType: string;
                requireWeightAtMost: string;
                call: string;
            };
            HrmpNewChannelOpenRequest: {
                sender: string;
                maxMessageSize: string;
                maxCapacity: string;
            };
            HrmpChannelAccepted: {
                recipient: string;
            };
            HrmpChannelClosing: {
                initiator: string;
                sender: string;
                recipient: string;
            };
            ClearOrigin: string;
            DescendOrigin: string;
            ReportError: {
                queryId: string;
                dest: string;
                maxResponseWeight: string;
            };
            DepositAsset: {
                assets: string;
                maxAssets: string;
                beneficiary: string;
            };
            DepositReserveAsset: {
                assets: string;
                maxAssets: string;
                dest: string;
                xcm: string;
            };
            ExchangeAsset: {
                give: string;
                receive: string;
            };
            InitiateReserveWithdraw: {
                assets: string;
                reserve: string;
                xcm: string;
            };
            InitiateTeleport: {
                assets: string;
                dest: string;
                xcm: string;
            };
            QueryHolding: {
                queryId: string;
                dest: string;
                assets: string;
                maxResponseWeight: string;
            };
            BuyExecution: {
                fees: string;
                weightLimit: string;
            };
            RefundSurplus: string;
            SetErrorHandler: string;
            SetAppendix: string;
            ClearError: string;
            ClaimAsset: {
                assets: string;
                ticket: string;
            };
            Trap: string;
            SubscribeVersion: {
                queryId: string;
                maxResponseWeight: string;
            };
            UnsubscribeVersion: string;
        };
    };
    /**
     * Lookup95: xcm::v1::multiasset::MultiAssets
     **/
    XcmV1MultiassetMultiAssets: string;
    /**
     * Lookup97: xcm::v1::multiasset::MultiAsset
     **/
    XcmV1MultiAsset: {
        id: string;
        fun: string;
    };
    /**
     * Lookup98: xcm::v1::multiasset::AssetId
     **/
    XcmV1MultiassetAssetId: {
        _enum: {
            Concrete: string;
            Abstract: string;
        };
    };
    /**
     * Lookup99: xcm::v1::multiasset::Fungibility
     **/
    XcmV1MultiassetFungibility: {
        _enum: {
            Fungible: string;
            NonFungible: string;
        };
    };
    /**
     * Lookup100: xcm::v1::multiasset::AssetInstance
     **/
    XcmV1MultiassetAssetInstance: {
        _enum: {
            Undefined: string;
            Index: string;
            Array4: string;
            Array8: string;
            Array16: string;
            Array32: string;
            Blob: string;
        };
    };
    /**
     * Lookup102: xcm::v2::Response
     **/
    XcmV2Response: {
        _enum: {
            Null: string;
            Assets: string;
            ExecutionResult: string;
            Version: string;
        };
    };
    /**
     * Lookup105: xcm::v0::OriginKind
     **/
    XcmV0OriginKind: {
        _enum: string[];
    };
    /**
     * Lookup106: xcm::double_encoded::DoubleEncoded<T>
     **/
    XcmDoubleEncoded: {
        encoded: string;
    };
    /**
     * Lookup107: xcm::v1::multiasset::MultiAssetFilter
     **/
    XcmV1MultiassetMultiAssetFilter: {
        _enum: {
            Definite: string;
            Wild: string;
        };
    };
    /**
     * Lookup108: xcm::v1::multiasset::WildMultiAsset
     **/
    XcmV1MultiassetWildMultiAsset: {
        _enum: {
            All: string;
            AllOf: {
                id: string;
                fun: string;
            };
        };
    };
    /**
     * Lookup109: xcm::v1::multiasset::WildFungibility
     **/
    XcmV1MultiassetWildFungibility: {
        _enum: string[];
    };
    /**
     * Lookup110: xcm::v2::WeightLimit
     **/
    XcmV2WeightLimit: {
        _enum: {
            Unlimited: string;
            Limited: string;
        };
    };
    /**
     * Lookup112: xcm::VersionedMultiAssets
     **/
    XcmVersionedMultiAssets: {
        _enum: {
            V0: string;
            V1: string;
        };
    };
    /**
     * Lookup114: xcm::v0::multi_asset::MultiAsset
     **/
    XcmV0MultiAsset: {
        _enum: {
            None: string;
            All: string;
            AllFungible: string;
            AllNonFungible: string;
            AllAbstractFungible: {
                id: string;
            };
            AllAbstractNonFungible: {
                class: string;
            };
            AllConcreteFungible: {
                id: string;
            };
            AllConcreteNonFungible: {
                class: string;
            };
            AbstractFungible: {
                id: string;
                amount: string;
            };
            AbstractNonFungible: {
                class: string;
                instance: string;
            };
            ConcreteFungible: {
                id: string;
                amount: string;
            };
            ConcreteNonFungible: {
                class: string;
                instance: string;
            };
        };
    };
    /**
     * Lookup115: xcm::v0::multi_location::MultiLocation
     **/
    XcmV0MultiLocation: {
        _enum: {
            Null: string;
            X1: string;
            X2: string;
            X3: string;
            X4: string;
            X5: string;
            X6: string;
            X7: string;
            X8: string;
        };
    };
    /**
     * Lookup116: xcm::v0::junction::Junction
     **/
    XcmV0Junction: {
        _enum: {
            Parent: string;
            Parachain: string;
            AccountId32: {
                network: string;
                id: string;
            };
            AccountIndex64: {
                network: string;
                index: string;
            };
            AccountKey20: {
                network: string;
                key: string;
            };
            PalletInstance: string;
            GeneralIndex: string;
            GeneralKey: string;
            OnlyChild: string;
            Plurality: {
                id: string;
                part: string;
            };
        };
    };
    /**
     * Lookup117: xcm::VersionedMultiLocation
     **/
    XcmVersionedMultiLocation: {
        _enum: {
            V0: string;
            V1: string;
        };
    };
    /**
     * Lookup118: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: string;
            Finalization: string;
            Initialization: string;
        };
    };
    /**
     * Lookup122: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: string;
        specName: string;
    };
    /**
     * Lookup124: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            fill_block: {
                ratio: string;
            };
            remark: {
                remark: string;
            };
            set_heap_pages: {
                pages: string;
            };
            set_code: {
                code: string;
            };
            set_code_without_checks: {
                code: string;
            };
            set_storage: {
                items: string;
            };
            kill_storage: {
                _alias: {
                    keys_: string;
                };
                keys_: string;
            };
            kill_prefix: {
                prefix: string;
                subkeys: string;
            };
            remark_with_event: {
                remark: string;
            };
        };
    };
    /**
     * Lookup129: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: string;
        maxBlock: string;
        perClass: string;
    };
    /**
     * Lookup130: frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportWeightsPerDispatchClassWeightsPerClass: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup131: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: string;
        maxExtrinsic: string;
        maxTotal: string;
        reserved: string;
    };
    /**
     * Lookup133: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: string;
    };
    /**
     * Lookup134: frame_support::weights::PerDispatchClass<T>
     **/
    FrameSupportWeightsPerDispatchClassU32: {
        normal: string;
        operational: string;
        mandatory: string;
    };
    /**
     * Lookup135: frame_support::weights::RuntimeDbWeight
     **/
    FrameSupportWeightsRuntimeDbWeight: {
        read: string;
        write: string;
    };
    /**
     * Lookup136: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: string;
        implName: string;
        authoringVersion: string;
        specVersion: string;
        implVersion: string;
        apis: string;
        transactionVersion: string;
    };
    /**
     * Lookup140: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: string[];
    };
    /**
     * Lookup143: sp_consensus_babe::app::Public
     **/
    SpConsensusBabeAppPublic: string;
    /**
     * Lookup146: sp_consensus_babe::digests::NextConfigDescriptor
     **/
    SpConsensusBabeDigestsNextConfigDescriptor: {
        _enum: {
            __Unused0: string;
            V1: {
                c: string;
                allowedSlots: string;
            };
        };
    };
    /**
     * Lookup148: sp_consensus_babe::AllowedSlots
     **/
    SpConsensusBabeAllowedSlots: {
        _enum: string[];
    };
    /**
     * Lookup152: sp_consensus_babe::BabeEpochConfiguration
     **/
    SpConsensusBabeBabeEpochConfiguration: {
        c: string;
        allowedSlots: string;
    };
    /**
     * Lookup153: pallet_babe::pallet::Call<T>
     **/
    PalletBabeCall: {
        _enum: {
            report_equivocation: {
                equivocationProof: string;
                keyOwnerProof: string;
            };
            report_equivocation_unsigned: {
                equivocationProof: string;
                keyOwnerProof: string;
            };
            plan_config_change: {
                config: string;
            };
        };
    };
    /**
     * Lookup154: sp_consensus_slots::EquivocationProof<sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>, sp_consensus_babe::app::Public>
     **/
    SpConsensusSlotsEquivocationProof: {
        offender: string;
        slot: string;
        firstHeader: string;
        secondHeader: string;
    };
    /**
     * Lookup155: sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>
     **/
    SpRuntimeHeader: {
        parentHash: string;
        number: string;
        stateRoot: string;
        extrinsicsRoot: string;
        digest: string;
    };
    /**
     * Lookup156: sp_runtime::traits::BlakeTwo256
     **/
    SpRuntimeBlakeTwo256: string;
    /**
     * Lookup157: sp_session::MembershipProof
     **/
    SpSessionMembershipProof: {
        session: string;
        trieNodes: string;
        validatorCount: string;
    };
    /**
     * Lookup158: pallet_babe::pallet::Error<T>
     **/
    PalletBabeError: {
        _enum: string[];
    };
    /**
     * Lookup159: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: string;
            };
        };
    };
    /**
     * Lookup161: pallet_indices::pallet::Call<T>
     **/
    PalletIndicesCall: {
        _enum: {
            claim: {
                index: string;
            };
            transfer: {
                _alias: {
                    new_: string;
                };
                new_: string;
                index: string;
            };
            free: {
                index: string;
            };
            force_transfer: {
                _alias: {
                    new_: string;
                };
                new_: string;
                index: string;
                freeze: string;
            };
            freeze: {
                index: string;
            };
        };
    };
    /**
     * Lookup162: pallet_indices::pallet::Error<T>
     **/
    PalletIndicesError: {
        _enum: string[];
    };
    /**
     * Lookup164: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: string;
        amount: string;
        reasons: string;
    };
    /**
     * Lookup165: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: string[];
    };
    /**
     * Lookup168: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: string;
        amount: string;
    };
    /**
     * Lookup170: pallet_balances::Releases
     **/
    PalletBalancesReleases: {
        _enum: string[];
    };
    /**
     * Lookup171: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer: {
                dest: string;
                value: string;
            };
            set_balance: {
                who: string;
                newFree: string;
                newReserved: string;
            };
            force_transfer: {
                source: string;
                dest: string;
                value: string;
            };
            transfer_keep_alive: {
                dest: string;
                value: string;
            };
            transfer_all: {
                dest: string;
                keepAlive: string;
            };
            force_unreserve: {
                who: string;
                amount: string;
            };
        };
    };
    /**
     * Lookup174: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: string[];
    };
    /**
     * Lookup176: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: string[];
    };
    /**
     * Lookup178: frame_support::weights::WeightToFeeCoefficient<Balance>
     **/
    FrameSupportWeightsWeightToFeeCoefficient: {
        coeffInteger: string;
        coeffFrac: string;
        negative: string;
        degree: string;
    };
    /**
     * Lookup180: pallet_authorship::UncleEntryItem<BlockNumber, primitive_types::H256, sp_core::crypto::AccountId32>
     **/
    PalletAuthorshipUncleEntryItem: {
        _enum: {
            InclusionHeight: string;
            Uncle: string;
        };
    };
    /**
     * Lookup182: pallet_authorship::pallet::Call<T>
     **/
    PalletAuthorshipCall: {
        _enum: {
            set_uncles: {
                newUncles: string;
            };
        };
    };
    /**
     * Lookup184: pallet_authorship::pallet::Error<T>
     **/
    PalletAuthorshipError: {
        _enum: string[];
    };
    /**
     * Lookup185: sp_staking::offence::OffenceDetails<sp_core::crypto::AccountId32, Offender>
     **/
    SpStakingOffenceOffenceDetails: {
        offender: string;
        reporters: string;
    };
    /**
     * Lookup189: rococo_runtime::SessionKeys
     **/
    RococoRuntimeSessionKeys: {
        grandpa: string;
        babe: string;
        imOnline: string;
        paraValidator: string;
        paraAssignment: string;
        authorityDiscovery: string;
        beefy: string;
    };
    /**
     * Lookup190: polkadot_primitives::v0::validator_app::Public
     **/
    PolkadotPrimitivesV0ValidatorAppPublic: string;
    /**
     * Lookup191: polkadot_primitives::v1::assignment_app::Public
     **/
    PolkadotPrimitivesV1AssignmentAppPublic: string;
    /**
     * Lookup192: sp_authority_discovery::app::Public
     **/
    SpAuthorityDiscoveryAppPublic: string;
    /**
     * Lookup193: beefy_primitives::crypto::Public
     **/
    BeefyPrimitivesCryptoPublic: string;
    /**
     * Lookup194: sp_core::ecdsa::Public
     **/
    SpCoreEcdsaPublic: string;
    /**
     * Lookup198: sp_core::crypto::KeyTypeId
     **/
    SpCoreCryptoKeyTypeId: string;
    /**
     * Lookup199: pallet_session::pallet::Call<T>
     **/
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: string;
                };
                keys_: string;
                proof: string;
            };
            purge_keys: string;
        };
    };
    /**
     * Lookup200: pallet_session::pallet::Error<T>
     **/
    PalletSessionError: {
        _enum: string[];
    };
    /**
     * Lookup201: pallet_grandpa::StoredState<N>
     **/
    PalletGrandpaStoredState: {
        _enum: {
            Live: string;
            PendingPause: {
                scheduledAt: string;
                delay: string;
            };
            Paused: string;
            PendingResume: {
                scheduledAt: string;
                delay: string;
            };
        };
    };
    /**
     * Lookup202: pallet_grandpa::StoredPendingChange<N, Limit>
     **/
    PalletGrandpaStoredPendingChange: {
        scheduledAt: string;
        delay: string;
        nextAuthorities: string;
        forced: string;
    };
    /**
     * Lookup205: pallet_grandpa::pallet::Call<T>
     **/
    PalletGrandpaCall: {
        _enum: {
            report_equivocation: {
                equivocationProof: string;
                keyOwnerProof: string;
            };
            report_equivocation_unsigned: {
                equivocationProof: string;
                keyOwnerProof: string;
            };
            note_stalled: {
                delay: string;
                bestFinalizedBlockNumber: string;
            };
        };
    };
    /**
     * Lookup206: sp_finality_grandpa::EquivocationProof<primitive_types::H256, N>
     **/
    SpFinalityGrandpaEquivocationProof: {
        setId: string;
        equivocation: string;
    };
    /**
     * Lookup207: sp_finality_grandpa::Equivocation<primitive_types::H256, N>
     **/
    SpFinalityGrandpaEquivocation: {
        _enum: {
            Prevote: string;
            Precommit: string;
        };
    };
    /**
     * Lookup208: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrevote: {
        roundNumber: string;
        identity: string;
        first: string;
        second: string;
    };
    /**
     * Lookup209: finality_grandpa::Prevote<primitive_types::H256, N>
     **/
    FinalityGrandpaPrevote: {
        targetHash: string;
        targetNumber: string;
    };
    /**
     * Lookup210: sp_finality_grandpa::app::Signature
     **/
    SpFinalityGrandpaAppSignature: string;
    /**
     * Lookup211: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: string;
    /**
     * Lookup213: finality_grandpa::Equivocation<sp_finality_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_finality_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrecommit: {
        roundNumber: string;
        identity: string;
        first: string;
        second: string;
    };
    /**
     * Lookup214: finality_grandpa::Precommit<primitive_types::H256, N>
     **/
    FinalityGrandpaPrecommit: {
        targetHash: string;
        targetNumber: string;
    };
    /**
     * Lookup216: pallet_grandpa::pallet::Error<T>
     **/
    PalletGrandpaError: {
        _enum: string[];
    };
    /**
     * Lookup220: pallet_im_online::BoundedOpaqueNetworkState<PeerIdEncodingLimit, MultiAddrEncodingLimit, AddressesLimit>
     **/
    PalletImOnlineBoundedOpaqueNetworkState: {
        peerId: string;
        externalAddresses: string;
    };
    /**
     * Lookup225: pallet_im_online::pallet::Call<T>
     **/
    PalletImOnlineCall: {
        _enum: {
            heartbeat: {
                heartbeat: string;
                signature: string;
            };
        };
    };
    /**
     * Lookup226: pallet_im_online::Heartbeat<BlockNumber>
     **/
    PalletImOnlineHeartbeat: {
        blockNumber: string;
        networkState: string;
        sessionIndex: string;
        authorityIndex: string;
        validatorsLen: string;
    };
    /**
     * Lookup227: sp_core::offchain::OpaqueNetworkState
     **/
    SpCoreOffchainOpaqueNetworkState: {
        peerId: string;
        externalAddresses: string;
    };
    /**
     * Lookup231: pallet_im_online::sr25519::app_sr25519::Signature
     **/
    PalletImOnlineSr25519AppSr25519Signature: string;
    /**
     * Lookup232: pallet_im_online::pallet::Error<T>
     **/
    PalletImOnlineError: {
        _enum: string[];
    };
    /**
     * Lookup233: polkadot_runtime_parachains::configuration::HostConfiguration<BlockNumber>
     **/
    PolkadotRuntimeParachainsConfigurationHostConfiguration: {
        maxCodeSize: string;
        maxHeadDataSize: string;
        maxUpwardQueueCount: string;
        maxUpwardQueueSize: string;
        maxUpwardMessageSize: string;
        maxUpwardMessageNumPerCandidate: string;
        hrmpMaxMessageNumPerCandidate: string;
        validationUpgradeFrequency: string;
        validationUpgradeDelay: string;
        maxPovSize: string;
        maxDownwardMessageSize: string;
        umpServiceTotalWeight: string;
        hrmpMaxParachainOutboundChannels: string;
        hrmpMaxParathreadOutboundChannels: string;
        hrmpSenderDeposit: string;
        hrmpRecipientDeposit: string;
        hrmpChannelMaxCapacity: string;
        hrmpChannelMaxTotalSize: string;
        hrmpMaxParachainInboundChannels: string;
        hrmpMaxParathreadInboundChannels: string;
        hrmpChannelMaxMessageSize: string;
        codeRetentionPeriod: string;
        parathreadCores: string;
        parathreadRetries: string;
        groupRotationFrequency: string;
        chainAvailabilityPeriod: string;
        threadAvailabilityPeriod: string;
        schedulingLookahead: string;
        maxValidatorsPerCore: string;
        maxValidators: string;
        disputePeriod: string;
        disputePostConclusionAcceptancePeriod: string;
        disputeMaxSpamSlots: string;
        disputeConclusionByTimeOutPeriod: string;
        noShowSlots: string;
        nDelayTranches: string;
        zerothDelayTrancheWidth: string;
        neededApprovals: string;
        relayVrfModuloSamples: string;
        umpMaxIndividualWeight: string;
    };
    /**
     * Lookup234: polkadot_runtime_parachains::configuration::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsConfigurationPalletCall: {
        _enum: {
            set_validation_upgrade_frequency: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_validation_upgrade_delay: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_code_retention_period: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_code_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_pov_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_head_data_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_parathread_cores: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_parathread_retries: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_group_rotation_frequency: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_chain_availability_period: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_thread_availability_period: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_scheduling_lookahead: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_validators_per_core: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_validators: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_dispute_period: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_dispute_post_conclusion_acceptance_period: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_dispute_max_spam_slots: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_dispute_conclusion_by_time_out_period: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_no_show_slots: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_n_delay_tranches: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_zeroth_delay_tranche_width: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_needed_approvals: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_relay_vrf_modulo_samples: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_upward_queue_count: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_upward_queue_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_downward_message_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_ump_service_total_weight: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_upward_message_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_max_upward_message_num_per_candidate: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_open_request_ttl: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_sender_deposit: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_recipient_deposit: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_channel_max_capacity: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_channel_max_total_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_max_parachain_inbound_channels: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_max_parathread_inbound_channels: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_channel_max_message_size: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_max_parachain_outbound_channels: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_max_parathread_outbound_channels: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_hrmp_max_message_num_per_candidate: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_ump_max_individual_weight: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
        };
    };
    /**
     * Lookup235: polkadot_runtime_parachains::configuration::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsConfigurationPalletError: {
        _enum: string[];
    };
    /**
     * Lookup239: polkadot_runtime_parachains::shared::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsSharedPalletCall: string;
    /**
     * Lookup240: polkadot_runtime_parachains::inclusion::AvailabilityBitfieldRecord<N>
     **/
    PolkadotRuntimeParachainsInclusionAvailabilityBitfieldRecord: {
        bitfield: string;
        submittedAt: string;
    };
    /**
     * Lookup243: bitvec::order::Lsb0
     **/
    BitvecOrderLsb0: string;
    /**
     * Lookup244: polkadot_runtime_parachains::inclusion::CandidatePendingAvailability<primitive_types::H256, N>
     **/
    PolkadotRuntimeParachainsInclusionCandidatePendingAvailability: {
        _alias: {
            hash_: string;
        };
        core: string;
        hash_: string;
        descriptor: string;
        availabilityVotes: string;
        backers: string;
        relayParentNumber: string;
        backedInNumber: string;
        backingGroup: string;
    };
    /**
     * Lookup245: polkadot_primitives::v1::CandidateCommitments<N>
     **/
    PolkadotPrimitivesV1CandidateCommitments: {
        upwardMessages: string;
        horizontalMessages: string;
        newValidationCode: string;
        headData: string;
        processedDownwardMessages: string;
        hrmpWatermark: string;
    };
    /**
     * Lookup247: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
     **/
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: string;
        data: string;
    };
    /**
     * Lookup250: polkadot_runtime_parachains::inclusion::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsInclusionPalletCall: string;
    /**
     * Lookup251: polkadot_runtime_parachains::inclusion::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsInclusionPalletError: {
        _enum: string[];
    };
    /**
     * Lookup252: polkadot_primitives::v1::ScrapedOnChainVotes<primitive_types::H256>
     **/
    PolkadotPrimitivesV1ScrapedOnChainVotes: {
        session: string;
        backingValidatorsPerCandidate: string;
        disputes: string;
    };
    /**
     * Lookup257: polkadot_primitives::v0::ValidityAttestation
     **/
    PolkadotPrimitivesV0ValidityAttestation: {
        _enum: {
            __Unused0: string;
            Implicit: string;
            Explicit: string;
        };
    };
    /**
     * Lookup258: polkadot_primitives::v0::validator_app::Signature
     **/
    PolkadotPrimitivesV0ValidatorAppSignature: string;
    /**
     * Lookup260: polkadot_primitives::v1::DisputeStatementSet
     **/
    PolkadotPrimitivesV1DisputeStatementSet: {
        candidateHash: string;
        session: string;
        statements: string;
    };
    /**
     * Lookup263: polkadot_primitives::v1::DisputeStatement
     **/
    PolkadotPrimitivesV1DisputeStatement: {
        _enum: {
            Valid: string;
            Invalid: string;
        };
    };
    /**
     * Lookup264: polkadot_primitives::v1::ValidDisputeStatementKind
     **/
    PolkadotPrimitivesV1ValidDisputeStatementKind: {
        _enum: {
            Explicit: string;
            BackingSeconded: string;
            BackingValid: string;
            ApprovalChecking: string;
        };
    };
    /**
     * Lookup265: polkadot_primitives::v1::InvalidDisputeStatementKind
     **/
    PolkadotPrimitivesV1InvalidDisputeStatementKind: {
        _enum: string[];
    };
    /**
     * Lookup266: polkadot_runtime_parachains::paras_inherent::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsParasInherentPalletCall: {
        _enum: {
            enter: {
                data: string;
            };
        };
    };
    /**
     * Lookup267: polkadot_primitives::v1::InherentData<sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>>
     **/
    PolkadotPrimitivesV1InherentData: {
        bitfields: string;
        backedCandidates: string;
        disputes: string;
        parentHeader: string;
    };
    /**
     * Lookup269: polkadot_primitives::v1::signed::UncheckedSigned<polkadot_primitives::v1::AvailabilityBitfield, polkadot_primitives::v1::AvailabilityBitfield>
     **/
    PolkadotPrimitivesV1SignedUncheckedSigned: {
        payload: string;
        validatorIndex: string;
        signature: string;
    };
    /**
     * Lookup271: polkadot_primitives::v1::BackedCandidate<primitive_types::H256>
     **/
    PolkadotPrimitivesV1BackedCandidate: {
        candidate: string;
        validityVotes: string;
        validatorIndices: string;
    };
    /**
     * Lookup272: polkadot_primitives::v1::CommittedCandidateReceipt<primitive_types::H256>
     **/
    PolkadotPrimitivesV1CommittedCandidateReceipt: {
        descriptor: string;
        commitments: string;
    };
    /**
     * Lookup274: polkadot_runtime_parachains::paras_inherent::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsParasInherentPalletError: {
        _enum: string[];
    };
    /**
     * Lookup276: polkadot_runtime_parachains::scheduler::ParathreadClaimQueue
     **/
    PolkadotRuntimeParachainsSchedulerParathreadClaimQueue: {
        queue: string;
        nextCoreOffset: string;
    };
    /**
     * Lookup278: polkadot_runtime_parachains::scheduler::QueuedParathread
     **/
    PolkadotRuntimeParachainsSchedulerQueuedParathread: {
        claim: string;
        coreOffset: string;
    };
    /**
     * Lookup279: polkadot_primitives::v1::ParathreadEntry
     **/
    PolkadotPrimitivesV1ParathreadEntry: {
        claim: string;
        retries: string;
    };
    /**
     * Lookup280: polkadot_primitives::v1::ParathreadClaim
     **/
    PolkadotPrimitivesV1ParathreadClaim: string;
    /**
     * Lookup283: polkadot_primitives::v1::CoreOccupied
     **/
    PolkadotPrimitivesV1CoreOccupied: {
        _enum: {
            Parathread: string;
            Parachain: string;
        };
    };
    /**
     * Lookup286: polkadot_runtime_parachains::scheduler::CoreAssignment
     **/
    PolkadotRuntimeParachainsSchedulerCoreAssignment: {
        core: string;
        paraId: string;
        kind: string;
        groupIdx: string;
    };
    /**
     * Lookup287: polkadot_runtime_parachains::scheduler::AssignmentKind
     **/
    PolkadotRuntimeParachainsSchedulerAssignmentKind: {
        _enum: {
            Parachain: string;
            Parathread: string;
        };
    };
    /**
     * Lookup288: polkadot_runtime_parachains::paras::ParaLifecycle
     **/
    PolkadotRuntimeParachainsParasParaLifecycle: {
        _enum: string[];
    };
    /**
     * Lookup290: polkadot_runtime_parachains::paras::ParaPastCodeMeta<N>
     **/
    PolkadotRuntimeParachainsParasParaPastCodeMeta: {
        upgradeTimes: string;
        lastPruned: string;
    };
    /**
     * Lookup292: polkadot_runtime_parachains::paras::ReplacementTimes<N>
     **/
    PolkadotRuntimeParachainsParasReplacementTimes: {
        expectedAt: string;
        activatedAt: string;
    };
    /**
     * Lookup294: polkadot_primitives::v1::UpgradeGoAhead
     **/
    PolkadotPrimitivesV1UpgradeGoAhead: {
        _enum: string[];
    };
    /**
     * Lookup295: polkadot_primitives::v1::UpgradeRestriction
     **/
    PolkadotPrimitivesV1UpgradeRestriction: {
        _enum: string[];
    };
    /**
     * Lookup296: polkadot_runtime_parachains::paras::ParaGenesisArgs
     **/
    PolkadotRuntimeParachainsParasParaGenesisArgs: {
        genesisHead: string;
        validationCode: string;
        parachain: string;
    };
    /**
     * Lookup297: polkadot_runtime_parachains::paras::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsParasPalletCall: {
        _enum: {
            force_set_current_code: {
                para: string;
                newCode: string;
            };
            force_set_current_head: {
                para: string;
                newHead: string;
            };
            force_schedule_code_upgrade: {
                para: string;
                newCode: string;
                relayParentNumber: string;
            };
            force_note_new_head: {
                para: string;
                newHead: string;
            };
            force_queue_action: {
                para: string;
            };
        };
    };
    /**
     * Lookup298: polkadot_runtime_parachains::paras::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsParasPalletError: {
        _enum: string[];
    };
    /**
     * Lookup300: polkadot_runtime_parachains::initializer::BufferedSessionChange
     **/
    PolkadotRuntimeParachainsInitializerBufferedSessionChange: {
        validators: string;
        queued: string;
        sessionIndex: string;
    };
    /**
     * Lookup301: polkadot_runtime_parachains::initializer::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsInitializerPalletCall: {
        _enum: {
            force_approve: {
                upTo: string;
            };
        };
    };
    /**
     * Lookup303: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: string;
        msg: string;
    };
    /**
     * Lookup304: polkadot_runtime_parachains::dmp::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsDmpPalletCall: string;
    /**
     * Lookup306: polkadot_runtime_parachains::ump::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsUmpPalletCall: {
        _enum: {
            service_overweight: {
                index: string;
                weightLimit: string;
            };
        };
    };
    /**
     * Lookup307: polkadot_runtime_parachains::ump::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsUmpPalletError: {
        _enum: string[];
    };
    /**
     * Lookup308: polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest
     **/
    PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest: {
        confirmed: string;
        age: string;
        senderDeposit: string;
        maxMessageSize: string;
        maxCapacity: string;
        maxTotalSize: string;
    };
    /**
     * Lookup310: polkadot_runtime_parachains::hrmp::HrmpChannel
     **/
    PolkadotRuntimeParachainsHrmpHrmpChannel: {
        maxCapacity: string;
        maxTotalSize: string;
        maxMessageSize: string;
        msgCount: string;
        totalSize: string;
        mqcHead: string;
        senderDeposit: string;
        recipientDeposit: string;
    };
    /**
     * Lookup313: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: string;
        data: string;
    };
    /**
     * Lookup316: polkadot_runtime_parachains::hrmp::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsHrmpPalletCall: {
        _enum: {
            hrmp_init_open_channel: {
                recipient: string;
                proposedMaxCapacity: string;
                proposedMaxMessageSize: string;
            };
            hrmp_accept_open_channel: {
                sender: string;
            };
            hrmp_close_channel: {
                channelId: string;
            };
            force_clean_hrmp: {
                para: string;
            };
            force_process_hrmp_open: string;
            force_process_hrmp_close: string;
            hrmp_cancel_open_request: {
                channelId: string;
            };
        };
    };
    /**
     * Lookup317: polkadot_runtime_parachains::hrmp::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsHrmpPalletError: {
        _enum: string[];
    };
    /**
     * Lookup319: polkadot_primitives::v1::SessionInfo
     **/
    PolkadotPrimitivesV1SessionInfo: {
        validators: string;
        discoveryKeys: string;
        assignmentKeys: string;
        validatorGroups: string;
        nCores: string;
        zerothDelayTrancheWidth: string;
        relayVrfModuloSamples: string;
        nDelayTranches: string;
        noShowSlots: string;
        neededApprovals: string;
    };
    /**
     * Lookup322: polkadot_primitives::v1::DisputeState<N>
     **/
    PolkadotPrimitivesV1DisputeState: {
        validatorsFor: string;
        validatorsAgainst: string;
        start: string;
        concludedAt: string;
    };
    /**
     * Lookup323: polkadot_runtime_parachains::disputes::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsDisputesPalletCall: {
        _enum: string[];
    };
    /**
     * Lookup324: polkadot_runtime_parachains::disputes::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsDisputesPalletError: {
        _enum: string[];
    };
    /**
     * Lookup325: polkadot_runtime_common::paras_registrar::ParaInfo<sp_core::crypto::AccountId32, Balance>
     **/
    PolkadotRuntimeCommonParasRegistrarParaInfo: {
        manager: string;
        deposit: string;
        locked: string;
    };
    /**
     * Lookup326: polkadot_runtime_common::paras_registrar::pallet::Call<T>
     **/
    PolkadotRuntimeCommonParasRegistrarPalletCall: {
        _enum: {
            register: {
                id: string;
                genesisHead: string;
                validationCode: string;
            };
            force_register: {
                who: string;
                deposit: string;
                id: string;
                genesisHead: string;
                validationCode: string;
            };
            deregister: {
                id: string;
            };
            swap: {
                id: string;
                other: string;
            };
            force_remove_lock: {
                para: string;
            };
            reserve: string;
        };
    };
    /**
     * Lookup327: polkadot_runtime_common::paras_registrar::pallet::Error<T>
     **/
    PolkadotRuntimeCommonParasRegistrarPalletError: {
        _enum: string[];
    };
    /**
     * Lookup332: polkadot_runtime_common::auctions::pallet::Call<T>
     **/
    PolkadotRuntimeCommonAuctionsPalletCall: {
        _enum: {
            new_auction: {
                duration: string;
                leasePeriodIndex: string;
            };
            bid: {
                para: string;
                auctionIndex: string;
                firstSlot: string;
                lastSlot: string;
                amount: string;
            };
            cancel_auction: string;
        };
    };
    /**
     * Lookup334: polkadot_runtime_common::auctions::pallet::Error<T>
     **/
    PolkadotRuntimeCommonAuctionsPalletError: {
        _enum: string[];
    };
    /**
     * Lookup335: polkadot_runtime_common::crowdloan::FundInfo<sp_core::crypto::AccountId32, Balance, BlockNumber, LeasePeriod>
     **/
    PolkadotRuntimeCommonCrowdloanFundInfo: {
        depositor: string;
        verifier: string;
        deposit: string;
        raised: string;
        end: string;
        cap: string;
        lastContribution: string;
        firstPeriod: string;
        lastPeriod: string;
        trieIndex: string;
    };
    /**
     * Lookup337: sp_runtime::MultiSigner
     **/
    SpRuntimeMultiSigner: {
        _enum: {
            Ed25519: string;
            Sr25519: string;
            Ecdsa: string;
        };
    };
    /**
     * Lookup338: polkadot_runtime_common::crowdloan::LastContribution<BlockNumber>
     **/
    PolkadotRuntimeCommonCrowdloanLastContribution: {
        _enum: {
            Never: string;
            PreEnding: string;
            Ending: string;
        };
    };
    /**
     * Lookup339: polkadot_runtime_common::crowdloan::pallet::Call<T>
     **/
    PolkadotRuntimeCommonCrowdloanPalletCall: {
        _enum: {
            create: {
                index: string;
                cap: string;
                firstPeriod: string;
                lastPeriod: string;
                end: string;
                verifier: string;
            };
            contribute: {
                index: string;
                value: string;
                signature: string;
            };
            withdraw: {
                who: string;
                index: string;
            };
            refund: {
                index: string;
            };
            dissolve: {
                index: string;
            };
            edit: {
                index: string;
                cap: string;
                firstPeriod: string;
                lastPeriod: string;
                end: string;
                verifier: string;
            };
            add_memo: {
                index: string;
                memo: string;
            };
            poke: {
                index: string;
            };
        };
    };
    /**
     * Lookup341: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: string;
            Sr25519: string;
            Ecdsa: string;
        };
    };
    /**
     * Lookup342: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: string;
    /**
     * Lookup344: frame_support::PalletId
     **/
    FrameSupportPalletId: string;
    /**
     * Lookup345: polkadot_runtime_common::crowdloan::pallet::Error<T>
     **/
    PolkadotRuntimeCommonCrowdloanPalletError: {
        _enum: string[];
    };
    /**
     * Lookup349: polkadot_runtime_common::slots::pallet::Call<T>
     **/
    PolkadotRuntimeCommonSlotsPalletCall: {
        _enum: {
            force_lease: {
                para: string;
                leaser: string;
                amount: string;
                periodBegin: string;
                periodCount: string;
            };
            clear_all_leases: {
                para: string;
            };
            trigger_onboard: {
                para: string;
            };
        };
    };
    /**
     * Lookup350: polkadot_runtime_common::slots::pallet::Error<T>
     **/
    PolkadotRuntimeCommonSlotsPalletError: {
        _enum: string[];
    };
    /**
     * Lookup351: polkadot_runtime_common::paras_sudo_wrapper::pallet::Call<T>
     **/
    PolkadotRuntimeCommonParasSudoWrapperPalletCall: {
        _enum: {
            sudo_schedule_para_initialize: {
                id: string;
                genesis: string;
            };
            sudo_schedule_para_cleanup: {
                id: string;
            };
            sudo_schedule_parathread_upgrade: {
                id: string;
            };
            sudo_schedule_parachain_downgrade: {
                id: string;
            };
            sudo_queue_downward_xcm: {
                id: string;
                xcm: string;
            };
            sudo_establish_hrmp_channel: {
                sender: string;
                recipient: string;
                maxCapacity: string;
                maxMessageSize: string;
            };
        };
    };
    /**
     * Lookup352: xcm::VersionedXcm<Call>
     **/
    XcmVersionedXcm: {
        _enum: {
            V0: string;
            V1: string;
            V2: string;
        };
    };
    /**
     * Lookup353: xcm::v0::Xcm<Call>
     **/
    XcmV0Xcm: {
        _enum: {
            WithdrawAsset: {
                assets: string;
                effects: string;
            };
            ReserveAssetDeposit: {
                assets: string;
                effects: string;
            };
            TeleportAsset: {
                assets: string;
                effects: string;
            };
            QueryResponse: {
                queryId: string;
                response: string;
            };
            TransferAsset: {
                assets: string;
                dest: string;
            };
            TransferReserveAsset: {
                assets: string;
                dest: string;
                effects: string;
            };
            Transact: {
                originType: string;
                requireWeightAtMost: string;
                call: string;
            };
            HrmpNewChannelOpenRequest: {
                sender: string;
                maxMessageSize: string;
                maxCapacity: string;
            };
            HrmpChannelAccepted: {
                recipient: string;
            };
            HrmpChannelClosing: {
                initiator: string;
                sender: string;
                recipient: string;
            };
            RelayedFrom: {
                who: string;
                message: string;
            };
        };
    };
    /**
     * Lookup355: xcm::v0::order::Order<Call>
     **/
    XcmV0Order: {
        _enum: {
            Null: string;
            DepositAsset: {
                assets: string;
                dest: string;
            };
            DepositReserveAsset: {
                assets: string;
                dest: string;
                effects: string;
            };
            ExchangeAsset: {
                give: string;
                receive: string;
            };
            InitiateReserveWithdraw: {
                assets: string;
                reserve: string;
                effects: string;
            };
            InitiateTeleport: {
                assets: string;
                dest: string;
                effects: string;
            };
            QueryHolding: {
                queryId: string;
                dest: string;
                assets: string;
            };
            BuyExecution: {
                fees: string;
                weight: string;
                debt: string;
                haltOnError: string;
                xcm: string;
            };
        };
    };
    /**
     * Lookup357: xcm::v0::Response
     **/
    XcmV0Response: {
        _enum: {
            Assets: string;
        };
    };
    /**
     * Lookup358: xcm::v1::Xcm<Call>
     **/
    XcmV1Xcm: {
        _enum: {
            WithdrawAsset: {
                assets: string;
                effects: string;
            };
            ReserveAssetDeposited: {
                assets: string;
                effects: string;
            };
            ReceiveTeleportedAsset: {
                assets: string;
                effects: string;
            };
            QueryResponse: {
                queryId: string;
                response: string;
            };
            TransferAsset: {
                assets: string;
                beneficiary: string;
            };
            TransferReserveAsset: {
                assets: string;
                dest: string;
                effects: string;
            };
            Transact: {
                originType: string;
                requireWeightAtMost: string;
                call: string;
            };
            HrmpNewChannelOpenRequest: {
                sender: string;
                maxMessageSize: string;
                maxCapacity: string;
            };
            HrmpChannelAccepted: {
                recipient: string;
            };
            HrmpChannelClosing: {
                initiator: string;
                sender: string;
                recipient: string;
            };
            RelayedFrom: {
                who: string;
                message: string;
            };
            SubscribeVersion: {
                queryId: string;
                maxResponseWeight: string;
            };
            UnsubscribeVersion: string;
        };
    };
    /**
     * Lookup360: xcm::v1::order::Order<Call>
     **/
    XcmV1Order: {
        _enum: {
            Noop: string;
            DepositAsset: {
                assets: string;
                maxAssets: string;
                beneficiary: string;
            };
            DepositReserveAsset: {
                assets: string;
                maxAssets: string;
                dest: string;
                effects: string;
            };
            ExchangeAsset: {
                give: string;
                receive: string;
            };
            InitiateReserveWithdraw: {
                assets: string;
                reserve: string;
                effects: string;
            };
            InitiateTeleport: {
                assets: string;
                dest: string;
                effects: string;
            };
            QueryHolding: {
                queryId: string;
                dest: string;
                assets: string;
            };
            BuyExecution: {
                fees: string;
                weight: string;
                debt: string;
                haltOnError: string;
                instructions: string;
            };
        };
    };
    /**
     * Lookup362: xcm::v1::Response
     **/
    XcmV1Response: {
        _enum: {
            Assets: string;
            Version: string;
        };
    };
    /**
     * Lookup363: polkadot_runtime_common::paras_sudo_wrapper::pallet::Error<T>
     **/
    PolkadotRuntimeCommonParasSudoWrapperPalletError: {
        _enum: string[];
    };
    /**
     * Lookup364: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: string;
            };
            sudo_unchecked_weight: {
                call: string;
                weight: string;
            };
            set_key: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            sudo_as: {
                who: string;
                call: string;
            };
        };
    };
    /**
     * Lookup366: rococo_runtime::validator_manager::Call<T>
     **/
    RococoRuntimeValidatorManagerCall: {
        _enum: {
            register_validators: {
                validators: string;
            };
            deregister_validators: {
                validators: string;
            };
        };
    };
    /**
     * Lookup367: pallet_collective::pallet::Call<T, I>
     **/
    PalletCollectiveCall: {
        _enum: {
            set_members: {
                newMembers: string;
                prime: string;
                oldCount: string;
            };
            execute: {
                proposal: string;
                lengthBound: string;
            };
            propose: {
                threshold: string;
                proposal: string;
                lengthBound: string;
            };
            vote: {
                proposal: string;
                index: string;
                approve: string;
            };
            close: {
                proposalHash: string;
                index: string;
                proposalWeightBound: string;
                lengthBound: string;
            };
            disapprove_proposal: {
                proposalHash: string;
            };
        };
    };
    /**
     * Lookup368: pallet_membership::pallet::Call<T, I>
     **/
    PalletMembershipCall: {
        _enum: {
            add_member: {
                who: string;
            };
            remove_member: {
                who: string;
            };
            swap_member: {
                remove: string;
                add: string;
            };
            reset_members: {
                members: string;
            };
            change_key: {
                _alias: {
                    new_: string;
                };
                new_: string;
            };
            set_prime: {
                who: string;
            };
            clear_prime: string;
        };
    };
    /**
     * Lookup369: pallet_utility::pallet::Call<T>
     **/
    PalletUtilityCall: {
        _enum: {
            batch: {
                calls: string;
            };
            as_derivative: {
                index: string;
                call: string;
            };
            batch_all: {
                calls: string;
            };
            dispatch_as: {
                asOrigin: string;
                call: string;
            };
        };
    };
    /**
     * Lookup371: rococo_runtime::OriginCaller
     **/
    RococoRuntimeOriginCaller: {
        _enum: {
            system: string;
            __Unused1: string;
            __Unused2: string;
            __Unused3: string;
            Void: string;
            __Unused5: string;
            __Unused6: string;
            __Unused7: string;
            __Unused8: string;
            __Unused9: string;
            __Unused10: string;
            __Unused11: string;
            __Unused12: string;
            ParachainsOrigin: string;
            __Unused14: string;
            __Unused15: string;
            __Unused16: string;
            __Unused17: string;
            __Unused18: string;
            __Unused19: string;
            __Unused20: string;
            __Unused21: string;
            __Unused22: string;
            __Unused23: string;
            __Unused24: string;
            __Unused25: string;
            __Unused26: string;
            __Unused27: string;
            __Unused28: string;
            __Unused29: string;
            __Unused30: string;
            __Unused31: string;
            __Unused32: string;
            __Unused33: string;
            __Unused34: string;
            __Unused35: string;
            __Unused36: string;
            __Unused37: string;
            __Unused38: string;
            __Unused39: string;
            __Unused40: string;
            __Unused41: string;
            __Unused42: string;
            __Unused43: string;
            __Unused44: string;
            __Unused45: string;
            __Unused46: string;
            __Unused47: string;
            __Unused48: string;
            __Unused49: string;
            __Unused50: string;
            __Unused51: string;
            __Unused52: string;
            __Unused53: string;
            __Unused54: string;
            __Unused55: string;
            __Unused56: string;
            __Unused57: string;
            __Unused58: string;
            __Unused59: string;
            __Unused60: string;
            __Unused61: string;
            __Unused62: string;
            __Unused63: string;
            __Unused64: string;
            __Unused65: string;
            __Unused66: string;
            __Unused67: string;
            __Unused68: string;
            __Unused69: string;
            __Unused70: string;
            __Unused71: string;
            __Unused72: string;
            __Unused73: string;
            __Unused74: string;
            __Unused75: string;
            __Unused76: string;
            __Unused77: string;
            __Unused78: string;
            __Unused79: string;
            Collective: string;
            __Unused81: string;
            __Unused82: string;
            __Unused83: string;
            __Unused84: string;
            __Unused85: string;
            __Unused86: string;
            __Unused87: string;
            __Unused88: string;
            __Unused89: string;
            __Unused90: string;
            __Unused91: string;
            __Unused92: string;
            __Unused93: string;
            __Unused94: string;
            __Unused95: string;
            __Unused96: string;
            __Unused97: string;
            __Unused98: string;
            XcmPallet: string;
        };
    };
    /**
     * Lookup372: frame_system::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSystemRawOrigin: {
        _enum: {
            Root: string;
            Signed: string;
            None: string;
        };
    };
    /**
     * Lookup373: polkadot_runtime_parachains::origin::pallet::Origin
     **/
    PolkadotRuntimeParachainsOriginPalletOrigin: {
        _enum: {
            Parachain: string;
        };
    };
    /**
     * Lookup374: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
     **/
    PalletCollectiveRawOrigin: {
        _enum: {
            Members: string;
            Member: string;
            _Phantom: string;
        };
    };
    /**
     * Lookup375: pallet_xcm::pallet::Origin
     **/
    PalletXcmOrigin: {
        _enum: {
            Xcm: string;
            Response: string;
        };
    };
    /**
     * Lookup376: sp_core::Void
     **/
    SpCoreVoid: string;
    /**
     * Lookup377: pallet_proxy::pallet::Call<T>
     **/
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: string;
                forceProxyType: string;
                call: string;
            };
            add_proxy: {
                delegate: string;
                proxyType: string;
                delay: string;
            };
            remove_proxy: {
                delegate: string;
                proxyType: string;
                delay: string;
            };
            remove_proxies: string;
            anonymous: {
                proxyType: string;
                delay: string;
                index: string;
            };
            kill_anonymous: {
                spawner: string;
                proxyType: string;
                index: string;
                height: string;
                extIndex: string;
            };
            announce: {
                real: string;
                callHash: string;
            };
            remove_announcement: {
                real: string;
                callHash: string;
            };
            reject_announcement: {
                delegate: string;
                callHash: string;
            };
            proxy_announced: {
                delegate: string;
                real: string;
                forceProxyType: string;
                call: string;
            };
        };
    };
    /**
     * Lookup379: pallet_multisig::pallet::Call<T>
     **/
    PalletMultisigCall: {
        _enum: {
            as_multi_threshold_1: {
                otherSignatories: string;
                call: string;
            };
            as_multi: {
                threshold: string;
                otherSignatories: string;
                maybeTimepoint: string;
                call: string;
                storeCall: string;
                maxWeight: string;
            };
            approve_as_multi: {
                threshold: string;
                otherSignatories: string;
                maybeTimepoint: string;
                callHash: string;
                maxWeight: string;
            };
            cancel_as_multi: {
                threshold: string;
                otherSignatories: string;
                timepoint: string;
                callHash: string;
            };
        };
    };
    /**
     * Lookup382: pallet_xcm::pallet::Call<T>
     **/
    PalletXcmCall: {
        _enum: {
            send: {
                dest: string;
                message: string;
            };
            teleport_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
            };
            reserve_transfer_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
            };
            execute: {
                message: string;
                maxWeight: string;
            };
            force_xcm_version: {
                location: string;
                xcmVersion: string;
            };
            force_default_xcm_version: {
                maybeXcmVersion: string;
            };
            force_subscribe_version_notify: {
                location: string;
            };
            force_unsubscribe_version_notify: {
                location: string;
            };
            limited_reserve_transfer_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
                weightLimit: string;
            };
            limited_teleport_assets: {
                dest: string;
                beneficiary: string;
                assets: string;
                feeAssetItem: string;
                weightLimit: string;
            };
        };
    };
    /**
     * Lookup396: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: string[];
    };
    /**
     * Lookup398: beefy_primitives::mmr::BeefyNextAuthoritySet<primitive_types::H256>
     **/
    BeefyPrimitivesMmrBeefyNextAuthoritySet: {
        id: string;
        len: string;
        root: string;
    };
    /**
     * Lookup399: rococo_runtime::validator_manager::Error<T>
     **/
    RococoRuntimeValidatorManagerError: string;
    /**
     * Lookup401: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletCollectiveVotes: {
        index: string;
        threshold: string;
        ayes: string;
        nays: string;
        end: string;
    };
    /**
     * Lookup402: pallet_collective::pallet::Error<T, I>
     **/
    PalletCollectiveError: {
        _enum: string[];
    };
    /**
     * Lookup403: pallet_membership::pallet::Error<T, I>
     **/
    PalletMembershipError: {
        _enum: string[];
    };
    /**
     * Lookup404: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: string[];
    };
    /**
     * Lookup407: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, rococo_runtime::ProxyType, BlockNumber>
     **/
    PalletProxyProxyDefinition: {
        delegate: string;
        proxyType: string;
        delay: string;
    };
    /**
     * Lookup411: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
     **/
    PalletProxyAnnouncement: {
        real: string;
        callHash: string;
        height: string;
    };
    /**
     * Lookup413: pallet_proxy::pallet::Error<T>
     **/
    PalletProxyError: {
        _enum: string[];
    };
    /**
     * Lookup415: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32>
     **/
    PalletMultisigMultisig: {
        when: string;
        deposit: string;
        depositor: string;
        approvals: string;
    };
    /**
     * Lookup417: pallet_multisig::pallet::Error<T>
     **/
    PalletMultisigError: {
        _enum: string[];
    };
    /**
     * Lookup418: pallet_xcm::pallet::QueryStatus<BlockNumber>
     **/
    PalletXcmQueryStatus: {
        _enum: {
            Pending: {
                responder: string;
                maybeNotify: string;
                timeout: string;
            };
            VersionNotifier: {
                origin: string;
                isActive: string;
            };
            Ready: {
                response: string;
                at: string;
            };
        };
    };
    /**
     * Lookup421: xcm::VersionedResponse
     **/
    XcmVersionedResponse: {
        _enum: {
            V0: string;
            V1: string;
            V2: string;
        };
    };
    /**
     * Lookup427: pallet_xcm::pallet::VersionMigrationStage
     **/
    PalletXcmVersionMigrationStage: {
        _enum: {
            MigrateSupportedVersion: string;
            MigrateVersionNotifiers: string;
            NotifyCurrentTargets: string;
            MigrateAndNotifyOldTargets: string;
        };
    };
    /**
     * Lookup429: pallet_xcm::pallet::Error<T>
     **/
    PalletXcmError: {
        _enum: string[];
    };
    /**
     * Lookup432: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: string;
    /**
     * Lookup433: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: string;
    /**
     * Lookup434: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: string;
    /**
     * Lookup437: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: string;
    /**
     * Lookup438: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: string;
    /**
     * Lookup439: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: string;
    /**
     * Lookup440: rococo_runtime::Runtime
     **/
    RococoRuntimeRuntime: string;
};
export default _default;
