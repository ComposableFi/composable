// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
   **/
  FrameSystemAccountInfo: {
    nonce: 'u32',
    consumers: 'u32',
    providers: 'u32',
    sufficients: 'u32',
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    miscFrozen: 'u128',
    feeFrozen: 'u128'
  },
  /**
   * Lookup7: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU64: {
    normal: 'u64',
    operational: 'u64',
    mandatory: 'u64'
  },
  /**
   * Lookup11: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup13: sp_runtime::generic::digest::DigestItem
   **/
  SpRuntimeDigestDigestItem: {
    _enum: {
      Other: 'Bytes',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      Consensus: '([u8;4],Bytes)',
      Seal: '([u8;4],Bytes)',
      PreRuntime: '([u8;4],Bytes)',
      __Unused7: 'Null',
      RuntimeEnvironmentUpdated: 'Null'
    }
  },
  /**
   * Lookup16: frame_system::EventRecord<dali_runtime::Event, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup18: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
      },
      CodeUpdated: 'Null',
      NewAccount: {
        account: 'AccountId32',
      },
      KilledAccount: {
        account: 'AccountId32',
      },
      Remarked: {
        _alias: {
          hash_: 'hash',
        },
        sender: 'AccountId32',
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup19: frame_support::weights::DispatchInfo
   **/
  FrameSupportWeightsDispatchInfo: {
    weight: 'u64',
    class: 'FrameSupportWeightsDispatchClass',
    paysFee: 'FrameSupportWeightsPays'
  },
  /**
   * Lookup20: frame_support::weights::DispatchClass
   **/
  FrameSupportWeightsDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup21: frame_support::weights::Pays
   **/
  FrameSupportWeightsPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup22: sp_runtime::DispatchError
   **/
  SpRuntimeDispatchError: {
    _enum: {
      Other: 'Null',
      CannotLookup: 'Null',
      BadOrigin: 'Null',
      Module: 'SpRuntimeModuleError',
      ConsumerRemaining: 'Null',
      NoProviders: 'Null',
      TooManyConsumers: 'Null',
      Token: 'SpRuntimeTokenError',
      Arithmetic: 'SpRuntimeArithmeticError',
      Transactional: 'SpRuntimeTransactionalError'
    }
  },
  /**
   * Lookup23: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup24: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported']
  },
  /**
   * Lookup25: sp_runtime::ArithmeticError
   **/
  SpRuntimeArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup26: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup27: pallet_sudo::pallet::Event<T>
   **/
  PalletSudoEvent: {
    _enum: {
      Sudid: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>',
      },
      KeyChanged: {
        oldSudoer: 'Option<AccountId32>',
      },
      SudoAsDone: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup31: pallet_transaction_payment::pallet::Event<T>
   **/
  PalletTransactionPaymentEvent: {
    _enum: {
      TransactionFeePaid: {
        who: 'AccountId32',
        actualFee: 'u128',
        tip: 'u128'
      }
    }
  },
  /**
   * Lookup32: pallet_indices::pallet::Event<T>
   **/
  PalletIndicesEvent: {
    _enum: {
      IndexAssigned: {
        who: 'AccountId32',
        index: 'u32',
      },
      IndexFreed: {
        index: 'u32',
      },
      IndexFrozen: {
        index: 'u32',
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup33: pallet_balances::pallet::Event<T, I>
   **/
  PalletBalancesEvent: {
    _enum: {
      Endowed: {
        account: 'AccountId32',
        freeBalance: 'u128',
      },
      DustLost: {
        account: 'AccountId32',
        amount: 'u128',
      },
      Transfer: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      BalanceSet: {
        who: 'AccountId32',
        free: 'u128',
        reserved: 'u128',
      },
      Reserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unreserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      ReserveRepatriated: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
      },
      Deposit: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Withdraw: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Slashed: {
        who: 'AccountId32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup34: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup35: pallet_identity::pallet::Event<T>
   **/
  PalletIdentityEvent: {
    _enum: {
      IdentitySet: {
        who: 'AccountId32',
      },
      IdentityCleared: {
        who: 'AccountId32',
        deposit: 'u128',
      },
      IdentityKilled: {
        who: 'AccountId32',
        deposit: 'u128',
      },
      JudgementRequested: {
        who: 'AccountId32',
        registrarIndex: 'u32',
      },
      JudgementUnrequested: {
        who: 'AccountId32',
        registrarIndex: 'u32',
      },
      JudgementGiven: {
        target: 'AccountId32',
        registrarIndex: 'u32',
      },
      RegistrarAdded: {
        registrarIndex: 'u32',
      },
      SubIdentityAdded: {
        sub: 'AccountId32',
        main: 'AccountId32',
        deposit: 'u128',
      },
      SubIdentityRemoved: {
        sub: 'AccountId32',
        main: 'AccountId32',
        deposit: 'u128',
      },
      SubIdentityRevoked: {
        sub: 'AccountId32',
        main: 'AccountId32',
        deposit: 'u128'
      }
    }
  },
  /**
   * Lookup36: pallet_multisig::pallet::Event<T>
   **/
  PalletMultisigEvent: {
    _enum: {
      NewMultisig: {
        approving: 'AccountId32',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
      },
      MultisigApproval: {
        approving: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
      },
      MultisigExecuted: {
        approving: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      MultisigCancelled: {
        cancelling: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]'
      }
    }
  },
  /**
   * Lookup37: pallet_multisig::Timepoint<BlockNumber>
   **/
  PalletMultisigTimepoint: {
    height: 'u32',
    index: 'u32'
  },
  /**
   * Lookup38: cumulus_pallet_parachain_system::pallet::Event<T>
   **/
  CumulusPalletParachainSystemEvent: {
    _enum: {
      ValidationFunctionStored: 'Null',
      ValidationFunctionApplied: {
        relayChainBlockNum: 'u32',
      },
      ValidationFunctionDiscarded: 'Null',
      UpgradeAuthorized: {
        codeHash: 'H256',
      },
      DownwardMessagesReceived: {
        count: 'u32',
      },
      DownwardMessagesProcessed: {
        weightUsed: 'u64',
        dmqHead: 'H256'
      }
    }
  },
  /**
   * Lookup39: pallet_collator_selection::pallet::Event<T>
   **/
  PalletCollatorSelectionEvent: {
    _enum: {
      NewInvulnerables: {
        invulnerables: 'Vec<AccountId32>',
      },
      NewDesiredCandidates: {
        desiredCandidates: 'u32',
      },
      NewCandidacyBond: {
        bondAmount: 'u128',
      },
      CandidateAdded: {
        accountId: 'AccountId32',
        deposit: 'u128',
      },
      CandidateRemoved: {
        accountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup41: pallet_session::pallet::Event
   **/
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: 'u32'
      }
    }
  },
  /**
   * Lookup42: pallet_collective::pallet::Event<T, I>
   **/
  PalletCollectiveEvent: {
    _enum: {
      Proposed: {
        account: 'AccountId32',
        proposalIndex: 'u32',
        proposalHash: 'H256',
        threshold: 'u32',
      },
      Voted: {
        account: 'AccountId32',
        proposalHash: 'H256',
        voted: 'bool',
        yes: 'u32',
        no: 'u32',
      },
      Approved: {
        proposalHash: 'H256',
      },
      Disapproved: {
        proposalHash: 'H256',
      },
      Executed: {
        proposalHash: 'H256',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      MemberExecuted: {
        proposalHash: 'H256',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      Closed: {
        proposalHash: 'H256',
        yes: 'u32',
        no: 'u32'
      }
    }
  },
  /**
   * Lookup44: pallet_membership::pallet::Event<T, I>
   **/
  PalletMembershipEvent: {
    _enum: ['MemberAdded', 'MemberRemoved', 'MembersSwapped', 'MembersReset', 'KeyChanged', 'Dummy']
  },
  /**
   * Lookup45: pallet_treasury::pallet::Event<T, I>
   **/
  PalletTreasuryEvent: {
    _enum: {
      Proposed: {
        proposalIndex: 'u32',
      },
      Spending: {
        budgetRemaining: 'u128',
      },
      Awarded: {
        proposalIndex: 'u32',
        award: 'u128',
        account: 'AccountId32',
      },
      Rejected: {
        proposalIndex: 'u32',
        slashed: 'u128',
      },
      Burnt: {
        burntFunds: 'u128',
      },
      Rollover: {
        rolloverBalance: 'u128',
      },
      Deposit: {
        value: 'u128',
      },
      SpendApproved: {
        proposalIndex: 'u32',
        amount: 'u128',
        beneficiary: 'AccountId32'
      }
    }
  },
  /**
   * Lookup46: pallet_democracy::pallet::Event<T, I>
   **/
  PalletDemocracyEvent: {
    _enum: {
      Proposed: {
        proposalIndex: 'u32',
        deposit: 'u128',
      },
      Tabled: {
        proposalIndex: 'u32',
        deposit: 'u128',
        depositors: 'Vec<AccountId32>',
      },
      ExternalTabled: 'Null',
      Started: {
        refIndex: 'u32',
        threshold: 'PalletDemocracyVoteThreshold',
      },
      Passed: {
        refIndex: 'u32',
      },
      NotPassed: {
        refIndex: 'u32',
      },
      Cancelled: {
        refIndex: 'u32',
      },
      Executed: {
        refIndex: 'u32',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      Delegated: {
        who: 'AccountId32',
        target: 'AccountId32',
      },
      Undelegated: {
        account: 'AccountId32',
      },
      Vetoed: {
        who: 'AccountId32',
        proposalHash: 'H256',
        until: 'u32',
      },
      PreimageNoted: {
        proposalHash: 'H256',
        who: 'AccountId32',
        deposit: 'u128',
      },
      PreimageUsed: {
        proposalHash: 'H256',
        provider: 'AccountId32',
        deposit: 'u128',
      },
      PreimageInvalid: {
        proposalHash: 'H256',
        refIndex: 'u32',
      },
      PreimageMissing: {
        proposalHash: 'H256',
        refIndex: 'u32',
      },
      PreimageReaped: {
        proposalHash: 'H256',
        provider: 'AccountId32',
        deposit: 'u128',
        reaper: 'AccountId32',
      },
      Blacklisted: {
        proposalHash: 'H256',
      },
      Voted: {
        voter: 'AccountId32',
        refIndex: 'u32',
        vote: 'PalletDemocracyVoteAccountVote',
      },
      Seconded: {
        seconder: 'AccountId32',
        propIndex: 'u32',
      },
      ProposalCanceled: {
        propIndex: 'u32'
      }
    }
  },
  /**
   * Lookup47: pallet_democracy::vote_threshold::VoteThreshold
   **/
  PalletDemocracyVoteThreshold: {
    _enum: ['SuperMajorityApprove', 'SuperMajorityAgainst', 'SimpleMajority']
  },
  /**
   * Lookup48: pallet_democracy::vote::AccountVote<Balance>
   **/
  PalletDemocracyVoteAccountVote: {
    _enum: {
      Standard: {
        vote: 'Vote',
        balance: 'u128',
      },
      Split: {
        aye: 'u128',
        nay: 'u128'
      }
    }
  },
  /**
   * Lookup52: pallet_scheduler::pallet::Event<T>
   **/
  PalletSchedulerEvent: {
    _enum: {
      Scheduled: {
        when: 'u32',
        index: 'u32',
      },
      Canceled: {
        when: 'u32',
        index: 'u32',
      },
      Dispatched: {
        task: '(u32,u32)',
        id: 'Option<Bytes>',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      CallLookupFailed: {
        task: '(u32,u32)',
        id: 'Option<Bytes>',
        error: 'FrameSupportScheduleLookupError'
      }
    }
  },
  /**
   * Lookup55: frame_support::traits::schedule::LookupError
   **/
  FrameSupportScheduleLookupError: {
    _enum: ['Unknown', 'BadFormat']
  },
  /**
   * Lookup56: pallet_utility::pallet::Event
   **/
  PalletUtilityEvent: {
    _enum: {
      BatchInterrupted: {
        index: 'u32',
        error: 'SpRuntimeDispatchError',
      },
      BatchCompleted: 'Null',
      BatchCompletedWithErrors: 'Null',
      ItemCompleted: 'Null',
      ItemFailed: {
        error: 'SpRuntimeDispatchError',
      },
      DispatchedAs: {
        result: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup57: pallet_preimage::pallet::Event<T>
   **/
  PalletPreimageEvent: {
    _enum: {
      Noted: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Requested: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Cleared: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup58: pallet_account_proxy::pallet::Event<T>
   **/
  PalletAccountProxyEvent: {
    _enum: {
      ProxyExecuted: {
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      AnonymousCreated: {
        anonymous: 'AccountId32',
        who: 'AccountId32',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        disambiguationIndex: 'u16',
      },
      Announced: {
        real: 'AccountId32',
        proxy: 'AccountId32',
        callHash: 'H256',
      },
      ProxyAdded: {
        delegator: 'AccountId32',
        delegatee: 'AccountId32',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32',
      },
      ProxyRemoved: {
        delegator: 'AccountId32',
        delegatee: 'AccountId32',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup59: composable_traits::account_proxy::ProxyType
   **/
  ComposableTraitsAccountProxyProxyType: {
    _enum: ['Any', 'Governance', 'CancelProxy']
  },
  /**
   * Lookup61: cumulus_pallet_xcmp_queue::pallet::Event<T>
   **/
  CumulusPalletXcmpQueueEvent: {
    _enum: {
      Success: {
        messageHash: 'Option<H256>',
        weight: 'u64',
      },
      Fail: {
        messageHash: 'Option<H256>',
        error: 'XcmV2TraitsError',
        weight: 'u64',
      },
      BadVersion: {
        messageHash: 'Option<H256>',
      },
      BadFormat: {
        messageHash: 'Option<H256>',
      },
      UpwardMessageSent: {
        messageHash: 'Option<H256>',
      },
      XcmpMessageSent: {
        messageHash: 'Option<H256>',
      },
      OverweightEnqueued: {
        sender: 'u32',
        sentAt: 'u32',
        index: 'u64',
        required: 'u64',
      },
      OverweightServiced: {
        index: 'u64',
        used: 'u64'
      }
    }
  },
  /**
   * Lookup63: xcm::v2::traits::Error
   **/
  XcmV2TraitsError: {
    _enum: {
      Overflow: 'Null',
      Unimplemented: 'Null',
      UntrustedReserveLocation: 'Null',
      UntrustedTeleportLocation: 'Null',
      MultiLocationFull: 'Null',
      MultiLocationNotInvertible: 'Null',
      BadOrigin: 'Null',
      InvalidLocation: 'Null',
      AssetNotFound: 'Null',
      FailedToTransactAsset: 'Null',
      NotWithdrawable: 'Null',
      LocationCannotHold: 'Null',
      ExceedsMaxMessageSize: 'Null',
      DestinationUnsupported: 'Null',
      Transport: 'Null',
      Unroutable: 'Null',
      UnknownClaim: 'Null',
      FailedToDecode: 'Null',
      MaxWeightInvalid: 'Null',
      NotHoldingFees: 'Null',
      TooExpensive: 'Null',
      Trap: 'u64',
      UnhandledXcmVersion: 'Null',
      WeightLimitReached: 'u64',
      Barrier: 'Null',
      WeightNotComputable: 'Null'
    }
  },
  /**
   * Lookup65: pallet_xcm::pallet::Event<T>
   **/
  PalletXcmEvent: {
    _enum: {
      Attempted: 'XcmV2TraitsOutcome',
      Sent: '(XcmV1MultiLocation,XcmV1MultiLocation,XcmV2Xcm)',
      UnexpectedResponse: '(XcmV1MultiLocation,u64)',
      ResponseReady: '(u64,XcmV2Response)',
      Notified: '(u64,u8,u8)',
      NotifyOverweight: '(u64,u8,u8,u64,u64)',
      NotifyDispatchError: '(u64,u8,u8)',
      NotifyDecodeFailed: '(u64,u8,u8)',
      InvalidResponder: '(XcmV1MultiLocation,u64,Option<XcmV1MultiLocation>)',
      InvalidResponderVersion: '(XcmV1MultiLocation,u64)',
      ResponseTaken: 'u64',
      AssetsTrapped: '(H256,XcmV1MultiLocation,XcmVersionedMultiAssets)',
      VersionChangeNotified: '(XcmV1MultiLocation,u32)',
      SupportedVersionChanged: '(XcmV1MultiLocation,u32)',
      NotifyTargetSendFail: '(XcmV1MultiLocation,u64,XcmV2TraitsError)',
      NotifyTargetMigrationFail: '(XcmVersionedMultiLocation,u64)'
    }
  },
  /**
   * Lookup66: xcm::v2::traits::Outcome
   **/
  XcmV2TraitsOutcome: {
    _enum: {
      Complete: 'u64',
      Incomplete: '(u64,XcmV2TraitsError)',
      Error: 'XcmV2TraitsError'
    }
  },
  /**
   * Lookup67: xcm::v1::multilocation::MultiLocation
   **/
  XcmV1MultiLocation: {
    parents: 'u8',
    interior: 'XcmV1MultilocationJunctions'
  },
  /**
   * Lookup68: xcm::v1::multilocation::Junctions
   **/
  XcmV1MultilocationJunctions: {
    _enum: {
      Here: 'Null',
      X1: 'XcmV1Junction',
      X2: '(XcmV1Junction,XcmV1Junction)',
      X3: '(XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X4: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X5: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X6: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X7: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)',
      X8: '(XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction,XcmV1Junction)'
    }
  },
  /**
   * Lookup69: xcm::v1::junction::Junction
   **/
  XcmV1Junction: {
    _enum: {
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'XcmV0JunctionNetworkId',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'XcmV0JunctionNetworkId',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'XcmV0JunctionNetworkId',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: 'Bytes',
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV0JunctionBodyId',
        part: 'XcmV0JunctionBodyPart'
      }
    }
  },
  /**
   * Lookup71: xcm::v0::junction::NetworkId
   **/
  XcmV0JunctionNetworkId: {
    _enum: {
      Any: 'Null',
      Named: 'Bytes',
      Polkadot: 'Null',
      Kusama: 'Null'
    }
  },
  /**
   * Lookup76: xcm::v0::junction::BodyId
   **/
  XcmV0JunctionBodyId: {
    _enum: {
      Unit: 'Null',
      Named: 'Bytes',
      Index: 'Compact<u32>',
      Executive: 'Null',
      Technical: 'Null',
      Legislative: 'Null',
      Judicial: 'Null'
    }
  },
  /**
   * Lookup77: xcm::v0::junction::BodyPart
   **/
  XcmV0JunctionBodyPart: {
    _enum: {
      Voice: 'Null',
      Members: {
        count: 'Compact<u32>',
      },
      Fraction: {
        nom: 'Compact<u32>',
        denom: 'Compact<u32>',
      },
      AtLeastProportion: {
        nom: 'Compact<u32>',
        denom: 'Compact<u32>',
      },
      MoreThanProportion: {
        nom: 'Compact<u32>',
        denom: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup78: xcm::v2::Xcm<Call>
   **/
  XcmV2Xcm: 'Vec<XcmV2Instruction>',
  /**
   * Lookup80: xcm::v2::Instruction<Call>
   **/
  XcmV2Instruction: {
    _enum: {
      WithdrawAsset: 'XcmV1MultiassetMultiAssets',
      ReserveAssetDeposited: 'XcmV1MultiassetMultiAssets',
      ReceiveTeleportedAsset: 'XcmV1MultiassetMultiAssets',
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV2Response',
        maxWeight: 'Compact<u64>',
      },
      TransferAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        beneficiary: 'XcmV1MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        dest: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      Transact: {
        originType: 'XcmV0OriginKind',
        requireWeightAtMost: 'Compact<u64>',
        call: 'XcmDoubleEncoded',
      },
      HrmpNewChannelOpenRequest: {
        sender: 'Compact<u32>',
        maxMessageSize: 'Compact<u32>',
        maxCapacity: 'Compact<u32>',
      },
      HrmpChannelAccepted: {
        recipient: 'Compact<u32>',
      },
      HrmpChannelClosing: {
        initiator: 'Compact<u32>',
        sender: 'Compact<u32>',
        recipient: 'Compact<u32>',
      },
      ClearOrigin: 'Null',
      DescendOrigin: 'XcmV1MultilocationJunctions',
      ReportError: {
        queryId: 'Compact<u64>',
        dest: 'XcmV1MultiLocation',
        maxResponseWeight: 'Compact<u64>',
      },
      DepositAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        beneficiary: 'XcmV1MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        dest: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      ExchangeAsset: {
        give: 'XcmV1MultiassetMultiAssetFilter',
        receive: 'XcmV1MultiassetMultiAssets',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        reserve: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      InitiateTeleport: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        dest: 'XcmV1MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV1MultiLocation',
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxResponseWeight: 'Compact<u64>',
      },
      BuyExecution: {
        fees: 'XcmV1MultiAsset',
        weightLimit: 'XcmV2WeightLimit',
      },
      RefundSurplus: 'Null',
      SetErrorHandler: 'XcmV2Xcm',
      SetAppendix: 'XcmV2Xcm',
      ClearError: 'Null',
      ClaimAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        ticket: 'XcmV1MultiLocation',
      },
      Trap: 'Compact<u64>',
      SubscribeVersion: {
        queryId: 'Compact<u64>',
        maxResponseWeight: 'Compact<u64>',
      },
      UnsubscribeVersion: 'Null'
    }
  },
  /**
   * Lookup81: xcm::v1::multiasset::MultiAssets
   **/
  XcmV1MultiassetMultiAssets: 'Vec<XcmV1MultiAsset>',
  /**
   * Lookup83: xcm::v1::multiasset::MultiAsset
   **/
  XcmV1MultiAsset: {
    id: 'XcmV1MultiassetAssetId',
    fun: 'XcmV1MultiassetFungibility'
  },
  /**
   * Lookup84: xcm::v1::multiasset::AssetId
   **/
  XcmV1MultiassetAssetId: {
    _enum: {
      Concrete: 'XcmV1MultiLocation',
      Abstract: 'Bytes'
    }
  },
  /**
   * Lookup85: xcm::v1::multiasset::Fungibility
   **/
  XcmV1MultiassetFungibility: {
    _enum: {
      Fungible: 'Compact<u128>',
      NonFungible: 'XcmV1MultiassetAssetInstance'
    }
  },
  /**
   * Lookup86: xcm::v1::multiasset::AssetInstance
   **/
  XcmV1MultiassetAssetInstance: {
    _enum: {
      Undefined: 'Null',
      Index: 'Compact<u128>',
      Array4: '[u8;4]',
      Array8: '[u8;8]',
      Array16: '[u8;16]',
      Array32: '[u8;32]',
      Blob: 'Bytes'
    }
  },
  /**
   * Lookup89: xcm::v2::Response
   **/
  XcmV2Response: {
    _enum: {
      Null: 'Null',
      Assets: 'XcmV1MultiassetMultiAssets',
      ExecutionResult: 'Option<(u32,XcmV2TraitsError)>',
      Version: 'u32'
    }
  },
  /**
   * Lookup92: xcm::v0::OriginKind
   **/
  XcmV0OriginKind: {
    _enum: ['Native', 'SovereignAccount', 'Superuser', 'Xcm']
  },
  /**
   * Lookup93: xcm::double_encoded::DoubleEncoded<T>
   **/
  XcmDoubleEncoded: {
    encoded: 'Bytes'
  },
  /**
   * Lookup94: xcm::v1::multiasset::MultiAssetFilter
   **/
  XcmV1MultiassetMultiAssetFilter: {
    _enum: {
      Definite: 'XcmV1MultiassetMultiAssets',
      Wild: 'XcmV1MultiassetWildMultiAsset'
    }
  },
  /**
   * Lookup95: xcm::v1::multiasset::WildMultiAsset
   **/
  XcmV1MultiassetWildMultiAsset: {
    _enum: {
      All: 'Null',
      AllOf: {
        id: 'XcmV1MultiassetAssetId',
        fun: 'XcmV1MultiassetWildFungibility'
      }
    }
  },
  /**
   * Lookup96: xcm::v1::multiasset::WildFungibility
   **/
  XcmV1MultiassetWildFungibility: {
    _enum: ['Fungible', 'NonFungible']
  },
  /**
   * Lookup97: xcm::v2::WeightLimit
   **/
  XcmV2WeightLimit: {
    _enum: {
      Unlimited: 'Null',
      Limited: 'Compact<u64>'
    }
  },
  /**
   * Lookup99: xcm::VersionedMultiAssets
   **/
  XcmVersionedMultiAssets: {
    _enum: {
      V0: 'Vec<XcmV0MultiAsset>',
      V1: 'XcmV1MultiassetMultiAssets'
    }
  },
  /**
   * Lookup101: xcm::v0::multi_asset::MultiAsset
   **/
  XcmV0MultiAsset: {
    _enum: {
      None: 'Null',
      All: 'Null',
      AllFungible: 'Null',
      AllNonFungible: 'Null',
      AllAbstractFungible: {
        id: 'Bytes',
      },
      AllAbstractNonFungible: {
        class: 'Bytes',
      },
      AllConcreteFungible: {
        id: 'XcmV0MultiLocation',
      },
      AllConcreteNonFungible: {
        class: 'XcmV0MultiLocation',
      },
      AbstractFungible: {
        id: 'Bytes',
        amount: 'Compact<u128>',
      },
      AbstractNonFungible: {
        class: 'Bytes',
        instance: 'XcmV1MultiassetAssetInstance',
      },
      ConcreteFungible: {
        id: 'XcmV0MultiLocation',
        amount: 'Compact<u128>',
      },
      ConcreteNonFungible: {
        class: 'XcmV0MultiLocation',
        instance: 'XcmV1MultiassetAssetInstance'
      }
    }
  },
  /**
   * Lookup102: xcm::v0::multi_location::MultiLocation
   **/
  XcmV0MultiLocation: {
    _enum: {
      Null: 'Null',
      X1: 'XcmV0Junction',
      X2: '(XcmV0Junction,XcmV0Junction)',
      X3: '(XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X4: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X5: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X6: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X7: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)',
      X8: '(XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction,XcmV0Junction)'
    }
  },
  /**
   * Lookup103: xcm::v0::junction::Junction
   **/
  XcmV0Junction: {
    _enum: {
      Parent: 'Null',
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'XcmV0JunctionNetworkId',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'XcmV0JunctionNetworkId',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'XcmV0JunctionNetworkId',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: 'Bytes',
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV0JunctionBodyId',
        part: 'XcmV0JunctionBodyPart'
      }
    }
  },
  /**
   * Lookup104: xcm::VersionedMultiLocation
   **/
  XcmVersionedMultiLocation: {
    _enum: {
      V0: 'XcmV0MultiLocation',
      V1: 'XcmV1MultiLocation'
    }
  },
  /**
   * Lookup105: cumulus_pallet_xcm::pallet::Event<T>
   **/
  CumulusPalletXcmEvent: {
    _enum: {
      InvalidFormat: '[u8;8]',
      UnsupportedVersion: '[u8;8]',
      ExecutedDownward: '([u8;8],XcmV2TraitsOutcome)'
    }
  },
  /**
   * Lookup106: cumulus_pallet_dmp_queue::pallet::Event<T>
   **/
  CumulusPalletDmpQueueEvent: {
    _enum: {
      InvalidFormat: {
        messageId: '[u8;32]',
      },
      UnsupportedVersion: {
        messageId: '[u8;32]',
      },
      ExecutedDownward: {
        messageId: '[u8;32]',
        outcome: 'XcmV2TraitsOutcome',
      },
      WeightExhausted: {
        messageId: '[u8;32]',
        remainingWeight: 'u64',
        requiredWeight: 'u64',
      },
      OverweightEnqueued: {
        messageId: '[u8;32]',
        overweightIndex: 'u64',
        requiredWeight: 'u64',
      },
      OverweightServiced: {
        overweightIndex: 'u64',
        weightUsed: 'u64'
      }
    }
  },
  /**
   * Lookup107: orml_xtokens::module::Event<T>
   **/
  OrmlXtokensModuleEvent: {
    _enum: {
      TransferredMultiAssets: {
        sender: 'AccountId32',
        assets: 'XcmV1MultiassetMultiAssets',
        fee: 'XcmV1MultiAsset',
        dest: 'XcmV1MultiLocation'
      }
    }
  },
  /**
   * Lookup108: orml_unknown_tokens::module::Event
   **/
  OrmlUnknownTokensModuleEvent: {
    _enum: {
      Deposited: {
        asset: 'XcmV1MultiAsset',
        who: 'XcmV1MultiLocation',
      },
      Withdrawn: {
        asset: 'XcmV1MultiAsset',
        who: 'XcmV1MultiLocation'
      }
    }
  },
  /**
   * Lookup109: orml_tokens::module::Event<T>
   **/
  OrmlTokensModuleEvent: {
    _enum: {
      Endowed: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      DustLost: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      Transfer: {
        currencyId: 'u128',
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      Reserved: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      Unreserved: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      ReserveRepatriated: {
        currencyId: 'u128',
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        status: 'FrameSupportTokensMiscBalanceStatus',
      },
      BalanceSet: {
        currencyId: 'u128',
        who: 'AccountId32',
        free: 'u128',
        reserved: 'u128',
      },
      TotalIssuanceSet: {
        currencyId: 'u128',
        amount: 'u128',
      },
      Withdrawn: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      Slashed: {
        currencyId: 'u128',
        who: 'AccountId32',
        freeAmount: 'u128',
        reservedAmount: 'u128',
      },
      Deposited: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      LockSet: {
        lockId: '[u8;8]',
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      LockRemoved: {
        lockId: '[u8;8]',
        currencyId: 'u128',
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup111: pallet_oracle::pallet::Event<T>
   **/
  PalletOracleEvent: {
    _enum: {
      AssetInfoChange: '(u128,Percent,u32,u32,u32,u128,u128)',
      SignerSet: '(AccountId32,AccountId32)',
      StakeAdded: '(AccountId32,u128,u128)',
      StakeRemoved: '(AccountId32,u128,u32)',
      StakeReclaimed: '(AccountId32,u128)',
      PriceSubmitted: '(AccountId32,u128,u128)',
      UserSlashed: '(AccountId32,u128,u128)',
      OracleRewarded: '(AccountId32,u128,u128)',
      RewardingAdjustment: 'u64',
      AnswerPruned: '(AccountId32,u128)',
      PriceChanged: '(u128,u128)'
    }
  },
  /**
   * Lookup113: pallet_currency_factory::pallet::Event<T>
   **/
  PalletCurrencyFactoryEvent: {
    _enum: {
      RangeCreated: {
        range: {
          current: 'u128',
          end: 'u128'
        }
      }
    }
  },
  /**
   * Lookup115: pallet_vault::pallet::Event<T>
   **/
  PalletVaultEvent: {
    _enum: {
      VaultCreated: {
        id: 'u64',
      },
      Deposited: {
        account: 'AccountId32',
        assetAmount: 'u128',
        lpAmount: 'u128',
      },
      LiquidateStrategy: {
        account: 'AccountId32',
        amount: 'u128',
      },
      Withdrawn: {
        account: 'AccountId32',
        lpAmount: 'u128',
        assetAmount: 'u128',
      },
      EmergencyShutdown: {
        vault: 'u64',
      },
      VaultStarted: {
        vault: 'u64'
      }
    }
  },
  /**
   * Lookup116: pallet_assets_registry::pallet::Event<T>
   **/
  PalletAssetsRegistryEvent: {
    _enum: {
      AssetRegistered: {
        assetId: 'u128',
        location: 'ComposableTraitsXcmAssetsXcmAssetLocation',
      },
      AssetUpdated: {
        assetId: 'u128',
        location: 'ComposableTraitsXcmAssetsXcmAssetLocation',
      },
      MinFeeUpdated: {
        targetParachainId: 'u32',
        foreignAssetId: 'ComposableTraitsXcmAssetsXcmAssetLocation',
        amount: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup117: composable_traits::xcm::assets::XcmAssetLocation
   **/
  ComposableTraitsXcmAssetsXcmAssetLocation: 'XcmV1MultiLocation',
  /**
   * Lookup119: pallet_governance_registry::pallet::Event<T>
   **/
  PalletGovernanceRegistryEvent: {
    _enum: {
      Set: {
        assetId: 'u128',
        value: 'AccountId32',
      },
      GrantRoot: {
        assetId: 'u128',
      },
      Remove: {
        assetId: 'u128'
      }
    }
  },
  /**
   * Lookup120: pallet_crowdloan_rewards::pallet::Event<T>
   **/
  PalletCrowdloanRewardsEvent: {
    _enum: {
      Initialized: {
        at: 'u64',
      },
      Claimed: {
        remoteAccount: 'PalletCrowdloanRewardsModelsRemoteAccount',
        rewardAccount: 'AccountId32',
        amount: 'u128',
      },
      Associated: {
        remoteAccount: 'PalletCrowdloanRewardsModelsRemoteAccount',
        rewardAccount: 'AccountId32',
      },
      OverFunded: {
        excessFunds: 'u128'
      }
    }
  },
  /**
   * Lookup121: pallet_crowdloan_rewards::models::RemoteAccount<sp_core::crypto::AccountId32>
   **/
  PalletCrowdloanRewardsModelsRemoteAccount: {
    _enum: {
      RelayChain: 'AccountId32',
      Ethereum: 'ComposableSupportEthereumAddress'
    }
  },
  /**
   * Lookup122: composable_support::types::EthereumAddress
   **/
  ComposableSupportEthereumAddress: '[u8;20]',
  /**
   * Lookup123: pallet_vesting::module::Event<T>
   **/
  PalletVestingModuleEvent: {
    _enum: {
      VestingScheduleAdded: {
        from: 'AccountId32',
        to: 'AccountId32',
        asset: 'u128',
        vestingScheduleId: 'u128',
        schedule: 'ComposableTraitsVestingVestingSchedule',
        scheduleAmount: 'u128',
      },
      Claimed: {
        who: 'AccountId32',
        asset: 'u128',
        vestingScheduleIds: 'ComposableTraitsVestingVestingScheduleIdSet',
        lockedAmount: 'u128',
        claimedAmountPerSchedule: 'BTreeMap<u128, u128>',
      },
      VestingSchedulesUpdated: {
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup124: composable_traits::vesting::VestingSchedule<VestingScheduleId, BlockNumber, Moment, Balance>
   **/
  ComposableTraitsVestingVestingSchedule: {
    vestingScheduleId: 'u128',
    window: 'ComposableTraitsVestingVestingWindow',
    periodCount: 'u32',
    perPeriod: 'Compact<u128>',
    alreadyClaimed: 'u128'
  },
  /**
   * Lookup125: composable_traits::vesting::VestingWindow<BlockNumber, Moment>
   **/
  ComposableTraitsVestingVestingWindow: {
    _enum: {
      MomentBased: {
        start: 'u64',
        period: 'u64',
      },
      BlockNumberBased: {
        start: 'u32',
        period: 'u32'
      }
    }
  },
  /**
   * Lookup126: composable_traits::vesting::VestingScheduleIdSet<Id, MaxVestingSchedules>
   **/
  ComposableTraitsVestingVestingScheduleIdSet: {
    _enum: {
      All: 'Null',
      One: 'u128',
      Many: 'Vec<u128>'
    }
  },
  /**
   * Lookup133: pallet_bonded_finance::pallet::Event<T>
   **/
  PalletBondedFinanceEvent: {
    _enum: {
      NewOffer: {
        offerId: 'u128',
        beneficiary: 'AccountId32',
      },
      NewBond: {
        offerId: 'u128',
        who: 'AccountId32',
        nbOfBonds: 'u128',
      },
      OfferCancelled: {
        offerId: 'u128',
      },
      OfferCompleted: {
        offerId: 'u128'
      }
    }
  },
  /**
   * Lookup134: pallet_dutch_auction::pallet::Event<T>
   **/
  PalletDutchAuctionEvent: {
    _enum: {
      OrderAdded: {
        orderId: 'u128',
        order: 'PalletDutchAuctionSellOrder',
      },
      OrderTaken: {
        orderId: 'u128',
        taken: 'u128',
      },
      OrderRemoved: {
        orderId: 'u128',
      },
      ConfigurationAdded: {
        configurationId: 'u128',
        configuration: 'ComposableTraitsTimeTimeReleaseFunction'
      }
    }
  },
  /**
   * Lookup135: pallet_dutch_auction::types::SellOrder<primitives::currency::CurrencyId, Balance, sp_core::crypto::AccountId32, pallet_dutch_auction::types::EDContext<Balance>, composable_traits::time::TimeReleaseFunction>
   **/
  PalletDutchAuctionSellOrder: {
    fromTo: 'AccountId32',
    order: 'ComposableTraitsDefiSellCurrencyId',
    configuration: 'ComposableTraitsTimeTimeReleaseFunction',
    context: 'PalletDutchAuctionEdContext',
    totalAmountReceived: 'u128'
  },
  /**
   * Lookup136: pallet_dutch_auction::types::EDContext<Balance>
   **/
  PalletDutchAuctionEdContext: {
    addedAt: 'u64',
    deposit: 'u128'
  },
  /**
   * Lookup137: composable_traits::time::TimeReleaseFunction
   **/
  ComposableTraitsTimeTimeReleaseFunction: {
    _enum: {
      LinearDecrease: 'ComposableTraitsTimeLinearDecrease',
      StairstepExponentialDecrease: 'ComposableTraitsTimeStairstepExponentialDecrease'
    }
  },
  /**
   * Lookup138: composable_traits::time::LinearDecrease
   **/
  ComposableTraitsTimeLinearDecrease: {
    total: 'u64'
  },
  /**
   * Lookup139: composable_traits::time::StairstepExponentialDecrease
   **/
  ComposableTraitsTimeStairstepExponentialDecrease: {
    step: 'u64',
    cut: 'Permill'
  },
  /**
   * Lookup141: composable_traits::defi::Sell<primitives::currency::CurrencyId, Balance>
   **/
  ComposableTraitsDefiSellCurrencyId: {
    pair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
    take: 'ComposableTraitsDefiTake'
  },
  /**
   * Lookup142: composable_traits::defi::CurrencyPair<primitives::currency::CurrencyId>
   **/
  ComposableTraitsDefiCurrencyPairCurrencyId: {
    base: 'u128',
    quote: 'u128'
  },
  /**
   * Lookup143: composable_traits::defi::Take<Balance>
   **/
  ComposableTraitsDefiTake: {
    amount: 'u128',
    limit: 'u128'
  },
  /**
   * Lookup145: pallet_mosaic::pallet::Event<T>
   **/
  PalletMosaicEvent: {
    _enum: {
      RelayerSet: {
        relayer: 'AccountId32',
      },
      RelayerRotated: {
        ttl: 'u32',
        accountId: 'AccountId32',
      },
      BudgetUpdated: {
        assetId: 'u128',
        amount: 'u128',
        decay: 'PalletMosaicDecayBudgetPenaltyDecayer',
      },
      NetworksUpdated: {
        networkId: 'u32',
        networkInfo: 'PalletMosaicNetworkInfo',
      },
      TransferOut: {
        id: 'H256',
        to: 'ComposableSupportEthereumAddress',
        assetId: 'u128',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
        amount: 'u128',
        swapToNative: 'bool',
        sourceUserAccount: 'AccountId32',
        ammSwapInfo: 'Option<PalletMosaicAmmSwapInfo>',
        minimumAmountOut: 'u128',
      },
      StaleTxClaimed: {
        to: 'AccountId32',
        by: 'AccountId32',
        assetId: 'u128',
        amount: 'u128',
      },
      TransferInto: {
        id: 'H256',
        to: 'AccountId32',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
        assetId: 'u128',
        amount: 'u128',
      },
      TransferIntoRescined: {
        account: 'AccountId32',
        amount: 'u128',
        assetId: 'u128',
      },
      PartialTransferAccepted: {
        from: 'AccountId32',
        assetId: 'u128',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
        amount: 'u128',
      },
      TransferAccepted: {
        from: 'AccountId32',
        assetId: 'u128',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
        amount: 'u128',
      },
      TransferClaimed: {
        by: 'AccountId32',
        to: 'AccountId32',
        assetId: 'u128',
        amount: 'u128',
      },
      AssetMappingCreated: {
        assetId: 'u128',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
      },
      AssetMappingUpdated: {
        assetId: 'u128',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
      },
      AssetMappingDeleted: {
        assetId: 'u128',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId'
      }
    }
  },
  /**
   * Lookup146: pallet_mosaic::decay::BudgetPenaltyDecayer<Balance, BlockNumber>
   **/
  PalletMosaicDecayBudgetPenaltyDecayer: {
    _enum: {
      Linear: 'PalletMosaicDecayLinearDecay'
    }
  },
  /**
   * Lookup147: pallet_mosaic::decay::LinearDecay<Balance, BlockNumber>
   **/
  PalletMosaicDecayLinearDecay: {
    factor: 'u128'
  },
  /**
   * Lookup148: pallet_mosaic::pallet::NetworkInfo<Balance>
   **/
  PalletMosaicNetworkInfo: {
    enabled: 'bool',
    minTransferSize: 'u128',
    maxTransferSize: 'u128'
  },
  /**
   * Lookup149: common::types::MosaicRemoteAssetId
   **/
  CommonMosaicRemoteAssetId: {
    _enum: {
      EthereumTokenAddress: '[u8;20]'
    }
  },
  /**
   * Lookup151: pallet_mosaic::pallet::AmmSwapInfo<N, R, M>
   **/
  PalletMosaicAmmSwapInfo: {
    destinationTokenOutAddress: 'ComposableSupportEthereumAddress',
    destinationAmm: 'PalletMosaicRemoteAmm',
    minimumAmountOut: 'u128'
  },
  /**
   * Lookup152: pallet_mosaic::pallet::RemoteAmm<N, R>
   **/
  PalletMosaicRemoteAmm: {
    networkId: 'u32',
    ammId: 'u128'
  },
  /**
   * Lookup153: pallet_liquidations::pallet::Event<T>
   **/
  PalletLiquidationsEvent: {
    _enum: ['PositionWasSentToLiquidation']
  },
  /**
   * Lookup154: pallet_lending::pallet::Event<T>
   **/
  PalletLendingEvent: {
    _enum: {
      MarketCreated: {
        marketId: 'u32',
        vaultId: 'u64',
        manager: 'AccountId32',
        currencyPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
      },
      MarketUpdated: {
        marketId: 'u32',
        input: 'ComposableTraitsLendingUpdateInput',
      },
      CollateralDeposited: {
        sender: 'AccountId32',
        marketId: 'u32',
        amount: 'u128',
      },
      CollateralWithdrawn: {
        sender: 'AccountId32',
        marketId: 'u32',
        amount: 'u128',
      },
      Borrowed: {
        sender: 'AccountId32',
        marketId: 'u32',
        amount: 'u128',
      },
      BorrowRepaid: {
        sender: 'AccountId32',
        marketId: 'u32',
        beneficiary: 'AccountId32',
        amount: 'u128',
      },
      LiquidationInitiated: {
        marketId: 'u32',
        borrowers: 'Vec<AccountId32>',
      },
      MayGoUnderCollateralizedSoon: {
        marketId: 'u32',
        account: 'AccountId32'
      }
    }
  },
  /**
   * Lookup156: composable_traits::lending::UpdateInput<LiquidationStrategyId, BlockNumber>
   **/
  ComposableTraitsLendingUpdateInput: {
    collateralFactor: 'u128',
    underCollateralizedWarnPercent: 'Percent',
    liquidators: 'Vec<u32>',
    maxPriceAge: 'u32'
  },
  /**
   * Lookup158: pallet_pablo::pallet::Event<T>
   **/
  PalletPabloEvent: {
    _enum: {
      PoolCreated: {
        poolId: 'u128',
        owner: 'AccountId32',
        assets: 'ComposableTraitsDefiCurrencyPairCurrencyId',
      },
      PoolDeleted: {
        poolId: 'u128',
        baseAmount: 'u128',
        quoteAmount: 'u128',
      },
      LiquidityAdded: {
        who: 'AccountId32',
        poolId: 'u128',
        baseAmount: 'u128',
        quoteAmount: 'u128',
        mintedLp: 'u128',
      },
      LiquidityRemoved: {
        who: 'AccountId32',
        poolId: 'u128',
        baseAmount: 'u128',
        quoteAmount: 'u128',
        totalIssuance: 'u128',
      },
      Swapped: {
        poolId: 'u128',
        who: 'AccountId32',
        baseAsset: 'u128',
        quoteAsset: 'u128',
        baseAmount: 'u128',
        quoteAmount: 'u128',
        fee: 'ComposableTraitsDexFee',
      },
      TwapUpdated: {
        poolId: 'u128',
        timestamp: 'u64',
        twaps: 'BTreeMap<u128, u128>'
      }
    }
  },
  /**
   * Lookup159: composable_traits::dex::Fee<primitives::currency::CurrencyId, Balance>
   **/
  ComposableTraitsDexFee: {
    fee: 'u128',
    lpFee: 'u128',
    ownerFee: 'u128',
    protocolFee: 'u128',
    assetId: 'u128'
  },
  /**
   * Lookup163: pallet_dex_router::pallet::Event<T>
   **/
  PalletDexRouterEvent: {
    _enum: {
      RouteAdded: {
        xAssetId: 'u128',
        yAssetId: 'u128',
        route: 'Vec<u128>',
      },
      RouteDeleted: {
        xAssetId: 'u128',
        yAssetId: 'u128',
        route: 'Vec<u128>',
      },
      RouteUpdated: {
        xAssetId: 'u128',
        yAssetId: 'u128',
        oldRoute: 'Vec<u128>',
        updatedRoute: 'Vec<u128>'
      }
    }
  },
  /**
   * Lookup164: pallet_fnft::pallet::Event<T>
   **/
  PalletFnftEvent: {
    _enum: {
      FinancialNftCollectionCreated: {
        collectionId: 'u128',
        who: 'AccountId32',
        admin: 'AccountId32',
      },
      FinancialNftCreated: {
        collectionId: 'u128',
        instanceId: 'u64',
      },
      FinancialNftBurned: {
        collectionId: 'u128',
        instanceId: 'u64',
      },
      FinancialNftTransferred: {
        collectionId: 'u128',
        instanceId: 'u64',
        to: 'AccountId32'
      }
    }
  },
  /**
   * Lookup165: pallet_staking_rewards::pallet::Event<T>
   **/
  PalletStakingRewardsEvent: {
    _enum: {
      RewardPoolCreated: {
        poolId: 'u128',
        owner: 'AccountId32',
        endBlock: 'u32',
      },
      Staked: {
        poolId: 'u128',
        owner: 'AccountId32',
        amount: 'u128',
        durationPreset: 'u64',
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
        rewardMultiplier: 'u64',
        keepAlive: 'bool',
      },
      Claimed: {
        owner: 'AccountId32',
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
      },
      StakeAmountExtended: {
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
        amount: 'u128',
      },
      Unstaked: {
        owner: 'AccountId32',
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
        slash: 'Option<u128>',
      },
      SplitPosition: {
        positions: 'Vec<(u128,u64,u128)>',
      },
      RewardTransferred: {
        from: 'AccountId32',
        poolId: 'u128',
        rewardCurrency: 'u128',
        rewardIncrement: 'u128',
      },
      RewardAccumulationHookError: {
        poolId: 'u128',
        assetId: 'u128',
        error: 'PalletStakingRewardsRewardAccumulationHookError',
      },
      MaxRewardsAccumulated: {
        poolId: 'u128',
        assetId: 'u128',
      },
      RewardPoolUpdated: {
        poolId: 'u128',
      },
      RewardsPotIncreased: {
        poolId: 'u128',
        assetId: 'u128',
        amount: 'u128',
      },
      UnstakeRewardSlashed: {
        poolId: 'u128',
        owner: 'AccountId32',
        fnftInstanceId: 'u64',
        rewardAssetId: 'u128',
        amountSlashed: 'u128'
      }
    }
  },
  /**
   * Lookup169: pallet_staking_rewards::pallet::RewardAccumulationHookError
   **/
  PalletStakingRewardsRewardAccumulationHookError: {
    _enum: ['BackToTheFuture', 'RewardsPotEmpty']
  },
  /**
   * Lookup170: pallet_call_filter::pallet::Event<T>
   **/
  PalletCallFilterEvent: {
    _enum: {
      Disabled: {
        entry: 'ComposableTraitsCallFilterCallFilterEntry',
      },
      Enabled: {
        entry: 'ComposableTraitsCallFilterCallFilterEntry'
      }
    }
  },
  /**
   * Lookup171: composable_traits::call_filter::CallFilterEntry<common::MaxStringSize>
   **/
  ComposableTraitsCallFilterCallFilterEntry: {
    palletName: 'Bytes',
    functionName: 'Bytes'
  },
  /**
   * Lookup172: common::MaxStringSize
   **/
  CommonMaxStringSize: 'Null',
  /**
   * Lookup174: pallet_ibc_ping::pallet::Event<T>
   **/
  PalletIbcPingEvent: {
    _enum: {
      PacketSent: 'Null',
      ChannelOpened: {
        channelId: 'Bytes',
        portId: 'Bytes'
      }
    }
  },
  /**
   * Lookup175: ibc_transfer::pallet::Event<T>
   **/
  IbcTransferEvent: {
    _enum: {
      TokenTransferInitiated: {
        from: 'AccountId32',
        to: 'Bytes',
        amount: 'u128',
      },
      ChannelOpened: {
        channelId: 'Bytes',
        portId: 'Bytes',
      },
      PalletParamsUpdated: {
        sendEnabled: 'bool',
        receiveEnabled: 'bool'
      }
    }
  },
  /**
   * Lookup176: pallet_ibc::pallet::Event<T>
   **/
  PalletIbcEvent: {
    _enum: {
      IbcEvents: {
        events: 'Vec<PalletIbcEventsIbcEvent>',
      },
      IbcErrors: {
        errors: 'Vec<PalletIbcErrorsIbcError>'
      }
    }
  },
  /**
   * Lookup178: pallet_ibc::events::IbcEvent
   **/
  PalletIbcEventsIbcEvent: {
    _enum: {
      NewBlock: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
      },
      CreateClient: {
        clientId: 'Bytes',
        clientType: 'Bytes',
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        consensusHeight: 'u64',
        consensusRevisionNumber: 'u64',
      },
      UpdateClient: {
        clientId: 'Bytes',
        clientType: 'Bytes',
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        consensusHeight: 'u64',
        consensusRevisionNumber: 'u64',
      },
      UpgradeClient: {
        clientId: 'Bytes',
        clientType: 'Bytes',
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        consensusHeight: 'u64',
        consensusRevisionNumber: 'u64',
      },
      ClientMisbehaviour: {
        clientId: 'Bytes',
        clientType: 'Bytes',
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        consensusHeight: 'u64',
        consensusRevisionNumber: 'u64',
      },
      OpenInitConnection: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        connectionId: 'Option<Bytes>',
        clientId: 'Bytes',
        counterpartyConnectionId: 'Option<Bytes>',
        counterpartyClientId: 'Bytes',
      },
      OpenConfirmConnection: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        connectionId: 'Option<Bytes>',
        clientId: 'Bytes',
        counterpartyConnectionId: 'Option<Bytes>',
        counterpartyClientId: 'Bytes',
      },
      OpenTryConnection: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        connectionId: 'Option<Bytes>',
        clientId: 'Bytes',
        counterpartyConnectionId: 'Option<Bytes>',
        counterpartyClientId: 'Bytes',
      },
      OpenAckConnection: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        connectionId: 'Option<Bytes>',
        clientId: 'Bytes',
        counterpartyConnectionId: 'Option<Bytes>',
        counterpartyClientId: 'Bytes',
      },
      OpenInitChannel: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Option<Bytes>',
        connectionId: 'Bytes',
        counterpartyPortId: 'Bytes',
        counterpartyChannelId: 'Option<Bytes>',
      },
      OpenConfirmChannel: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Option<Bytes>',
        connectionId: 'Bytes',
        counterpartyPortId: 'Bytes',
        counterpartyChannelId: 'Option<Bytes>',
      },
      OpenTryChannel: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Option<Bytes>',
        connectionId: 'Bytes',
        counterpartyPortId: 'Bytes',
        counterpartyChannelId: 'Option<Bytes>',
      },
      OpenAckChannel: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Option<Bytes>',
        connectionId: 'Bytes',
        counterpartyPortId: 'Bytes',
        counterpartyChannelId: 'Option<Bytes>',
      },
      CloseInitChannel: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Bytes',
        connectionId: 'Bytes',
        counterpartyPortId: 'Bytes',
        counterpartyChannelId: 'Option<Bytes>',
      },
      CloseConfirmChannel: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        channelId: 'Option<Bytes>',
        portId: 'Bytes',
        connectionId: 'Bytes',
        counterpartyPortId: 'Bytes',
        counterpartyChannelId: 'Option<Bytes>',
      },
      ReceivePacket: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Bytes',
        destPort: 'Bytes',
        destChannel: 'Bytes',
        sequence: 'u64',
      },
      SendPacket: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Bytes',
        destPort: 'Bytes',
        destChannel: 'Bytes',
        sequence: 'u64',
      },
      AcknowledgePacket: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Bytes',
        sequence: 'u64',
      },
      WriteAcknowledgement: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Bytes',
        destPort: 'Bytes',
        destChannel: 'Bytes',
        sequence: 'u64',
      },
      TimeoutPacket: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Bytes',
        sequence: 'u64',
      },
      TimeoutOnClosePacket: {
        revisionHeight: 'u64',
        revisionNumber: 'u64',
        portId: 'Bytes',
        channelId: 'Bytes',
        sequence: 'u64',
      },
      Empty: 'Null',
      ChainError: 'Null',
      AppModule: {
        kind: 'Bytes',
        moduleId: 'Bytes'
      }
    }
  },
  /**
   * Lookup180: pallet_ibc::errors::IbcError
   **/
  PalletIbcErrorsIbcError: {
    _enum: {
      Ics02Client: {
        message: 'Bytes',
      },
      Ics03Connection: {
        message: 'Bytes',
      },
      Ics04Channel: {
        message: 'Bytes',
      },
      Ics20FungibleTokenTransfer: {
        message: 'Bytes',
      },
      UnknownMessageTypeUrl: {
        message: 'Bytes',
      },
      MalformedMessageBytes: {
        message: 'Bytes'
      }
    }
  },
  /**
   * Lookup181: pallet_cosmwasm::pallet::Event<T>
   **/
  PalletCosmwasmEvent: {
    _enum: {
      Uploaded: {
        codeHash: 'H256',
        codeId: 'u64',
      },
      Instantiated: {
        contract: 'AccountId32',
        info: 'PalletCosmwasmContractInfo',
      },
      Executed: {
        contract: 'AccountId32',
        entrypoint: 'PalletCosmwasmEntryPoint',
        data: 'Option<Bytes>',
      },
      ExecutionFailed: {
        contract: 'AccountId32',
        entrypoint: 'PalletCosmwasmEntryPoint',
        error: 'Bytes',
      },
      Emitted: {
        contract: 'AccountId32',
        ty: 'Bytes',
        attributes: 'Vec<(Bytes,Bytes)>'
      }
    }
  },
  /**
   * Lookup182: pallet_cosmwasm::types::ContractInfo<sp_core::crypto::AccountId32, sp_runtime::bounded::bounded_vec::BoundedVec<T, S>, sp_runtime::bounded::bounded_vec::BoundedVec<T, S>>
   **/
  PalletCosmwasmContractInfo: {
    codeId: 'u64',
    trieId: 'Bytes',
    instantiator: 'AccountId32',
    admin: 'Option<AccountId32>',
    label: 'Bytes'
  },
  /**
   * Lookup185: pallet_cosmwasm::pallet::EntryPoint
   **/
  PalletCosmwasmEntryPoint: {
    _enum: ['Instantiate', 'Execute', 'Migrate', 'Reply', 'Sudo', 'Query']
  },
  /**
   * Lookup188: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup191: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup193: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
      fill_block: {
        ratio: 'Perbill',
      },
      remark: {
        remark: 'Bytes',
      },
      set_heap_pages: {
        pages: 'u64',
      },
      set_code: {
        code: 'Bytes',
      },
      set_code_without_checks: {
        code: 'Bytes',
      },
      set_storage: {
        items: 'Vec<(Bytes,Bytes)>',
      },
      kill_storage: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'Vec<Bytes>',
      },
      kill_prefix: {
        prefix: 'Bytes',
        subkeys: 'u32',
      },
      remark_with_event: {
        remark: 'Bytes'
      }
    }
  },
  /**
   * Lookup196: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'u64',
    maxBlock: 'u64',
    perClass: 'FrameSupportWeightsPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup197: frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportWeightsPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup198: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'u64',
    maxExtrinsic: 'Option<u64>',
    maxTotal: 'Option<u64>',
    reserved: 'Option<u64>'
  },
  /**
   * Lookup200: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportWeightsPerDispatchClassU32'
  },
  /**
   * Lookup201: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup202: frame_support::weights::RuntimeDbWeight
   **/
  FrameSupportWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup203: sp_version::RuntimeVersion
   **/
  SpVersionRuntimeVersion: {
    specName: 'Text',
    implName: 'Text',
    authoringVersion: 'u32',
    specVersion: 'u32',
    implVersion: 'u32',
    apis: 'Vec<([u8;8],u32)>',
    transactionVersion: 'u32',
    stateVersion: 'u8'
  },
  /**
   * Lookup207: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup208: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup209: pallet_sudo::pallet::Call<T>
   **/
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: 'Call',
      },
      sudo_unchecked_weight: {
        call: 'Call',
        weight: 'u64',
      },
      set_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
      },
      sudo_as: {
        who: 'MultiAddress',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup211: pallet_asset_tx_payment::pallet::Call<T>
   **/
  PalletAssetTxPaymentCall: {
    _enum: {
      set_payment_asset: {
        payer: 'AccountId32',
        assetId: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup213: pallet_indices::pallet::Call<T>
   **/
  PalletIndicesCall: {
    _enum: {
      claim: {
        index: 'u32',
      },
      transfer: {
        _alias: {
          new_: 'new',
        },
        new_: 'AccountId32',
        index: 'u32',
      },
      free: {
        index: 'u32',
      },
      force_transfer: {
        _alias: {
          new_: 'new',
        },
        new_: 'AccountId32',
        index: 'u32',
        freeze: 'bool',
      },
      freeze: {
        index: 'u32'
      }
    }
  },
  /**
   * Lookup214: pallet_balances::pallet::Call<T, I>
   **/
  PalletBalancesCall: {
    _enum: {
      transfer: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      set_balance: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>',
        newReserved: 'Compact<u128>',
      },
      force_transfer: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_keep_alive: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_all: {
        dest: 'MultiAddress',
        keepAlive: 'bool',
      },
      force_unreserve: {
        who: 'MultiAddress',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup216: pallet_identity::pallet::Call<T>
   **/
  PalletIdentityCall: {
    _enum: {
      add_registrar: {
        account: 'AccountId32',
      },
      set_identity: {
        info: 'PalletIdentityIdentityInfo',
      },
      set_subs: {
        subs: 'Vec<(AccountId32,Data)>',
      },
      clear_identity: 'Null',
      request_judgement: {
        regIndex: 'Compact<u32>',
        maxFee: 'Compact<u128>',
      },
      cancel_request: {
        regIndex: 'u32',
      },
      set_fee: {
        index: 'Compact<u32>',
        fee: 'Compact<u128>',
      },
      set_account_id: {
        _alias: {
          new_: 'new',
        },
        index: 'Compact<u32>',
        new_: 'AccountId32',
      },
      set_fields: {
        index: 'Compact<u32>',
        fields: 'PalletIdentityBitFlags',
      },
      provide_judgement: {
        regIndex: 'Compact<u32>',
        target: 'MultiAddress',
        judgement: 'PalletIdentityJudgement',
      },
      kill_identity: {
        target: 'MultiAddress',
      },
      add_sub: {
        sub: 'MultiAddress',
        data: 'Data',
      },
      rename_sub: {
        sub: 'MultiAddress',
        data: 'Data',
      },
      remove_sub: {
        sub: 'MultiAddress',
      },
      quit_sub: 'Null'
    }
  },
  /**
   * Lookup217: pallet_identity::types::IdentityInfo<FieldLimit>
   **/
  PalletIdentityIdentityInfo: {
    additional: 'Vec<(Data,Data)>',
    display: 'Data',
    legal: 'Data',
    web: 'Data',
    riot: 'Data',
    email: 'Data',
    pgpFingerprint: 'Option<[u8;20]>',
    image: 'Data',
    twitter: 'Data'
  },
  /**
   * Lookup253: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
   **/
  PalletIdentityBitFlags: {
    _bitLength: 64,
    Display: 1,
    Legal: 2,
    Web: 4,
    Riot: 8,
    Email: 16,
    PgpFingerprint: 32,
    Image: 64,
    Twitter: 128
  },
  /**
   * Lookup254: pallet_identity::types::IdentityField
   **/
  PalletIdentityIdentityField: {
    _enum: ['__Unused0', 'Display', 'Legal', '__Unused3', 'Web', '__Unused5', '__Unused6', '__Unused7', 'Riot', '__Unused9', '__Unused10', '__Unused11', '__Unused12', '__Unused13', '__Unused14', '__Unused15', 'Email', '__Unused17', '__Unused18', '__Unused19', '__Unused20', '__Unused21', '__Unused22', '__Unused23', '__Unused24', '__Unused25', '__Unused26', '__Unused27', '__Unused28', '__Unused29', '__Unused30', '__Unused31', 'PgpFingerprint', '__Unused33', '__Unused34', '__Unused35', '__Unused36', '__Unused37', '__Unused38', '__Unused39', '__Unused40', '__Unused41', '__Unused42', '__Unused43', '__Unused44', '__Unused45', '__Unused46', '__Unused47', '__Unused48', '__Unused49', '__Unused50', '__Unused51', '__Unused52', '__Unused53', '__Unused54', '__Unused55', '__Unused56', '__Unused57', '__Unused58', '__Unused59', '__Unused60', '__Unused61', '__Unused62', '__Unused63', 'Image', '__Unused65', '__Unused66', '__Unused67', '__Unused68', '__Unused69', '__Unused70', '__Unused71', '__Unused72', '__Unused73', '__Unused74', '__Unused75', '__Unused76', '__Unused77', '__Unused78', '__Unused79', '__Unused80', '__Unused81', '__Unused82', '__Unused83', '__Unused84', '__Unused85', '__Unused86', '__Unused87', '__Unused88', '__Unused89', '__Unused90', '__Unused91', '__Unused92', '__Unused93', '__Unused94', '__Unused95', '__Unused96', '__Unused97', '__Unused98', '__Unused99', '__Unused100', '__Unused101', '__Unused102', '__Unused103', '__Unused104', '__Unused105', '__Unused106', '__Unused107', '__Unused108', '__Unused109', '__Unused110', '__Unused111', '__Unused112', '__Unused113', '__Unused114', '__Unused115', '__Unused116', '__Unused117', '__Unused118', '__Unused119', '__Unused120', '__Unused121', '__Unused122', '__Unused123', '__Unused124', '__Unused125', '__Unused126', '__Unused127', 'Twitter']
  },
  /**
   * Lookup255: pallet_identity::types::Judgement<Balance>
   **/
  PalletIdentityJudgement: {
    _enum: {
      Unknown: 'Null',
      FeePaid: 'u128',
      Reasonable: 'Null',
      KnownGood: 'Null',
      OutOfDate: 'Null',
      LowQuality: 'Null',
      Erroneous: 'Null'
    }
  },
  /**
   * Lookup256: pallet_multisig::pallet::Call<T>
   **/
  PalletMultisigCall: {
    _enum: {
      as_multi_threshold_1: {
        otherSignatories: 'Vec<AccountId32>',
        call: 'Call',
      },
      as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        maybeTimepoint: 'Option<PalletMultisigTimepoint>',
        call: 'WrapperKeepOpaque<Call>',
        storeCall: 'bool',
        maxWeight: 'u64',
      },
      approve_as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        maybeTimepoint: 'Option<PalletMultisigTimepoint>',
        callHash: '[u8;32]',
        maxWeight: 'u64',
      },
      cancel_as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        timepoint: 'PalletMultisigTimepoint',
        callHash: '[u8;32]'
      }
    }
  },
  /**
   * Lookup259: cumulus_pallet_parachain_system::pallet::Call<T>
   **/
  CumulusPalletParachainSystemCall: {
    _enum: {
      set_validation_data: {
        data: 'CumulusPrimitivesParachainInherentParachainInherentData',
      },
      sudo_send_upward_message: {
        message: 'Bytes',
      },
      authorize_upgrade: {
        codeHash: 'H256',
      },
      enact_authorized_upgrade: {
        code: 'Bytes'
      }
    }
  },
  /**
   * Lookup260: cumulus_primitives_parachain_inherent::ParachainInherentData
   **/
  CumulusPrimitivesParachainInherentParachainInherentData: {
    validationData: 'PolkadotPrimitivesV2PersistedValidationData',
    relayChainState: 'SpTrieStorageProof',
    downwardMessages: 'Vec<PolkadotCorePrimitivesInboundDownwardMessage>',
    horizontalMessages: 'BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>'
  },
  /**
   * Lookup261: polkadot_primitives::v2::PersistedValidationData<primitive_types::H256, N>
   **/
  PolkadotPrimitivesV2PersistedValidationData: {
    parentHead: 'Bytes',
    relayParentNumber: 'u32',
    relayParentStorageRoot: 'H256',
    maxPovSize: 'u32'
  },
  /**
   * Lookup263: sp_trie::storage_proof::StorageProof
   **/
  SpTrieStorageProof: {
    trieNodes: 'BTreeSet<Bytes>'
  },
  /**
   * Lookup266: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundDownwardMessage: {
    sentAt: 'u32',
    msg: 'Bytes'
  },
  /**
   * Lookup269: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundHrmpMessage: {
    sentAt: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup272: parachain_info::pallet::Call<T>
   **/
  ParachainInfoCall: 'Null',
  /**
   * Lookup273: pallet_authorship::pallet::Call<T>
   **/
  PalletAuthorshipCall: {
    _enum: {
      set_uncles: {
        newUncles: 'Vec<SpRuntimeHeader>'
      }
    }
  },
  /**
   * Lookup275: sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>
   **/
  SpRuntimeHeader: {
    parentHash: 'H256',
    number: 'Compact<u32>',
    stateRoot: 'H256',
    extrinsicsRoot: 'H256',
    digest: 'SpRuntimeDigest'
  },
  /**
   * Lookup276: sp_runtime::traits::BlakeTwo256
   **/
  SpRuntimeBlakeTwo256: 'Null',
  /**
   * Lookup277: pallet_collator_selection::pallet::Call<T>
   **/
  PalletCollatorSelectionCall: {
    _enum: {
      set_invulnerables: {
        _alias: {
          new_: 'new',
        },
        new_: 'Vec<AccountId32>',
      },
      set_desired_candidates: {
        max: 'u32',
      },
      set_candidacy_bond: {
        bond: 'u128',
      },
      register_as_candidate: 'Null',
      leave_intent: 'Null'
    }
  },
  /**
   * Lookup278: pallet_session::pallet::Call<T>
   **/
  PalletSessionCall: {
    _enum: {
      set_keys: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'DaliRuntimeOpaqueSessionKeys',
        proof: 'Bytes',
      },
      purge_keys: 'Null'
    }
  },
  /**
   * Lookup279: dali_runtime::opaque::SessionKeys
   **/
  DaliRuntimeOpaqueSessionKeys: {
    aura: 'SpConsensusAuraSr25519AppSr25519Public'
  },
  /**
   * Lookup280: sp_consensus_aura::sr25519::app_sr25519::Public
   **/
  SpConsensusAuraSr25519AppSr25519Public: 'SpCoreSr25519Public',
  /**
   * Lookup281: sp_core::sr25519::Public
   **/
  SpCoreSr25519Public: '[u8;32]',
  /**
   * Lookup282: pallet_collective::pallet::Call<T, I>
   **/
  PalletCollectiveCall: {
    _enum: {
      set_members: {
        newMembers: 'Vec<AccountId32>',
        prime: 'Option<AccountId32>',
        oldCount: 'u32',
      },
      execute: {
        proposal: 'Call',
        lengthBound: 'Compact<u32>',
      },
      propose: {
        threshold: 'Compact<u32>',
        proposal: 'Call',
        lengthBound: 'Compact<u32>',
      },
      vote: {
        proposal: 'H256',
        index: 'Compact<u32>',
        approve: 'bool',
      },
      close: {
        proposalHash: 'H256',
        index: 'Compact<u32>',
        proposalWeightBound: 'Compact<u64>',
        lengthBound: 'Compact<u32>',
      },
      disapprove_proposal: {
        proposalHash: 'H256'
      }
    }
  },
  /**
   * Lookup283: pallet_membership::pallet::Call<T, I>
   **/
  PalletMembershipCall: {
    _enum: {
      add_member: {
        who: 'AccountId32',
      },
      remove_member: {
        who: 'AccountId32',
      },
      swap_member: {
        remove: 'AccountId32',
        add: 'AccountId32',
      },
      reset_members: {
        members: 'Vec<AccountId32>',
      },
      change_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'AccountId32',
      },
      set_prime: {
        who: 'AccountId32',
      },
      clear_prime: 'Null'
    }
  },
  /**
   * Lookup284: pallet_treasury::pallet::Call<T, I>
   **/
  PalletTreasuryCall: {
    _enum: {
      propose_spend: {
        value: 'Compact<u128>',
        beneficiary: 'MultiAddress',
      },
      reject_proposal: {
        proposalId: 'Compact<u32>',
      },
      approve_proposal: {
        proposalId: 'Compact<u32>',
      },
      spend: {
        amount: 'Compact<u128>',
        beneficiary: 'MultiAddress',
      },
      remove_approval: {
        proposalId: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup285: pallet_democracy::pallet::Call<T, I>
   **/
  PalletDemocracyCall: {
    _enum: {
      propose: {
        proposalHash: 'H256',
        value: 'Compact<u128>',
      },
      second: {
        proposal: 'Compact<u32>',
        secondsUpperBound: 'Compact<u32>',
      },
      vote: {
        refIndex: 'Compact<u32>',
        vote: 'PalletDemocracyVoteAccountVote',
      },
      emergency_cancel: {
        refIndex: 'u32',
      },
      external_propose: {
        proposalHash: 'H256',
      },
      external_propose_majority: {
        proposalHash: 'H256',
      },
      external_propose_default: {
        proposalHash: 'H256',
      },
      fast_track: {
        proposalHash: 'H256',
        votingPeriod: 'u32',
        delay: 'u32',
      },
      veto_external: {
        proposalHash: 'H256',
      },
      cancel_referendum: {
        refIndex: 'Compact<u32>',
      },
      cancel_queued: {
        which: 'u32',
      },
      delegate: {
        to: 'AccountId32',
        conviction: 'PalletDemocracyConviction',
        balance: 'u128',
      },
      undelegate: 'Null',
      clear_public_proposals: 'Null',
      note_preimage: {
        encodedProposal: 'Bytes',
      },
      note_preimage_operational: {
        encodedProposal: 'Bytes',
      },
      note_imminent_preimage: {
        encodedProposal: 'Bytes',
      },
      note_imminent_preimage_operational: {
        encodedProposal: 'Bytes',
      },
      reap_preimage: {
        proposalHash: 'H256',
        proposalLenUpperBound: 'Compact<u32>',
      },
      unlock: {
        target: 'AccountId32',
      },
      remove_vote: {
        index: 'u32',
      },
      remove_other_vote: {
        target: 'AccountId32',
        index: 'u32',
      },
      enact_proposal: {
        proposalHash: 'H256',
        index: 'u32',
      },
      blacklist: {
        proposalHash: 'H256',
        maybeRefIndex: 'Option<u32>',
      },
      cancel_proposal: {
        propIndex: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup286: pallet_democracy::conviction::Conviction
   **/
  PalletDemocracyConviction: {
    _enum: ['None', 'Locked1x', 'Locked2x', 'Locked3x', 'Locked4x', 'Locked5x', 'Locked6x']
  },
  /**
   * Lookup290: pallet_scheduler::pallet::Call<T>
   **/
  PalletSchedulerCall: {
    _enum: {
      schedule: {
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed',
      },
      cancel: {
        when: 'u32',
        index: 'u32',
      },
      schedule_named: {
        id: 'Bytes',
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed',
      },
      cancel_named: {
        id: 'Bytes',
      },
      schedule_after: {
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed',
      },
      schedule_named_after: {
        id: 'Bytes',
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'FrameSupportScheduleMaybeHashed'
      }
    }
  },
  /**
   * Lookup292: frame_support::traits::schedule::MaybeHashed<dali_runtime::Call, primitive_types::H256>
   **/
  FrameSupportScheduleMaybeHashed: {
    _enum: {
      Value: 'Call',
      Hash: 'H256'
    }
  },
  /**
   * Lookup293: pallet_utility::pallet::Call<T>
   **/
  PalletUtilityCall: {
    _enum: {
      batch: {
        calls: 'Vec<Call>',
      },
      as_derivative: {
        index: 'u16',
        call: 'Call',
      },
      batch_all: {
        calls: 'Vec<Call>',
      },
      dispatch_as: {
        asOrigin: 'DaliRuntimeOriginCaller',
        call: 'Call',
      },
      force_batch: {
        calls: 'Vec<Call>'
      }
    }
  },
  /**
   * Lookup295: dali_runtime::OriginCaller
   **/
  DaliRuntimeOriginCaller: {
    _enum: {
      system: 'FrameSupportDispatchRawOrigin',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      __Unused4: 'Null',
      Void: 'SpCoreVoid',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      Council: 'PalletCollectiveRawOrigin',
      __Unused31: 'Null',
      __Unused32: 'Null',
      __Unused33: 'Null',
      __Unused34: 'Null',
      __Unused35: 'Null',
      __Unused36: 'Null',
      __Unused37: 'Null',
      __Unused38: 'Null',
      __Unused39: 'Null',
      __Unused40: 'Null',
      RelayerXcm: 'PalletXcmOrigin',
      CumulusXcm: 'CumulusPalletXcmOrigin',
      __Unused43: 'Null',
      __Unused44: 'Null',
      __Unused45: 'Null',
      __Unused46: 'Null',
      __Unused47: 'Null',
      __Unused48: 'Null',
      __Unused49: 'Null',
      __Unused50: 'Null',
      __Unused51: 'Null',
      __Unused52: 'Null',
      __Unused53: 'Null',
      __Unused54: 'Null',
      __Unused55: 'Null',
      __Unused56: 'Null',
      __Unused57: 'Null',
      __Unused58: 'Null',
      __Unused59: 'Null',
      __Unused60: 'Null',
      __Unused61: 'Null',
      __Unused62: 'Null',
      __Unused63: 'Null',
      __Unused64: 'Null',
      __Unused65: 'Null',
      __Unused66: 'Null',
      __Unused67: 'Null',
      __Unused68: 'Null',
      __Unused69: 'Null',
      TechnicalCollective: 'PalletCollectiveRawOrigin'
    }
  },
  /**
   * Lookup296: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup297: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
   **/
  PalletCollectiveRawOrigin: {
    _enum: {
      Members: '(u32,u32)',
      Member: 'AccountId32',
      _Phantom: 'Null'
    }
  },
  /**
   * Lookup299: pallet_xcm::pallet::Origin
   **/
  PalletXcmOrigin: {
    _enum: {
      Xcm: 'XcmV1MultiLocation',
      Response: 'XcmV1MultiLocation'
    }
  },
  /**
   * Lookup300: cumulus_pallet_xcm::pallet::Origin
   **/
  CumulusPalletXcmOrigin: {
    _enum: {
      Relay: 'Null',
      SiblingParachain: 'u32'
    }
  },
  /**
   * Lookup301: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup302: pallet_preimage::pallet::Call<T>
   **/
  PalletPreimageCall: {
    _enum: {
      note_preimage: {
        bytes: 'Bytes',
      },
      unnote_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      request_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      unrequest_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup303: pallet_account_proxy::pallet::Call<T>
   **/
  PalletAccountProxyCall: {
    _enum: {
      proxy: {
        real: 'AccountId32',
        forceProxyType: 'Option<ComposableTraitsAccountProxyProxyType>',
        call: 'Call',
      },
      add_proxy: {
        delegate: 'AccountId32',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32',
      },
      remove_proxy: {
        delegate: 'AccountId32',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32',
      },
      remove_proxies: 'Null',
      anonymous: {
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32',
        index: 'u16',
      },
      kill_anonymous: {
        spawner: 'AccountId32',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        index: 'u16',
        height: 'Compact<u32>',
        extIndex: 'Compact<u32>',
      },
      announce: {
        real: 'AccountId32',
        callHash: 'H256',
      },
      remove_announcement: {
        real: 'AccountId32',
        callHash: 'H256',
      },
      reject_announcement: {
        delegate: 'AccountId32',
        callHash: 'H256',
      },
      proxy_announced: {
        delegate: 'AccountId32',
        real: 'AccountId32',
        forceProxyType: 'Option<ComposableTraitsAccountProxyProxyType>',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup305: cumulus_pallet_xcmp_queue::pallet::Call<T>
   **/
  CumulusPalletXcmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'u64',
      },
      suspend_xcm_execution: 'Null',
      resume_xcm_execution: 'Null',
      update_suspend_threshold: {
        _alias: {
          new_: 'new',
        },
        new_: 'u32',
      },
      update_drop_threshold: {
        _alias: {
          new_: 'new',
        },
        new_: 'u32',
      },
      update_resume_threshold: {
        _alias: {
          new_: 'new',
        },
        new_: 'u32',
      },
      update_threshold_weight: {
        _alias: {
          new_: 'new',
        },
        new_: 'u64',
      },
      update_weight_restrict_decay: {
        _alias: {
          new_: 'new',
        },
        new_: 'u64',
      },
      update_xcmp_max_individual_weight: {
        _alias: {
          new_: 'new',
        },
        new_: 'u64'
      }
    }
  },
  /**
   * Lookup306: pallet_xcm::pallet::Call<T>
   **/
  PalletXcmCall: {
    _enum: {
      send: {
        dest: 'XcmVersionedMultiLocation',
        message: 'XcmVersionedXcm',
      },
      teleport_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
      },
      reserve_transfer_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
      },
      execute: {
        message: 'XcmVersionedXcm',
        maxWeight: 'u64',
      },
      force_xcm_version: {
        location: 'XcmV1MultiLocation',
        xcmVersion: 'u32',
      },
      force_default_xcm_version: {
        maybeXcmVersion: 'Option<u32>',
      },
      force_subscribe_version_notify: {
        location: 'XcmVersionedMultiLocation',
      },
      force_unsubscribe_version_notify: {
        location: 'XcmVersionedMultiLocation',
      },
      limited_reserve_transfer_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
        weightLimit: 'XcmV2WeightLimit',
      },
      limited_teleport_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
        weightLimit: 'XcmV2WeightLimit'
      }
    }
  },
  /**
   * Lookup307: xcm::VersionedXcm<Call>
   **/
  XcmVersionedXcm: {
    _enum: {
      V0: 'XcmV0Xcm',
      V1: 'XcmV1Xcm',
      V2: 'XcmV2Xcm'
    }
  },
  /**
   * Lookup308: xcm::v0::Xcm<Call>
   **/
  XcmV0Xcm: {
    _enum: {
      WithdrawAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        effects: 'Vec<XcmV0Order>',
      },
      ReserveAssetDeposit: {
        assets: 'Vec<XcmV0MultiAsset>',
        effects: 'Vec<XcmV0Order>',
      },
      TeleportAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        effects: 'Vec<XcmV0Order>',
      },
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV0Response',
      },
      TransferAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      Transact: {
        originType: 'XcmV0OriginKind',
        requireWeightAtMost: 'u64',
        call: 'XcmDoubleEncoded',
      },
      HrmpNewChannelOpenRequest: {
        sender: 'Compact<u32>',
        maxMessageSize: 'Compact<u32>',
        maxCapacity: 'Compact<u32>',
      },
      HrmpChannelAccepted: {
        recipient: 'Compact<u32>',
      },
      HrmpChannelClosing: {
        initiator: 'Compact<u32>',
        sender: 'Compact<u32>',
        recipient: 'Compact<u32>',
      },
      RelayedFrom: {
        who: 'XcmV0MultiLocation',
        message: 'XcmV0Xcm'
      }
    }
  },
  /**
   * Lookup310: xcm::v0::order::Order<Call>
   **/
  XcmV0Order: {
    _enum: {
      Null: 'Null',
      DepositAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      ExchangeAsset: {
        give: 'Vec<XcmV0MultiAsset>',
        receive: 'Vec<XcmV0MultiAsset>',
      },
      InitiateReserveWithdraw: {
        assets: 'Vec<XcmV0MultiAsset>',
        reserve: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      InitiateTeleport: {
        assets: 'Vec<XcmV0MultiAsset>',
        dest: 'XcmV0MultiLocation',
        effects: 'Vec<XcmV0Order>',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV0MultiLocation',
        assets: 'Vec<XcmV0MultiAsset>',
      },
      BuyExecution: {
        fees: 'XcmV0MultiAsset',
        weight: 'u64',
        debt: 'u64',
        haltOnError: 'bool',
        xcm: 'Vec<XcmV0Xcm>'
      }
    }
  },
  /**
   * Lookup312: xcm::v0::Response
   **/
  XcmV0Response: {
    _enum: {
      Assets: 'Vec<XcmV0MultiAsset>'
    }
  },
  /**
   * Lookup313: xcm::v1::Xcm<Call>
   **/
  XcmV1Xcm: {
    _enum: {
      WithdrawAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        effects: 'Vec<XcmV1Order>',
      },
      ReserveAssetDeposited: {
        assets: 'XcmV1MultiassetMultiAssets',
        effects: 'Vec<XcmV1Order>',
      },
      ReceiveTeleportedAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        effects: 'Vec<XcmV1Order>',
      },
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV1Response',
      },
      TransferAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        beneficiary: 'XcmV1MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssets',
        dest: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      Transact: {
        originType: 'XcmV0OriginKind',
        requireWeightAtMost: 'u64',
        call: 'XcmDoubleEncoded',
      },
      HrmpNewChannelOpenRequest: {
        sender: 'Compact<u32>',
        maxMessageSize: 'Compact<u32>',
        maxCapacity: 'Compact<u32>',
      },
      HrmpChannelAccepted: {
        recipient: 'Compact<u32>',
      },
      HrmpChannelClosing: {
        initiator: 'Compact<u32>',
        sender: 'Compact<u32>',
        recipient: 'Compact<u32>',
      },
      RelayedFrom: {
        who: 'XcmV1MultilocationJunctions',
        message: 'XcmV1Xcm',
      },
      SubscribeVersion: {
        queryId: 'Compact<u64>',
        maxResponseWeight: 'Compact<u64>',
      },
      UnsubscribeVersion: 'Null'
    }
  },
  /**
   * Lookup315: xcm::v1::order::Order<Call>
   **/
  XcmV1Order: {
    _enum: {
      Noop: 'Null',
      DepositAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'u32',
        beneficiary: 'XcmV1MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        maxAssets: 'u32',
        dest: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      ExchangeAsset: {
        give: 'XcmV1MultiassetMultiAssetFilter',
        receive: 'XcmV1MultiassetMultiAssets',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        reserve: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      InitiateTeleport: {
        assets: 'XcmV1MultiassetMultiAssetFilter',
        dest: 'XcmV1MultiLocation',
        effects: 'Vec<XcmV1Order>',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV1MultiLocation',
        assets: 'XcmV1MultiassetMultiAssetFilter',
      },
      BuyExecution: {
        fees: 'XcmV1MultiAsset',
        weight: 'u64',
        debt: 'u64',
        haltOnError: 'bool',
        instructions: 'Vec<XcmV1Xcm>'
      }
    }
  },
  /**
   * Lookup317: xcm::v1::Response
   **/
  XcmV1Response: {
    _enum: {
      Assets: 'XcmV1MultiassetMultiAssets',
      Version: 'u32'
    }
  },
  /**
   * Lookup331: cumulus_pallet_xcm::pallet::Call<T>
   **/
  CumulusPalletXcmCall: 'Null',
  /**
   * Lookup332: cumulus_pallet_dmp_queue::pallet::Call<T>
   **/
  CumulusPalletDmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'u64'
      }
    }
  },
  /**
   * Lookup333: orml_xtokens::module::Call<T>
   **/
  OrmlXtokensModuleCall: {
    _enum: {
      transfer: {
        currencyId: 'u128',
        amount: 'u128',
        dest: 'XcmVersionedMultiLocation',
        destWeight: 'u64',
      },
      transfer_multiasset: {
        asset: 'XcmVersionedMultiAsset',
        dest: 'XcmVersionedMultiLocation',
        destWeight: 'u64',
      },
      transfer_with_fee: {
        currencyId: 'u128',
        amount: 'u128',
        fee: 'u128',
        dest: 'XcmVersionedMultiLocation',
        destWeight: 'u64',
      },
      transfer_multiasset_with_fee: {
        asset: 'XcmVersionedMultiAsset',
        fee: 'XcmVersionedMultiAsset',
        dest: 'XcmVersionedMultiLocation',
        destWeight: 'u64',
      },
      transfer_multicurrencies: {
        currencies: 'Vec<(u128,u128)>',
        feeItem: 'u32',
        dest: 'XcmVersionedMultiLocation',
        destWeight: 'u64',
      },
      transfer_multiassets: {
        assets: 'XcmVersionedMultiAssets',
        feeItem: 'u32',
        dest: 'XcmVersionedMultiLocation',
        destWeight: 'u64'
      }
    }
  },
  /**
   * Lookup334: xcm::VersionedMultiAsset
   **/
  XcmVersionedMultiAsset: {
    _enum: {
      V0: 'XcmV0MultiAsset',
      V1: 'XcmV1MultiAsset'
    }
  },
  /**
   * Lookup337: orml_unknown_tokens::module::Call<T>
   **/
  OrmlUnknownTokensModuleCall: 'Null',
  /**
   * Lookup338: orml_tokens::module::Call<T>
   **/
  OrmlTokensModuleCall: {
    _enum: {
      transfer: {
        dest: 'MultiAddress',
        currencyId: 'u128',
        amount: 'Compact<u128>',
      },
      transfer_all: {
        dest: 'MultiAddress',
        currencyId: 'u128',
        keepAlive: 'bool',
      },
      transfer_keep_alive: {
        dest: 'MultiAddress',
        currencyId: 'u128',
        amount: 'Compact<u128>',
      },
      force_transfer: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        currencyId: 'u128',
        amount: 'Compact<u128>',
      },
      set_balance: {
        who: 'MultiAddress',
        currencyId: 'u128',
        newFree: 'Compact<u128>',
        newReserved: 'Compact<u128>'
      }
    }
  },
  /**
   * Lookup339: pallet_oracle::pallet::Call<T>
   **/
  PalletOracleCall: {
    _enum: {
      add_asset_and_info: {
        assetId: 'u128',
        threshold: 'Percent',
        minAnswers: 'u32',
        maxAnswers: 'u32',
        blockInterval: 'u32',
        rewardWeight: 'u128',
        slash: 'u128',
        emitPriceChanges: 'bool',
      },
      set_signer: {
        signer: 'AccountId32',
      },
      adjust_rewards: {
        annualCostPerOracle: 'u128',
        numIdealOracles: 'u8',
      },
      add_stake: {
        stake: 'u128',
      },
      remove_stake: 'Null',
      reclaim_stake: 'Null',
      submit_price: {
        price: 'u128',
        assetId: 'u128'
      }
    }
  },
  /**
   * Lookup340: pallet_currency_factory::pallet::Call<T>
   **/
  PalletCurrencyFactoryCall: {
    _enum: {
      add_range: {
        length: 'u64',
      },
      set_metadata: {
        assetId: 'u128',
        metadata: 'ComposableTraitsAssetsBasicAssetMetadata'
      }
    }
  },
  /**
   * Lookup341: composable_traits::assets::BasicAssetMetadata
   **/
  ComposableTraitsAssetsBasicAssetMetadata: {
    symbol: 'ComposableSupportCollectionsVecBoundedBiBoundedVec',
    name: 'ComposableSupportCollectionsVecBoundedBiBoundedVec'
  },
  /**
   * Lookup342: composable_support::collections::vec::bounded::bi_bounded_vec::BiBoundedVec<T>
   **/
  ComposableSupportCollectionsVecBoundedBiBoundedVec: {
    inner: 'Bytes'
  },
  /**
   * Lookup344: pallet_vault::pallet::Call<T>
   **/
  PalletVaultCall: {
    _enum: {
      create: {
        vault: 'ComposableTraitsVaultVaultConfig',
        depositAmount: 'u128',
      },
      claim_surcharge: {
        dest: 'u64',
        address: 'Option<AccountId32>',
      },
      add_surcharge: {
        dest: 'u64',
        amount: 'u128',
      },
      delete_tombstoned: {
        dest: 'u64',
        address: 'Option<AccountId32>',
      },
      deposit: {
        vault: 'u64',
        assetAmount: 'u128',
      },
      withdraw: {
        vault: 'u64',
        lpAmount: 'u128',
      },
      emergency_shutdown: {
        vault: 'u64',
      },
      start: {
        vault: 'u64',
      },
      liquidate_strategy: {
        vaultIdx: 'u64',
        strategyAccountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup345: composable_traits::vault::VaultConfig<sp_core::crypto::AccountId32, primitives::currency::CurrencyId>
   **/
  ComposableTraitsVaultVaultConfig: {
    assetId: 'u128',
    reserved: 'Perquintill',
    manager: 'AccountId32',
    strategies: 'BTreeMap<AccountId32, Perquintill>'
  },
  /**
   * Lookup350: pallet_assets_registry::pallet::Call<T>
   **/
  PalletAssetsRegistryCall: {
    _enum: {
      register_asset: {
        location: 'ComposableTraitsXcmAssetsXcmAssetLocation',
        ed: 'u128',
        ratio: 'Option<u128>',
        decimals: 'Option<u32>',
      },
      update_asset: {
        assetId: 'u128',
        location: 'ComposableTraitsXcmAssetsXcmAssetLocation',
        ratio: 'Option<u128>',
        decimals: 'Option<u32>',
      },
      set_min_fee: {
        targetParachainId: 'u32',
        foreignAssetId: 'ComposableTraitsXcmAssetsXcmAssetLocation',
        amount: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup352: pallet_governance_registry::pallet::Call<T>
   **/
  PalletGovernanceRegistryCall: {
    _enum: {
      set: {
        assetId: 'u128',
        value: 'AccountId32',
      },
      grant_root: {
        assetId: 'u128',
      },
      remove: {
        assetId: 'u128'
      }
    }
  },
  /**
   * Lookup353: pallet_assets::pallet::Call<T>
   **/
  PalletAssetsCall: {
    _enum: {
      transfer: {
        asset: 'u128',
        dest: 'MultiAddress',
        amount: 'Compact<u128>',
        keepAlive: 'bool',
      },
      transfer_native: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
        keepAlive: 'bool',
      },
      force_transfer: {
        asset: 'u128',
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'Compact<u128>',
        keepAlive: 'bool',
      },
      force_transfer_native: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'Compact<u128>',
        keepAlive: 'bool',
      },
      transfer_all: {
        asset: 'u128',
        dest: 'MultiAddress',
        keepAlive: 'bool',
      },
      transfer_all_native: {
        dest: 'MultiAddress',
        keepAlive: 'bool',
      },
      mint_initialize: {
        amount: 'Compact<u128>',
        dest: 'MultiAddress',
      },
      mint_initialize_with_governance: {
        amount: 'Compact<u128>',
        governanceOrigin: 'MultiAddress',
        dest: 'MultiAddress',
      },
      mint_into: {
        assetId: 'u128',
        dest: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      burn_from: {
        assetId: 'u128',
        dest: 'MultiAddress',
        amount: 'Compact<u128>'
      }
    }
  },
  /**
   * Lookup354: pallet_crowdloan_rewards::pallet::Call<T>
   **/
  PalletCrowdloanRewardsCall: {
    _enum: {
      initialize: 'Null',
      initialize_at: {
        at: 'u64',
      },
      populate: {
        rewards: 'Vec<(PalletCrowdloanRewardsModelsRemoteAccount,u128,u64)>',
      },
      associate: {
        rewardAccount: 'AccountId32',
        proof: 'PalletCrowdloanRewardsModelsProof',
      },
      claim: 'Null'
    }
  },
  /**
   * Lookup357: pallet_crowdloan_rewards::models::Proof<sp_core::crypto::AccountId32>
   **/
  PalletCrowdloanRewardsModelsProof: {
    _enum: {
      RelayChain: '(AccountId32,SpRuntimeMultiSignature)',
      Ethereum: 'ComposableSupportEcdsaSignature'
    }
  },
  /**
   * Lookup358: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup359: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup361: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup362: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup364: composable_support::types::EcdsaSignature
   **/
  ComposableSupportEcdsaSignature: '[u8;65]',
  /**
   * Lookup365: pallet_vesting::module::Call<T>
   **/
  PalletVestingModuleCall: {
    _enum: {
      claim: {
        asset: 'u128',
        vestingScheduleIds: 'ComposableTraitsVestingVestingScheduleIdSet',
      },
      vested_transfer: {
        from: 'MultiAddress',
        beneficiary: 'MultiAddress',
        asset: 'u128',
        scheduleInfo: 'ComposableTraitsVestingVestingScheduleInfo',
      },
      update_vesting_schedules: {
        who: 'MultiAddress',
        asset: 'u128',
        vestingSchedules: 'Vec<ComposableTraitsVestingVestingSchedule>',
      },
      claim_for: {
        dest: 'MultiAddress',
        asset: 'u128',
        vestingScheduleIds: 'ComposableTraitsVestingVestingScheduleIdSet'
      }
    }
  },
  /**
   * Lookup366: composable_traits::vesting::VestingScheduleInfo<BlockNumber, Moment, Balance>
   **/
  ComposableTraitsVestingVestingScheduleInfo: {
    window: 'ComposableTraitsVestingVestingWindow',
    periodCount: 'u32',
    perPeriod: 'Compact<u128>'
  },
  /**
   * Lookup368: pallet_bonded_finance::pallet::Call<T>
   **/
  PalletBondedFinanceCall: {
    _enum: {
      offer: {
        offer: 'ComposableTraitsBondedFinanceBondOffer',
        keepAlive: 'bool',
      },
      bond: {
        offerId: 'u128',
        nbOfBonds: 'u128',
        keepAlive: 'bool',
      },
      cancel: {
        offerId: 'u128'
      }
    }
  },
  /**
   * Lookup369: composable_traits::bonded_finance::BondOffer<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, Balance, BlockNumber>
   **/
  ComposableTraitsBondedFinanceBondOffer: {
    beneficiary: 'AccountId32',
    asset: 'u128',
    bondPrice: 'u128',
    nbOfBonds: 'u128',
    maturity: 'ComposableTraitsBondedFinanceBondDuration',
    reward: 'ComposableTraitsBondedFinanceBondOfferReward'
  },
  /**
   * Lookup370: composable_traits::bonded_finance::BondDuration<BlockNumber>
   **/
  ComposableTraitsBondedFinanceBondDuration: {
    _enum: {
      Finite: {
        returnIn: 'u32',
      },
      Infinite: 'Null'
    }
  },
  /**
   * Lookup371: composable_traits::bonded_finance::BondOfferReward<primitives::currency::CurrencyId, Balance, BlockNumber>
   **/
  ComposableTraitsBondedFinanceBondOfferReward: {
    asset: 'u128',
    amount: 'u128',
    maturity: 'u32'
  },
  /**
   * Lookup372: pallet_dutch_auction::pallet::Call<T>
   **/
  PalletDutchAuctionCall: {
    _enum: {
      add_configuration: {
        configurationId: 'u128',
        configuration: 'ComposableTraitsTimeTimeReleaseFunction',
      },
      ask: {
        order: 'ComposableTraitsDefiSellCurrencyId',
        configuration: 'ComposableTraitsTimeTimeReleaseFunction',
      },
      take: {
        orderId: 'u128',
        take: 'ComposableTraitsDefiTake',
      },
      liquidate: {
        orderId: 'u128',
      },
      xcm_sell: {
        request: 'ComposableTraitsXcmXcmSellRequest'
      }
    }
  },
  /**
   * Lookup373: composable_traits::xcm::XcmSellRequest
   **/
  ComposableTraitsXcmXcmSellRequest: {
    orderId: 'u64',
    fromTo: '[u8;32]',
    order: 'ComposableTraitsDefiSellU128',
    configuration: 'u128'
  },
  /**
   * Lookup374: composable_traits::defi::Sell<AssetId, Balance>
   **/
  ComposableTraitsDefiSellU128: {
    pair: 'ComposableTraitsDefiCurrencyPairU128',
    take: 'ComposableTraitsDefiTake'
  },
  /**
   * Lookup375: composable_traits::defi::CurrencyPair<AssetId>
   **/
  ComposableTraitsDefiCurrencyPairU128: {
    base: 'u128',
    quote: 'u128'
  },
  /**
   * Lookup376: pallet_mosaic::pallet::Call<T>
   **/
  PalletMosaicCall: {
    _enum: {
      set_relayer: {
        relayer: 'AccountId32',
      },
      rotate_relayer: {
        _alias: {
          new_: 'new',
        },
        new_: 'AccountId32',
        validatedTtl: 'u32',
      },
      set_network: {
        networkId: 'u32',
        networkInfo: 'PalletMosaicNetworkInfo',
      },
      set_budget: {
        assetId: 'u128',
        amount: 'u128',
        decay: 'PalletMosaicDecayBudgetPenaltyDecayer',
      },
      transfer_to: {
        networkId: 'u32',
        assetId: 'u128',
        address: 'ComposableSupportEthereumAddress',
        amount: 'u128',
        minimumAmountOut: 'u128',
        swapToNative: 'bool',
        sourceUserAccount: 'AccountId32',
        ammSwapInfo: 'Option<PalletMosaicAmmSwapInfo>',
        keepAlive: 'bool',
      },
      accept_transfer: {
        from: 'AccountId32',
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
        amount: 'u128',
      },
      claim_stale_to: {
        assetId: 'u128',
        to: 'AccountId32',
      },
      timelocked_mint: {
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
        to: 'AccountId32',
        amount: 'u128',
        lockTime: 'u32',
        id: 'H256',
      },
      set_timelock_duration: {
        period: 'u32',
      },
      rescind_timelocked_mint: {
        networkId: 'u32',
        remoteAssetId: 'CommonMosaicRemoteAssetId',
        account: 'AccountId32',
        untrustedAmount: 'u128',
      },
      claim_to: {
        assetId: 'u128',
        to: 'AccountId32',
      },
      update_asset_mapping: {
        assetId: 'u128',
        networkId: 'u32',
        remoteAssetId: 'Option<CommonMosaicRemoteAssetId>',
      },
      add_remote_amm_id: {
        networkId: 'u32',
        ammId: 'u128',
      },
      remove_remote_amm_id: {
        networkId: 'u32',
        ammId: 'u128'
      }
    }
  },
  /**
   * Lookup378: pallet_liquidations::pallet::Call<T>
   **/
  PalletLiquidationsCall: {
    _enum: {
      add_liquidation_strategy: {
        configuration: 'PalletLiquidationsLiquidationStrategyConfiguration',
      },
      sell: {
        order: 'ComposableTraitsDefiSellCurrencyId',
        configuration: 'Vec<u32>'
      }
    }
  },
  /**
   * Lookup379: pallet_liquidations::pallet::LiquidationStrategyConfiguration
   **/
  PalletLiquidationsLiquidationStrategyConfiguration: {
    _enum: {
      DutchAuction: 'ComposableTraitsTimeTimeReleaseFunction',
      Pablo: {
        slippage: 'Perquintill',
      },
      Xcm: 'ComposableTraitsXcmXcmSellRequestTransactConfiguration'
    }
  },
  /**
   * Lookup380: composable_traits::xcm::XcmSellRequestTransactConfiguration
   **/
  ComposableTraitsXcmXcmSellRequestTransactConfiguration: {
    location: 'ComposableTraitsXcmXcmTransactConfiguration',
    configurationId: 'u128',
    fee: 'u128'
  },
  /**
   * Lookup381: composable_traits::xcm::XcmTransactConfiguration
   **/
  ComposableTraitsXcmXcmTransactConfiguration: {
    parachainId: 'u32',
    methodId: 'ComposableTraitsXcmCumulusMethodId'
  },
  /**
   * Lookup382: composable_traits::xcm::CumulusMethodId
   **/
  ComposableTraitsXcmCumulusMethodId: {
    palletInstance: 'u8',
    methodId: 'u8'
  },
  /**
   * Lookup383: pallet_lending::pallet::Call<T>
   **/
  PalletLendingCall: {
    _enum: {
      create_market: {
        input: 'ComposableTraitsLendingCreateInput',
        keepAlive: 'bool',
      },
      update_market: {
        marketId: 'u32',
        input: 'ComposableTraitsLendingUpdateInput',
      },
      deposit_collateral: {
        marketId: 'u32',
        amount: 'u128',
        keepAlive: 'bool',
      },
      withdraw_collateral: {
        marketId: 'u32',
        amount: 'u128',
      },
      borrow: {
        marketId: 'u32',
        amountToBorrow: 'u128',
      },
      repay_borrow: {
        marketId: 'u32',
        beneficiary: 'AccountId32',
        amount: 'ComposableTraitsLendingRepayStrategy',
        keepAlive: 'bool',
      },
      liquidate: {
        marketId: 'u32',
        borrowers: 'Vec<AccountId32>'
      }
    }
  },
  /**
   * Lookup384: composable_traits::lending::CreateInput<LiquidationStrategyId, primitives::currency::CurrencyId, BlockNumber>
   **/
  ComposableTraitsLendingCreateInput: {
    updatable: 'ComposableTraitsLendingUpdateInput',
    currencyPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
    reservedFactor: 'Perquintill',
    interestRateModel: 'ComposableTraitsLendingMathInterestRateModel'
  },
  /**
   * Lookup385: composable_traits::lending::math::InterestRateModel
   **/
  ComposableTraitsLendingMathInterestRateModel: {
    _enum: {
      Jump: 'ComposableTraitsLendingMathJumpModel',
      Curve: 'ComposableTraitsLendingMathCurveModel',
      DynamicPIDController: 'ComposableTraitsLendingMathDynamicPIDControllerModel',
      DoubleExponent: 'ComposableTraitsLendingMathDoubleExponentModel'
    }
  },
  /**
   * Lookup386: composable_traits::lending::math::JumpModel
   **/
  ComposableTraitsLendingMathJumpModel: {
    baseRate: 'u128',
    jumpRate: 'u128',
    fullRate: 'u128',
    targetUtilization: 'Percent'
  },
  /**
   * Lookup387: composable_traits::lending::math::CurveModel
   **/
  ComposableTraitsLendingMathCurveModel: {
    baseRate: 'u128'
  },
  /**
   * Lookup388: composable_traits::lending::math::DynamicPIDControllerModel
   **/
  ComposableTraitsLendingMathDynamicPIDControllerModel: {
    proportionalParameter: 'i128',
    integralParameter: 'i128',
    derivativeParameter: 'i128',
    previousErrorValue: 'i128',
    previousIntegralTerm: 'i128',
    previousInterestRate: 'u128',
    targetUtilization: 'u128'
  },
  /**
   * Lookup391: composable_traits::lending::math::DoubleExponentModel
   **/
  ComposableTraitsLendingMathDoubleExponentModel: {
    coefficients: '[u8;16]'
  },
  /**
   * Lookup392: composable_traits::lending::RepayStrategy<T>
   **/
  ComposableTraitsLendingRepayStrategy: {
    _enum: {
      TotalDebt: 'Null',
      PartialAmount: 'u128'
    }
  },
  /**
   * Lookup394: pallet_pablo::pallet::Call<T>
   **/
  PalletPabloCall: {
    _enum: {
      create: {
        pool: 'PalletPabloPoolInitConfiguration',
      },
      buy: {
        poolId: 'u128',
        assetId: 'u128',
        amount: 'u128',
        minReceive: 'u128',
        keepAlive: 'bool',
      },
      sell: {
        poolId: 'u128',
        assetId: 'u128',
        amount: 'u128',
        minReceive: 'u128',
        keepAlive: 'bool',
      },
      swap: {
        poolId: 'u128',
        pair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        quoteAmount: 'u128',
        minReceive: 'u128',
        keepAlive: 'bool',
      },
      add_liquidity: {
        poolId: 'u128',
        baseAmount: 'u128',
        quoteAmount: 'u128',
        minMintAmount: 'u128',
        keepAlive: 'bool',
      },
      remove_liquidity: {
        poolId: 'u128',
        lpAmount: 'u128',
        minBaseAmount: 'u128',
        minQuoteAmount: 'u128',
      },
      enable_twap: {
        poolId: 'u128'
      }
    }
  },
  /**
   * Lookup395: pallet_pablo::pallet::PoolInitConfiguration<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, BlockNumber>
   **/
  PalletPabloPoolInitConfiguration: {
    _enum: {
      StableSwap: {
        owner: 'AccountId32',
        pair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        amplificationCoefficient: 'u16',
        fee: 'Permill',
      },
      ConstantProduct: {
        owner: 'AccountId32',
        pair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        fee: 'Permill',
        baseWeight: 'Permill',
      },
      LiquidityBootstrapping: 'ComposableTraitsDexLiquidityBootstrappingPoolInfo'
    }
  },
  /**
   * Lookup396: composable_traits::dex::LiquidityBootstrappingPoolInfo<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, BlockNumber>
   **/
  ComposableTraitsDexLiquidityBootstrappingPoolInfo: {
    owner: 'AccountId32',
    pair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
    sale: 'ComposableTraitsDexSale',
    feeConfig: 'ComposableTraitsDexFeeConfig'
  },
  /**
   * Lookup397: composable_traits::dex::Sale<BlockNumber>
   **/
  ComposableTraitsDexSale: {
    start: 'u32',
    end: 'u32',
    initialWeight: 'Permill',
    finalWeight: 'Permill'
  },
  /**
   * Lookup398: composable_traits::dex::FeeConfig
   **/
  ComposableTraitsDexFeeConfig: {
    feeRate: 'Permill',
    ownerFeeRate: 'Permill',
    protocolFeeRate: 'Permill'
  },
  /**
   * Lookup399: pallet_dex_router::pallet::Call<T>
   **/
  PalletDexRouterCall: {
    _enum: {
      update_route: {
        assetPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        route: 'Option<Vec<u128>>',
      },
      exchange: {
        assetPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        amount: 'u128',
        minReceive: 'u128',
      },
      sell: {
        assetPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        amount: 'u128',
        minReceive: 'u128',
      },
      buy: {
        assetPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        amount: 'u128',
        minReceive: 'u128',
      },
      add_liquidity: {
        assetPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        baseAmount: 'u128',
        quoteAmount: 'u128',
        minMintAmount: 'u128',
        keepAlive: 'bool',
      },
      remove_liquidity: {
        assetPair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
        lpAmount: 'u128',
        minBaseAmount: 'u128',
        minQuoteAmount: 'u128'
      }
    }
  },
  /**
   * Lookup402: pallet_staking_rewards::pallet::Call<T>
   **/
  PalletStakingRewardsCall: {
    _enum: {
      create_reward_pool: {
        poolConfig: 'ComposableTraitsStakingRewardPoolConfiguration',
      },
      stake: {
        poolId: 'u128',
        amount: 'u128',
        durationPreset: 'u64',
      },
      extend: {
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
        amount: 'u128',
      },
      unstake: {
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
      },
      split: {
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
        ratio: 'Permill',
      },
      update_rewards_pool: {
        poolId: 'u128',
        rewardUpdates: 'BTreeMap<u128, ComposableTraitsStakingRewardUpdate>',
      },
      claim: {
        fnftCollectionId: 'u128',
        fnftInstanceId: 'u64',
      },
      add_to_rewards_pot: {
        poolId: 'u128',
        assetId: 'u128',
        amount: 'u128',
        keepAlive: 'bool'
      }
    }
  },
  /**
   * Lookup403: composable_traits::staking::RewardPoolConfiguration<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, Balance, BlockNumber, MaxRewardConfigs, MaxDurationPresets>
   **/
  ComposableTraitsStakingRewardPoolConfiguration: {
    _enum: {
      RewardRateBasedIncentive: {
        owner: 'AccountId32',
        assetId: 'u128',
        startBlock: 'u32',
        endBlock: 'u32',
        rewardConfigs: 'BTreeMap<u128, ComposableTraitsStakingRewardConfig>',
        lock: 'ComposableTraitsStakingLockLockConfig',
        shareAssetId: 'u128',
        financialNftAssetId: 'u128',
        minimumStakingAmount: 'u128'
      }
    }
  },
  /**
   * Lookup405: composable_traits::staking::RewardConfig<Balance>
   **/
  ComposableTraitsStakingRewardConfig: {
    maxRewards: 'u128',
    rewardRate: 'ComposableTraitsStakingRewardRate'
  },
  /**
   * Lookup406: composable_traits::staking::RewardRate<Balance>
   **/
  ComposableTraitsStakingRewardRate: {
    period: 'ComposableTraitsStakingRewardRatePeriod',
    amount: 'u128'
  },
  /**
   * Lookup407: composable_traits::staking::RewardRatePeriod
   **/
  ComposableTraitsStakingRewardRatePeriod: {
    _enum: ['PerSecond']
  },
  /**
   * Lookup411: composable_traits::staking::lock::LockConfig<MaxDurationPresets>
   **/
  ComposableTraitsStakingLockLockConfig: {
    durationPresets: 'BTreeMap<u64, u64>',
    unlockPenalty: 'Perbill'
  },
  /**
   * Lookup417: composable_traits::staking::RewardUpdate<Balance>
   **/
  ComposableTraitsStakingRewardUpdate: {
    rewardRate: 'ComposableTraitsStakingRewardRate'
  },
  /**
   * Lookup421: pallet_call_filter::pallet::Call<T>
   **/
  PalletCallFilterCall: {
    _enum: {
      disable: {
        entry: 'ComposableTraitsCallFilterCallFilterEntry',
      },
      enable: {
        entry: 'ComposableTraitsCallFilterCallFilterEntry'
      }
    }
  },
  /**
   * Lookup422: pallet_ibc_ping::pallet::Call<T>
   **/
  PalletIbcPingCall: {
    _enum: {
      open_channel: {
        params: 'IbcTraitOpenChannelParams',
      },
      send_ping: {
        params: 'PalletIbcPingSendPingParams'
      }
    }
  },
  /**
   * Lookup423: ibc_trait::OpenChannelParams
   **/
  IbcTraitOpenChannelParams: {
    order: 'u8',
    connectionId: 'Bytes',
    counterpartyPortId: 'Bytes',
    version: 'Bytes'
  },
  /**
   * Lookup424: pallet_ibc_ping::SendPingParams
   **/
  PalletIbcPingSendPingParams: {
    data: 'Bytes',
    timeoutHeight: 'u64',
    timeoutTimestamp: 'u64',
    channelId: 'Bytes',
    destPortId: 'Bytes',
    destChannelId: 'Bytes'
  },
  /**
   * Lookup425: ibc_transfer::pallet::Call<T>
   **/
  IbcTransferCall: {
    _enum: {
      transfer: {
        params: 'IbcTransferTransferParams',
        assetId: 'u128',
        amount: 'u128',
      },
      open_channel: {
        params: 'IbcTraitOpenChannelParams',
      },
      set_pallet_params: {
        params: 'IbcTransferPalletParams'
      }
    }
  },
  /**
   * Lookup426: ibc_transfer::pallet::TransferParams
   **/
  IbcTransferTransferParams: {
    to: 'Bytes',
    sourceChannel: 'Bytes',
    timeoutTimestamp: 'u64',
    timeoutHeight: 'u64',
    revisionNumber: 'Option<u64>'
  },
  /**
   * Lookup427: ibc_transfer::pallet::PalletParams
   **/
  IbcTransferPalletParams: {
    sendEnabled: 'bool',
    receiveEnabled: 'bool'
  },
  /**
   * Lookup428: pallet_ibc::pallet::Call<T>
   **/
  PalletIbcCall: {
    _enum: {
      deliver: {
        messages: 'Vec<PalletIbcAny>',
      },
      create_client: {
        msg: 'PalletIbcAny',
      },
      initiate_connection: {
        params: 'PalletIbcConnectionParams'
      }
    }
  },
  /**
   * Lookup430: pallet_ibc::Any
   **/
  PalletIbcAny: {
    typeUrl: 'Bytes',
    value: 'Bytes'
  },
  /**
   * Lookup431: pallet_ibc::ConnectionParams
   **/
  PalletIbcConnectionParams: {
    version: '(Bytes,Vec<Bytes>)',
    clientId: 'Bytes',
    counterpartyClientId: 'Bytes',
    commitmentPrefix: 'Bytes',
    delayPeriod: 'u64'
  },
  /**
   * Lookup433: pallet_cosmwasm::pallet::Call<T>
   **/
  PalletCosmwasmCall: {
    _enum: {
      upload: {
        code: 'Bytes',
      },
      instantiate: {
        codeId: 'u64',
        salt: 'Bytes',
        admin: 'Option<AccountId32>',
        label: 'Bytes',
        funds: 'BTreeMap<u128, (u128,bool)>',
        gas: 'u64',
        message: 'Bytes',
      },
      execute: {
        contract: 'AccountId32',
        funds: 'BTreeMap<u128, (u128,bool)>',
        gas: 'u64',
        message: 'Bytes'
      }
    }
  },
  /**
   * Lookup442: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup444: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup446: pallet_indices::pallet::Error<T>
   **/
  PalletIndicesError: {
    _enum: ['NotAssigned', 'NotOwner', 'InUse', 'NotTransfer', 'Permanent']
  },
  /**
   * Lookup448: pallet_balances::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup449: pallet_balances::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup452: pallet_balances::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup454: pallet_balances::Releases
   **/
  PalletBalancesReleases: {
    _enum: ['V1_0_0', 'V2_0_0']
  },
  /**
   * Lookup455: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'KeepAlive', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup456: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
   **/
  PalletIdentityRegistration: {
    judgements: 'Vec<(u32,PalletIdentityJudgement)>',
    deposit: 'u128',
    info: 'PalletIdentityIdentityInfo'
  },
  /**
   * Lookup464: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
   **/
  PalletIdentityRegistrarInfo: {
    account: 'AccountId32',
    fee: 'u128',
    fields: 'PalletIdentityBitFlags'
  },
  /**
   * Lookup466: pallet_identity::pallet::Error<T>
   **/
  PalletIdentityError: {
    _enum: ['TooManySubAccounts', 'NotFound', 'NotNamed', 'EmptyIndex', 'FeeChanged', 'NoIdentity', 'StickyJudgement', 'JudgementGiven', 'InvalidJudgement', 'InvalidIndex', 'InvalidTarget', 'TooManyFields', 'TooManyRegistrars', 'AlreadyClaimed', 'NotSub', 'NotOwned']
  },
  /**
   * Lookup468: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32>
   **/
  PalletMultisigMultisig: {
    when: 'PalletMultisigTimepoint',
    deposit: 'u128',
    depositor: 'AccountId32',
    approvals: 'Vec<AccountId32>'
  },
  /**
   * Lookup470: pallet_multisig::pallet::Error<T>
   **/
  PalletMultisigError: {
    _enum: ['MinimumThreshold', 'AlreadyApproved', 'NoApprovalsNeeded', 'TooFewSignatories', 'TooManySignatories', 'SignatoriesOutOfOrder', 'SenderInSignatories', 'NotFound', 'NotOwner', 'NoTimepoint', 'WrongTimepoint', 'UnexpectedTimepoint', 'MaxWeightTooLow', 'AlreadyStored']
  },
  /**
   * Lookup472: polkadot_primitives::v2::UpgradeRestriction
   **/
  PolkadotPrimitivesV2UpgradeRestriction: {
    _enum: ['Present']
  },
  /**
   * Lookup473: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
   **/
  CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
    dmqMqcHead: 'H256',
    relayDispatchQueueSize: '(u32,u32)',
    ingressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>',
    egressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>'
  },
  /**
   * Lookup476: polkadot_primitives::v2::AbridgedHrmpChannel
   **/
  PolkadotPrimitivesV2AbridgedHrmpChannel: {
    maxCapacity: 'u32',
    maxTotalSize: 'u32',
    maxMessageSize: 'u32',
    msgCount: 'u32',
    totalSize: 'u32',
    mqcHead: 'Option<H256>'
  },
  /**
   * Lookup477: polkadot_primitives::v2::AbridgedHostConfiguration
   **/
  PolkadotPrimitivesV2AbridgedHostConfiguration: {
    maxCodeSize: 'u32',
    maxHeadDataSize: 'u32',
    maxUpwardQueueCount: 'u32',
    maxUpwardQueueSize: 'u32',
    maxUpwardMessageSize: 'u32',
    maxUpwardMessageNumPerCandidate: 'u32',
    hrmpMaxMessageNumPerCandidate: 'u32',
    validationUpgradeCooldown: 'u32',
    validationUpgradeDelay: 'u32'
  },
  /**
   * Lookup483: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
   **/
  PolkadotCorePrimitivesOutboundHrmpMessage: {
    recipient: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup484: cumulus_pallet_parachain_system::pallet::Error<T>
   **/
  CumulusPalletParachainSystemError: {
    _enum: ['OverlappingUpgrades', 'ProhibitedByPolkadot', 'TooBig', 'ValidationDataNotAvailable', 'HostConfigurationNotAvailable', 'NotScheduled', 'NothingAuthorized', 'Unauthorized']
  },
  /**
   * Lookup486: pallet_authorship::UncleEntryItem<BlockNumber, primitive_types::H256, sp_core::crypto::AccountId32>
   **/
  PalletAuthorshipUncleEntryItem: {
    _enum: {
      InclusionHeight: 'u32',
      Uncle: '(H256,Option<AccountId32>)'
    }
  },
  /**
   * Lookup488: pallet_authorship::pallet::Error<T>
   **/
  PalletAuthorshipError: {
    _enum: ['InvalidUncleParent', 'UnclesAlreadySet', 'TooManyUncles', 'GenesisUncle', 'TooHighUncle', 'UncleAlreadyIncluded', 'OldUncle']
  },
  /**
   * Lookup491: pallet_collator_selection::pallet::CandidateInfo<sp_core::crypto::AccountId32, Balance>
   **/
  PalletCollatorSelectionCandidateInfo: {
    who: 'AccountId32',
    deposit: 'u128'
  },
  /**
   * Lookup493: pallet_collator_selection::pallet::Error<T>
   **/
  PalletCollatorSelectionError: {
    _enum: ['TooManyCandidates', 'TooFewCandidates', 'Unknown', 'Permission', 'AlreadyCandidate', 'NotCandidate', 'TooManyInvulnerables', 'AlreadyInvulnerable', 'NoAssociatedValidatorId', 'ValidatorNotRegistered']
  },
  /**
   * Lookup497: sp_core::crypto::KeyTypeId
   **/
  SpCoreCryptoKeyTypeId: '[u8;4]',
  /**
   * Lookup498: pallet_session::pallet::Error<T>
   **/
  PalletSessionError: {
    _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount']
  },
  /**
   * Lookup503: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletCollectiveVotes: {
    index: 'u32',
    threshold: 'u32',
    ayes: 'Vec<AccountId32>',
    nays: 'Vec<AccountId32>',
    end: 'u32'
  },
  /**
   * Lookup504: pallet_collective::pallet::Error<T, I>
   **/
  PalletCollectiveError: {
    _enum: ['NotMember', 'DuplicateProposal', 'ProposalMissing', 'WrongIndex', 'DuplicateVote', 'AlreadyInitialized', 'TooEarly', 'TooManyProposals', 'WrongProposalWeight', 'WrongProposalLength']
  },
  /**
   * Lookup506: pallet_membership::pallet::Error<T, I>
   **/
  PalletMembershipError: {
    _enum: ['AlreadyMember', 'NotMember', 'TooManyMembers']
  },
  /**
   * Lookup507: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
   **/
  PalletTreasuryProposal: {
    proposer: 'AccountId32',
    value: 'u128',
    beneficiary: 'AccountId32',
    bond: 'u128'
  },
  /**
   * Lookup509: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup510: pallet_treasury::pallet::Error<T, I>
   **/
  PalletTreasuryError: {
    _enum: ['InsufficientProposersBalance', 'InvalidIndex', 'TooManyApprovals', 'InsufficientPermission', 'ProposalNotApproved']
  },
  /**
   * Lookup514: pallet_democracy::PreimageStatus<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  PalletDemocracyPreimageStatus: {
    _enum: {
      Missing: 'u32',
      Available: {
        data: 'Bytes',
        provider: 'AccountId32',
        deposit: 'u128',
        since: 'u32',
        expiry: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup515: pallet_democracy::types::ReferendumInfo<BlockNumber, primitive_types::H256, Balance>
   **/
  PalletDemocracyReferendumInfo: {
    _enum: {
      Ongoing: 'PalletDemocracyReferendumStatus',
      Finished: {
        approved: 'bool',
        end: 'u32'
      }
    }
  },
  /**
   * Lookup516: pallet_democracy::types::ReferendumStatus<BlockNumber, primitive_types::H256, Balance>
   **/
  PalletDemocracyReferendumStatus: {
    end: 'u32',
    proposalHash: 'H256',
    threshold: 'PalletDemocracyVoteThreshold',
    delay: 'u32',
    tally: 'PalletDemocracyTally'
  },
  /**
   * Lookup517: pallet_democracy::types::Tally<Balance>
   **/
  PalletDemocracyTally: {
    ayes: 'u128',
    nays: 'u128',
    turnout: 'u128'
  },
  /**
   * Lookup518: pallet_democracy::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletDemocracyVoteVoting: {
    _enum: {
      Direct: {
        votes: 'Vec<(u32,PalletDemocracyVoteAccountVote)>',
        delegations: 'PalletDemocracyDelegations',
        prior: 'PalletDemocracyVotePriorLock',
      },
      Delegating: {
        balance: 'u128',
        target: 'AccountId32',
        conviction: 'PalletDemocracyConviction',
        delegations: 'PalletDemocracyDelegations',
        prior: 'PalletDemocracyVotePriorLock'
      }
    }
  },
  /**
   * Lookup521: pallet_democracy::types::Delegations<Balance>
   **/
  PalletDemocracyDelegations: {
    votes: 'u128',
    capital: 'u128'
  },
  /**
   * Lookup522: pallet_democracy::vote::PriorLock<BlockNumber, Balance>
   **/
  PalletDemocracyVotePriorLock: '(u32,u128)',
  /**
   * Lookup525: pallet_democracy::Releases
   **/
  PalletDemocracyReleases: {
    _enum: ['V1']
  },
  /**
   * Lookup526: pallet_democracy::pallet::Error<T, I>
   **/
  PalletDemocracyError: {
    _enum: ['ValueLow', 'ProposalMissing', 'AlreadyCanceled', 'DuplicateProposal', 'ProposalBlacklisted', 'NotSimpleMajority', 'InvalidHash', 'NoProposal', 'AlreadyVetoed', 'DuplicatePreimage', 'NotImminent', 'TooEarly', 'Imminent', 'PreimageMissing', 'ReferendumInvalid', 'PreimageInvalid', 'NoneWaiting', 'NotVoter', 'NoPermission', 'AlreadyDelegating', 'InsufficientFunds', 'NotDelegating', 'VotesExist', 'InstantNotAllowed', 'Nonsense', 'WrongUpperBound', 'MaxVotesReached', 'TooManyProposals', 'VotingPeriodLow']
  },
  /**
   * Lookup531: pallet_scheduler::ScheduledV3<frame_support::traits::schedule::MaybeHashed<dali_runtime::Call, primitive_types::H256>, BlockNumber, dali_runtime::OriginCaller, sp_core::crypto::AccountId32>
   **/
  PalletSchedulerScheduledV3: {
    maybeId: 'Option<Bytes>',
    priority: 'u8',
    call: 'FrameSupportScheduleMaybeHashed',
    maybePeriodic: 'Option<(u32,u32)>',
    origin: 'DaliRuntimeOriginCaller'
  },
  /**
   * Lookup532: pallet_scheduler::pallet::Error<T>
   **/
  PalletSchedulerError: {
    _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange']
  },
  /**
   * Lookup533: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup534: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
   **/
  PalletPreimageRequestStatus: {
    _enum: {
      Unrequested: 'Option<(AccountId32,u128)>',
      Requested: 'u32'
    }
  },
  /**
   * Lookup538: pallet_preimage::pallet::Error<T>
   **/
  PalletPreimageError: {
    _enum: ['TooLarge', 'AlreadyNoted', 'NotAuthorized', 'NotNoted', 'Requested', 'NotRequested']
  },
  /**
   * Lookup541: composable_traits::account_proxy::ProxyDefinition<sp_core::crypto::AccountId32, composable_traits::account_proxy::ProxyType, BlockNumber>
   **/
  ComposableTraitsAccountProxyProxyDefinition: {
    delegate: 'AccountId32',
    proxyType: 'ComposableTraitsAccountProxyProxyType',
    delay: 'u32'
  },
  /**
   * Lookup545: pallet_account_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
   **/
  PalletAccountProxyAnnouncement: {
    real: 'AccountId32',
    callHash: 'H256',
    height: 'u32'
  },
  /**
   * Lookup547: pallet_account_proxy::pallet::Error<T>
   **/
  PalletAccountProxyError: {
    _enum: ['TooMany', 'NotFound', 'NotProxy', 'Unproxyable', 'Duplicate', 'NoPermission', 'Unannounced', 'NoSelfProxy']
  },
  /**
   * Lookup549: cumulus_pallet_xcmp_queue::InboundChannelDetails
   **/
  CumulusPalletXcmpQueueInboundChannelDetails: {
    sender: 'u32',
    state: 'CumulusPalletXcmpQueueInboundState',
    messageMetadata: 'Vec<(u32,PolkadotParachainPrimitivesXcmpMessageFormat)>'
  },
  /**
   * Lookup550: cumulus_pallet_xcmp_queue::InboundState
   **/
  CumulusPalletXcmpQueueInboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup553: polkadot_parachain::primitives::XcmpMessageFormat
   **/
  PolkadotParachainPrimitivesXcmpMessageFormat: {
    _enum: ['ConcatenatedVersionedXcm', 'ConcatenatedEncodedBlob', 'Signals']
  },
  /**
   * Lookup556: cumulus_pallet_xcmp_queue::OutboundChannelDetails
   **/
  CumulusPalletXcmpQueueOutboundChannelDetails: {
    recipient: 'u32',
    state: 'CumulusPalletXcmpQueueOutboundState',
    signalsExist: 'bool',
    firstIndex: 'u16',
    lastIndex: 'u16'
  },
  /**
   * Lookup557: cumulus_pallet_xcmp_queue::OutboundState
   **/
  CumulusPalletXcmpQueueOutboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup559: cumulus_pallet_xcmp_queue::QueueConfigData
   **/
  CumulusPalletXcmpQueueQueueConfigData: {
    suspendThreshold: 'u32',
    dropThreshold: 'u32',
    resumeThreshold: 'u32',
    thresholdWeight: 'u64',
    weightRestrictDecay: 'u64',
    xcmpMaxIndividualWeight: 'u64'
  },
  /**
   * Lookup561: cumulus_pallet_xcmp_queue::pallet::Error<T>
   **/
  CumulusPalletXcmpQueueError: {
    _enum: ['FailedToSend', 'BadXcmOrigin', 'BadXcm', 'BadOverweightIndex', 'WeightOverLimit']
  },
  /**
   * Lookup562: pallet_xcm::pallet::QueryStatus<BlockNumber>
   **/
  PalletXcmQueryStatus: {
    _enum: {
      Pending: {
        responder: 'XcmVersionedMultiLocation',
        maybeNotify: 'Option<(u8,u8)>',
        timeout: 'u32',
      },
      VersionNotifier: {
        origin: 'XcmVersionedMultiLocation',
        isActive: 'bool',
      },
      Ready: {
        response: 'XcmVersionedResponse',
        at: 'u32'
      }
    }
  },
  /**
   * Lookup565: xcm::VersionedResponse
   **/
  XcmVersionedResponse: {
    _enum: {
      V0: 'XcmV0Response',
      V1: 'XcmV1Response',
      V2: 'XcmV2Response'
    }
  },
  /**
   * Lookup571: pallet_xcm::pallet::VersionMigrationStage
   **/
  PalletXcmVersionMigrationStage: {
    _enum: {
      MigrateSupportedVersion: 'Null',
      MigrateVersionNotifiers: 'Null',
      NotifyCurrentTargets: 'Option<Bytes>',
      MigrateAndNotifyOldTargets: 'Null'
    }
  },
  /**
   * Lookup572: pallet_xcm::pallet::Error<T>
   **/
  PalletXcmError: {
    _enum: ['Unreachable', 'SendFailure', 'Filtered', 'UnweighableMessage', 'DestinationNotInvertible', 'Empty', 'CannotReanchor', 'TooManyAssets', 'InvalidOrigin', 'BadVersion', 'BadLocation', 'NoSubscription', 'AlreadySubscribed']
  },
  /**
   * Lookup573: cumulus_pallet_xcm::pallet::Error<T>
   **/
  CumulusPalletXcmError: 'Null',
  /**
   * Lookup574: cumulus_pallet_dmp_queue::ConfigData
   **/
  CumulusPalletDmpQueueConfigData: {
    maxIndividual: 'u64'
  },
  /**
   * Lookup575: cumulus_pallet_dmp_queue::PageIndexData
   **/
  CumulusPalletDmpQueuePageIndexData: {
    beginUsed: 'u32',
    endUsed: 'u32',
    overweightCount: 'u64'
  },
  /**
   * Lookup578: cumulus_pallet_dmp_queue::pallet::Error<T>
   **/
  CumulusPalletDmpQueueError: {
    _enum: ['Unknown', 'OverLimit']
  },
  /**
   * Lookup579: orml_xtokens::module::Error<T>
   **/
  OrmlXtokensModuleError: {
    _enum: ['AssetHasNoReserve', 'NotCrossChainTransfer', 'InvalidDest', 'NotCrossChainTransferableCurrency', 'UnweighableMessage', 'XcmExecutionFailed', 'CannotReanchor', 'InvalidAncestry', 'InvalidAsset', 'DestinationNotInvertible', 'BadVersion', 'DistinctReserveForAssetAndFee', 'ZeroFee', 'ZeroAmount', 'TooManyAssetsBeingSent', 'AssetIndexNonExistent', 'FeeNotEnough', 'NotSupportedMultiLocation', 'MinXcmFeeNotDefined']
  },
  /**
   * Lookup582: orml_unknown_tokens::module::Error<T>
   **/
  OrmlUnknownTokensModuleError: {
    _enum: ['BalanceTooLow', 'BalanceOverflow', 'UnhandledAsset']
  },
  /**
   * Lookup585: orml_tokens::BalanceLock<Balance>
   **/
  OrmlTokensBalanceLock: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup587: orml_tokens::AccountData<Balance>
   **/
  OrmlTokensAccountData: {
    free: 'u128',
    reserved: 'u128',
    frozen: 'u128'
  },
  /**
   * Lookup589: orml_tokens::ReserveData<ReserveIdentifier, Balance>
   **/
  OrmlTokensReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup591: orml_tokens::module::Error<T>
   **/
  OrmlTokensModuleError: {
    _enum: ['BalanceTooLow', 'AmountIntoBalanceFailed', 'LiquidityRestrictions', 'MaxLocksExceeded', 'KeepAlive', 'ExistentialDeposit', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup592: composable_traits::oracle::RewardTracker<Balance, Timestamp>
   **/
  ComposableTraitsOracleRewardTracker: {
    period: 'u64',
    start: 'u64',
    totalAlreadyRewarded: 'u128',
    currentBlockReward: 'u128',
    totalRewardWeight: 'u128'
  },
  /**
   * Lookup593: pallet_oracle::pallet::Withdraw<Balance, BlockNumber>
   **/
  PalletOracleWithdraw: {
    stake: 'u128',
    unlockBlock: 'u32'
  },
  /**
   * Lookup594: composable_traits::oracle::Price<PriceValue, BlockNumber>
   **/
  ComposableTraitsOraclePrice: {
    price: 'u128',
    block: 'u32'
  },
  /**
   * Lookup598: pallet_oracle::pallet::PrePrice<PriceValue, BlockNumber, sp_core::crypto::AccountId32>
   **/
  PalletOraclePrePrice: {
    price: 'u128',
    block: 'u32',
    who: 'AccountId32'
  },
  /**
   * Lookup600: pallet_oracle::pallet::AssetInfo<sp_arithmetic::per_things::Percent, BlockNumber, Balance>
   **/
  PalletOracleAssetInfo: {
    threshold: 'Percent',
    minAnswers: 'u32',
    maxAnswers: 'u32',
    blockInterval: 'u32',
    rewardWeight: 'u128',
    slash: 'u128',
    emitPriceChanges: 'bool'
  },
  /**
   * Lookup601: pallet_oracle::pallet::Error<T>
   **/
  PalletOracleError: {
    _enum: ['Unknown', 'NoPermission', 'NoStake', 'StakeLocked', 'NotEnoughStake', 'NotEnoughFunds', 'InvalidAssetId', 'AlreadySubmitted', 'MaxPrices', 'PriceNotRequested', 'UnsetSigner', 'AlreadySet', 'UnsetController', 'ControllerUsed', 'SignerUsed', 'AvoidPanic', 'ExceedMaxAnswers', 'InvalidMinAnswers', 'MaxAnswersLessThanMinAnswers', 'ExceedThreshold', 'ExceedAssetsCount', 'PriceNotFound', 'ExceedStake', 'MustSumTo100', 'DepthTooLarge', 'ArithmeticError', 'BlockIntervalLength', 'TransferError', 'MaxHistory', 'MaxPrePrices', 'NoRewardTrackerSet', 'AnnualRewardLessThanAlreadyRewarded']
  },
  /**
   * Lookup602: pallet_currency_factory::ranges::Ranges<primitives::currency::CurrencyId>
   **/
  PalletCurrencyFactoryRanges: {
    ranges: 'Vec<{"current":"u128","end":"u128"}>'
  },
  /**
   * Lookup605: pallet_currency_factory::pallet::Error<T>
   **/
  PalletCurrencyFactoryError: {
    _enum: ['AssetNotFound']
  },
  /**
   * Lookup606: pallet_vault::models::VaultInfo<sp_core::crypto::AccountId32, Balance, primitives::currency::CurrencyId, BlockNumber>
   **/
  PalletVaultModelsVaultInfo: {
    assetId: 'u128',
    lpTokenId: 'u128',
    manager: 'AccountId32',
    deposit: 'ComposableTraitsVaultDeposit',
    capabilities: 'PalletVaultCapabilities'
  },
  /**
   * Lookup607: composable_traits::vault::Deposit<Balance, BlockNumber>
   **/
  ComposableTraitsVaultDeposit: {
    _enum: {
      Existential: 'Null',
      Rent: {
        amount: 'u128',
        at: 'u32'
      }
    }
  },
  /**
   * Lookup608: pallet_vault::capabilities::Capabilities
   **/
  PalletVaultCapabilities: {
    bits: 'u32'
  },
  /**
   * Lookup610: pallet_vault::models::StrategyOverview<Balance>
   **/
  PalletVaultModelsStrategyOverview: {
    allocation: 'Perquintill',
    balance: 'u128',
    lifetimeWithdrawn: 'u128',
    lifetimeDeposited: 'u128'
  },
  /**
   * Lookup611: pallet_vault::pallet::Error<T>
   **/
  PalletVaultError: {
    _enum: ['AccountIsNotManager', 'CannotCreateAsset', 'TransferFromFailed', 'MintFailed', 'InsufficientLpTokens', 'VaultDoesNotExist', 'NoFreeVaultAllocation', 'AllocationMustSumToOne', 'TooManyStrategies', 'InsufficientFunds', 'AmountMustGteMinimumDeposit', 'AmountMustGteMinimumWithdrawal', 'NotEnoughLiquidity', 'InsufficientCreationDeposit', 'InvalidSurchargeClaim', 'NotVaultLpToken', 'DepositsHalted', 'WithdrawalsHalted', 'OnlyManagerCanDoThisOperation', 'InvalidDeletionClaim', 'VaultNotTombstoned', 'TombstoneDurationNotExceeded', 'InvalidAddSurcharge']
  },
  /**
   * Lookup612: composable_traits::xcm::assets::ForeignMetadata<composable_traits::xcm::assets::XcmAssetLocation>
   **/
  ComposableTraitsXcmAssetsForeignMetadata: {
    decimals: 'Option<u32>',
    location: 'ComposableTraitsXcmAssetsXcmAssetLocation'
  },
  /**
   * Lookup614: pallet_assets_registry::pallet::Error<T>
   **/
  PalletAssetsRegistryError: {
    _enum: ['AssetNotFound', 'ForeignAssetAlreadyRegistered']
  },
  /**
   * Lookup615: composable_traits::governance::SignedRawOrigin<sp_core::crypto::AccountId32>
   **/
  ComposableTraitsGovernanceSignedRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32'
    }
  },
  /**
   * Lookup616: pallet_governance_registry::pallet::Error<T>
   **/
  PalletGovernanceRegistryError: {
    _enum: ['NoneError']
  },
  /**
   * Lookup617: pallet_assets::pallet::Error<T>
   **/
  PalletAssetsError: {
    _enum: ['CannotSetNewCurrencyToRegistry', 'InvalidCurrency']
  },
  /**
   * Lookup618: pallet_crowdloan_rewards::models::Reward<Balance, Period>
   **/
  PalletCrowdloanRewardsModelsReward: {
    total: 'u128',
    claimed: 'u128',
    vestingPeriod: 'u64'
  },
  /**
   * Lookup619: pallet_crowdloan_rewards::pallet::Error<T>
   **/
  PalletCrowdloanRewardsError: {
    _enum: ['NotInitialized', 'AlreadyInitialized', 'BackToTheFuture', 'RewardsNotFunded', 'InvalidProof', 'InvalidClaim', 'NothingToClaim', 'NotAssociated', 'AlreadyAssociated', 'NotClaimableYet']
  },
  /**
   * Lookup624: pallet_vesting::module::Error<T>
   **/
  PalletVestingModuleError: {
    _enum: ['ZeroVestingPeriod', 'ZeroVestingPeriodCount', 'InsufficientBalanceToLock', 'TooManyVestingSchedules', 'AmountLow', 'MaxVestingSchedulesExceeded', 'TryingToSelfVest', 'VestingScheduleNotFound']
  },
  /**
   * Lookup626: pallet_bonded_finance::pallet::Error<T>
   **/
  PalletBondedFinanceError: {
    _enum: ['BondOfferNotFound', 'InvalidBondOffer', 'OfferCompleted', 'InvalidNumberOfBonds']
  },
  /**
   * Lookup629: pallet_dutch_auction::types::TakeOrder<Balance, sp_core::crypto::AccountId32>
   **/
  PalletDutchAuctionTakeOrder: {
    fromTo: 'AccountId32',
    take: 'ComposableTraitsDefiTake'
  },
  /**
   * Lookup630: pallet_dutch_auction::pallet::Error<T>
   **/
  PalletDutchAuctionError: {
    _enum: ['RequestedOrderDoesNotExists', 'OrderParametersIsInvalid', 'TakeParametersIsInvalid', 'TakeLimitDoesNotSatisfyOrder', 'OrderNotFound', 'TakeOrderDidNotHappen', 'NotEnoughNativeCurrencyToPayForAuction', 'XcmCannotDecodeRemoteParametersToLocalRepresentations', 'XcmCannotFindLocalIdentifiersAsDecodedFromRemote', 'XcmNotFoundConfigurationById']
  },
  /**
   * Lookup631: pallet_mosaic::relayer::StaleRelayer<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletMosaicRelayerStaleRelayer: {
    relayer: 'PalletMosaicRelayerRelayerConfig'
  },
  /**
   * Lookup632: pallet_mosaic::relayer::RelayerConfig<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletMosaicRelayerRelayerConfig: {
    current: 'AccountId32',
    next: 'Option<PalletMosaicRelayerNext>'
  },
  /**
   * Lookup634: pallet_mosaic::relayer::Next<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletMosaicRelayerNext: {
    ttl: 'u32',
    account: 'AccountId32'
  },
  /**
   * Lookup635: pallet_mosaic::pallet::AssetInfo<BlockNumber, Balance, pallet_mosaic::decay::BudgetPenaltyDecayer<Balance, BlockNumber>>
   **/
  PalletMosaicAssetInfo: {
    lastMintBlock: 'u32',
    budget: 'u128',
    penalty: 'u128',
    penaltyDecayer: 'PalletMosaicDecayBudgetPenaltyDecayer'
  },
  /**
   * Lookup640: pallet_mosaic::pallet::Error<T>
   **/
  PalletMosaicError: {
    _enum: ['RelayerNotSet', 'BadTTL', 'BadTimelockPeriod', 'UnsupportedAsset', 'NetworkDisabled', 'UnsupportedNetwork', 'Overflow', 'NoStaleTransactions', 'InsufficientBudget', 'ExceedsMaxTransferSize', 'BelowMinTransferSize', 'NoClaimableTx', 'TxStillLocked', 'NoOutgoingTx', 'AmountMismatch', 'AssetNotMapped', 'RemoteAmmIdNotFound', 'RemoteAmmIdAlreadyExists', 'DestinationAmmIdNotWhitelisted']
  },
  /**
   * Lookup641: pallet_liquidations::pallet::Error<T>
   **/
  PalletLiquidationsError: {
    _enum: ['NoLiquidationEngineFound', 'InvalidLiquidationStrategiesVector', 'OnlyDutchAuctionStrategyIsImplemented']
  },
  /**
   * Lookup642: composable_traits::lending::MarketConfig<VaultId, primitives::currency::CurrencyId, sp_core::crypto::AccountId32, LiquidationStrategyId, BlockNumber>
   **/
  ComposableTraitsLendingMarketConfig: {
    manager: 'AccountId32',
    borrowAssetVault: 'u64',
    collateralAsset: 'u128',
    maxPriceAge: 'u32',
    collateralFactor: 'u128',
    interestRateModel: 'ComposableTraitsLendingMathInterestRateModel',
    underCollateralizedWarnPercent: 'Percent',
    liquidators: 'Vec<u32>'
  },
  /**
   * Lookup644: pallet_lending::pallet::Error<T>
   **/
  PalletLendingError: {
    _enum: ['MarketDoesNotExist', 'AccountCollateralAbsent', 'InvalidCollateralFactor', 'MarketIsClosing', 'InvalidTimestampOnBorrowRequest', 'NotEnoughCollateralToWithdraw', 'WouldGoUnderCollateralized', 'NotEnoughCollateralToBorrow', 'CannotCalculateBorrowRate', 'BorrowAndRepayInSameBlockIsNotSupported', 'BorrowDoesNotExist', 'ExceedLendingCount', 'BorrowLimitCalculationFailed', 'Unauthorized', 'InitialMarketVolumeIncorrect', 'CannotRepayZeroBalance', 'CannotRepayMoreThanTotalDebt', 'BorrowRentDoesNotExist', 'PriceTooOld', 'CannotIncreaseCollateralFactorOfOpenMarket', 'CannotBorrowFromMarketWithUnbalancedVault']
  },
  /**
   * Lookup645: pallet_pablo::pallet::PoolConfiguration<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, BlockNumber>
   **/
  PalletPabloPoolConfiguration: {
    _enum: {
      StableSwap: 'ComposableTraitsDexStableSwapPoolInfo',
      ConstantProduct: 'ComposableTraitsDexConstantProductPoolInfo',
      LiquidityBootstrapping: 'ComposableTraitsDexLiquidityBootstrappingPoolInfo'
    }
  },
  /**
   * Lookup646: composable_traits::dex::StableSwapPoolInfo<sp_core::crypto::AccountId32, primitives::currency::CurrencyId>
   **/
  ComposableTraitsDexStableSwapPoolInfo: {
    owner: 'AccountId32',
    pair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
    lpToken: 'u128',
    amplificationCoefficient: 'u16',
    feeConfig: 'ComposableTraitsDexFeeConfig'
  },
  /**
   * Lookup647: composable_traits::dex::ConstantProductPoolInfo<sp_core::crypto::AccountId32, primitives::currency::CurrencyId>
   **/
  ComposableTraitsDexConstantProductPoolInfo: {
    owner: 'AccountId32',
    pair: 'ComposableTraitsDefiCurrencyPairCurrencyId',
    lpToken: 'u128',
    feeConfig: 'ComposableTraitsDexFeeConfig',
    baseWeight: 'Permill',
    quoteWeight: 'Permill'
  },
  /**
   * Lookup648: pallet_pablo::types::TimeWeightedAveragePrice<Timestamp, Balance>
   **/
  PalletPabloTimeWeightedAveragePrice: {
    timestamp: 'u64',
    basePriceCumulative: 'u128',
    quotePriceCumulative: 'u128',
    baseTwap: 'u128',
    quoteTwap: 'u128'
  },
  /**
   * Lookup649: pallet_pablo::types::PriceCumulative<Timestamp, Balance>
   **/
  PalletPabloPriceCumulative: {
    timestamp: 'u64',
    basePriceCumulative: 'u128',
    quotePriceCumulative: 'u128'
  },
  /**
   * Lookup650: pallet_pablo::pallet::Error<T>
   **/
  PalletPabloError: {
    _enum: ['PoolNotFound', 'NotEnoughLiquidity', 'NotEnoughLpToken', 'PairMismatch', 'MustBeOwner', 'InvalidSaleState', 'InvalidAmount', 'InvalidAsset', 'CannotRespectMinimumRequested', 'AssetAmountMustBePositiveNumber', 'InvalidPair', 'InvalidFees', 'AmpFactorMustBeGreaterThanZero', 'MissingAmount', 'MissingMinExpectedAmount', 'MoreThanTwoAssetsNotYetSupported', 'NoLpTokenForLbp', 'NoXTokenForLbp', 'WeightsMustBeNonZero', 'WeightsMustSumToOne', 'StakingPoolConfigError']
  },
  /**
   * Lookup652: composable_traits::dex::DexRoute<PoolId, dali_runtime::MaxHopsCount>
   **/
  ComposableTraitsDexDexRoute: {
    _enum: {
      Direct: 'Vec<u128>'
    }
  },
  /**
   * Lookup653: dali_runtime::MaxHopsCount
   **/
  DaliRuntimeMaxHopsCount: 'Null',
  /**
   * Lookup654: pallet_dex_router::pallet::Error<T>
   **/
  PalletDexRouterError: {
    _enum: ['MaxHopsExceeded', 'NoRouteFound', 'UnexpectedNodeFoundWhileValidation', 'CanNotRespectMinAmountRequested', 'UnsupportedOperation', 'LoopSuspectedInRouteUpdate']
  },
  /**
   * Lookup661: pallet_fnft::pallet::Error<T>
   **/
  PalletFnftError: {
    _enum: ['CollectionAlreadyExists', 'InstanceAlreadyExists', 'CollectionNotFound', 'InstanceNotFound', 'MustBeOwner']
  },
  /**
   * Lookup662: composable_traits::staking::RewardPool<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, Balance, BlockNumber, MaxDurationPresets, MaxRewards>
   **/
  ComposableTraitsStakingRewardPool: {
    owner: 'AccountId32',
    assetId: 'u128',
    rewards: 'BTreeMap<u128, ComposableTraitsStakingReward>',
    totalShares: 'u128',
    claimedShares: 'u128',
    startBlock: 'u32',
    endBlock: 'u32',
    lock: 'ComposableTraitsStakingLockLockConfig',
    shareAssetId: 'u128',
    financialNftAssetId: 'u128',
    minimumStakingAmount: 'u128'
  },
  /**
   * Lookup664: composable_traits::staking::Reward<Balance>
   **/
  ComposableTraitsStakingReward: {
    totalRewards: 'u128',
    claimedRewards: 'u128',
    totalDilutionAdjustment: 'u128',
    maxRewards: 'u128',
    rewardRate: 'ComposableTraitsStakingRewardRate',
    lastUpdatedTimestamp: 'u64'
  },
  /**
   * Lookup668: composable_traits::staking::Stake<primitives::currency::CurrencyId, ItemId, primitives::currency::CurrencyId, Balance, MaxReductions>
   **/
  ComposableTraitsStakingStake: {
    fnftInstanceId: 'u64',
    rewardPoolId: 'u128',
    stake: 'u128',
    share: 'u128',
    reductions: 'BTreeMap<u128, u128>',
    lock: 'ComposableTraitsStakingLock'
  },
  /**
   * Lookup671: composable_traits::staking::lock::Lock
   **/
  ComposableTraitsStakingLock: {
    startedAt: 'u64',
    duration: 'u64',
    unlockPenalty: 'Perbill'
  },
  /**
   * Lookup672: pallet_staking_rewards::pallet::Error<T>
   **/
  PalletStakingRewardsError: {
    _enum: ['RewardConfigProblem', 'InvalidAssetId', 'RewardsPoolAlreadyExists', 'NoDurationPresetsConfigured', 'TooManyRewardAssetTypes', 'StartBlockMustBeAfterCurrentBlock', 'EndBlockMustBeAfterStartBlock', 'UnimplementedRewardPoolConfiguration', 'RewardsPoolNotFound', 'RewardsPoolHasNotStarted', 'ReductionConfigProblem', 'NotEnoughAssets', 'StakeNotFound', 'MaxRewardLimitReached', 'OnlyStakeOwnerCanInteractWithStake', 'RewardAssetNotFound', 'BackToTheFuture', 'RewardsPotEmpty', 'FnftNotFound', 'NoDurationPresetsProvided', 'SlashedAmountTooLow', 'SlashedMinimumStakingAmountTooLow', 'StakedAmountTooLow', 'StakedAmountTooLowAfterSplit']
  },
  /**
   * Lookup673: pallet_call_filter::pallet::Error<T>
   **/
  PalletCallFilterError: {
    _enum: ['CannotDisable', 'InvalidString']
  },
  /**
   * Lookup674: pallet_ibc_ping::pallet::Error<T>
   **/
  PalletIbcPingError: {
    _enum: ['InvalidParams', 'ChannelInitError', 'PacketSendError']
  },
  /**
   * Lookup675: ibc_transfer::pallet::Error<T>
   **/
  IbcTransferError: {
    _enum: ['TransferFailed', 'Utf8Error', 'InvalidAssetId', 'InvalidIbcDenom', 'InvalidAmount', 'InvalidTimestamp', 'FailedToGetRevisionNumber', 'InvalidParams', 'ChannelInitError']
  },
  /**
   * Lookup677: pallet_ibc::IbcConsensusState
   **/
  PalletIbcIbcConsensusState: {
    timestamp: 'u64',
    commitmentRoot: 'Bytes'
  },
  /**
   * Lookup681: pallet_ibc::pallet::Error<T>
   **/
  PalletIbcError: {
    _enum: ['ProcessingError', 'DecodingError', 'EncodingError', 'ProofGenerationError', 'ConsensusStateNotFound', 'ChannelNotFound', 'ClientStateNotFound', 'ConnectionNotFound', 'PacketCommitmentNotFound', 'PacketReceiptNotFound', 'PacketAcknowledgmentNotFound', 'SendPacketError', 'Other', 'InvalidRoute', 'InvalidMessageType']
  },
  /**
   * Lookup683: pallet_cosmwasm::types::CodeInfo<sp_core::crypto::AccountId32, primitive_types::H256>
   **/
  PalletCosmwasmCodeInfo: {
    creator: 'AccountId32',
    pristineCodeHash: 'H256',
    instrumentationVersion: 'u16',
    refcount: 'u32'
  },
  /**
   * Lookup684: pallet_cosmwasm::pallet::Error<T>
   **/
  PalletCosmwasmError: {
    _enum: ['Instrumentation', 'VmCreation', 'ContractTrapped', 'ContractHasNoInfo', 'CodeDecoding', 'CodeValidation', 'CodeEncoding', 'CodeInstrumentation', 'InstrumentedCodeIsTooBig', 'CodeAlreadyExists', 'CodeNotFound', 'ContractAlreadyExists', 'ContractNotFound', 'TransferFailed', 'ChargeGas', 'RefundGas', 'LabelTooBig', 'UnknownDenom', 'StackOverflow', 'NotEnoughFundsForUpload', 'NonceOverflow', 'RefcountOverflow', 'VMDepthOverflow', 'SignatureVerificationError', 'IteratorIdOverflow', 'IteratorNotFound']
  },
  /**
   * Lookup687: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup688: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup689: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup690: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup693: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup694: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup695: pallet_asset_tx_payment::ChargeAssetTxPayment<T>
   **/
  PalletAssetTxPaymentChargeAssetTxPayment: {
    tip: 'Compact<u128>',
    assetId: 'Option<u128>'
  },
  /**
   * Lookup696: dali_runtime::Runtime
   **/
  DaliRuntimeRuntime: 'Null'
};
