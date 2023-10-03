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
   * Lookup7: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
   **/
  FrameSupportDispatchPerDispatchClassWeight: {
    normal: 'SpWeightsWeightV2Weight',
    operational: 'SpWeightsWeightV2Weight',
    mandatory: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup8: sp_weights::weight_v2::Weight
   **/
  SpWeightsWeightV2Weight: {
    refTime: 'Compact<u64>',
    proofSize: 'Compact<u64>'
  },
  /**
   * Lookup13: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup15: sp_runtime::generic::digest::DigestItem
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
   * Lookup18: frame_system::EventRecord<picasso_runtime::RuntimeEvent, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup20: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
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
   * Lookup21: frame_support::dispatch::DispatchInfo
   **/
  FrameSupportDispatchDispatchInfo: {
    weight: 'SpWeightsWeightV2Weight',
    class: 'FrameSupportDispatchDispatchClass',
    paysFee: 'FrameSupportDispatchPays'
  },
  /**
   * Lookup22: frame_support::dispatch::DispatchClass
   **/
  FrameSupportDispatchDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup23: frame_support::dispatch::Pays
   **/
  FrameSupportDispatchPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup24: sp_runtime::DispatchError
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
      Arithmetic: 'SpArithmeticArithmeticError',
      Transactional: 'SpRuntimeTransactionalError',
      Exhausted: 'Null',
      Corruption: 'Null',
      Unavailable: 'Null'
    }
  },
  /**
   * Lookup25: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup26: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported']
  },
  /**
   * Lookup27: sp_arithmetic::ArithmeticError
   **/
  SpArithmeticArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup28: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup29: pallet_sudo::pallet::Event<T>
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
   * Lookup33: pallet_transaction_payment::pallet::Event<T>
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
   * Lookup34: pallet_indices::pallet::Event<T>
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
   * Lookup35: pallet_balances::pallet::Event<T, I>
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
   * Lookup36: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup37: pallet_identity::pallet::Event<T>
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
   * Lookup38: pallet_multisig::pallet::Event<T>
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
   * Lookup39: pallet_multisig::Timepoint<BlockNumber>
   **/
  PalletMultisigTimepoint: {
    height: 'u32',
    index: 'u32'
  },
  /**
   * Lookup40: cumulus_pallet_parachain_system::pallet::Event<T>
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
        weightUsed: 'SpWeightsWeightV2Weight',
        dmqHead: 'H256',
      },
      UpwardMessageSent: {
        messageHash: 'Option<[u8;32]>'
      }
    }
  },
  /**
   * Lookup42: pallet_collator_selection::pallet::Event<T>
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
   * Lookup44: pallet_session::pallet::Event
   **/
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: 'u32'
      }
    }
  },
  /**
   * Lookup45: pallet_collective::pallet::Event<T, I>
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
   * Lookup47: pallet_membership::pallet::Event<T, I>
   **/
  PalletMembershipEvent: {
    _enum: ['MemberAdded', 'MemberRemoved', 'MembersSwapped', 'MembersReset', 'KeyChanged', 'Dummy']
  },
  /**
   * Lookup48: pallet_treasury::pallet::Event<T, I>
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
        beneficiary: 'AccountId32',
      },
      UpdatedInactive: {
        reactivated: 'u128',
        deactivated: 'u128'
      }
    }
  },
  /**
   * Lookup49: pallet_democracy::pallet::Event<T>
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
        propIndex: 'u32',
      },
      MetadataSet: {
        _alias: {
          hash_: 'hash',
        },
        owner: 'PalletDemocracyMetadataOwner',
        hash_: 'H256',
      },
      MetadataCleared: {
        _alias: {
          hash_: 'hash',
        },
        owner: 'PalletDemocracyMetadataOwner',
        hash_: 'H256',
      },
      MetadataTransferred: {
        _alias: {
          hash_: 'hash',
        },
        prevOwner: 'PalletDemocracyMetadataOwner',
        owner: 'PalletDemocracyMetadataOwner',
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup50: pallet_democracy::vote_threshold::VoteThreshold
   **/
  PalletDemocracyVoteThreshold: {
    _enum: ['SuperMajorityApprove', 'SuperMajorityAgainst', 'SimpleMajority']
  },
  /**
   * Lookup51: pallet_democracy::vote::AccountVote<Balance>
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
   * Lookup53: pallet_democracy::types::MetadataOwner
   **/
  PalletDemocracyMetadataOwner: {
    _enum: {
      External: 'Null',
      Proposal: 'u32',
      Referendum: 'u32'
    }
  },
  /**
   * Lookup58: pallet_scheduler::pallet::Event<T>
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
        id: 'Option<[u8;32]>',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      CallUnavailable: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>',
      },
      PeriodicFailed: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>',
      },
      PermanentlyOverweight: {
        task: '(u32,u32)',
        id: 'Option<[u8;32]>'
      }
    }
  },
  /**
   * Lookup60: pallet_utility::pallet::Event
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
   * Lookup61: pallet_preimage::pallet::Event<T>
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
   * Lookup62: pallet_proxy::pallet::Event<T>
   **/
  PalletProxyEvent: {
    _enum: {
      ProxyExecuted: {
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      PureCreated: {
        pure: 'AccountId32',
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
   * Lookup63: composable_traits::account_proxy::ProxyType
   **/
  ComposableTraitsAccountProxyProxyType: {
    _enum: ['Any', 'Governance', 'CancelProxy', 'Bridge', 'Assets', 'Defi', 'Oracle', 'Contracts']
  },
  /**
   * Lookup65: cumulus_pallet_xcmp_queue::pallet::Event<T>
   **/
  CumulusPalletXcmpQueueEvent: {
    _enum: {
      Success: {
        messageHash: 'Option<[u8;32]>',
        weight: 'SpWeightsWeightV2Weight',
      },
      Fail: {
        messageHash: 'Option<[u8;32]>',
        error: 'XcmV3TraitsError',
        weight: 'SpWeightsWeightV2Weight',
      },
      BadVersion: {
        messageHash: 'Option<[u8;32]>',
      },
      BadFormat: {
        messageHash: 'Option<[u8;32]>',
      },
      XcmpMessageSent: {
        messageHash: 'Option<[u8;32]>',
      },
      OverweightEnqueued: {
        sender: 'u32',
        sentAt: 'u32',
        index: 'u64',
        required: 'SpWeightsWeightV2Weight',
      },
      OverweightServiced: {
        index: 'u64',
        used: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup66: xcm::v3::traits::Error
   **/
  XcmV3TraitsError: {
    _enum: {
      Overflow: 'Null',
      Unimplemented: 'Null',
      UntrustedReserveLocation: 'Null',
      UntrustedTeleportLocation: 'Null',
      LocationFull: 'Null',
      LocationNotInvertible: 'Null',
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
      ExpectationFalse: 'Null',
      PalletNotFound: 'Null',
      NameMismatch: 'Null',
      VersionIncompatible: 'Null',
      HoldingWouldOverflow: 'Null',
      ExportError: 'Null',
      ReanchorFailed: 'Null',
      NoDeal: 'Null',
      FeesNotMet: 'Null',
      LockError: 'Null',
      NoPermission: 'Null',
      Unanchored: 'Null',
      NotDepositable: 'Null',
      UnhandledXcmVersion: 'Null',
      WeightLimitReached: 'SpWeightsWeightV2Weight',
      Barrier: 'Null',
      WeightNotComputable: 'Null',
      ExceedsStackLimit: 'Null'
    }
  },
  /**
   * Lookup68: pallet_xcm::pallet::Event<T>
   **/
  PalletXcmEvent: {
    _enum: {
      Attempted: 'XcmV3TraitsOutcome',
      Sent: '(XcmV3MultiLocation,XcmV3MultiLocation,XcmV3Xcm)',
      UnexpectedResponse: '(XcmV3MultiLocation,u64)',
      ResponseReady: '(u64,XcmV3Response)',
      Notified: '(u64,u8,u8)',
      NotifyOverweight: '(u64,u8,u8,SpWeightsWeightV2Weight,SpWeightsWeightV2Weight)',
      NotifyDispatchError: '(u64,u8,u8)',
      NotifyDecodeFailed: '(u64,u8,u8)',
      InvalidResponder: '(XcmV3MultiLocation,u64,Option<XcmV3MultiLocation>)',
      InvalidResponderVersion: '(XcmV3MultiLocation,u64)',
      ResponseTaken: 'u64',
      AssetsTrapped: '(H256,XcmV3MultiLocation,XcmVersionedMultiAssets)',
      VersionChangeNotified: '(XcmV3MultiLocation,u32,XcmV3MultiassetMultiAssets)',
      SupportedVersionChanged: '(XcmV3MultiLocation,u32)',
      NotifyTargetSendFail: '(XcmV3MultiLocation,u64,XcmV3TraitsError)',
      NotifyTargetMigrationFail: '(XcmVersionedMultiLocation,u64)',
      InvalidQuerierVersion: '(XcmV3MultiLocation,u64)',
      InvalidQuerier: '(XcmV3MultiLocation,u64,XcmV3MultiLocation,Option<XcmV3MultiLocation>)',
      VersionNotifyStarted: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
      VersionNotifyRequested: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
      VersionNotifyUnrequested: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
      FeesPaid: '(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)',
      AssetsClaimed: '(H256,XcmV3MultiLocation,XcmVersionedMultiAssets)'
    }
  },
  /**
   * Lookup69: xcm::v3::traits::Outcome
   **/
  XcmV3TraitsOutcome: {
    _enum: {
      Complete: 'SpWeightsWeightV2Weight',
      Incomplete: '(SpWeightsWeightV2Weight,XcmV3TraitsError)',
      Error: 'XcmV3TraitsError'
    }
  },
  /**
   * Lookup70: xcm::v3::multilocation::MultiLocation
   **/
  XcmV3MultiLocation: {
    parents: 'u8',
    interior: 'XcmV3Junctions'
  },
  /**
   * Lookup71: xcm::v3::junctions::Junctions
   **/
  XcmV3Junctions: {
    _enum: {
      Here: 'Null',
      X1: 'XcmV3Junction',
      X2: '(XcmV3Junction,XcmV3Junction)',
      X3: '(XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X4: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X5: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X6: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X7: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)',
      X8: '(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)'
    }
  },
  /**
   * Lookup72: xcm::v3::junction::Junction
   **/
  XcmV3Junction: {
    _enum: {
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'Option<XcmV3JunctionNetworkId>',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'Option<XcmV3JunctionNetworkId>',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'Option<XcmV3JunctionNetworkId>',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: {
        length: 'u8',
        data: '[u8;32]',
      },
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV3JunctionBodyId',
        part: 'XcmV3JunctionBodyPart',
      },
      GlobalConsensus: 'XcmV3JunctionNetworkId'
    }
  },
  /**
   * Lookup75: xcm::v3::junction::NetworkId
   **/
  XcmV3JunctionNetworkId: {
    _enum: {
      ByGenesis: '[u8;32]',
      ByFork: {
        blockNumber: 'u64',
        blockHash: '[u8;32]',
      },
      Polkadot: 'Null',
      Kusama: 'Null',
      Westend: 'Null',
      Rococo: 'Null',
      Wococo: 'Null',
      Ethereum: {
        chainId: 'Compact<u64>',
      },
      BitcoinCore: 'Null',
      BitcoinCash: 'Null'
    }
  },
  /**
   * Lookup78: xcm::v3::junction::BodyId
   **/
  XcmV3JunctionBodyId: {
    _enum: {
      Unit: 'Null',
      Moniker: '[u8;4]',
      Index: 'Compact<u32>',
      Executive: 'Null',
      Technical: 'Null',
      Legislative: 'Null',
      Judicial: 'Null',
      Defense: 'Null',
      Administration: 'Null',
      Treasury: 'Null'
    }
  },
  /**
   * Lookup79: xcm::v3::junction::BodyPart
   **/
  XcmV3JunctionBodyPart: {
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
   * Lookup80: xcm::v3::Xcm<Call>
   **/
  XcmV3Xcm: 'Vec<XcmV3Instruction>',
  /**
   * Lookup82: xcm::v3::Instruction<Call>
   **/
  XcmV3Instruction: {
    _enum: {
      WithdrawAsset: 'XcmV3MultiassetMultiAssets',
      ReserveAssetDeposited: 'XcmV3MultiassetMultiAssets',
      ReceiveTeleportedAsset: 'XcmV3MultiassetMultiAssets',
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV3Response',
        maxWeight: 'SpWeightsWeightV2Weight',
        querier: 'Option<XcmV3MultiLocation>',
      },
      TransferAsset: {
        assets: 'XcmV3MultiassetMultiAssets',
        beneficiary: 'XcmV3MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV3MultiassetMultiAssets',
        dest: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      Transact: {
        originKind: 'XcmV2OriginKind',
        requireWeightAtMost: 'SpWeightsWeightV2Weight',
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
      DescendOrigin: 'XcmV3Junctions',
      ReportError: 'XcmV3QueryResponseInfo',
      DepositAsset: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        beneficiary: 'XcmV3MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        dest: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      ExchangeAsset: {
        give: 'XcmV3MultiassetMultiAssetFilter',
        want: 'XcmV3MultiassetMultiAssets',
        maximal: 'bool',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        reserve: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      InitiateTeleport: {
        assets: 'XcmV3MultiassetMultiAssetFilter',
        dest: 'XcmV3MultiLocation',
        xcm: 'XcmV3Xcm',
      },
      ReportHolding: {
        responseInfo: 'XcmV3QueryResponseInfo',
        assets: 'XcmV3MultiassetMultiAssetFilter',
      },
      BuyExecution: {
        fees: 'XcmV3MultiAsset',
        weightLimit: 'XcmV3WeightLimit',
      },
      RefundSurplus: 'Null',
      SetErrorHandler: 'XcmV3Xcm',
      SetAppendix: 'XcmV3Xcm',
      ClearError: 'Null',
      ClaimAsset: {
        assets: 'XcmV3MultiassetMultiAssets',
        ticket: 'XcmV3MultiLocation',
      },
      Trap: 'Compact<u64>',
      SubscribeVersion: {
        queryId: 'Compact<u64>',
        maxResponseWeight: 'SpWeightsWeightV2Weight',
      },
      UnsubscribeVersion: 'Null',
      BurnAsset: 'XcmV3MultiassetMultiAssets',
      ExpectAsset: 'XcmV3MultiassetMultiAssets',
      ExpectOrigin: 'Option<XcmV3MultiLocation>',
      ExpectError: 'Option<(u32,XcmV3TraitsError)>',
      ExpectTransactStatus: 'XcmV3MaybeErrorCode',
      QueryPallet: {
        moduleName: 'Bytes',
        responseInfo: 'XcmV3QueryResponseInfo',
      },
      ExpectPallet: {
        index: 'Compact<u32>',
        name: 'Bytes',
        moduleName: 'Bytes',
        crateMajor: 'Compact<u32>',
        minCrateMinor: 'Compact<u32>',
      },
      ReportTransactStatus: 'XcmV3QueryResponseInfo',
      ClearTransactStatus: 'Null',
      UniversalOrigin: 'XcmV3Junction',
      ExportMessage: {
        network: 'XcmV3JunctionNetworkId',
        destination: 'XcmV3Junctions',
        xcm: 'XcmV3Xcm',
      },
      LockAsset: {
        asset: 'XcmV3MultiAsset',
        unlocker: 'XcmV3MultiLocation',
      },
      UnlockAsset: {
        asset: 'XcmV3MultiAsset',
        target: 'XcmV3MultiLocation',
      },
      NoteUnlockable: {
        asset: 'XcmV3MultiAsset',
        owner: 'XcmV3MultiLocation',
      },
      RequestUnlock: {
        asset: 'XcmV3MultiAsset',
        locker: 'XcmV3MultiLocation',
      },
      SetFeesMode: {
        jitWithdraw: 'bool',
      },
      SetTopic: '[u8;32]',
      ClearTopic: 'Null',
      AliasOrigin: 'XcmV3MultiLocation',
      UnpaidExecution: {
        weightLimit: 'XcmV3WeightLimit',
        checkOrigin: 'Option<XcmV3MultiLocation>'
      }
    }
  },
  /**
   * Lookup83: xcm::v3::multiasset::MultiAssets
   **/
  XcmV3MultiassetMultiAssets: 'Vec<XcmV3MultiAsset>',
  /**
   * Lookup85: xcm::v3::multiasset::MultiAsset
   **/
  XcmV3MultiAsset: {
    id: 'XcmV3MultiassetAssetId',
    fun: 'XcmV3MultiassetFungibility'
  },
  /**
   * Lookup86: xcm::v3::multiasset::AssetId
   **/
  XcmV3MultiassetAssetId: {
    _enum: {
      Concrete: 'XcmV3MultiLocation',
      Abstract: '[u8;32]'
    }
  },
  /**
   * Lookup87: xcm::v3::multiasset::Fungibility
   **/
  XcmV3MultiassetFungibility: {
    _enum: {
      Fungible: 'Compact<u128>',
      NonFungible: 'XcmV3MultiassetAssetInstance'
    }
  },
  /**
   * Lookup88: xcm::v3::multiasset::AssetInstance
   **/
  XcmV3MultiassetAssetInstance: {
    _enum: {
      Undefined: 'Null',
      Index: 'Compact<u128>',
      Array4: '[u8;4]',
      Array8: '[u8;8]',
      Array16: '[u8;16]',
      Array32: '[u8;32]'
    }
  },
  /**
   * Lookup91: xcm::v3::Response
   **/
  XcmV3Response: {
    _enum: {
      Null: 'Null',
      Assets: 'XcmV3MultiassetMultiAssets',
      ExecutionResult: 'Option<(u32,XcmV3TraitsError)>',
      Version: 'u32',
      PalletsInfo: 'Vec<XcmV3PalletInfo>',
      DispatchResult: 'XcmV3MaybeErrorCode'
    }
  },
  /**
   * Lookup95: xcm::v3::PalletInfo
   **/
  XcmV3PalletInfo: {
    index: 'Compact<u32>',
    name: 'Bytes',
    moduleName: 'Bytes',
    major: 'Compact<u32>',
    minor: 'Compact<u32>',
    patch: 'Compact<u32>'
  },
  /**
   * Lookup98: xcm::v3::MaybeErrorCode
   **/
  XcmV3MaybeErrorCode: {
    _enum: {
      Success: 'Null',
      Error: 'Bytes',
      TruncatedError: 'Bytes'
    }
  },
  /**
   * Lookup101: xcm::v2::OriginKind
   **/
  XcmV2OriginKind: {
    _enum: ['Native', 'SovereignAccount', 'Superuser', 'Xcm']
  },
  /**
   * Lookup102: xcm::double_encoded::DoubleEncoded<T>
   **/
  XcmDoubleEncoded: {
    encoded: 'Bytes'
  },
  /**
   * Lookup103: xcm::v3::QueryResponseInfo
   **/
  XcmV3QueryResponseInfo: {
    destination: 'XcmV3MultiLocation',
    queryId: 'Compact<u64>',
    maxWeight: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup104: xcm::v3::multiasset::MultiAssetFilter
   **/
  XcmV3MultiassetMultiAssetFilter: {
    _enum: {
      Definite: 'XcmV3MultiassetMultiAssets',
      Wild: 'XcmV3MultiassetWildMultiAsset'
    }
  },
  /**
   * Lookup105: xcm::v3::multiasset::WildMultiAsset
   **/
  XcmV3MultiassetWildMultiAsset: {
    _enum: {
      All: 'Null',
      AllOf: {
        id: 'XcmV3MultiassetAssetId',
        fun: 'XcmV3MultiassetWildFungibility',
      },
      AllCounted: 'Compact<u32>',
      AllOfCounted: {
        id: 'XcmV3MultiassetAssetId',
        fun: 'XcmV3MultiassetWildFungibility',
        count: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup106: xcm::v3::multiasset::WildFungibility
   **/
  XcmV3MultiassetWildFungibility: {
    _enum: ['Fungible', 'NonFungible']
  },
  /**
   * Lookup107: xcm::v3::WeightLimit
   **/
  XcmV3WeightLimit: {
    _enum: {
      Unlimited: 'Null',
      Limited: 'SpWeightsWeightV2Weight'
    }
  },
  /**
   * Lookup108: xcm::VersionedMultiAssets
   **/
  XcmVersionedMultiAssets: {
    _enum: {
      __Unused0: 'Null',
      V2: 'XcmV2MultiassetMultiAssets',
      __Unused2: 'Null',
      V3: 'XcmV3MultiassetMultiAssets'
    }
  },
  /**
   * Lookup109: xcm::v2::multiasset::MultiAssets
   **/
  XcmV2MultiassetMultiAssets: 'Vec<XcmV2MultiAsset>',
  /**
   * Lookup111: xcm::v2::multiasset::MultiAsset
   **/
  XcmV2MultiAsset: {
    id: 'XcmV2MultiassetAssetId',
    fun: 'XcmV2MultiassetFungibility'
  },
  /**
   * Lookup112: xcm::v2::multiasset::AssetId
   **/
  XcmV2MultiassetAssetId: {
    _enum: {
      Concrete: 'XcmV2MultiLocation',
      Abstract: 'Bytes'
    }
  },
  /**
   * Lookup113: xcm::v2::multilocation::MultiLocation
   **/
  XcmV2MultiLocation: {
    parents: 'u8',
    interior: 'XcmV2MultilocationJunctions'
  },
  /**
   * Lookup114: xcm::v2::multilocation::Junctions
   **/
  XcmV2MultilocationJunctions: {
    _enum: {
      Here: 'Null',
      X1: 'XcmV2Junction',
      X2: '(XcmV2Junction,XcmV2Junction)',
      X3: '(XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X4: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X5: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X6: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X7: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)',
      X8: '(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)'
    }
  },
  /**
   * Lookup115: xcm::v2::junction::Junction
   **/
  XcmV2Junction: {
    _enum: {
      Parachain: 'Compact<u32>',
      AccountId32: {
        network: 'XcmV2NetworkId',
        id: '[u8;32]',
      },
      AccountIndex64: {
        network: 'XcmV2NetworkId',
        index: 'Compact<u64>',
      },
      AccountKey20: {
        network: 'XcmV2NetworkId',
        key: '[u8;20]',
      },
      PalletInstance: 'u8',
      GeneralIndex: 'Compact<u128>',
      GeneralKey: 'Bytes',
      OnlyChild: 'Null',
      Plurality: {
        id: 'XcmV2BodyId',
        part: 'XcmV2BodyPart'
      }
    }
  },
  /**
   * Lookup116: xcm::v2::NetworkId
   **/
  XcmV2NetworkId: {
    _enum: {
      Any: 'Null',
      Named: 'Bytes',
      Polkadot: 'Null',
      Kusama: 'Null'
    }
  },
  /**
   * Lookup118: xcm::v2::BodyId
   **/
  XcmV2BodyId: {
    _enum: {
      Unit: 'Null',
      Named: 'Bytes',
      Index: 'Compact<u32>',
      Executive: 'Null',
      Technical: 'Null',
      Legislative: 'Null',
      Judicial: 'Null',
      Defense: 'Null',
      Administration: 'Null',
      Treasury: 'Null'
    }
  },
  /**
   * Lookup119: xcm::v2::BodyPart
   **/
  XcmV2BodyPart: {
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
   * Lookup120: xcm::v2::multiasset::Fungibility
   **/
  XcmV2MultiassetFungibility: {
    _enum: {
      Fungible: 'Compact<u128>',
      NonFungible: 'XcmV2MultiassetAssetInstance'
    }
  },
  /**
   * Lookup121: xcm::v2::multiasset::AssetInstance
   **/
  XcmV2MultiassetAssetInstance: {
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
   * Lookup122: xcm::VersionedMultiLocation
   **/
  XcmVersionedMultiLocation: {
    _enum: {
      __Unused0: 'Null',
      V2: 'XcmV2MultiLocation',
      __Unused2: 'Null',
      V3: 'XcmV3MultiLocation'
    }
  },
  /**
   * Lookup123: cumulus_pallet_xcm::pallet::Event<T>
   **/
  CumulusPalletXcmEvent: {
    _enum: {
      InvalidFormat: '[u8;32]',
      UnsupportedVersion: '[u8;32]',
      ExecutedDownward: '([u8;32],XcmV3TraitsOutcome)'
    }
  },
  /**
   * Lookup124: cumulus_pallet_dmp_queue::pallet::Event<T>
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
        outcome: 'XcmV3TraitsOutcome',
      },
      WeightExhausted: {
        messageId: '[u8;32]',
        remainingWeight: 'SpWeightsWeightV2Weight',
        requiredWeight: 'SpWeightsWeightV2Weight',
      },
      OverweightEnqueued: {
        messageId: '[u8;32]',
        overweightIndex: 'u64',
        requiredWeight: 'SpWeightsWeightV2Weight',
      },
      OverweightServiced: {
        overweightIndex: 'u64',
        weightUsed: 'SpWeightsWeightV2Weight',
      },
      MaxMessagesExhausted: {
        messageId: '[u8;32]'
      }
    }
  },
  /**
   * Lookup125: orml_xtokens::module::Event<T>
   **/
  OrmlXtokensModuleEvent: {
    _enum: {
      TransferredMultiAssets: {
        sender: 'AccountId32',
        assets: 'XcmV3MultiassetMultiAssets',
        fee: 'XcmV3MultiAsset',
        dest: 'XcmV3MultiLocation'
      }
    }
  },
  /**
   * Lookup126: orml_unknown_tokens::module::Event
   **/
  OrmlUnknownTokensModuleEvent: {
    _enum: {
      Deposited: {
        asset: 'XcmV3MultiAsset',
        who: 'XcmV3MultiLocation',
      },
      Withdrawn: {
        asset: 'XcmV3MultiAsset',
        who: 'XcmV3MultiLocation'
      }
    }
  },
  /**
   * Lookup127: orml_tokens::module::Event<T>
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
        who: 'AccountId32',
      },
      Locked: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      Unlocked: {
        currencyId: 'u128',
        who: 'AccountId32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup129: pallet_currency_factory::pallet::Event<T>
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
   * Lookup131: pallet_crowdloan_rewards::pallet::Event<T>
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
        excessFunds: 'u128',
      },
      RewardsUnlocked: {
        at: 'u64',
      },
      RewardsAdded: {
        additions: 'Vec<(PalletCrowdloanRewardsModelsRemoteAccount,u128,u64)>',
      },
      RewardsDeleted: {
        deletions: 'Vec<PalletCrowdloanRewardsModelsRemoteAccount>'
      }
    }
  },
  /**
   * Lookup132: pallet_crowdloan_rewards::models::RemoteAccount<sp_core::crypto::AccountId32>
   **/
  PalletCrowdloanRewardsModelsRemoteAccount: {
    _enum: {
      RelayChain: 'AccountId32',
      Ethereum: 'ComposableSupportEthereumAddress'
    }
  },
  /**
   * Lookup133: composable_support::types::EthereumAddress
   **/
  ComposableSupportEthereumAddress: '[u8;20]',
  /**
   * Lookup137: pallet_vesting::module::Event<T>
   **/
  PalletVestingModuleEvent: {
    _enum: {
      VestingScheduleAdded: {
        from: 'AccountId32',
        to: 'AccountId32',
        asset: 'u128',
        vestingScheduleId: 'u128',
        schedule: 'PalletVestingVestingSchedule',
        scheduleAmount: 'u128',
      },
      Claimed: {
        who: 'AccountId32',
        asset: 'u128',
        vestingScheduleIds: 'PalletVestingVestingScheduleIdSet',
        lockedAmount: 'u128',
        claimedAmountPerSchedule: 'BTreeMap<u128, u128>',
      },
      VestingSchedulesUpdated: {
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup138: pallet_vesting::types::VestingSchedule<VestingScheduleId, BlockNumber, Moment, Balance>
   **/
  PalletVestingVestingSchedule: {
    vestingScheduleId: 'u128',
    window: 'PalletVestingVestingWindow',
    periodCount: 'u32',
    perPeriod: 'Compact<u128>',
    alreadyClaimed: 'u128'
  },
  /**
   * Lookup139: pallet_vesting::types::VestingWindow<BlockNumber, Moment>
   **/
  PalletVestingVestingWindow: {
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
   * Lookup140: pallet_vesting::types::VestingScheduleIdSet<Id, MaxVestingSchedules>
   **/
  PalletVestingVestingScheduleIdSet: {
    _enum: {
      All: 'Null',
      One: 'u128',
      Many: 'Vec<u128>'
    }
  },
  /**
   * Lookup147: pallet_bonded_finance::pallet::Event<T>
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
   * Lookup148: pallet_assets_registry::pallet::Event<T>
   **/
  PalletAssetsRegistryEvent: {
    _enum: {
      AssetRegistered: {
        assetId: 'u128',
        location: 'Option<PrimitivesCurrencyForeignAssetId>',
        assetInfo: 'ComposableTraitsAssetsAssetInfo',
      },
      AssetUpdated: {
        assetId: 'u128',
        assetInfo: 'ComposableTraitsAssetsAssetInfoUpdate',
      },
      AssetLocationUpdated: {
        assetId: 'u128',
        location: 'PrimitivesCurrencyForeignAssetId',
      },
      AssetLocationRemoved: {
        assetId: 'u128',
      },
      MinFeeUpdated: {
        targetParachainId: 'u32',
        foreignAssetId: 'PrimitivesCurrencyForeignAssetId',
        amount: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup150: primitives::currency::ForeignAssetId
   **/
  PrimitivesCurrencyForeignAssetId: {
    _enum: {
      Xcm: 'PrimitivesCurrencyVersionedMultiLocation',
      IbcIcs20: 'PrimitivesCurrencyPrefixedDenom'
    }
  },
  /**
   * Lookup151: primitives::currency::VersionedMultiLocation
   **/
  PrimitivesCurrencyVersionedMultiLocation: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      V3: 'XcmV3MultiLocation'
    }
  },
  /**
   * Lookup152: primitives::currency::PrefixedDenom
   **/
  PrimitivesCurrencyPrefixedDenom: 'IbcApplicationsTransferDenomPrefixedDenom',
  /**
   * Lookup153: ibc::applications::transfer::denom::PrefixedDenom
   **/
  IbcApplicationsTransferDenomPrefixedDenom: {
    tracePath: 'IbcApplicationsTransferDenomTracePath',
    baseDenom: 'Text'
  },
  /**
   * Lookup154: ibc::applications::transfer::denom::TracePath
   **/
  IbcApplicationsTransferDenomTracePath: 'Vec<IbcApplicationsTransferDenomTracePrefix>',
  /**
   * Lookup156: ibc::applications::transfer::denom::TracePrefix
   **/
  IbcApplicationsTransferDenomTracePrefix: {
    portId: 'Text',
    channelId: 'Text'
  },
  /**
   * Lookup161: composable_traits::assets::AssetInfo<Balance>
   **/
  ComposableTraitsAssetsAssetInfo: {
    name: 'Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>',
    symbol: 'Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>',
    decimals: 'Option<u8>',
    existentialDeposit: 'u128',
    ratio: 'Option<ComposableTraitsCurrencyRational64>'
  },
  /**
   * Lookup163: composable_support::collections::vec::bounded::bi_bounded_vec::BiBoundedVec<T>
   **/
  ComposableSupportCollectionsVecBoundedBiBoundedVec: {
    inner: 'Bytes'
  },
  /**
   * Lookup168: composable_traits::currency::Rational64
   **/
  ComposableTraitsCurrencyRational64: {
    n: 'u64',
    d: 'u64'
  },
  /**
   * Lookup169: composable_traits::assets::AssetInfoUpdate<Balance>
   **/
  ComposableTraitsAssetsAssetInfoUpdate: {
    name: {
      _enum: {
        Ignore: 'Null',
        Set: 'Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>'
      }
    },
    symbol: {
      _enum: {
        Ignore: 'Null',
        Set: 'Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>'
      }
    },
    decimals: {
      _enum: {
        Ignore: 'Null',
        Set: 'Option<u8>'
      }
    },
    existentialDeposit: 'ComposableTraitsStorageUpdateValueU128',
    ratio: 'ComposableTraitsStorageUpdateValueOption'
  },
  /**
   * Lookup173: composable_traits::storage::UpdateValue<T>
   **/
  ComposableTraitsStorageUpdateValueU128: {
    _enum: {
      Ignore: 'Null',
      Set: 'u128'
    }
  },
  /**
   * Lookup174: composable_traits::storage::UpdateValue<Option<composable_traits::currency::Rational64>>
   **/
  ComposableTraitsStorageUpdateValueOption: {
    _enum: {
      Ignore: 'Null',
      Set: 'Option<ComposableTraitsCurrencyRational64>'
    }
  },
  /**
   * Lookup176: pallet_pablo::pallet::Event<T>
   **/
  PalletPabloEvent: {
    _enum: {
      PoolCreated: {
        poolId: 'u128',
        owner: 'AccountId32',
        assetWeights: 'BTreeMap<u128, Permill>',
        lpTokenId: 'u128',
      },
      LiquidityAdded: {
        who: 'AccountId32',
        poolId: 'u128',
        assetAmounts: 'BTreeMap<u128, u128>',
        mintedLp: 'u128',
      },
      LiquidityRemoved: {
        who: 'AccountId32',
        poolId: 'u128',
        assetAmounts: 'BTreeMap<u128, u128>',
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
   * Lookup184: composable_traits::dex::Fee<primitives::currency::CurrencyId, Balance>
   **/
  ComposableTraitsDexFee: {
    fee: 'u128',
    lpFee: 'u128',
    ownerFee: 'u128',
    protocolFee: 'u128',
    assetId: 'u128'
  },
  /**
   * Lookup189: pallet_oracle::pallet::Event<T>
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
      PriceChanged: '(u128,u128)',
      SignerRemoved: '(AccountId32,AccountId32,u128)'
    }
  },
  /**
   * Lookup191: reward::pallet::Event<T, I>
   **/
  RewardEvent: {
    _enum: {
      DepositStake: {
        poolId: 'u128',
        stakeId: 'AccountId32',
        amount: 'i128',
      },
      DistributeReward: {
        currencyId: 'u128',
        amount: 'i128',
      },
      WithdrawStake: {
        poolId: 'u128',
        stakeId: 'AccountId32',
        amount: 'i128',
      },
      WithdrawReward: {
        poolId: 'u128',
        stakeId: 'AccountId32',
        currencyId: 'u128',
        amount: 'i128'
      }
    }
  },
  /**
   * Lookup194: farming::pallet::Event<T>
   **/
  FarmingEvent: {
    _enum: {
      RewardScheduleUpdated: {
        poolCurrencyId: 'u128',
        rewardCurrencyId: 'u128',
        periodCount: 'u32',
        perPeriod: 'u128',
      },
      RewardDistributed: {
        poolCurrencyId: 'u128',
        rewardCurrencyId: 'u128',
        amount: 'u128',
      },
      RewardClaimed: {
        accountId: 'AccountId32',
        poolCurrencyId: 'u128',
        rewardCurrencyId: 'u128',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup195: pallet_referenda::pallet::Event<T, I>
   **/
  PalletReferendaEvent: {
    _enum: {
      Submitted: {
        index: 'u32',
        track: 'u16',
        proposal: 'FrameSupportPreimagesBounded',
      },
      DecisionDepositPlaced: {
        index: 'u32',
        who: 'AccountId32',
        amount: 'u128',
      },
      DecisionDepositRefunded: {
        index: 'u32',
        who: 'AccountId32',
        amount: 'u128',
      },
      DepositSlashed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      DecisionStarted: {
        index: 'u32',
        track: 'u16',
        proposal: 'FrameSupportPreimagesBounded',
        tally: 'PalletConvictionVotingTally',
      },
      ConfirmStarted: {
        index: 'u32',
      },
      ConfirmAborted: {
        index: 'u32',
      },
      Confirmed: {
        index: 'u32',
        tally: 'PalletConvictionVotingTally',
      },
      Approved: {
        index: 'u32',
      },
      Rejected: {
        index: 'u32',
        tally: 'PalletConvictionVotingTally',
      },
      TimedOut: {
        index: 'u32',
        tally: 'PalletConvictionVotingTally',
      },
      Cancelled: {
        index: 'u32',
        tally: 'PalletConvictionVotingTally',
      },
      Killed: {
        index: 'u32',
        tally: 'PalletConvictionVotingTally',
      },
      SubmissionDepositRefunded: {
        index: 'u32',
        who: 'AccountId32',
        amount: 'u128',
      },
      MetadataSet: {
        _alias: {
          hash_: 'hash',
        },
        index: 'u32',
        hash_: 'H256',
      },
      MetadataCleared: {
        _alias: {
          hash_: 'hash',
        },
        index: 'u32',
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup196: frame_support::traits::preimages::Bounded<picasso_runtime::RuntimeCall>
   **/
  FrameSupportPreimagesBounded: {
    _enum: {
      Legacy: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Inline: 'Bytes',
      Lookup: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
        len: 'u32'
      }
    }
  },
  /**
   * Lookup198: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
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
   * Lookup202: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup203: pallet_sudo::pallet::Call<T>
   **/
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: 'Call',
      },
      sudo_unchecked_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight',
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
   * Lookup205: pallet_asset_tx_payment::pallet::Call<T>
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
   * Lookup207: pallet_indices::pallet::Call<T>
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
        new_: 'MultiAddress',
        index: 'u32',
      },
      free: {
        index: 'u32',
      },
      force_transfer: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
        index: 'u32',
        freeze: 'bool',
      },
      freeze: {
        index: 'u32'
      }
    }
  },
  /**
   * Lookup208: pallet_balances::pallet::Call<T, I>
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
   * Lookup209: pallet_identity::pallet::Call<T>
   **/
  PalletIdentityCall: {
    _enum: {
      add_registrar: {
        account: 'MultiAddress',
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
        new_: 'MultiAddress',
      },
      set_fields: {
        index: 'Compact<u32>',
        fields: 'PalletIdentityBitFlags',
      },
      provide_judgement: {
        regIndex: 'Compact<u32>',
        target: 'MultiAddress',
        judgement: 'PalletIdentityJudgement',
        identity: 'H256',
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
   * Lookup210: pallet_identity::types::IdentityInfo<FieldLimit>
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
   * Lookup246: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
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
   * Lookup247: pallet_identity::types::IdentityField
   **/
  PalletIdentityIdentityField: {
    _enum: ['__Unused0', 'Display', 'Legal', '__Unused3', 'Web', '__Unused5', '__Unused6', '__Unused7', 'Riot', '__Unused9', '__Unused10', '__Unused11', '__Unused12', '__Unused13', '__Unused14', '__Unused15', 'Email', '__Unused17', '__Unused18', '__Unused19', '__Unused20', '__Unused21', '__Unused22', '__Unused23', '__Unused24', '__Unused25', '__Unused26', '__Unused27', '__Unused28', '__Unused29', '__Unused30', '__Unused31', 'PgpFingerprint', '__Unused33', '__Unused34', '__Unused35', '__Unused36', '__Unused37', '__Unused38', '__Unused39', '__Unused40', '__Unused41', '__Unused42', '__Unused43', '__Unused44', '__Unused45', '__Unused46', '__Unused47', '__Unused48', '__Unused49', '__Unused50', '__Unused51', '__Unused52', '__Unused53', '__Unused54', '__Unused55', '__Unused56', '__Unused57', '__Unused58', '__Unused59', '__Unused60', '__Unused61', '__Unused62', '__Unused63', 'Image', '__Unused65', '__Unused66', '__Unused67', '__Unused68', '__Unused69', '__Unused70', '__Unused71', '__Unused72', '__Unused73', '__Unused74', '__Unused75', '__Unused76', '__Unused77', '__Unused78', '__Unused79', '__Unused80', '__Unused81', '__Unused82', '__Unused83', '__Unused84', '__Unused85', '__Unused86', '__Unused87', '__Unused88', '__Unused89', '__Unused90', '__Unused91', '__Unused92', '__Unused93', '__Unused94', '__Unused95', '__Unused96', '__Unused97', '__Unused98', '__Unused99', '__Unused100', '__Unused101', '__Unused102', '__Unused103', '__Unused104', '__Unused105', '__Unused106', '__Unused107', '__Unused108', '__Unused109', '__Unused110', '__Unused111', '__Unused112', '__Unused113', '__Unused114', '__Unused115', '__Unused116', '__Unused117', '__Unused118', '__Unused119', '__Unused120', '__Unused121', '__Unused122', '__Unused123', '__Unused124', '__Unused125', '__Unused126', '__Unused127', 'Twitter']
  },
  /**
   * Lookup248: pallet_identity::types::Judgement<Balance>
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
   * Lookup249: pallet_multisig::pallet::Call<T>
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
        call: 'Call',
        maxWeight: 'SpWeightsWeightV2Weight',
      },
      approve_as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        maybeTimepoint: 'Option<PalletMultisigTimepoint>',
        callHash: '[u8;32]',
        maxWeight: 'SpWeightsWeightV2Weight',
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
   * Lookup251: cumulus_pallet_parachain_system::pallet::Call<T>
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
   * Lookup252: cumulus_primitives_parachain_inherent::ParachainInherentData
   **/
  CumulusPrimitivesParachainInherentParachainInherentData: {
    validationData: 'PolkadotPrimitivesV2PersistedValidationData',
    relayChainState: 'SpTrieStorageProof',
    downwardMessages: 'Vec<PolkadotCorePrimitivesInboundDownwardMessage>',
    horizontalMessages: 'BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>'
  },
  /**
   * Lookup253: polkadot_primitives::v2::PersistedValidationData<primitive_types::H256, N>
   **/
  PolkadotPrimitivesV2PersistedValidationData: {
    parentHead: 'Bytes',
    relayParentNumber: 'u32',
    relayParentStorageRoot: 'H256',
    maxPovSize: 'u32'
  },
  /**
   * Lookup255: sp_trie::storage_proof::StorageProof
   **/
  SpTrieStorageProof: {
    trieNodes: 'BTreeSet<Bytes>'
  },
  /**
   * Lookup258: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundDownwardMessage: {
    sentAt: 'u32',
    msg: 'Bytes'
  },
  /**
   * Lookup261: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
   **/
  PolkadotCorePrimitivesInboundHrmpMessage: {
    sentAt: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup264: parachain_info::pallet::Call<T>
   **/
  ParachainInfoCall: 'Null',
  /**
   * Lookup265: pallet_collator_selection::pallet::Call<T>
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
   * Lookup266: pallet_session::pallet::Call<T>
   **/
  PalletSessionCall: {
    _enum: {
      set_keys: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'PicassoRuntimeOpaqueSessionKeys',
        proof: 'Bytes',
      },
      purge_keys: 'Null'
    }
  },
  /**
   * Lookup267: picasso_runtime::opaque::SessionKeys
   **/
  PicassoRuntimeOpaqueSessionKeys: {
    aura: 'SpConsensusAuraSr25519AppSr25519Public'
  },
  /**
   * Lookup268: sp_consensus_aura::sr25519::app_sr25519::Public
   **/
  SpConsensusAuraSr25519AppSr25519Public: 'SpCoreSr25519Public',
  /**
   * Lookup269: sp_core::sr25519::Public
   **/
  SpCoreSr25519Public: '[u8;32]',
  /**
   * Lookup270: pallet_collective::pallet::Call<T, I>
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
      close_old_weight: {
        proposalHash: 'H256',
        index: 'Compact<u32>',
        proposalWeightBound: 'Compact<u64>',
        lengthBound: 'Compact<u32>',
      },
      disapprove_proposal: {
        proposalHash: 'H256',
      },
      close: {
        proposalHash: 'H256',
        index: 'Compact<u32>',
        proposalWeightBound: 'SpWeightsWeightV2Weight',
        lengthBound: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup273: pallet_membership::pallet::Call<T, I>
   **/
  PalletMembershipCall: {
    _enum: {
      add_member: {
        who: 'MultiAddress',
      },
      remove_member: {
        who: 'MultiAddress',
      },
      swap_member: {
        remove: 'MultiAddress',
        add: 'MultiAddress',
      },
      reset_members: {
        members: 'Vec<AccountId32>',
      },
      change_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
      },
      set_prime: {
        who: 'MultiAddress',
      },
      clear_prime: 'Null'
    }
  },
  /**
   * Lookup274: pallet_treasury::pallet::Call<T, I>
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
   * Lookup275: pallet_democracy::pallet::Call<T>
   **/
  PalletDemocracyCall: {
    _enum: {
      propose: {
        proposal: 'FrameSupportPreimagesBounded',
        value: 'Compact<u128>',
      },
      second: {
        proposal: 'Compact<u32>',
      },
      vote: {
        refIndex: 'Compact<u32>',
        vote: 'PalletDemocracyVoteAccountVote',
      },
      emergency_cancel: {
        refIndex: 'u32',
      },
      external_propose: {
        proposal: 'FrameSupportPreimagesBounded',
      },
      external_propose_majority: {
        proposal: 'FrameSupportPreimagesBounded',
      },
      external_propose_default: {
        proposal: 'FrameSupportPreimagesBounded',
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
      delegate: {
        to: 'MultiAddress',
        conviction: 'PalletDemocracyConviction',
        balance: 'u128',
      },
      undelegate: 'Null',
      clear_public_proposals: 'Null',
      unlock: {
        target: 'MultiAddress',
      },
      remove_vote: {
        index: 'u32',
      },
      remove_other_vote: {
        target: 'MultiAddress',
        index: 'u32',
      },
      blacklist: {
        proposalHash: 'H256',
        maybeRefIndex: 'Option<u32>',
      },
      cancel_proposal: {
        propIndex: 'Compact<u32>',
      },
      set_metadata: {
        owner: 'PalletDemocracyMetadataOwner',
        maybeHash: 'Option<H256>'
      }
    }
  },
  /**
   * Lookup276: pallet_democracy::conviction::Conviction
   **/
  PalletDemocracyConviction: {
    _enum: ['None', 'Locked1x', 'Locked2x', 'Locked3x', 'Locked4x', 'Locked5x', 'Locked6x']
  },
  /**
   * Lookup283: pallet_scheduler::pallet::Call<T>
   **/
  PalletSchedulerCall: {
    _enum: {
      schedule: {
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel: {
        when: 'u32',
        index: 'u32',
      },
      schedule_named: {
        id: '[u8;32]',
        when: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel_named: {
        id: '[u8;32]',
      },
      schedule_after: {
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      schedule_named_after: {
        id: '[u8;32]',
        after: 'u32',
        maybePeriodic: 'Option<(u32,u32)>',
        priority: 'u8',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup285: pallet_utility::pallet::Call<T>
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
        asOrigin: 'PicassoRuntimeOriginCaller',
        call: 'Call',
      },
      force_batch: {
        calls: 'Vec<Call>',
      },
      with_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup287: picasso_runtime::OriginCaller
   **/
  PicassoRuntimeOriginCaller: {
    _enum: {
      system: 'FrameSupportDispatchRawOrigin',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      Void: 'SpCoreVoid',
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
      PolkadotXcm: 'PalletXcmOrigin',
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
      __Unused70: 'Null',
      __Unused71: 'Null',
      TechnicalCommittee: 'PalletCollectiveRawOrigin',
      __Unused73: 'Null',
      ReleaseCommittee: 'PalletCollectiveRawOrigin',
      __Unused75: 'Null',
      __Unused76: 'Null',
      __Unused77: 'Null',
      __Unused78: 'Null',
      Origins: 'PalletCustomOriginsOrigin'
    }
  },
  /**
   * Lookup288: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup289: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
   **/
  PalletCollectiveRawOrigin: {
    _enum: {
      Members: '(u32,u32)',
      Member: 'AccountId32',
      _Phantom: 'Null'
    }
  },
  /**
   * Lookup292: pallet_xcm::pallet::Origin
   **/
  PalletXcmOrigin: {
    _enum: {
      Xcm: 'XcmV3MultiLocation',
      Response: 'XcmV3MultiLocation'
    }
  },
  /**
   * Lookup293: cumulus_pallet_xcm::pallet::Origin
   **/
  CumulusPalletXcmOrigin: {
    _enum: {
      Relay: 'Null',
      SiblingParachain: 'u32'
    }
  },
  /**
   * Lookup294: pallet_custom_origins::pallet::Origin
   **/
  PalletCustomOriginsOrigin: {
    _enum: ['WhitelistedCaller']
  },
  /**
   * Lookup295: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup296: pallet_preimage::pallet::Call<T>
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
   * Lookup297: pallet_proxy::pallet::Call<T>
   **/
  PalletProxyCall: {
    _enum: {
      proxy: {
        real: 'MultiAddress',
        forceProxyType: 'Option<ComposableTraitsAccountProxyProxyType>',
        call: 'Call',
      },
      add_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32',
      },
      remove_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32',
      },
      remove_proxies: 'Null',
      create_pure: {
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        delay: 'u32',
        index: 'u16',
      },
      kill_pure: {
        spawner: 'MultiAddress',
        proxyType: 'ComposableTraitsAccountProxyProxyType',
        index: 'u16',
        height: 'Compact<u32>',
        extIndex: 'Compact<u32>',
      },
      announce: {
        real: 'MultiAddress',
        callHash: 'H256',
      },
      remove_announcement: {
        real: 'MultiAddress',
        callHash: 'H256',
      },
      reject_announcement: {
        delegate: 'MultiAddress',
        callHash: 'H256',
      },
      proxy_announced: {
        delegate: 'MultiAddress',
        real: 'MultiAddress',
        forceProxyType: 'Option<ComposableTraitsAccountProxyProxyType>',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup299: cumulus_pallet_xcmp_queue::pallet::Call<T>
   **/
  CumulusPalletXcmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'SpWeightsWeightV2Weight',
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
        new_: 'SpWeightsWeightV2Weight',
      },
      update_weight_restrict_decay: {
        _alias: {
          new_: 'new',
        },
        new_: 'SpWeightsWeightV2Weight',
      },
      update_xcmp_max_individual_weight: {
        _alias: {
          new_: 'new',
        },
        new_: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup300: pallet_xcm::pallet::Call<T>
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
        maxWeight: 'SpWeightsWeightV2Weight',
      },
      force_xcm_version: {
        location: 'XcmV3MultiLocation',
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
        weightLimit: 'XcmV3WeightLimit',
      },
      limited_teleport_assets: {
        dest: 'XcmVersionedMultiLocation',
        beneficiary: 'XcmVersionedMultiLocation',
        assets: 'XcmVersionedMultiAssets',
        feeAssetItem: 'u32',
        weightLimit: 'XcmV3WeightLimit'
      }
    }
  },
  /**
   * Lookup301: xcm::VersionedXcm<RuntimeCall>
   **/
  XcmVersionedXcm: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      V2: 'XcmV2Xcm',
      V3: 'XcmV3Xcm'
    }
  },
  /**
   * Lookup302: xcm::v2::Xcm<RuntimeCall>
   **/
  XcmV2Xcm: 'Vec<XcmV2Instruction>',
  /**
   * Lookup304: xcm::v2::Instruction<RuntimeCall>
   **/
  XcmV2Instruction: {
    _enum: {
      WithdrawAsset: 'XcmV2MultiassetMultiAssets',
      ReserveAssetDeposited: 'XcmV2MultiassetMultiAssets',
      ReceiveTeleportedAsset: 'XcmV2MultiassetMultiAssets',
      QueryResponse: {
        queryId: 'Compact<u64>',
        response: 'XcmV2Response',
        maxWeight: 'Compact<u64>',
      },
      TransferAsset: {
        assets: 'XcmV2MultiassetMultiAssets',
        beneficiary: 'XcmV2MultiLocation',
      },
      TransferReserveAsset: {
        assets: 'XcmV2MultiassetMultiAssets',
        dest: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      Transact: {
        originType: 'XcmV2OriginKind',
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
      DescendOrigin: 'XcmV2MultilocationJunctions',
      ReportError: {
        queryId: 'Compact<u64>',
        dest: 'XcmV2MultiLocation',
        maxResponseWeight: 'Compact<u64>',
      },
      DepositAsset: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        beneficiary: 'XcmV2MultiLocation',
      },
      DepositReserveAsset: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        maxAssets: 'Compact<u32>',
        dest: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      ExchangeAsset: {
        give: 'XcmV2MultiassetMultiAssetFilter',
        receive: 'XcmV2MultiassetMultiAssets',
      },
      InitiateReserveWithdraw: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        reserve: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      InitiateTeleport: {
        assets: 'XcmV2MultiassetMultiAssetFilter',
        dest: 'XcmV2MultiLocation',
        xcm: 'XcmV2Xcm',
      },
      QueryHolding: {
        queryId: 'Compact<u64>',
        dest: 'XcmV2MultiLocation',
        assets: 'XcmV2MultiassetMultiAssetFilter',
        maxResponseWeight: 'Compact<u64>',
      },
      BuyExecution: {
        fees: 'XcmV2MultiAsset',
        weightLimit: 'XcmV2WeightLimit',
      },
      RefundSurplus: 'Null',
      SetErrorHandler: 'XcmV2Xcm',
      SetAppendix: 'XcmV2Xcm',
      ClearError: 'Null',
      ClaimAsset: {
        assets: 'XcmV2MultiassetMultiAssets',
        ticket: 'XcmV2MultiLocation',
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
   * Lookup305: xcm::v2::Response
   **/
  XcmV2Response: {
    _enum: {
      Null: 'Null',
      Assets: 'XcmV2MultiassetMultiAssets',
      ExecutionResult: 'Option<(u32,XcmV2TraitsError)>',
      Version: 'u32'
    }
  },
  /**
   * Lookup308: xcm::v2::traits::Error
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
   * Lookup309: xcm::v2::multiasset::MultiAssetFilter
   **/
  XcmV2MultiassetMultiAssetFilter: {
    _enum: {
      Definite: 'XcmV2MultiassetMultiAssets',
      Wild: 'XcmV2MultiassetWildMultiAsset'
    }
  },
  /**
   * Lookup310: xcm::v2::multiasset::WildMultiAsset
   **/
  XcmV2MultiassetWildMultiAsset: {
    _enum: {
      All: 'Null',
      AllOf: {
        id: 'XcmV2MultiassetAssetId',
        fun: 'XcmV2MultiassetWildFungibility'
      }
    }
  },
  /**
   * Lookup311: xcm::v2::multiasset::WildFungibility
   **/
  XcmV2MultiassetWildFungibility: {
    _enum: ['Fungible', 'NonFungible']
  },
  /**
   * Lookup312: xcm::v2::WeightLimit
   **/
  XcmV2WeightLimit: {
    _enum: {
      Unlimited: 'Null',
      Limited: 'Compact<u64>'
    }
  },
  /**
   * Lookup321: cumulus_pallet_xcm::pallet::Call<T>
   **/
  CumulusPalletXcmCall: 'Null',
  /**
   * Lookup322: cumulus_pallet_dmp_queue::pallet::Call<T>
   **/
  CumulusPalletDmpQueueCall: {
    _enum: {
      service_overweight: {
        index: 'u64',
        weightLimit: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup323: orml_xtokens::module::Call<T>
   **/
  OrmlXtokensModuleCall: {
    _enum: {
      transfer: {
        currencyId: 'u128',
        amount: 'u128',
        dest: 'XcmVersionedMultiLocation',
        destWeightLimit: 'XcmV3WeightLimit',
      },
      transfer_multiasset: {
        asset: 'XcmVersionedMultiAsset',
        dest: 'XcmVersionedMultiLocation',
        destWeightLimit: 'XcmV3WeightLimit',
      },
      transfer_with_fee: {
        currencyId: 'u128',
        amount: 'u128',
        fee: 'u128',
        dest: 'XcmVersionedMultiLocation',
        destWeightLimit: 'XcmV3WeightLimit',
      },
      transfer_multiasset_with_fee: {
        asset: 'XcmVersionedMultiAsset',
        fee: 'XcmVersionedMultiAsset',
        dest: 'XcmVersionedMultiLocation',
        destWeightLimit: 'XcmV3WeightLimit',
      },
      transfer_multicurrencies: {
        currencies: 'Vec<(u128,u128)>',
        feeItem: 'u32',
        dest: 'XcmVersionedMultiLocation',
        destWeightLimit: 'XcmV3WeightLimit',
      },
      transfer_multiassets: {
        assets: 'XcmVersionedMultiAssets',
        feeItem: 'u32',
        dest: 'XcmVersionedMultiLocation',
        destWeightLimit: 'XcmV3WeightLimit'
      }
    }
  },
  /**
   * Lookup324: xcm::VersionedMultiAsset
   **/
  XcmVersionedMultiAsset: {
    _enum: {
      __Unused0: 'Null',
      V2: 'XcmV2MultiAsset',
      __Unused2: 'Null',
      V3: 'XcmV3MultiAsset'
    }
  },
  /**
   * Lookup325: orml_unknown_tokens::module::Call<T>
   **/
  OrmlUnknownTokensModuleCall: 'Null',
  /**
   * Lookup326: orml_tokens::module::Call<T>
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
   * Lookup327: pallet_currency_factory::pallet::Call<T>
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
   * Lookup328: composable_traits::assets::BasicAssetMetadata
   **/
  ComposableTraitsAssetsBasicAssetMetadata: {
    symbol: 'ComposableSupportCollectionsVecBoundedBiBoundedVec',
    name: 'ComposableSupportCollectionsVecBoundedBiBoundedVec'
  },
  /**
   * Lookup331: pallet_crowdloan_rewards::pallet::Call<T>
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
      claim: 'Null',
      unlock_rewards_for: {
        rewardAccounts: 'Vec<AccountId32>',
      },
      add: {
        additions: 'Vec<(PalletCrowdloanRewardsModelsRemoteAccount,u128,u64)>'
      }
    }
  },
  /**
   * Lookup332: pallet_crowdloan_rewards::models::Proof<sp_core::crypto::AccountId32>
   **/
  PalletCrowdloanRewardsModelsProof: {
    _enum: {
      RelayChain: '(AccountId32,SpRuntimeMultiSignature)',
      Ethereum: 'ComposableSupportEcdsaSignature'
    }
  },
  /**
   * Lookup333: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup334: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup336: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup337: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup339: composable_support::types::EcdsaSignature
   **/
  ComposableSupportEcdsaSignature: '[u8;65]',
  /**
   * Lookup340: pallet_vesting::module::Call<T>
   **/
  PalletVestingModuleCall: {
    _enum: {
      claim: {
        asset: 'u128',
        vestingScheduleIds: 'PalletVestingVestingScheduleIdSet',
      },
      vested_transfer: {
        from: 'MultiAddress',
        beneficiary: 'MultiAddress',
        asset: 'u128',
        scheduleInfo: 'PalletVestingVestingScheduleInfo',
      },
      update_vesting_schedules: {
        who: 'MultiAddress',
        asset: 'u128',
        vestingSchedules: 'Vec<PalletVestingVestingScheduleInfo>',
      },
      claim_for: {
        dest: 'MultiAddress',
        asset: 'u128',
        vestingScheduleIds: 'PalletVestingVestingScheduleIdSet'
      }
    }
  },
  /**
   * Lookup341: pallet_vesting::types::VestingScheduleInfo<BlockNumber, Moment, Balance>
   **/
  PalletVestingVestingScheduleInfo: {
    window: 'PalletVestingVestingWindow',
    periodCount: 'u32',
    perPeriod: 'Compact<u128>'
  },
  /**
   * Lookup343: pallet_bonded_finance::pallet::Call<T>
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
   * Lookup344: composable_traits::bonded_finance::BondOffer<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, Balance, BlockNumber>
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
   * Lookup345: composable_traits::bonded_finance::BondDuration<BlockNumber>
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
   * Lookup346: composable_traits::bonded_finance::BondOfferReward<primitives::currency::CurrencyId, Balance, BlockNumber>
   **/
  ComposableTraitsBondedFinanceBondOfferReward: {
    asset: 'u128',
    amount: 'u128',
    maturity: 'u32'
  },
  /**
   * Lookup347: pallet_assets_registry::pallet::Call<T>
   **/
  PalletAssetsRegistryCall: {
    _enum: {
      register_asset: {
        protocolId: '[u8;4]',
        nonce: 'u64',
        location: 'Option<PrimitivesCurrencyForeignAssetId>',
        assetInfo: 'ComposableTraitsAssetsAssetInfo',
      },
      update_asset: {
        assetId: 'u128',
        assetInfo: 'ComposableTraitsAssetsAssetInfoUpdate',
      },
      set_min_fee: {
        targetParachainId: 'u32',
        foreignAssetId: 'PrimitivesCurrencyForeignAssetId',
        amount: 'Option<u128>',
      },
      update_asset_location: {
        assetId: 'u128',
        location: 'Option<PrimitivesCurrencyForeignAssetId>'
      }
    }
  },
  /**
   * Lookup348: pallet_pablo::pallet::Call<T>
   **/
  PalletPabloCall: {
    _enum: {
      create: {
        pool: 'PalletPabloPoolInitConfiguration',
      },
      buy: {
        poolId: 'u128',
        inAssetId: 'u128',
        outAsset: 'ComposableTraitsDexAssetAmount',
        keepAlive: 'bool',
      },
      swap: {
        poolId: 'u128',
        inAsset: 'ComposableTraitsDexAssetAmount',
        minReceive: 'ComposableTraitsDexAssetAmount',
        keepAlive: 'bool',
      },
      add_liquidity: {
        poolId: 'u128',
        assets: 'BTreeMap<u128, u128>',
        minMintAmount: 'u128',
        keepAlive: 'bool',
      },
      remove_liquidity: {
        poolId: 'u128',
        lpAmount: 'u128',
        minReceive: 'BTreeMap<u128, u128>',
      },
      enable_twap: {
        poolId: 'u128'
      }
    }
  },
  /**
   * Lookup349: pallet_pablo::pallet::PoolInitConfiguration<sp_core::crypto::AccountId32, primitives::currency::CurrencyId>
   **/
  PalletPabloPoolInitConfiguration: {
    _enum: {
      DualAssetConstantProduct: {
        owner: 'AccountId32',
        assetsWeights: 'Vec<(u128,Permill)>',
        fee: 'Permill'
      }
    }
  },
  /**
   * Lookup350: composable_traits::dex::AssetAmount<primitives::currency::CurrencyId, Balance>
   **/
  ComposableTraitsDexAssetAmount: {
    assetId: 'u128',
    amount: 'u128'
  },
  /**
   * Lookup351: pallet_oracle::pallet::Call<T>
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
        who: 'AccountId32',
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
        assetId: 'u128',
      },
      remove_signer: {
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup352: pallet_assets_transactor_router::pallet::Call<T>
   **/
  PalletAssetsTransactorRouterCall: {
    _enum: {
      transfer: {
        asset: 'u128',
        dest: 'MultiAddress',
        amount: 'u128',
        keepAlive: 'bool',
      },
      transfer_native: {
        dest: 'MultiAddress',
        value: 'u128',
        keepAlive: 'bool',
      },
      force_transfer: {
        asset: 'u128',
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'u128',
        keepAlive: 'bool',
      },
      force_transfer_native: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'u128',
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
      mint_into: {
        assetId: 'u128',
        dest: 'MultiAddress',
        amount: 'u128',
      },
      burn_from: {
        assetId: 'u128',
        dest: 'MultiAddress',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup353: reward::pallet::Call<T, I>
   **/
  RewardCall: 'Null',
  /**
   * Lookup354: farming::pallet::Call<T>
   **/
  FarmingCall: {
    _enum: {
      update_reward_schedule: {
        poolCurrencyId: 'u128',
        rewardCurrencyId: 'u128',
        periodCount: 'u32',
        amount: 'Compact<u128>',
      },
      remove_reward_schedule: {
        poolCurrencyId: 'u128',
        rewardCurrencyId: 'u128',
      },
      deposit: {
        poolCurrencyId: 'u128',
        amount: 'u128',
      },
      withdraw: {
        poolCurrencyId: 'u128',
        amount: 'u128',
      },
      claim: {
        poolCurrencyId: 'u128',
        rewardCurrencyId: 'u128'
      }
    }
  },
  /**
   * Lookup355: pallet_referenda::pallet::Call<T, I>
   **/
  PalletReferendaCall: {
    _enum: {
      submit: {
        proposalOrigin: 'PicassoRuntimeOriginCaller',
        proposal: 'FrameSupportPreimagesBounded',
        enactmentMoment: 'FrameSupportScheduleDispatchTime',
      },
      place_decision_deposit: {
        index: 'u32',
      },
      refund_decision_deposit: {
        index: 'u32',
      },
      cancel: {
        index: 'u32',
      },
      kill: {
        index: 'u32',
      },
      nudge_referendum: {
        index: 'u32',
      },
      one_fewer_deciding: {
        track: 'u16',
      },
      refund_submission_deposit: {
        index: 'u32',
      },
      set_metadata: {
        index: 'u32',
        maybeHash: 'Option<H256>'
      }
    }
  },
  /**
   * Lookup356: frame_support::traits::schedule::DispatchTime<BlockNumber>
   **/
  FrameSupportScheduleDispatchTime: {
    _enum: {
      At: 'u32',
      After: 'u32'
    }
  },
  /**
   * Lookup357: pallet_conviction_voting::pallet::Call<T, I>
   **/
  PalletConvictionVotingCall: {
    _enum: {
      vote: {
        pollIndex: 'Compact<u32>',
        vote: 'PalletConvictionVotingVoteAccountVote',
      },
      delegate: {
        class: 'u16',
        to: 'MultiAddress',
        conviction: 'PalletConvictionVotingConviction',
        balance: 'u128',
      },
      undelegate: {
        class: 'u16',
      },
      unlock: {
        class: 'u16',
        target: 'MultiAddress',
      },
      remove_vote: {
        class: 'Option<u16>',
        index: 'u32',
      },
      remove_other_vote: {
        target: 'MultiAddress',
        class: 'u16',
        index: 'u32'
      }
    }
  },
  /**
   * Lookup358: pallet_conviction_voting::vote::AccountVote<Balance>
   **/
  PalletConvictionVotingVoteAccountVote: {
    _enum: {
      Standard: {
        vote: 'Vote',
        balance: 'u128',
      },
      Split: {
        aye: 'u128',
        nay: 'u128',
      },
      SplitAbstain: {
        aye: 'u128',
        nay: 'u128',
        abstain: 'u128'
      }
    }
  },
  /**
   * Lookup360: pallet_conviction_voting::conviction::Conviction
   **/
  PalletConvictionVotingConviction: {
    _enum: ['None', 'Locked1x', 'Locked2x', 'Locked3x', 'Locked4x', 'Locked5x', 'Locked6x']
  },
  /**
   * Lookup363: pallet_whitelist::pallet::Call<T>
   **/
  PalletWhitelistCall: {
    _enum: {
      whitelist_call: {
        callHash: 'H256',
      },
      remove_whitelisted_call: {
        callHash: 'H256',
      },
      dispatch_whitelisted_call: {
        callHash: 'H256',
        callEncodedLen: 'u32',
        callWeightWitness: 'SpWeightsWeightV2Weight',
      },
      dispatch_whitelisted_call_with_preimage: {
        call: 'Call'
      }
    }
  },
  /**
   * Lookup364: pallet_call_filter::pallet::Call<T>
   **/
  PalletCallFilterCall: {
    _enum: {
      disable: {
        entry: 'PalletCallFilterCallFilterEntry',
      },
      enable: {
        entry: 'PalletCallFilterCallFilterEntry'
      }
    }
  },
  /**
   * Lookup365: pallet_call_filter::types::CallFilterEntry<common::MaxStringSize>
   **/
  PalletCallFilterCallFilterEntry: {
    palletName: 'Bytes',
    functionName: 'Bytes'
  },
  /**
   * Lookup366: common::MaxStringSize
   **/
  CommonMaxStringSize: 'Null',
  /**
   * Lookup368: pallet_cosmwasm::pallet::Call<T>
   **/
  PalletCosmwasmCall: {
    _enum: {
      upload: {
        code: 'Bytes',
      },
      instantiate: {
        codeIdentifier: 'PalletCosmwasmCodeIdentifier',
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
        message: 'Bytes',
      },
      migrate: {
        contract: 'AccountId32',
        newCodeIdentifier: 'PalletCosmwasmCodeIdentifier',
        gas: 'u64',
        message: 'Bytes',
      },
      update_admin: {
        contract: 'AccountId32',
        newAdmin: 'Option<AccountId32>',
        gas: 'u64'
      }
    }
  },
  /**
   * Lookup370: pallet_cosmwasm::types::CodeIdentifier
   **/
  PalletCosmwasmCodeIdentifier: {
    _enum: {
      CodeId: 'u64',
      CodeHash: '[u8;32]'
    }
  },
  /**
   * Lookup379: pallet_ibc::pallet::Call<T>
   **/
  PalletIbcCall: {
    _enum: {
      deliver: {
        messages: 'Vec<PalletIbcAny>',
      },
      transfer: {
        params: 'PalletIbcTransferParams',
        assetId: 'u128',
        amount: 'u128',
        memo: 'Option<Text>',
      },
      __Unused2: 'Null',
      upgrade_client: {
        params: 'PalletIbcUpgradeParams',
      },
      freeze_client: {
        clientId: 'Bytes',
        height: 'u64',
      },
      increase_counters: 'Null',
      add_channels_to_feeless_channel_list: {
        sourceChannel: 'u64',
        destinationChannel: 'u64',
      },
      remove_channels_from_feeless_channel_list: {
        sourceChannel: 'u64',
        destinationChannel: 'u64',
      },
      set_child_storage: {
        key: 'Bytes',
        value: 'Bytes',
      },
      substitute_client_state: {
        clientId: 'Text',
        height: 'IbcCoreIcs02ClientHeight',
        clientStateBytes: 'Bytes',
        consensusStateBytes: 'Bytes'
      }
    }
  },
  /**
   * Lookup381: pallet_ibc::Any
   **/
  PalletIbcAny: {
    typeUrl: 'Text',
    value: 'Bytes'
  },
  /**
   * Lookup382: pallet_ibc::TransferParams<sp_core::crypto::AccountId32>
   **/
  PalletIbcTransferParams: {
    to: 'PalletIbcMultiAddress',
    sourceChannel: 'u64',
    timeout: 'IbcPrimitivesTimeout'
  },
  /**
   * Lookup383: pallet_ibc::MultiAddress<sp_core::crypto::AccountId32>
   **/
  PalletIbcMultiAddress: {
    _enum: {
      Id: 'AccountId32',
      Raw: 'Bytes'
    }
  },
  /**
   * Lookup384: ibc_primitives::Timeout
   **/
  IbcPrimitivesTimeout: {
    _enum: {
      Offset: {
        timestamp: 'Option<u64>',
        height: 'Option<u64>',
      },
      Absolute: {
        timestamp: 'Option<u64>',
        height: 'Option<u64>'
      }
    }
  },
  /**
   * Lookup387: pallet_ibc::UpgradeParams
   **/
  PalletIbcUpgradeParams: {
    clientState: 'Bytes',
    consensusState: 'Bytes'
  },
  /**
   * Lookup388: ibc::core::ics02_client::height::Height
   **/
  IbcCoreIcs02ClientHeight: {
    revisionNumber: 'u64',
    revisionHeight: 'u64'
  },
  /**
   * Lookup389: pallet_ibc::ics20_fee::pallet::Call<T>
   **/
  PalletIbcIcs20FeePalletCall: {
    _enum: {
      set_charge: {
        charge: 'Perbill',
      },
      add_channels_to_feeless_channel_list: {
        sourceChannel: 'u64',
        destinationChannel: 'u64',
      },
      remove_channels_from_feeless_channel_list: {
        sourceChannel: 'u64',
        destinationChannel: 'u64'
      }
    }
  },
  /**
   * Lookup391: pallet_multihop_xcm_ibc::pallet::Call<T>
   **/
  PalletMultihopXcmIbcCall: {
    _enum: {
      add_route: {
        routeId: 'u128',
        route: 'Vec<(ComposableTraitsXcmMemoChainInfo,Bytes)>'
      }
    }
  },
  /**
   * Lookup394: composable_traits::xcm::memo::ChainInfo
   **/
  ComposableTraitsXcmMemoChainInfo: {
    chainId: 'u32',
    order: 'u8',
    channelId: 'u64',
    timestamp: 'Option<u64>',
    height: 'Option<u64>',
    retries: 'Option<u8>',
    timeout: 'Option<u64>',
    chainHop: 'ComposableTraitsXcmMemoChainHop',
    paraId: 'Option<u32>'
  },
  /**
   * Lookup395: composable_traits::xcm::memo::ChainHop
   **/
  ComposableTraitsXcmMemoChainHop: {
    _enum: ['SubstrateIbc', 'CosmosIbc', 'Xcm']
  },
  /**
   * Lookup399: pallet_conviction_voting::types::Tally<Votes, Total>
   **/
  PalletConvictionVotingTally: {
    ayes: 'u128',
    nays: 'u128',
    support: 'u128'
  },
  /**
   * Lookup400: pallet_conviction_voting::pallet::Event<T, I>
   **/
  PalletConvictionVotingEvent: {
    _enum: {
      Delegated: '(AccountId32,AccountId32)',
      Undelegated: 'AccountId32'
    }
  },
  /**
   * Lookup402: pallet_whitelist::pallet::Event<T>
   **/
  PalletWhitelistEvent: {
    _enum: {
      CallWhitelisted: {
        callHash: 'H256',
      },
      WhitelistedCallRemoved: {
        callHash: 'H256',
      },
      WhitelistedCallDispatched: {
        callHash: 'H256',
        result: 'Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo>'
      }
    }
  },
  /**
   * Lookup404: frame_support::dispatch::PostDispatchInfo
   **/
  FrameSupportDispatchPostDispatchInfo: {
    actualWeight: 'Option<SpWeightsWeightV2Weight>',
    paysFee: 'FrameSupportDispatchPays'
  },
  /**
   * Lookup406: sp_runtime::DispatchErrorWithPostInfo<frame_support::dispatch::PostDispatchInfo>
   **/
  SpRuntimeDispatchErrorWithPostInfo: {
    postInfo: 'FrameSupportDispatchPostDispatchInfo',
    error: 'SpRuntimeDispatchError'
  },
  /**
   * Lookup407: pallet_call_filter::pallet::Event<T>
   **/
  PalletCallFilterEvent: {
    _enum: {
      Disabled: {
        entry: 'PalletCallFilterCallFilterEntry',
      },
      Enabled: {
        entry: 'PalletCallFilterCallFilterEntry'
      }
    }
  },
  /**
   * Lookup408: pallet_cosmwasm::pallet::Event<T>
   **/
  PalletCosmwasmEvent: {
    _enum: {
      Uploaded: {
        codeHash: '[u8;32]',
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
        attributes: 'Vec<(Bytes,Bytes)>',
      },
      Migrated: {
        contract: 'AccountId32',
        to: 'u64',
      },
      AdminUpdated: {
        contract: 'AccountId32',
        newAdmin: 'Option<AccountId32>'
      }
    }
  },
  /**
   * Lookup409: pallet_cosmwasm::types::ContractInfo<sp_core::crypto::AccountId32, bounded_collections::bounded_vec::BoundedVec<T, S>, bounded_collections::bounded_vec::BoundedVec<T, S>>
   **/
  PalletCosmwasmContractInfo: {
    codeId: 'u64',
    trieId: 'Bytes',
    instantiator: 'AccountId32',
    admin: 'Option<AccountId32>',
    label: 'Bytes'
  },
  /**
   * Lookup411: pallet_cosmwasm::types::EntryPoint
   **/
  PalletCosmwasmEntryPoint: {
    _enum: ['Instantiate', 'Execute', 'Migrate', 'Reply', 'IbcChannelOpen', 'IbcChannelConnect', 'IbcChannelClose', 'IbcPacketTimeout', 'IbcPacketReceive', 'IbcPacketAck']
  },
  /**
   * Lookup413: pallet_ibc::pallet::Event<T>
   **/
  PalletIbcEvent: {
    _enum: {
      Events: {
        events: 'Vec<Result<PalletIbcEventsIbcEvent, PalletIbcErrorsIbcError>>',
      },
      TokenTransferInitiated: {
        from: 'Bytes',
        to: 'Bytes',
        ibcDenom: 'Bytes',
        localAssetId: 'Option<u128>',
        amount: 'u128',
        isSenderSource: 'bool',
        sourceChannel: 'Bytes',
        destinationChannel: 'Bytes',
      },
      ChannelOpened: {
        channelId: 'Bytes',
        portId: 'Bytes',
      },
      ParamsUpdated: {
        sendEnabled: 'bool',
        receiveEnabled: 'bool',
      },
      TokenTransferCompleted: {
        from: 'Text',
        to: 'Text',
        ibcDenom: 'Bytes',
        localAssetId: 'Option<u128>',
        amount: 'u128',
        isSenderSource: 'bool',
        sourceChannel: 'Bytes',
        destinationChannel: 'Bytes',
      },
      TokenReceived: {
        from: 'Text',
        to: 'Text',
        ibcDenom: 'Bytes',
        localAssetId: 'Option<u128>',
        amount: 'u128',
        isReceiverSource: 'bool',
        sourceChannel: 'Bytes',
        destinationChannel: 'Bytes',
      },
      TokenTransferFailed: {
        from: 'Text',
        to: 'Text',
        ibcDenom: 'Bytes',
        localAssetId: 'Option<u128>',
        amount: 'u128',
        isSenderSource: 'bool',
        sourceChannel: 'Bytes',
        destinationChannel: 'Bytes',
      },
      TokenTransferTimeout: {
        from: 'Text',
        to: 'Text',
        ibcDenom: 'Bytes',
        localAssetId: 'Option<u128>',
        amount: 'u128',
        isSenderSource: 'bool',
        sourceChannel: 'Bytes',
        destinationChannel: 'Bytes',
      },
      OnRecvPacketError: {
        msg: 'Bytes',
      },
      ClientUpgradeSet: 'Null',
      ClientFrozen: {
        clientId: 'Bytes',
        height: 'u64',
        revisionNumber: 'u64',
      },
      AssetAdminUpdated: {
        adminAccount: 'AccountId32',
      },
      FeeLessChannelIdsAdded: {
        sourceChannel: 'u64',
        destinationChannel: 'u64',
      },
      FeeLessChannelIdsRemoved: {
        sourceChannel: 'u64',
        destinationChannel: 'u64',
      },
      ChargingFeeOnTransferInitiated: {
        sequence: 'u64',
        from: 'Bytes',
        to: 'Bytes',
        ibcDenom: 'Bytes',
        localAssetId: 'Option<u128>',
        amount: 'u128',
        isFlatFee: 'bool',
        sourceChannel: 'Bytes',
        destinationChannel: 'Bytes',
      },
      ChargingFeeConfirmed: {
        sequence: 'u64',
      },
      ChargingFeeTimeout: {
        sequence: 'u64',
      },
      ChargingFeeFailedAcknowledgement: {
        sequence: 'u64',
      },
      ChildStateUpdated: 'Null',
      ClientStateSubstituted: {
        clientId: 'Text',
        height: 'IbcCoreIcs02ClientHeight',
      },
      ExecuteMemoStarted: {
        accountId: 'AccountId32',
        memo: 'Option<Text>',
      },
      ExecuteMemoIbcTokenTransferSuccess: {
        from: 'AccountId32',
        to: 'Bytes',
        assetId: 'u128',
        amount: 'u128',
        channel: 'u64',
        nextMemo: 'Option<Text>',
      },
      ExecuteMemoIbcTokenTransferFailedWithReason: {
        from: 'AccountId32',
        memo: 'Text',
        reason: 'u8',
      },
      ExecuteMemoIbcTokenTransferFailed: {
        from: 'AccountId32',
        to: 'Bytes',
        assetId: 'u128',
        amount: 'u128',
        channel: 'u64',
        nextMemo: 'Option<Text>',
      },
      ExecuteMemoXcmSuccess: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        assetId: 'u128',
        paraId: 'Option<u32>',
      },
      ExecuteMemoXcmFailed: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        assetId: 'u128',
        paraId: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup416: pallet_ibc::events::IbcEvent
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
        moduleId: 'Bytes',
      },
      PushWasmCode: {
        wasmCodeId: 'Bytes'
      }
    }
  },
  /**
   * Lookup417: pallet_ibc::errors::IbcError
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
   * Lookup419: pallet_ibc::ics20_fee::pallet::Event<T>
   **/
  PalletIbcIcs20FeePalletEvent: {
    _enum: {
      IbcTransferFeeCollected: {
        amount: 'u128',
        assetId: 'u128',
      },
      FeeLessChannelIdsAdded: {
        sourceChannel: 'u64',
        destinationChannel: 'u64',
      },
      FeeLessChannelIdsRemoved: {
        sourceChannel: 'u64',
        destinationChannel: 'u64'
      }
    }
  },
  /**
   * Lookup420: pallet_multihop_xcm_ibc::pallet::Event<T>
   **/
  PalletMultihopXcmIbcEvent: {
    _enum: {
      SuccessXcmToIbc: {
        originAddress: 'AccountId32',
        to: '[u8;32]',
        amount: 'u128',
        assetId: 'u128',
        memo: 'Option<Text>',
      },
      FailedXcmToIbc: {
        originAddress: 'AccountId32',
        to: '[u8;32]',
        amount: 'u128',
        assetId: 'u128',
        memo: 'Option<Text>',
      },
      FailedCallback: {
        originAddress: '[u8;32]',
        routeId: 'u128',
        reason: 'PalletMultihopXcmIbcMultihopEventReason',
      },
      MultihopXcmMemo: {
        reason: 'PalletMultihopXcmIbcMultihopEventReason',
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        assetId: 'u128',
        isError: 'bool',
      },
      FailedMatchLocation: 'Null'
    }
  },
  /**
   * Lookup421: pallet_multihop_xcm_ibc::pallet::MultihopEventReason
   **/
  PalletMultihopXcmIbcMultihopEventReason: {
    _enum: ['FailedToConvertAddressToBytes', 'XcmTransferInitiated', 'IncorrectPalletId', 'MultiHopRouteDoesNotExist', 'MultiHopRouteExistButNotConfigured', 'IncorrectCountOfAddresses', 'FailedToDeriveCosmosAddressFromBytes', 'FailedToDeriveChainNameFromUtf8', 'FailedToEncodeBech32Address', 'FailedToDecodeDestAccountId', 'FailedToDecodeSenderAccountId', 'DoesNotSupportNonFungible', 'FailedCreateMemo', 'FailedToConvertMemoIntoPalletIbcMemoMessageType']
  },
  /**
   * Lookup422: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup425: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup426: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'SpWeightsWeightV2Weight',
    maxBlock: 'SpWeightsWeightV2Weight',
    perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup427: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup428: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'SpWeightsWeightV2Weight',
    maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
    maxTotal: 'Option<SpWeightsWeightV2Weight>',
    reserved: 'Option<SpWeightsWeightV2Weight>'
  },
  /**
   * Lookup429: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportDispatchPerDispatchClassU32'
  },
  /**
   * Lookup430: frame_support::dispatch::PerDispatchClass<T>
   **/
  FrameSupportDispatchPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup431: sp_weights::RuntimeDbWeight
   **/
  SpWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup432: sp_version::RuntimeVersion
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
   * Lookup436: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup437: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup438: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup440: pallet_indices::pallet::Error<T>
   **/
  PalletIndicesError: {
    _enum: ['NotAssigned', 'NotOwner', 'InUse', 'NotTransfer', 'Permanent']
  },
  /**
   * Lookup442: pallet_balances::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup443: pallet_balances::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup446: pallet_balances::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup448: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'KeepAlive', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup449: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
   **/
  PalletIdentityRegistration: {
    judgements: 'Vec<(u32,PalletIdentityJudgement)>',
    deposit: 'u128',
    info: 'PalletIdentityIdentityInfo'
  },
  /**
   * Lookup457: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
   **/
  PalletIdentityRegistrarInfo: {
    account: 'AccountId32',
    fee: 'u128',
    fields: 'PalletIdentityBitFlags'
  },
  /**
   * Lookup459: pallet_identity::pallet::Error<T>
   **/
  PalletIdentityError: {
    _enum: ['TooManySubAccounts', 'NotFound', 'NotNamed', 'EmptyIndex', 'FeeChanged', 'NoIdentity', 'StickyJudgement', 'JudgementGiven', 'InvalidJudgement', 'InvalidIndex', 'InvalidTarget', 'TooManyFields', 'TooManyRegistrars', 'AlreadyClaimed', 'NotSub', 'NotOwned', 'JudgementForDifferentIdentity', 'JudgementPaymentFailed']
  },
  /**
   * Lookup461: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
   **/
  PalletMultisigMultisig: {
    when: 'PalletMultisigTimepoint',
    deposit: 'u128',
    depositor: 'AccountId32',
    approvals: 'Vec<AccountId32>'
  },
  /**
   * Lookup463: pallet_multisig::pallet::Error<T>
   **/
  PalletMultisigError: {
    _enum: ['MinimumThreshold', 'AlreadyApproved', 'NoApprovalsNeeded', 'TooFewSignatories', 'TooManySignatories', 'SignatoriesOutOfOrder', 'SenderInSignatories', 'NotFound', 'NotOwner', 'NoTimepoint', 'WrongTimepoint', 'UnexpectedTimepoint', 'MaxWeightTooLow', 'AlreadyStored']
  },
  /**
   * Lookup465: polkadot_primitives::v2::UpgradeRestriction
   **/
  PolkadotPrimitivesV2UpgradeRestriction: {
    _enum: ['Present']
  },
  /**
   * Lookup466: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
   **/
  CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
    dmqMqcHead: 'H256',
    relayDispatchQueueSize: '(u32,u32)',
    ingressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>',
    egressChannels: 'Vec<(u32,PolkadotPrimitivesV2AbridgedHrmpChannel)>'
  },
  /**
   * Lookup469: polkadot_primitives::v2::AbridgedHrmpChannel
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
   * Lookup470: polkadot_primitives::v2::AbridgedHostConfiguration
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
   * Lookup476: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
   **/
  PolkadotCorePrimitivesOutboundHrmpMessage: {
    recipient: 'u32',
    data: 'Bytes'
  },
  /**
   * Lookup477: cumulus_pallet_parachain_system::pallet::Error<T>
   **/
  CumulusPalletParachainSystemError: {
    _enum: ['OverlappingUpgrades', 'ProhibitedByPolkadot', 'TooBig', 'ValidationDataNotAvailable', 'HostConfigurationNotAvailable', 'NotScheduled', 'NothingAuthorized', 'Unauthorized']
  },
  /**
   * Lookup480: pallet_collator_selection::pallet::CandidateInfo<sp_core::crypto::AccountId32, Balance>
   **/
  PalletCollatorSelectionCandidateInfo: {
    who: 'AccountId32',
    deposit: 'u128'
  },
  /**
   * Lookup482: pallet_collator_selection::pallet::Error<T>
   **/
  PalletCollatorSelectionError: {
    _enum: ['TooManyCandidates', 'TooFewCandidates', 'Unknown', 'Permission', 'AlreadyCandidate', 'NotCandidate', 'TooManyInvulnerables', 'AlreadyInvulnerable', 'NoAssociatedValidatorId', 'ValidatorNotRegistered']
  },
  /**
   * Lookup487: sp_core::crypto::KeyTypeId
   **/
  SpCoreCryptoKeyTypeId: '[u8;4]',
  /**
   * Lookup488: pallet_session::pallet::Error<T>
   **/
  PalletSessionError: {
    _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount']
  },
  /**
   * Lookup493: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletCollectiveVotes: {
    index: 'u32',
    threshold: 'u32',
    ayes: 'Vec<AccountId32>',
    nays: 'Vec<AccountId32>',
    end: 'u32'
  },
  /**
   * Lookup494: pallet_collective::pallet::Error<T, I>
   **/
  PalletCollectiveError: {
    _enum: ['NotMember', 'DuplicateProposal', 'ProposalMissing', 'WrongIndex', 'DuplicateVote', 'AlreadyInitialized', 'TooEarly', 'TooManyProposals', 'WrongProposalWeight', 'WrongProposalLength']
  },
  /**
   * Lookup496: pallet_membership::pallet::Error<T, I>
   **/
  PalletMembershipError: {
    _enum: ['AlreadyMember', 'NotMember', 'TooManyMembers']
  },
  /**
   * Lookup497: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
   **/
  PalletTreasuryProposal: {
    proposer: 'AccountId32',
    value: 'u128',
    beneficiary: 'AccountId32',
    bond: 'u128'
  },
  /**
   * Lookup499: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup500: pallet_treasury::pallet::Error<T, I>
   **/
  PalletTreasuryError: {
    _enum: ['InsufficientProposersBalance', 'InvalidIndex', 'TooManyApprovals', 'InsufficientPermission', 'ProposalNotApproved']
  },
  /**
   * Lookup506: pallet_democracy::types::ReferendumInfo<BlockNumber, frame_support::traits::preimages::Bounded<picasso_runtime::RuntimeCall>, Balance>
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
   * Lookup507: pallet_democracy::types::ReferendumStatus<BlockNumber, frame_support::traits::preimages::Bounded<picasso_runtime::RuntimeCall>, Balance>
   **/
  PalletDemocracyReferendumStatus: {
    end: 'u32',
    proposal: 'FrameSupportPreimagesBounded',
    threshold: 'PalletDemocracyVoteThreshold',
    delay: 'u32',
    tally: 'PalletDemocracyTally'
  },
  /**
   * Lookup508: pallet_democracy::types::Tally<Balance>
   **/
  PalletDemocracyTally: {
    ayes: 'u128',
    nays: 'u128',
    turnout: 'u128'
  },
  /**
   * Lookup509: pallet_democracy::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber, MaxVotes>
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
   * Lookup513: pallet_democracy::types::Delegations<Balance>
   **/
  PalletDemocracyDelegations: {
    votes: 'u128',
    capital: 'u128'
  },
  /**
   * Lookup514: pallet_democracy::vote::PriorLock<BlockNumber, Balance>
   **/
  PalletDemocracyVotePriorLock: '(u32,u128)',
  /**
   * Lookup517: pallet_democracy::pallet::Error<T>
   **/
  PalletDemocracyError: {
    _enum: ['ValueLow', 'ProposalMissing', 'AlreadyCanceled', 'DuplicateProposal', 'ProposalBlacklisted', 'NotSimpleMajority', 'InvalidHash', 'NoProposal', 'AlreadyVetoed', 'ReferendumInvalid', 'NoneWaiting', 'NotVoter', 'NoPermission', 'AlreadyDelegating', 'InsufficientFunds', 'NotDelegating', 'VotesExist', 'InstantNotAllowed', 'Nonsense', 'WrongUpperBound', 'MaxVotesReached', 'TooMany', 'VotingPeriodLow', 'PreimageNotExist']
  },
  /**
   * Lookup525: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<picasso_runtime::RuntimeCall>, BlockNumber, picasso_runtime::OriginCaller, sp_core::crypto::AccountId32>
   **/
  PalletSchedulerScheduled: {
    maybeId: 'Option<[u8;32]>',
    priority: 'u8',
    call: 'FrameSupportPreimagesBounded',
    maybePeriodic: 'Option<(u32,u32)>',
    origin: 'PicassoRuntimeOriginCaller'
  },
  /**
   * Lookup527: pallet_scheduler::pallet::Error<T>
   **/
  PalletSchedulerError: {
    _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange', 'Named']
  },
  /**
   * Lookup528: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup529: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
   **/
  PalletPreimageRequestStatus: {
    _enum: {
      Unrequested: {
        deposit: '(AccountId32,u128)',
        len: 'u32',
      },
      Requested: {
        deposit: 'Option<(AccountId32,u128)>',
        count: 'u32',
        len: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup534: pallet_preimage::pallet::Error<T>
   **/
  PalletPreimageError: {
    _enum: ['TooBig', 'AlreadyNoted', 'NotAuthorized', 'NotNoted', 'Requested', 'NotRequested']
  },
  /**
   * Lookup537: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, composable_traits::account_proxy::ProxyType, BlockNumber>
   **/
  PalletProxyProxyDefinition: {
    delegate: 'AccountId32',
    proxyType: 'ComposableTraitsAccountProxyProxyType',
    delay: 'u32'
  },
  /**
   * Lookup541: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
   **/
  PalletProxyAnnouncement: {
    real: 'AccountId32',
    callHash: 'H256',
    height: 'u32'
  },
  /**
   * Lookup543: pallet_proxy::pallet::Error<T>
   **/
  PalletProxyError: {
    _enum: ['TooMany', 'NotFound', 'NotProxy', 'Unproxyable', 'Duplicate', 'NoPermission', 'Unannounced', 'NoSelfProxy']
  },
  /**
   * Lookup545: cumulus_pallet_xcmp_queue::InboundChannelDetails
   **/
  CumulusPalletXcmpQueueInboundChannelDetails: {
    sender: 'u32',
    state: 'CumulusPalletXcmpQueueInboundState',
    messageMetadata: 'Vec<(u32,PolkadotParachainPrimitivesXcmpMessageFormat)>'
  },
  /**
   * Lookup546: cumulus_pallet_xcmp_queue::InboundState
   **/
  CumulusPalletXcmpQueueInboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup549: polkadot_parachain::primitives::XcmpMessageFormat
   **/
  PolkadotParachainPrimitivesXcmpMessageFormat: {
    _enum: ['ConcatenatedVersionedXcm', 'ConcatenatedEncodedBlob', 'Signals']
  },
  /**
   * Lookup552: cumulus_pallet_xcmp_queue::OutboundChannelDetails
   **/
  CumulusPalletXcmpQueueOutboundChannelDetails: {
    recipient: 'u32',
    state: 'CumulusPalletXcmpQueueOutboundState',
    signalsExist: 'bool',
    firstIndex: 'u16',
    lastIndex: 'u16'
  },
  /**
   * Lookup553: cumulus_pallet_xcmp_queue::OutboundState
   **/
  CumulusPalletXcmpQueueOutboundState: {
    _enum: ['Ok', 'Suspended']
  },
  /**
   * Lookup555: cumulus_pallet_xcmp_queue::QueueConfigData
   **/
  CumulusPalletXcmpQueueQueueConfigData: {
    suspendThreshold: 'u32',
    dropThreshold: 'u32',
    resumeThreshold: 'u32',
    thresholdWeight: 'SpWeightsWeightV2Weight',
    weightRestrictDecay: 'SpWeightsWeightV2Weight',
    xcmpMaxIndividualWeight: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup557: cumulus_pallet_xcmp_queue::pallet::Error<T>
   **/
  CumulusPalletXcmpQueueError: {
    _enum: ['FailedToSend', 'BadXcmOrigin', 'BadXcm', 'BadOverweightIndex', 'WeightOverLimit']
  },
  /**
   * Lookup558: pallet_xcm::pallet::QueryStatus<BlockNumber>
   **/
  PalletXcmQueryStatus: {
    _enum: {
      Pending: {
        responder: 'XcmVersionedMultiLocation',
        maybeMatchQuerier: 'Option<XcmVersionedMultiLocation>',
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
   * Lookup562: xcm::VersionedResponse
   **/
  XcmVersionedResponse: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      V2: 'XcmV2Response',
      V3: 'XcmV3Response'
    }
  },
  /**
   * Lookup568: pallet_xcm::pallet::VersionMigrationStage
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
   * Lookup570: xcm::VersionedAssetId
   **/
  XcmVersionedAssetId: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      V3: 'XcmV3MultiassetAssetId'
    }
  },
  /**
   * Lookup571: pallet_xcm::pallet::RemoteLockedFungibleRecord
   **/
  PalletXcmRemoteLockedFungibleRecord: {
    amount: 'u128',
    owner: 'XcmVersionedMultiLocation',
    locker: 'XcmVersionedMultiLocation',
    users: 'u32'
  },
  /**
   * Lookup575: pallet_xcm::pallet::Error<T>
   **/
  PalletXcmError: {
    _enum: ['Unreachable', 'SendFailure', 'Filtered', 'UnweighableMessage', 'DestinationNotInvertible', 'Empty', 'CannotReanchor', 'TooManyAssets', 'InvalidOrigin', 'BadVersion', 'BadLocation', 'NoSubscription', 'AlreadySubscribed', 'InvalidAsset', 'LowBalance', 'TooManyLocks', 'AccountNotSovereign', 'FeesNotMet', 'LockNotFound', 'InUse']
  },
  /**
   * Lookup576: cumulus_pallet_xcm::pallet::Error<T>
   **/
  CumulusPalletXcmError: 'Null',
  /**
   * Lookup577: cumulus_pallet_dmp_queue::ConfigData
   **/
  CumulusPalletDmpQueueConfigData: {
    maxIndividual: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup578: cumulus_pallet_dmp_queue::PageIndexData
   **/
  CumulusPalletDmpQueuePageIndexData: {
    beginUsed: 'u32',
    endUsed: 'u32',
    overweightCount: 'u64'
  },
  /**
   * Lookup581: cumulus_pallet_dmp_queue::pallet::Error<T>
   **/
  CumulusPalletDmpQueueError: {
    _enum: ['Unknown', 'OverLimit']
  },
  /**
   * Lookup582: orml_xtokens::module::Error<T>
   **/
  OrmlXtokensModuleError: {
    _enum: ['AssetHasNoReserve', 'NotCrossChainTransfer', 'InvalidDest', 'NotCrossChainTransferableCurrency', 'UnweighableMessage', 'XcmExecutionFailed', 'CannotReanchor', 'InvalidAncestry', 'InvalidAsset', 'DestinationNotInvertible', 'BadVersion', 'DistinctReserveForAssetAndFee', 'ZeroFee', 'ZeroAmount', 'TooManyAssetsBeingSent', 'AssetIndexNonExistent', 'FeeNotEnough', 'NotSupportedMultiLocation', 'MinXcmFeeNotDefined']
  },
  /**
   * Lookup585: orml_unknown_tokens::module::Error<T>
   **/
  OrmlUnknownTokensModuleError: {
    _enum: ['BalanceTooLow', 'BalanceOverflow', 'UnhandledAsset']
  },
  /**
   * Lookup588: orml_tokens::BalanceLock<Balance>
   **/
  OrmlTokensBalanceLock: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup590: orml_tokens::AccountData<Balance>
   **/
  OrmlTokensAccountData: {
    free: 'u128',
    reserved: 'u128',
    frozen: 'u128'
  },
  /**
   * Lookup592: orml_tokens::ReserveData<ReserveIdentifier, Balance>
   **/
  OrmlTokensReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup594: orml_tokens::module::Error<T>
   **/
  OrmlTokensModuleError: {
    _enum: ['BalanceTooLow', 'AmountIntoBalanceFailed', 'LiquidityRestrictions', 'MaxLocksExceeded', 'KeepAlive', 'ExistentialDeposit', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup595: pallet_currency_factory::ranges::Ranges<primitives::currency::CurrencyId>
   **/
  PalletCurrencyFactoryRanges: {
    ranges: 'Vec<{"current":"u128","end":"u128"}>'
  },
  /**
   * Lookup598: pallet_currency_factory::pallet::Error<T>
   **/
  PalletCurrencyFactoryError: {
    _enum: ['AssetNotFound']
  },
  /**
   * Lookup599: pallet_crowdloan_rewards::models::Reward<Balance, Period>
   **/
  PalletCrowdloanRewardsModelsReward: {
    total: 'u128',
    claimed: 'u128',
    vestingPeriod: 'u64'
  },
  /**
   * Lookup600: pallet_crowdloan_rewards::pallet::Error<T>
   **/
  PalletCrowdloanRewardsError: {
    _enum: ['NotInitialized', 'AlreadyInitialized', 'BackToTheFuture', 'RewardsNotFunded', 'InvalidProof', 'InvalidClaim', 'NothingToClaim', 'NotAssociated', 'AlreadyAssociated', 'NotClaimableYet', 'UnexpectedRewardAmount']
  },
  /**
   * Lookup605: pallet_vesting::module::Error<T>
   **/
  PalletVestingModuleError: {
    _enum: ['ZeroVestingPeriod', 'ZeroVestingPeriodCount', 'InsufficientBalanceToLock', 'TooManyVestingSchedules', 'AmountLow', 'MaxVestingSchedulesExceeded', 'TryingToSelfVest', 'VestingScheduleNotFound']
  },
  /**
   * Lookup607: pallet_bonded_finance::pallet::Error<T>
   **/
  PalletBondedFinanceError: {
    _enum: ['BondOfferNotFound', 'InvalidBondOffer', 'OfferCompleted', 'InvalidNumberOfBonds']
  },
  /**
   * Lookup609: pallet_assets_registry::pallet::Error<T>
   **/
  PalletAssetsRegistryError: {
    _enum: ['AssetNotFound', 'AssetAlreadyRegistered', 'AssetLocationIsNone', 'StringExceedsMaxLength', 'LocationIsUsed']
  },
  /**
   * Lookup610: pallet_pablo::pallet::PoolConfiguration<sp_core::crypto::AccountId32, primitives::currency::CurrencyId>
   **/
  PalletPabloPoolConfiguration: {
    _enum: {
      DualAssetConstantProduct: 'ComposableTraitsDexBasicPoolInfo'
    }
  },
  /**
   * Lookup611: composable_traits::dex::BasicPoolInfo<sp_core::crypto::AccountId32, primitives::currency::CurrencyId, MaxAssets>
   **/
  ComposableTraitsDexBasicPoolInfo: {
    owner: 'AccountId32',
    assetsWeights: 'BTreeMap<u128, Permill>',
    lpToken: 'u128',
    feeConfig: 'ComposableTraitsDexFeeConfig'
  },
  /**
   * Lookup613: composable_traits::dex::FeeConfig
   **/
  ComposableTraitsDexFeeConfig: {
    feeRate: 'Permill',
    ownerFeeRate: 'Permill',
    protocolFeeRate: 'Permill'
  },
  /**
   * Lookup614: pallet_pablo::types::TimeWeightedAveragePrice<Timestamp, Balance>
   **/
  PalletPabloTimeWeightedAveragePrice: {
    timestamp: 'u64',
    basePriceCumulative: 'u128',
    quotePriceCumulative: 'u128',
    baseTwap: 'u128',
    quoteTwap: 'u128'
  },
  /**
   * Lookup615: pallet_pablo::types::PriceCumulative<Timestamp, Balance>
   **/
  PalletPabloPriceCumulative: {
    timestamp: 'u64',
    basePriceCumulative: 'u128',
    quotePriceCumulative: 'u128'
  },
  /**
   * Lookup616: pallet_pablo::pallet::Error<T>
   **/
  PalletPabloError: {
    _enum: ['PoolNotFound', 'NotEnoughLiquidity', 'NotEnoughLpToken', 'PairMismatch', 'AssetNotFound', 'MustBeOwner', 'InvalidSaleState', 'InvalidAmount', 'InvalidAsset', 'CannotRespectMinimumRequested', 'AssetAmountMustBePositiveNumber', 'InvalidPair', 'InvalidFees', 'AmpFactorMustBeGreaterThanZero', 'MissingAmount', 'MissingMinExpectedAmount', 'MoreThanTwoAssetsNotYetSupported', 'NoLpTokenForLbp', 'NoXTokenForLbp', 'WeightsMustBeNonZero', 'WeightsMustSumToOne', 'StakingPoolConfigError', 'IncorrectAssetAmounts', 'UnsupportedOperation', 'InitialDepositCannotBeZero', 'InitialDepositMustContainAllAssets', 'MinAmountsMustContainAtLeastOneAsset', 'MustDepositMinimumOneAsset', 'CannotSwapSameAsset', 'CannotBuyAssetWithItself', 'IncorrectPoolConfig']
  },
  /**
   * Lookup617: composable_traits::oracle::RewardTracker<Balance, Timestamp>
   **/
  ComposableTraitsOracleRewardTracker: {
    period: 'u64',
    start: 'u64',
    totalAlreadyRewarded: 'u128',
    currentBlockReward: 'u128',
    totalRewardWeight: 'u128'
  },
  /**
   * Lookup618: pallet_oracle::pallet::Withdraw<Balance, BlockNumber>
   **/
  PalletOracleWithdraw: {
    stake: 'u128',
    unlockBlock: 'u32'
  },
  /**
   * Lookup619: composable_traits::oracle::Price<PriceValue, BlockNumber>
   **/
  ComposableTraitsOraclePrice: {
    price: 'u128',
    block: 'u32'
  },
  /**
   * Lookup623: pallet_oracle::pallet::PrePrice<PriceValue, BlockNumber, sp_core::crypto::AccountId32>
   **/
  PalletOraclePrePrice: {
    price: 'u128',
    block: 'u32',
    who: 'AccountId32'
  },
  /**
   * Lookup625: pallet_oracle::pallet::AssetInfo<sp_arithmetic::per_things::Percent, BlockNumber, Balance>
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
   * Lookup626: pallet_oracle::pallet::Error<T>
   **/
  PalletOracleError: {
    _enum: ['Unknown', 'NoPermission', 'NoStake', 'StakeLocked', 'NotEnoughStake', 'NotEnoughFunds', 'InvalidAssetId', 'AlreadySubmitted', 'MaxPrices', 'PriceNotRequested', 'UnsetSigner', 'AlreadySet', 'UnsetController', 'ControllerUsed', 'SignerUsed', 'AvoidPanic', 'ExceedMaxAnswers', 'InvalidMinAnswers', 'MaxAnswersLessThanMinAnswers', 'ExceedThreshold', 'ExceedAssetsCount', 'PriceNotFound', 'ExceedStake', 'MustSumTo100', 'DepthTooLarge', 'ArithmeticError', 'BlockIntervalLength', 'TransferError', 'MaxHistory', 'MaxPrePrices', 'NoRewardTrackerSet', 'AnnualRewardLessThanAlreadyRewarded']
  },
  /**
   * Lookup627: pallet_assets_transactor_router::pallet::Error<T>
   **/
  PalletAssetsTransactorRouterError: {
    _enum: ['CannotSetNewCurrencyToRegistry', 'InvalidCurrency']
  },
  /**
   * Lookup634: reward::pallet::Error<T, I>
   **/
  RewardError: {
    _enum: ['TryIntoIntError', 'InsufficientFunds', 'ZeroTotalStake', 'MaxRewardCurrencies']
  },
  /**
   * Lookup635: farming::RewardSchedule<Balance>
   **/
  FarmingRewardSchedule: {
    periodCount: 'u32',
    perPeriod: 'Compact<u128>'
  },
  /**
   * Lookup636: farming::pallet::Error<T>
   **/
  FarmingError: {
    _enum: ['InsufficientStake']
  },
  /**
   * Lookup637: pallet_referenda::types::ReferendumInfo<TrackId, picasso_runtime::OriginCaller, Moment, frame_support::traits::preimages::Bounded<picasso_runtime::RuntimeCall>, Balance, pallet_conviction_voting::types::Tally<Votes, Total>, sp_core::crypto::AccountId32, ScheduleAddress>
   **/
  PalletReferendaReferendumInfo: {
    _enum: {
      Ongoing: 'PalletReferendaReferendumStatus',
      Approved: '(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)',
      Rejected: '(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)',
      Cancelled: '(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)',
      TimedOut: '(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)',
      Killed: 'u32'
    }
  },
  /**
   * Lookup638: pallet_referenda::types::ReferendumStatus<TrackId, picasso_runtime::OriginCaller, Moment, frame_support::traits::preimages::Bounded<picasso_runtime::RuntimeCall>, Balance, pallet_conviction_voting::types::Tally<Votes, Total>, sp_core::crypto::AccountId32, ScheduleAddress>
   **/
  PalletReferendaReferendumStatus: {
    track: 'u16',
    origin: 'PicassoRuntimeOriginCaller',
    proposal: 'FrameSupportPreimagesBounded',
    enactment: 'FrameSupportScheduleDispatchTime',
    submitted: 'u32',
    submissionDeposit: 'PalletReferendaDeposit',
    decisionDeposit: 'Option<PalletReferendaDeposit>',
    deciding: 'Option<PalletReferendaDecidingStatus>',
    tally: 'PalletConvictionVotingTally',
    inQueue: 'bool',
    alarm: 'Option<(u32,(u32,u32))>'
  },
  /**
   * Lookup639: pallet_referenda::types::Deposit<sp_core::crypto::AccountId32, Balance>
   **/
  PalletReferendaDeposit: {
    who: 'AccountId32',
    amount: 'u128'
  },
  /**
   * Lookup642: pallet_referenda::types::DecidingStatus<BlockNumber>
   **/
  PalletReferendaDecidingStatus: {
    since: 'u32',
    confirming: 'Option<u32>'
  },
  /**
   * Lookup650: pallet_referenda::types::TrackInfo<Balance, Moment>
   **/
  PalletReferendaTrackInfo: {
    name: 'Text',
    maxDeciding: 'u32',
    decisionDeposit: 'u128',
    preparePeriod: 'u32',
    decisionPeriod: 'u32',
    confirmPeriod: 'u32',
    minEnactmentPeriod: 'u32',
    minApproval: 'PalletReferendaCurve',
    minSupport: 'PalletReferendaCurve'
  },
  /**
   * Lookup651: pallet_referenda::types::Curve
   **/
  PalletReferendaCurve: {
    _enum: {
      LinearDecreasing: {
        length: 'Perbill',
        floor: 'Perbill',
        ceil: 'Perbill',
      },
      SteppedDecreasing: {
        begin: 'Perbill',
        end: 'Perbill',
        step: 'Perbill',
        period: 'Perbill',
      },
      Reciprocal: {
        factor: 'i64',
        xOffset: 'i64',
        yOffset: 'i64'
      }
    }
  },
  /**
   * Lookup654: pallet_referenda::pallet::Error<T, I>
   **/
  PalletReferendaError: {
    _enum: ['NotOngoing', 'HasDeposit', 'BadTrack', 'Full', 'QueueEmpty', 'BadReferendum', 'NothingToDo', 'NoTrack', 'Unfinished', 'NoPermission', 'NoDeposit', 'BadStatus', 'PreimageNotExist']
  },
  /**
   * Lookup656: pallet_conviction_voting::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber, PollIndex, MaxVotes>
   **/
  PalletConvictionVotingVoteVoting: {
    _enum: {
      Casting: 'PalletConvictionVotingVoteCasting',
      Delegating: 'PalletConvictionVotingVoteDelegating'
    }
  },
  /**
   * Lookup657: pallet_conviction_voting::vote::Casting<Balance, BlockNumber, PollIndex, MaxVotes>
   **/
  PalletConvictionVotingVoteCasting: {
    votes: 'Vec<(u32,PalletConvictionVotingVoteAccountVote)>',
    delegations: 'PalletConvictionVotingDelegations',
    prior: 'PalletConvictionVotingVotePriorLock'
  },
  /**
   * Lookup661: pallet_conviction_voting::types::Delegations<Balance>
   **/
  PalletConvictionVotingDelegations: {
    votes: 'u128',
    capital: 'u128'
  },
  /**
   * Lookup662: pallet_conviction_voting::vote::PriorLock<BlockNumber, Balance>
   **/
  PalletConvictionVotingVotePriorLock: '(u32,u128)',
  /**
   * Lookup663: pallet_conviction_voting::vote::Delegating<Balance, sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletConvictionVotingVoteDelegating: {
    balance: 'u128',
    target: 'AccountId32',
    conviction: 'PalletConvictionVotingConviction',
    delegations: 'PalletConvictionVotingDelegations',
    prior: 'PalletConvictionVotingVotePriorLock'
  },
  /**
   * Lookup667: pallet_conviction_voting::pallet::Error<T, I>
   **/
  PalletConvictionVotingError: {
    _enum: ['NotOngoing', 'NotVoter', 'NoPermission', 'NoPermissionYet', 'AlreadyDelegating', 'AlreadyVoting', 'InsufficientFunds', 'NotDelegating', 'Nonsense', 'MaxVotesReached', 'ClassNeeded', 'BadClass']
  },
  /**
   * Lookup669: pallet_whitelist::pallet::Error<T>
   **/
  PalletWhitelistError: {
    _enum: ['UnavailablePreImage', 'UndecodableCall', 'InvalidCallWeightWitness', 'CallIsNotWhitelisted', 'CallAlreadyWhitelisted']
  },
  /**
   * Lookup670: pallet_call_filter::pallet::Error<T>
   **/
  PalletCallFilterError: {
    _enum: ['CannotDisable', 'InvalidString']
  },
  /**
   * Lookup672: pallet_cosmwasm::types::CodeInfo<sp_core::crypto::AccountId32>
   **/
  PalletCosmwasmCodeInfo: {
    creator: 'AccountId32',
    pristineCodeHash: '[u8;32]',
    instrumentationVersion: 'u16',
    refcount: 'u32',
    ibcCapable: 'bool'
  },
  /**
   * Lookup673: pallet_cosmwasm::instrument::CostRules<T>
   **/
  PalletCosmwasmInstrumentCostRules: {
    i64const: 'u32',
    f64const: 'u32',
    i64load: 'u32',
    f64load: 'u32',
    i64store: 'u32',
    f64store: 'u32',
    i64eq: 'u32',
    i64eqz: 'u32',
    i64ne: 'u32',
    i64lts: 'u32',
    i64gts: 'u32',
    i64les: 'u32',
    i64ges: 'u32',
    i64clz: 'u32',
    i64ctz: 'u32',
    i64popcnt: 'u32',
    i64add: 'u32',
    i64sub: 'u32',
    i64mul: 'u32',
    i64divs: 'u32',
    i64divu: 'u32',
    i64rems: 'u32',
    i64and: 'u32',
    i64or: 'u32',
    i64xor: 'u32',
    i64shl: 'u32',
    i64shrs: 'u32',
    i64rotl: 'u32',
    i64rotr: 'u32',
    i32wrapi64: 'u32',
    i64extendsi32: 'u32',
    f64eq: 'u32',
    f64ne: 'u32',
    f64lt: 'u32',
    f64gt: 'u32',
    f64le: 'u32',
    f64ge: 'u32',
    f64abs: 'u32',
    f64neg: 'u32',
    f64ceil: 'u32',
    f64floor: 'u32',
    f64trunc: 'u32',
    f64nearest: 'u32',
    f64sqrt: 'u32',
    f64add: 'u32',
    f64sub: 'u32',
    f64mul: 'u32',
    f64div: 'u32',
    f64min: 'u32',
    f64max: 'u32',
    f64copysign: 'u32',
    select: 'u32',
    if: 'u32',
    else: 'u32',
    getlocal: 'u32',
    setlocal: 'u32',
    teelocal: 'u32',
    setglobal: 'u32',
    getglobal: 'u32',
    currentmemory: 'u32',
    growmemory: 'u32',
    br: 'u32',
    brif: 'u32',
    brtable: 'u32',
    brtablePerElem: 'u32',
    call: 'u32',
    callIndirect: 'u32'
  },
  /**
   * Lookup674: pallet_cosmwasm::pallet::Error<T>
   **/
  PalletCosmwasmError: {
    _enum: ['Instrumentation', 'VmCreation', 'ContractHasNoInfo', 'CodeDecoding', 'CodeValidation', 'CodeEncoding', 'CodeInstrumentation', 'InstrumentedCodeIsTooBig', 'CodeAlreadyExists', 'CodeNotFound', 'ContractAlreadyExists', 'ContractNotFound', 'SubstrateDispatch', 'AssetConversion', 'TransferFailed', 'LabelTooBig', 'UnknownDenom', 'StackOverflow', 'NotEnoughFundsForUpload', 'NonceOverflow', 'RefcountOverflow', 'VMDepthOverflow', 'SignatureVerificationError', 'IteratorIdOverflow', 'IteratorNotFound', 'IteratorValueNotFound', 'NotAuthorized', 'NotImplemented', 'Unsupported', 'ExecuteDeserialize', 'Ibc', 'FailedToSerialize', 'OutOfGas', 'InvalidGasCheckpoint', 'InvalidSalt', 'InvalidAccount', 'Interpreter', 'VirtualMachine', 'AccountConversionFailure', 'Aborted', 'ReadOnlyViolation', 'Rpc', 'Precompile', 'QueryDeserialize', 'ExecuteSerialize']
  },
  /**
   * Lookup683: pallet_ibc::LightClientProtocol
   **/
  PalletIbcLightClientProtocol: {
    _enum: ['Beefy', 'Grandpa']
  },
  /**
   * Lookup684: pallet_ibc::pallet::Error<T>
   **/
  PalletIbcError: {
    _enum: ['ProcessingError', 'DecodingError', 'EncodingError', 'ProofGenerationError', 'ConsensusStateNotFound', 'ChannelNotFound', 'ClientStateNotFound', 'ConnectionNotFound', 'PacketCommitmentNotFound', 'PacketReceiptNotFound', 'PacketAcknowledgmentNotFound', 'SendPacketError', 'InvalidChannelId', 'InvalidPortId', 'Other', 'InvalidRoute', 'InvalidMessageType', 'TransferInternals', 'TransferSerde', 'TransferOther', 'TransferProtocol', 'TransferSend', 'Utf8Error', 'InvalidAssetId', 'PrefixedDenomParse', 'InvalidAmount', 'InvalidTimestamp', 'FailedToGetRevisionNumber', 'InvalidParams', 'ChannelInitError', 'TimestampAndHeightNotFound', 'ChannelEscrowAddress', 'WriteAckError', 'ClientUpdateNotFound', 'ClientFreezeFailed', 'AccessDenied', 'RateLimiter', 'FailedSendFeeToAccount', 'OriginAddress']
  },
  /**
   * Lookup685: pallet_multihop_xcm_ibc::pallet::Error<T>
   **/
  PalletMultihopXcmIbcError: {
    _enum: {
      IncorrectAddress: {
        chainId: 'u8',
      },
      IncorrectChainName: {
        chainId: 'u8',
      },
      FailedToEncodeBech32Address: {
        chainId: 'u8',
      },
      IncorrectMultiLocation: 'Null',
      XcmDepositFailed: 'Null',
      MultiHopRouteDoesNotExist: 'Null',
      DoesNotSupportNonFungible: 'Null',
      IncorrectCountOfAddresses: 'Null',
      FailedToConstructMemo: 'Null',
      FailedToDecodeAccountId: 'Null'
    }
  },
  /**
   * Lookup688: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup689: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup690: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup691: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup694: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup695: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup696: pallet_asset_tx_payment::ChargeAssetTxPayment<T>
   **/
  PalletAssetTxPaymentChargeAssetTxPayment: {
    tip: 'Compact<u128>',
    assetId: 'Option<u128>'
  },
  /**
   * Lookup697: picasso_runtime::Runtime
   **/
  PicassoRuntimeRuntime: 'Null'
};
