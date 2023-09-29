// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type { Data } from '@polkadot/types';
import type { BTreeMap, BTreeSet, Bytes, Compact, Enum, Null, Option, Result, Set, Struct, Text, U8aFixed, Vec, bool, i128, i64, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { Vote } from '@polkadot/types/interfaces/elections';
import type { AccountId32, Call, H256, MultiAddress, Perbill, Percent, Permill } from '@polkadot/types/interfaces/runtime';
import type { Event } from '@polkadot/types/interfaces/system';

declare module '@polkadot/types/lookup' {
  /** @name FrameSystemAccountInfo (3) */
  interface FrameSystemAccountInfo extends Struct {
    readonly nonce: u32;
    readonly consumers: u32;
    readonly providers: u32;
    readonly sufficients: u32;
    readonly data: PalletBalancesAccountData;
  }

  /** @name PalletBalancesAccountData (5) */
  interface PalletBalancesAccountData extends Struct {
    readonly free: u128;
    readonly reserved: u128;
    readonly miscFrozen: u128;
    readonly feeFrozen: u128;
  }

  /** @name FrameSupportDispatchPerDispatchClassWeight (7) */
  interface FrameSupportDispatchPerDispatchClassWeight extends Struct {
    readonly normal: SpWeightsWeightV2Weight;
    readonly operational: SpWeightsWeightV2Weight;
    readonly mandatory: SpWeightsWeightV2Weight;
  }

  /** @name SpWeightsWeightV2Weight (8) */
  interface SpWeightsWeightV2Weight extends Struct {
    readonly refTime: Compact<u64>;
    readonly proofSize: Compact<u64>;
  }

  /** @name SpRuntimeDigest (13) */
  interface SpRuntimeDigest extends Struct {
    readonly logs: Vec<SpRuntimeDigestDigestItem>;
  }

  /** @name SpRuntimeDigestDigestItem (15) */
  interface SpRuntimeDigestDigestItem extends Enum {
    readonly isOther: boolean;
    readonly asOther: Bytes;
    readonly isConsensus: boolean;
    readonly asConsensus: ITuple<[U8aFixed, Bytes]>;
    readonly isSeal: boolean;
    readonly asSeal: ITuple<[U8aFixed, Bytes]>;
    readonly isPreRuntime: boolean;
    readonly asPreRuntime: ITuple<[U8aFixed, Bytes]>;
    readonly isRuntimeEnvironmentUpdated: boolean;
    readonly type: 'Other' | 'Consensus' | 'Seal' | 'PreRuntime' | 'RuntimeEnvironmentUpdated';
  }

  /** @name FrameSystemEventRecord (18) */
  interface FrameSystemEventRecord extends Struct {
    readonly phase: FrameSystemPhase;
    readonly event: Event;
    readonly topics: Vec<H256>;
  }

  /** @name FrameSystemEvent (20) */
  interface FrameSystemEvent extends Enum {
    readonly isExtrinsicSuccess: boolean;
    readonly asExtrinsicSuccess: {
      readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
    } & Struct;
    readonly isExtrinsicFailed: boolean;
    readonly asExtrinsicFailed: {
      readonly dispatchError: SpRuntimeDispatchError;
      readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
    } & Struct;
    readonly isCodeUpdated: boolean;
    readonly isNewAccount: boolean;
    readonly asNewAccount: {
      readonly account: AccountId32;
    } & Struct;
    readonly isKilledAccount: boolean;
    readonly asKilledAccount: {
      readonly account: AccountId32;
    } & Struct;
    readonly isRemarked: boolean;
    readonly asRemarked: {
      readonly sender: AccountId32;
      readonly hash_: H256;
    } & Struct;
    readonly type: 'ExtrinsicSuccess' | 'ExtrinsicFailed' | 'CodeUpdated' | 'NewAccount' | 'KilledAccount' | 'Remarked';
  }

  /** @name FrameSupportDispatchDispatchInfo (21) */
  interface FrameSupportDispatchDispatchInfo extends Struct {
    readonly weight: SpWeightsWeightV2Weight;
    readonly class: FrameSupportDispatchDispatchClass;
    readonly paysFee: FrameSupportDispatchPays;
  }

  /** @name FrameSupportDispatchDispatchClass (22) */
  interface FrameSupportDispatchDispatchClass extends Enum {
    readonly isNormal: boolean;
    readonly isOperational: boolean;
    readonly isMandatory: boolean;
    readonly type: 'Normal' | 'Operational' | 'Mandatory';
  }

  /** @name FrameSupportDispatchPays (23) */
  interface FrameSupportDispatchPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: 'Yes' | 'No';
  }

  /** @name SpRuntimeDispatchError (24) */
  interface SpRuntimeDispatchError extends Enum {
    readonly isOther: boolean;
    readonly isCannotLookup: boolean;
    readonly isBadOrigin: boolean;
    readonly isModule: boolean;
    readonly asModule: SpRuntimeModuleError;
    readonly isConsumerRemaining: boolean;
    readonly isNoProviders: boolean;
    readonly isTooManyConsumers: boolean;
    readonly isToken: boolean;
    readonly asToken: SpRuntimeTokenError;
    readonly isArithmetic: boolean;
    readonly asArithmetic: SpArithmeticArithmeticError;
    readonly isTransactional: boolean;
    readonly asTransactional: SpRuntimeTransactionalError;
    readonly isExhausted: boolean;
    readonly isCorruption: boolean;
    readonly isUnavailable: boolean;
    readonly type: 'Other' | 'CannotLookup' | 'BadOrigin' | 'Module' | 'ConsumerRemaining' | 'NoProviders' | 'TooManyConsumers' | 'Token' | 'Arithmetic' | 'Transactional' | 'Exhausted' | 'Corruption' | 'Unavailable';
  }

  /** @name SpRuntimeModuleError (25) */
  interface SpRuntimeModuleError extends Struct {
    readonly index: u8;
    readonly error: U8aFixed;
  }

  /** @name SpRuntimeTokenError (26) */
  interface SpRuntimeTokenError extends Enum {
    readonly isNoFunds: boolean;
    readonly isWouldDie: boolean;
    readonly isBelowMinimum: boolean;
    readonly isCannotCreate: boolean;
    readonly isUnknownAsset: boolean;
    readonly isFrozen: boolean;
    readonly isUnsupported: boolean;
    readonly type: 'NoFunds' | 'WouldDie' | 'BelowMinimum' | 'CannotCreate' | 'UnknownAsset' | 'Frozen' | 'Unsupported';
  }

  /** @name SpArithmeticArithmeticError (27) */
  interface SpArithmeticArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
  }

  /** @name SpRuntimeTransactionalError (28) */
  interface SpRuntimeTransactionalError extends Enum {
    readonly isLimitReached: boolean;
    readonly isNoLayer: boolean;
    readonly type: 'LimitReached' | 'NoLayer';
  }

  /** @name PalletSudoEvent (29) */
  interface PalletSudoEvent extends Enum {
    readonly isSudid: boolean;
    readonly asSudid: {
      readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isKeyChanged: boolean;
    readonly asKeyChanged: {
      readonly oldSudoer: Option<AccountId32>;
    } & Struct;
    readonly isSudoAsDone: boolean;
    readonly asSudoAsDone: {
      readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly type: 'Sudid' | 'KeyChanged' | 'SudoAsDone';
  }

  /** @name PalletTransactionPaymentEvent (33) */
  interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: 'TransactionFeePaid';
  }

  /** @name PalletIndicesEvent (34) */
  interface PalletIndicesEvent extends Enum {
    readonly isIndexAssigned: boolean;
    readonly asIndexAssigned: {
      readonly who: AccountId32;
      readonly index: u32;
    } & Struct;
    readonly isIndexFreed: boolean;
    readonly asIndexFreed: {
      readonly index: u32;
    } & Struct;
    readonly isIndexFrozen: boolean;
    readonly asIndexFrozen: {
      readonly index: u32;
      readonly who: AccountId32;
    } & Struct;
    readonly type: 'IndexAssigned' | 'IndexFreed' | 'IndexFrozen';
  }

  /** @name PalletBalancesEvent (35) */
  interface PalletBalancesEvent extends Enum {
    readonly isEndowed: boolean;
    readonly asEndowed: {
      readonly account: AccountId32;
      readonly freeBalance: u128;
    } & Struct;
    readonly isDustLost: boolean;
    readonly asDustLost: {
      readonly account: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isBalanceSet: boolean;
    readonly asBalanceSet: {
      readonly who: AccountId32;
      readonly free: u128;
      readonly reserved: u128;
    } & Struct;
    readonly isReserved: boolean;
    readonly asReserved: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUnreserved: boolean;
    readonly asUnreserved: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isReserveRepatriated: boolean;
    readonly asReserveRepatriated: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly destinationStatus: FrameSupportTokensMiscBalanceStatus;
    } & Struct;
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isWithdraw: boolean;
    readonly asWithdraw: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isSlashed: boolean;
    readonly asSlashed: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly type: 'Endowed' | 'DustLost' | 'Transfer' | 'BalanceSet' | 'Reserved' | 'Unreserved' | 'ReserveRepatriated' | 'Deposit' | 'Withdraw' | 'Slashed';
  }

  /** @name FrameSupportTokensMiscBalanceStatus (36) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletIdentityEvent (37) */
  interface PalletIdentityEvent extends Enum {
    readonly isIdentitySet: boolean;
    readonly asIdentitySet: {
      readonly who: AccountId32;
    } & Struct;
    readonly isIdentityCleared: boolean;
    readonly asIdentityCleared: {
      readonly who: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isIdentityKilled: boolean;
    readonly asIdentityKilled: {
      readonly who: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isJudgementRequested: boolean;
    readonly asJudgementRequested: {
      readonly who: AccountId32;
      readonly registrarIndex: u32;
    } & Struct;
    readonly isJudgementUnrequested: boolean;
    readonly asJudgementUnrequested: {
      readonly who: AccountId32;
      readonly registrarIndex: u32;
    } & Struct;
    readonly isJudgementGiven: boolean;
    readonly asJudgementGiven: {
      readonly target: AccountId32;
      readonly registrarIndex: u32;
    } & Struct;
    readonly isRegistrarAdded: boolean;
    readonly asRegistrarAdded: {
      readonly registrarIndex: u32;
    } & Struct;
    readonly isSubIdentityAdded: boolean;
    readonly asSubIdentityAdded: {
      readonly sub: AccountId32;
      readonly main: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isSubIdentityRemoved: boolean;
    readonly asSubIdentityRemoved: {
      readonly sub: AccountId32;
      readonly main: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isSubIdentityRevoked: boolean;
    readonly asSubIdentityRevoked: {
      readonly sub: AccountId32;
      readonly main: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly type: 'IdentitySet' | 'IdentityCleared' | 'IdentityKilled' | 'JudgementRequested' | 'JudgementUnrequested' | 'JudgementGiven' | 'RegistrarAdded' | 'SubIdentityAdded' | 'SubIdentityRemoved' | 'SubIdentityRevoked';
  }

  /** @name PalletMultisigEvent (38) */
  interface PalletMultisigEvent extends Enum {
    readonly isNewMultisig: boolean;
    readonly asNewMultisig: {
      readonly approving: AccountId32;
      readonly multisig: AccountId32;
      readonly callHash: U8aFixed;
    } & Struct;
    readonly isMultisigApproval: boolean;
    readonly asMultisigApproval: {
      readonly approving: AccountId32;
      readonly timepoint: PalletMultisigTimepoint;
      readonly multisig: AccountId32;
      readonly callHash: U8aFixed;
    } & Struct;
    readonly isMultisigExecuted: boolean;
    readonly asMultisigExecuted: {
      readonly approving: AccountId32;
      readonly timepoint: PalletMultisigTimepoint;
      readonly multisig: AccountId32;
      readonly callHash: U8aFixed;
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isMultisigCancelled: boolean;
    readonly asMultisigCancelled: {
      readonly cancelling: AccountId32;
      readonly timepoint: PalletMultisigTimepoint;
      readonly multisig: AccountId32;
      readonly callHash: U8aFixed;
    } & Struct;
    readonly type: 'NewMultisig' | 'MultisigApproval' | 'MultisigExecuted' | 'MultisigCancelled';
  }

  /** @name PalletMultisigTimepoint (39) */
  interface PalletMultisigTimepoint extends Struct {
    readonly height: u32;
    readonly index: u32;
  }

  /** @name CumulusPalletParachainSystemEvent (40) */
  interface CumulusPalletParachainSystemEvent extends Enum {
    readonly isValidationFunctionStored: boolean;
    readonly isValidationFunctionApplied: boolean;
    readonly asValidationFunctionApplied: {
      readonly relayChainBlockNum: u32;
    } & Struct;
    readonly isValidationFunctionDiscarded: boolean;
    readonly isUpgradeAuthorized: boolean;
    readonly asUpgradeAuthorized: {
      readonly codeHash: H256;
    } & Struct;
    readonly isDownwardMessagesReceived: boolean;
    readonly asDownwardMessagesReceived: {
      readonly count: u32;
    } & Struct;
    readonly isDownwardMessagesProcessed: boolean;
    readonly asDownwardMessagesProcessed: {
      readonly weightUsed: SpWeightsWeightV2Weight;
      readonly dmqHead: H256;
    } & Struct;
    readonly isUpwardMessageSent: boolean;
    readonly asUpwardMessageSent: {
      readonly messageHash: Option<U8aFixed>;
    } & Struct;
    readonly type: 'ValidationFunctionStored' | 'ValidationFunctionApplied' | 'ValidationFunctionDiscarded' | 'UpgradeAuthorized' | 'DownwardMessagesReceived' | 'DownwardMessagesProcessed' | 'UpwardMessageSent';
  }

  /** @name PalletCollatorSelectionEvent (42) */
  interface PalletCollatorSelectionEvent extends Enum {
    readonly isNewInvulnerables: boolean;
    readonly asNewInvulnerables: {
      readonly invulnerables: Vec<AccountId32>;
    } & Struct;
    readonly isNewDesiredCandidates: boolean;
    readonly asNewDesiredCandidates: {
      readonly desiredCandidates: u32;
    } & Struct;
    readonly isNewCandidacyBond: boolean;
    readonly asNewCandidacyBond: {
      readonly bondAmount: u128;
    } & Struct;
    readonly isCandidateAdded: boolean;
    readonly asCandidateAdded: {
      readonly accountId: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isCandidateRemoved: boolean;
    readonly asCandidateRemoved: {
      readonly accountId: AccountId32;
    } & Struct;
    readonly type: 'NewInvulnerables' | 'NewDesiredCandidates' | 'NewCandidacyBond' | 'CandidateAdded' | 'CandidateRemoved';
  }

  /** @name PalletSessionEvent (44) */
  interface PalletSessionEvent extends Enum {
    readonly isNewSession: boolean;
    readonly asNewSession: {
      readonly sessionIndex: u32;
    } & Struct;
    readonly type: 'NewSession';
  }

  /** @name PalletCollectiveEvent (45) */
  interface PalletCollectiveEvent extends Enum {
    readonly isProposed: boolean;
    readonly asProposed: {
      readonly account: AccountId32;
      readonly proposalIndex: u32;
      readonly proposalHash: H256;
      readonly threshold: u32;
    } & Struct;
    readonly isVoted: boolean;
    readonly asVoted: {
      readonly account: AccountId32;
      readonly proposalHash: H256;
      readonly voted: bool;
      readonly yes: u32;
      readonly no: u32;
    } & Struct;
    readonly isApproved: boolean;
    readonly asApproved: {
      readonly proposalHash: H256;
    } & Struct;
    readonly isDisapproved: boolean;
    readonly asDisapproved: {
      readonly proposalHash: H256;
    } & Struct;
    readonly isExecuted: boolean;
    readonly asExecuted: {
      readonly proposalHash: H256;
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isMemberExecuted: boolean;
    readonly asMemberExecuted: {
      readonly proposalHash: H256;
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isClosed: boolean;
    readonly asClosed: {
      readonly proposalHash: H256;
      readonly yes: u32;
      readonly no: u32;
    } & Struct;
    readonly type: 'Proposed' | 'Voted' | 'Approved' | 'Disapproved' | 'Executed' | 'MemberExecuted' | 'Closed';
  }

  /** @name PalletMembershipEvent (47) */
  interface PalletMembershipEvent extends Enum {
    readonly isMemberAdded: boolean;
    readonly isMemberRemoved: boolean;
    readonly isMembersSwapped: boolean;
    readonly isMembersReset: boolean;
    readonly isKeyChanged: boolean;
    readonly isDummy: boolean;
    readonly type: 'MemberAdded' | 'MemberRemoved' | 'MembersSwapped' | 'MembersReset' | 'KeyChanged' | 'Dummy';
  }

  /** @name PalletTreasuryEvent (48) */
  interface PalletTreasuryEvent extends Enum {
    readonly isProposed: boolean;
    readonly asProposed: {
      readonly proposalIndex: u32;
    } & Struct;
    readonly isSpending: boolean;
    readonly asSpending: {
      readonly budgetRemaining: u128;
    } & Struct;
    readonly isAwarded: boolean;
    readonly asAwarded: {
      readonly proposalIndex: u32;
      readonly award: u128;
      readonly account: AccountId32;
    } & Struct;
    readonly isRejected: boolean;
    readonly asRejected: {
      readonly proposalIndex: u32;
      readonly slashed: u128;
    } & Struct;
    readonly isBurnt: boolean;
    readonly asBurnt: {
      readonly burntFunds: u128;
    } & Struct;
    readonly isRollover: boolean;
    readonly asRollover: {
      readonly rolloverBalance: u128;
    } & Struct;
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly value: u128;
    } & Struct;
    readonly isSpendApproved: boolean;
    readonly asSpendApproved: {
      readonly proposalIndex: u32;
      readonly amount: u128;
      readonly beneficiary: AccountId32;
    } & Struct;
    readonly isUpdatedInactive: boolean;
    readonly asUpdatedInactive: {
      readonly reactivated: u128;
      readonly deactivated: u128;
    } & Struct;
    readonly type: 'Proposed' | 'Spending' | 'Awarded' | 'Rejected' | 'Burnt' | 'Rollover' | 'Deposit' | 'SpendApproved' | 'UpdatedInactive';
  }

  /** @name PalletDemocracyEvent (49) */
  interface PalletDemocracyEvent extends Enum {
    readonly isProposed: boolean;
    readonly asProposed: {
      readonly proposalIndex: u32;
      readonly deposit: u128;
    } & Struct;
    readonly isTabled: boolean;
    readonly asTabled: {
      readonly proposalIndex: u32;
      readonly deposit: u128;
    } & Struct;
    readonly isExternalTabled: boolean;
    readonly isStarted: boolean;
    readonly asStarted: {
      readonly refIndex: u32;
      readonly threshold: PalletDemocracyVoteThreshold;
    } & Struct;
    readonly isPassed: boolean;
    readonly asPassed: {
      readonly refIndex: u32;
    } & Struct;
    readonly isNotPassed: boolean;
    readonly asNotPassed: {
      readonly refIndex: u32;
    } & Struct;
    readonly isCancelled: boolean;
    readonly asCancelled: {
      readonly refIndex: u32;
    } & Struct;
    readonly isDelegated: boolean;
    readonly asDelegated: {
      readonly who: AccountId32;
      readonly target: AccountId32;
    } & Struct;
    readonly isUndelegated: boolean;
    readonly asUndelegated: {
      readonly account: AccountId32;
    } & Struct;
    readonly isVetoed: boolean;
    readonly asVetoed: {
      readonly who: AccountId32;
      readonly proposalHash: H256;
      readonly until: u32;
    } & Struct;
    readonly isBlacklisted: boolean;
    readonly asBlacklisted: {
      readonly proposalHash: H256;
    } & Struct;
    readonly isVoted: boolean;
    readonly asVoted: {
      readonly voter: AccountId32;
      readonly refIndex: u32;
      readonly vote: PalletDemocracyVoteAccountVote;
    } & Struct;
    readonly isSeconded: boolean;
    readonly asSeconded: {
      readonly seconder: AccountId32;
      readonly propIndex: u32;
    } & Struct;
    readonly isProposalCanceled: boolean;
    readonly asProposalCanceled: {
      readonly propIndex: u32;
    } & Struct;
    readonly isMetadataSet: boolean;
    readonly asMetadataSet: {
      readonly owner: PalletDemocracyMetadataOwner;
      readonly hash_: H256;
    } & Struct;
    readonly isMetadataCleared: boolean;
    readonly asMetadataCleared: {
      readonly owner: PalletDemocracyMetadataOwner;
      readonly hash_: H256;
    } & Struct;
    readonly isMetadataTransferred: boolean;
    readonly asMetadataTransferred: {
      readonly prevOwner: PalletDemocracyMetadataOwner;
      readonly owner: PalletDemocracyMetadataOwner;
      readonly hash_: H256;
    } & Struct;
    readonly type: 'Proposed' | 'Tabled' | 'ExternalTabled' | 'Started' | 'Passed' | 'NotPassed' | 'Cancelled' | 'Delegated' | 'Undelegated' | 'Vetoed' | 'Blacklisted' | 'Voted' | 'Seconded' | 'ProposalCanceled' | 'MetadataSet' | 'MetadataCleared' | 'MetadataTransferred';
  }

  /** @name PalletDemocracyVoteThreshold (50) */
  interface PalletDemocracyVoteThreshold extends Enum {
    readonly isSuperMajorityApprove: boolean;
    readonly isSuperMajorityAgainst: boolean;
    readonly isSimpleMajority: boolean;
    readonly type: 'SuperMajorityApprove' | 'SuperMajorityAgainst' | 'SimpleMajority';
  }

  /** @name PalletDemocracyVoteAccountVote (51) */
  interface PalletDemocracyVoteAccountVote extends Enum {
    readonly isStandard: boolean;
    readonly asStandard: {
      readonly vote: Vote;
      readonly balance: u128;
    } & Struct;
    readonly isSplit: boolean;
    readonly asSplit: {
      readonly aye: u128;
      readonly nay: u128;
    } & Struct;
    readonly type: 'Standard' | 'Split';
  }

  /** @name PalletDemocracyMetadataOwner (53) */
  interface PalletDemocracyMetadataOwner extends Enum {
    readonly isExternal: boolean;
    readonly isProposal: boolean;
    readonly asProposal: u32;
    readonly isReferendum: boolean;
    readonly asReferendum: u32;
    readonly type: 'External' | 'Proposal' | 'Referendum';
  }

  /** @name PalletSchedulerEvent (58) */
  interface PalletSchedulerEvent extends Enum {
    readonly isScheduled: boolean;
    readonly asScheduled: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isCanceled: boolean;
    readonly asCanceled: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isDispatched: boolean;
    readonly asDispatched: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<U8aFixed>;
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isCallUnavailable: boolean;
    readonly asCallUnavailable: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<U8aFixed>;
    } & Struct;
    readonly isPeriodicFailed: boolean;
    readonly asPeriodicFailed: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<U8aFixed>;
    } & Struct;
    readonly isPermanentlyOverweight: boolean;
    readonly asPermanentlyOverweight: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<U8aFixed>;
    } & Struct;
    readonly type: 'Scheduled' | 'Canceled' | 'Dispatched' | 'CallUnavailable' | 'PeriodicFailed' | 'PermanentlyOverweight';
  }

  /** @name PalletUtilityEvent (60) */
  interface PalletUtilityEvent extends Enum {
    readonly isBatchInterrupted: boolean;
    readonly asBatchInterrupted: {
      readonly index: u32;
      readonly error: SpRuntimeDispatchError;
    } & Struct;
    readonly isBatchCompleted: boolean;
    readonly isBatchCompletedWithErrors: boolean;
    readonly isItemCompleted: boolean;
    readonly isItemFailed: boolean;
    readonly asItemFailed: {
      readonly error: SpRuntimeDispatchError;
    } & Struct;
    readonly isDispatchedAs: boolean;
    readonly asDispatchedAs: {
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly type: 'BatchInterrupted' | 'BatchCompleted' | 'BatchCompletedWithErrors' | 'ItemCompleted' | 'ItemFailed' | 'DispatchedAs';
  }

  /** @name PalletPreimageEvent (61) */
  interface PalletPreimageEvent extends Enum {
    readonly isNoted: boolean;
    readonly asNoted: {
      readonly hash_: H256;
    } & Struct;
    readonly isRequested: boolean;
    readonly asRequested: {
      readonly hash_: H256;
    } & Struct;
    readonly isCleared: boolean;
    readonly asCleared: {
      readonly hash_: H256;
    } & Struct;
    readonly type: 'Noted' | 'Requested' | 'Cleared';
  }

  /** @name PalletProxyEvent (62) */
  interface PalletProxyEvent extends Enum {
    readonly isProxyExecuted: boolean;
    readonly asProxyExecuted: {
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isPureCreated: boolean;
    readonly asPureCreated: {
      readonly pure: AccountId32;
      readonly who: AccountId32;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly disambiguationIndex: u16;
    } & Struct;
    readonly isAnnounced: boolean;
    readonly asAnnounced: {
      readonly real: AccountId32;
      readonly proxy: AccountId32;
      readonly callHash: H256;
    } & Struct;
    readonly isProxyAdded: boolean;
    readonly asProxyAdded: {
      readonly delegator: AccountId32;
      readonly delegatee: AccountId32;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
    } & Struct;
    readonly isProxyRemoved: boolean;
    readonly asProxyRemoved: {
      readonly delegator: AccountId32;
      readonly delegatee: AccountId32;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
    } & Struct;
    readonly type: 'ProxyExecuted' | 'PureCreated' | 'Announced' | 'ProxyAdded' | 'ProxyRemoved';
  }

  /** @name ComposableTraitsAccountProxyProxyType (63) */
  interface ComposableTraitsAccountProxyProxyType extends Enum {
    readonly isAny: boolean;
    readonly isGovernance: boolean;
    readonly isCancelProxy: boolean;
    readonly isBridge: boolean;
    readonly isAssets: boolean;
    readonly isDefi: boolean;
    readonly isOracle: boolean;
    readonly isContracts: boolean;
    readonly type: 'Any' | 'Governance' | 'CancelProxy' | 'Bridge' | 'Assets' | 'Defi' | 'Oracle' | 'Contracts';
  }

  /** @name CumulusPalletXcmpQueueEvent (65) */
  interface CumulusPalletXcmpQueueEvent extends Enum {
    readonly isSuccess: boolean;
    readonly asSuccess: {
      readonly messageHash: Option<U8aFixed>;
      readonly weight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isFail: boolean;
    readonly asFail: {
      readonly messageHash: Option<U8aFixed>;
      readonly error: XcmV3TraitsError;
      readonly weight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isBadVersion: boolean;
    readonly asBadVersion: {
      readonly messageHash: Option<U8aFixed>;
    } & Struct;
    readonly isBadFormat: boolean;
    readonly asBadFormat: {
      readonly messageHash: Option<U8aFixed>;
    } & Struct;
    readonly isXcmpMessageSent: boolean;
    readonly asXcmpMessageSent: {
      readonly messageHash: Option<U8aFixed>;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly sender: u32;
      readonly sentAt: u32;
      readonly index: u64;
      readonly required: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly index: u64;
      readonly used: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'Success' | 'Fail' | 'BadVersion' | 'BadFormat' | 'XcmpMessageSent' | 'OverweightEnqueued' | 'OverweightServiced';
  }

  /** @name XcmV3TraitsError (66) */
  interface XcmV3TraitsError extends Enum {
    readonly isOverflow: boolean;
    readonly isUnimplemented: boolean;
    readonly isUntrustedReserveLocation: boolean;
    readonly isUntrustedTeleportLocation: boolean;
    readonly isLocationFull: boolean;
    readonly isLocationNotInvertible: boolean;
    readonly isBadOrigin: boolean;
    readonly isInvalidLocation: boolean;
    readonly isAssetNotFound: boolean;
    readonly isFailedToTransactAsset: boolean;
    readonly isNotWithdrawable: boolean;
    readonly isLocationCannotHold: boolean;
    readonly isExceedsMaxMessageSize: boolean;
    readonly isDestinationUnsupported: boolean;
    readonly isTransport: boolean;
    readonly isUnroutable: boolean;
    readonly isUnknownClaim: boolean;
    readonly isFailedToDecode: boolean;
    readonly isMaxWeightInvalid: boolean;
    readonly isNotHoldingFees: boolean;
    readonly isTooExpensive: boolean;
    readonly isTrap: boolean;
    readonly asTrap: u64;
    readonly isExpectationFalse: boolean;
    readonly isPalletNotFound: boolean;
    readonly isNameMismatch: boolean;
    readonly isVersionIncompatible: boolean;
    readonly isHoldingWouldOverflow: boolean;
    readonly isExportError: boolean;
    readonly isReanchorFailed: boolean;
    readonly isNoDeal: boolean;
    readonly isFeesNotMet: boolean;
    readonly isLockError: boolean;
    readonly isNoPermission: boolean;
    readonly isUnanchored: boolean;
    readonly isNotDepositable: boolean;
    readonly isUnhandledXcmVersion: boolean;
    readonly isWeightLimitReached: boolean;
    readonly asWeightLimitReached: SpWeightsWeightV2Weight;
    readonly isBarrier: boolean;
    readonly isWeightNotComputable: boolean;
    readonly isExceedsStackLimit: boolean;
    readonly type: 'Overflow' | 'Unimplemented' | 'UntrustedReserveLocation' | 'UntrustedTeleportLocation' | 'LocationFull' | 'LocationNotInvertible' | 'BadOrigin' | 'InvalidLocation' | 'AssetNotFound' | 'FailedToTransactAsset' | 'NotWithdrawable' | 'LocationCannotHold' | 'ExceedsMaxMessageSize' | 'DestinationUnsupported' | 'Transport' | 'Unroutable' | 'UnknownClaim' | 'FailedToDecode' | 'MaxWeightInvalid' | 'NotHoldingFees' | 'TooExpensive' | 'Trap' | 'ExpectationFalse' | 'PalletNotFound' | 'NameMismatch' | 'VersionIncompatible' | 'HoldingWouldOverflow' | 'ExportError' | 'ReanchorFailed' | 'NoDeal' | 'FeesNotMet' | 'LockError' | 'NoPermission' | 'Unanchored' | 'NotDepositable' | 'UnhandledXcmVersion' | 'WeightLimitReached' | 'Barrier' | 'WeightNotComputable' | 'ExceedsStackLimit';
  }

  /** @name PalletXcmEvent (68) */
  interface PalletXcmEvent extends Enum {
    readonly isAttempted: boolean;
    readonly asAttempted: XcmV3TraitsOutcome;
    readonly isSent: boolean;
    readonly asSent: ITuple<[XcmV3MultiLocation, XcmV3MultiLocation, XcmV3Xcm]>;
    readonly isUnexpectedResponse: boolean;
    readonly asUnexpectedResponse: ITuple<[XcmV3MultiLocation, u64]>;
    readonly isResponseReady: boolean;
    readonly asResponseReady: ITuple<[u64, XcmV3Response]>;
    readonly isNotified: boolean;
    readonly asNotified: ITuple<[u64, u8, u8]>;
    readonly isNotifyOverweight: boolean;
    readonly asNotifyOverweight: ITuple<[u64, u8, u8, SpWeightsWeightV2Weight, SpWeightsWeightV2Weight]>;
    readonly isNotifyDispatchError: boolean;
    readonly asNotifyDispatchError: ITuple<[u64, u8, u8]>;
    readonly isNotifyDecodeFailed: boolean;
    readonly asNotifyDecodeFailed: ITuple<[u64, u8, u8]>;
    readonly isInvalidResponder: boolean;
    readonly asInvalidResponder: ITuple<[XcmV3MultiLocation, u64, Option<XcmV3MultiLocation>]>;
    readonly isInvalidResponderVersion: boolean;
    readonly asInvalidResponderVersion: ITuple<[XcmV3MultiLocation, u64]>;
    readonly isResponseTaken: boolean;
    readonly asResponseTaken: u64;
    readonly isAssetsTrapped: boolean;
    readonly asAssetsTrapped: ITuple<[H256, XcmV3MultiLocation, XcmVersionedMultiAssets]>;
    readonly isVersionChangeNotified: boolean;
    readonly asVersionChangeNotified: ITuple<[XcmV3MultiLocation, u32, XcmV3MultiassetMultiAssets]>;
    readonly isSupportedVersionChanged: boolean;
    readonly asSupportedVersionChanged: ITuple<[XcmV3MultiLocation, u32]>;
    readonly isNotifyTargetSendFail: boolean;
    readonly asNotifyTargetSendFail: ITuple<[XcmV3MultiLocation, u64, XcmV3TraitsError]>;
    readonly isNotifyTargetMigrationFail: boolean;
    readonly asNotifyTargetMigrationFail: ITuple<[XcmVersionedMultiLocation, u64]>;
    readonly isInvalidQuerierVersion: boolean;
    readonly asInvalidQuerierVersion: ITuple<[XcmV3MultiLocation, u64]>;
    readonly isInvalidQuerier: boolean;
    readonly asInvalidQuerier: ITuple<[XcmV3MultiLocation, u64, XcmV3MultiLocation, Option<XcmV3MultiLocation>]>;
    readonly isVersionNotifyStarted: boolean;
    readonly asVersionNotifyStarted: ITuple<[XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
    readonly isVersionNotifyRequested: boolean;
    readonly asVersionNotifyRequested: ITuple<[XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
    readonly isVersionNotifyUnrequested: boolean;
    readonly asVersionNotifyUnrequested: ITuple<[XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
    readonly isFeesPaid: boolean;
    readonly asFeesPaid: ITuple<[XcmV3MultiLocation, XcmV3MultiassetMultiAssets]>;
    readonly isAssetsClaimed: boolean;
    readonly asAssetsClaimed: ITuple<[H256, XcmV3MultiLocation, XcmVersionedMultiAssets]>;
    readonly type: 'Attempted' | 'Sent' | 'UnexpectedResponse' | 'ResponseReady' | 'Notified' | 'NotifyOverweight' | 'NotifyDispatchError' | 'NotifyDecodeFailed' | 'InvalidResponder' | 'InvalidResponderVersion' | 'ResponseTaken' | 'AssetsTrapped' | 'VersionChangeNotified' | 'SupportedVersionChanged' | 'NotifyTargetSendFail' | 'NotifyTargetMigrationFail' | 'InvalidQuerierVersion' | 'InvalidQuerier' | 'VersionNotifyStarted' | 'VersionNotifyRequested' | 'VersionNotifyUnrequested' | 'FeesPaid' | 'AssetsClaimed';
  }

  /** @name XcmV3TraitsOutcome (69) */
  interface XcmV3TraitsOutcome extends Enum {
    readonly isComplete: boolean;
    readonly asComplete: SpWeightsWeightV2Weight;
    readonly isIncomplete: boolean;
    readonly asIncomplete: ITuple<[SpWeightsWeightV2Weight, XcmV3TraitsError]>;
    readonly isError: boolean;
    readonly asError: XcmV3TraitsError;
    readonly type: 'Complete' | 'Incomplete' | 'Error';
  }

  /** @name XcmV3MultiLocation (70) */
  interface XcmV3MultiLocation extends Struct {
    readonly parents: u8;
    readonly interior: XcmV3Junctions;
  }

  /** @name XcmV3Junctions (71) */
  interface XcmV3Junctions extends Enum {
    readonly isHere: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV3Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV3Junction, XcmV3Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
    readonly type: 'Here' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV3Junction (72) */
  interface XcmV3Junction extends Enum {
    readonly isParachain: boolean;
    readonly asParachain: Compact<u32>;
    readonly isAccountId32: boolean;
    readonly asAccountId32: {
      readonly network: Option<XcmV3JunctionNetworkId>;
      readonly id: U8aFixed;
    } & Struct;
    readonly isAccountIndex64: boolean;
    readonly asAccountIndex64: {
      readonly network: Option<XcmV3JunctionNetworkId>;
      readonly index: Compact<u64>;
    } & Struct;
    readonly isAccountKey20: boolean;
    readonly asAccountKey20: {
      readonly network: Option<XcmV3JunctionNetworkId>;
      readonly key: U8aFixed;
    } & Struct;
    readonly isPalletInstance: boolean;
    readonly asPalletInstance: u8;
    readonly isGeneralIndex: boolean;
    readonly asGeneralIndex: Compact<u128>;
    readonly isGeneralKey: boolean;
    readonly asGeneralKey: {
      readonly length: u8;
      readonly data: U8aFixed;
    } & Struct;
    readonly isOnlyChild: boolean;
    readonly isPlurality: boolean;
    readonly asPlurality: {
      readonly id: XcmV3JunctionBodyId;
      readonly part: XcmV3JunctionBodyPart;
    } & Struct;
    readonly isGlobalConsensus: boolean;
    readonly asGlobalConsensus: XcmV3JunctionNetworkId;
    readonly type: 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality' | 'GlobalConsensus';
  }

  /** @name XcmV3JunctionNetworkId (75) */
  interface XcmV3JunctionNetworkId extends Enum {
    readonly isByGenesis: boolean;
    readonly asByGenesis: U8aFixed;
    readonly isByFork: boolean;
    readonly asByFork: {
      readonly blockNumber: u64;
      readonly blockHash: U8aFixed;
    } & Struct;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isWestend: boolean;
    readonly isRococo: boolean;
    readonly isWococo: boolean;
    readonly isEthereum: boolean;
    readonly asEthereum: {
      readonly chainId: Compact<u64>;
    } & Struct;
    readonly isBitcoinCore: boolean;
    readonly isBitcoinCash: boolean;
    readonly type: 'ByGenesis' | 'ByFork' | 'Polkadot' | 'Kusama' | 'Westend' | 'Rococo' | 'Wococo' | 'Ethereum' | 'BitcoinCore' | 'BitcoinCash';
  }

  /** @name XcmV3JunctionBodyId (78) */
  interface XcmV3JunctionBodyId extends Enum {
    readonly isUnit: boolean;
    readonly isMoniker: boolean;
    readonly asMoniker: U8aFixed;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u32>;
    readonly isExecutive: boolean;
    readonly isTechnical: boolean;
    readonly isLegislative: boolean;
    readonly isJudicial: boolean;
    readonly isDefense: boolean;
    readonly isAdministration: boolean;
    readonly isTreasury: boolean;
    readonly type: 'Unit' | 'Moniker' | 'Index' | 'Executive' | 'Technical' | 'Legislative' | 'Judicial' | 'Defense' | 'Administration' | 'Treasury';
  }

  /** @name XcmV3JunctionBodyPart (79) */
  interface XcmV3JunctionBodyPart extends Enum {
    readonly isVoice: boolean;
    readonly isMembers: boolean;
    readonly asMembers: {
      readonly count: Compact<u32>;
    } & Struct;
    readonly isFraction: boolean;
    readonly asFraction: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly isAtLeastProportion: boolean;
    readonly asAtLeastProportion: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly isMoreThanProportion: boolean;
    readonly asMoreThanProportion: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly type: 'Voice' | 'Members' | 'Fraction' | 'AtLeastProportion' | 'MoreThanProportion';
  }

  /** @name XcmV3Xcm (80) */
  interface XcmV3Xcm extends Vec<XcmV3Instruction> {}

  /** @name XcmV3Instruction (82) */
  interface XcmV3Instruction extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: XcmV3MultiassetMultiAssets;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: XcmV3MultiassetMultiAssets;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: XcmV3MultiassetMultiAssets;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV3Response;
      readonly maxWeight: SpWeightsWeightV2Weight;
      readonly querier: Option<XcmV3MultiLocation>;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: XcmV3MultiassetMultiAssets;
      readonly beneficiary: XcmV3MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: XcmV3MultiassetMultiAssets;
      readonly dest: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originKind: XcmV2OriginKind;
      readonly requireWeightAtMost: SpWeightsWeightV2Weight;
      readonly call: XcmDoubleEncoded;
    } & Struct;
    readonly isHrmpNewChannelOpenRequest: boolean;
    readonly asHrmpNewChannelOpenRequest: {
      readonly sender: Compact<u32>;
      readonly maxMessageSize: Compact<u32>;
      readonly maxCapacity: Compact<u32>;
    } & Struct;
    readonly isHrmpChannelAccepted: boolean;
    readonly asHrmpChannelAccepted: {
      readonly recipient: Compact<u32>;
    } & Struct;
    readonly isHrmpChannelClosing: boolean;
    readonly asHrmpChannelClosing: {
      readonly initiator: Compact<u32>;
      readonly sender: Compact<u32>;
      readonly recipient: Compact<u32>;
    } & Struct;
    readonly isClearOrigin: boolean;
    readonly isDescendOrigin: boolean;
    readonly asDescendOrigin: XcmV3Junctions;
    readonly isReportError: boolean;
    readonly asReportError: XcmV3QueryResponseInfo;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly beneficiary: XcmV3MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly dest: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: XcmV3MultiassetMultiAssetFilter;
      readonly want: XcmV3MultiassetMultiAssets;
      readonly maximal: bool;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly reserve: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV3MultiassetMultiAssetFilter;
      readonly dest: XcmV3MultiLocation;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isReportHolding: boolean;
    readonly asReportHolding: {
      readonly responseInfo: XcmV3QueryResponseInfo;
      readonly assets: XcmV3MultiassetMultiAssetFilter;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV3MultiAsset;
      readonly weightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isRefundSurplus: boolean;
    readonly isSetErrorHandler: boolean;
    readonly asSetErrorHandler: XcmV3Xcm;
    readonly isSetAppendix: boolean;
    readonly asSetAppendix: XcmV3Xcm;
    readonly isClearError: boolean;
    readonly isClaimAsset: boolean;
    readonly asClaimAsset: {
      readonly assets: XcmV3MultiassetMultiAssets;
      readonly ticket: XcmV3MultiLocation;
    } & Struct;
    readonly isTrap: boolean;
    readonly asTrap: Compact<u64>;
    readonly isSubscribeVersion: boolean;
    readonly asSubscribeVersion: {
      readonly queryId: Compact<u64>;
      readonly maxResponseWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isUnsubscribeVersion: boolean;
    readonly isBurnAsset: boolean;
    readonly asBurnAsset: XcmV3MultiassetMultiAssets;
    readonly isExpectAsset: boolean;
    readonly asExpectAsset: XcmV3MultiassetMultiAssets;
    readonly isExpectOrigin: boolean;
    readonly asExpectOrigin: Option<XcmV3MultiLocation>;
    readonly isExpectError: boolean;
    readonly asExpectError: Option<ITuple<[u32, XcmV3TraitsError]>>;
    readonly isExpectTransactStatus: boolean;
    readonly asExpectTransactStatus: XcmV3MaybeErrorCode;
    readonly isQueryPallet: boolean;
    readonly asQueryPallet: {
      readonly moduleName: Bytes;
      readonly responseInfo: XcmV3QueryResponseInfo;
    } & Struct;
    readonly isExpectPallet: boolean;
    readonly asExpectPallet: {
      readonly index: Compact<u32>;
      readonly name: Bytes;
      readonly moduleName: Bytes;
      readonly crateMajor: Compact<u32>;
      readonly minCrateMinor: Compact<u32>;
    } & Struct;
    readonly isReportTransactStatus: boolean;
    readonly asReportTransactStatus: XcmV3QueryResponseInfo;
    readonly isClearTransactStatus: boolean;
    readonly isUniversalOrigin: boolean;
    readonly asUniversalOrigin: XcmV3Junction;
    readonly isExportMessage: boolean;
    readonly asExportMessage: {
      readonly network: XcmV3JunctionNetworkId;
      readonly destination: XcmV3Junctions;
      readonly xcm: XcmV3Xcm;
    } & Struct;
    readonly isLockAsset: boolean;
    readonly asLockAsset: {
      readonly asset: XcmV3MultiAsset;
      readonly unlocker: XcmV3MultiLocation;
    } & Struct;
    readonly isUnlockAsset: boolean;
    readonly asUnlockAsset: {
      readonly asset: XcmV3MultiAsset;
      readonly target: XcmV3MultiLocation;
    } & Struct;
    readonly isNoteUnlockable: boolean;
    readonly asNoteUnlockable: {
      readonly asset: XcmV3MultiAsset;
      readonly owner: XcmV3MultiLocation;
    } & Struct;
    readonly isRequestUnlock: boolean;
    readonly asRequestUnlock: {
      readonly asset: XcmV3MultiAsset;
      readonly locker: XcmV3MultiLocation;
    } & Struct;
    readonly isSetFeesMode: boolean;
    readonly asSetFeesMode: {
      readonly jitWithdraw: bool;
    } & Struct;
    readonly isSetTopic: boolean;
    readonly asSetTopic: U8aFixed;
    readonly isClearTopic: boolean;
    readonly isAliasOrigin: boolean;
    readonly asAliasOrigin: XcmV3MultiLocation;
    readonly isUnpaidExecution: boolean;
    readonly asUnpaidExecution: {
      readonly weightLimit: XcmV3WeightLimit;
      readonly checkOrigin: Option<XcmV3MultiLocation>;
    } & Struct;
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposited' | 'ReceiveTeleportedAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'ClearOrigin' | 'DescendOrigin' | 'ReportError' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'ReportHolding' | 'BuyExecution' | 'RefundSurplus' | 'SetErrorHandler' | 'SetAppendix' | 'ClearError' | 'ClaimAsset' | 'Trap' | 'SubscribeVersion' | 'UnsubscribeVersion' | 'BurnAsset' | 'ExpectAsset' | 'ExpectOrigin' | 'ExpectError' | 'ExpectTransactStatus' | 'QueryPallet' | 'ExpectPallet' | 'ReportTransactStatus' | 'ClearTransactStatus' | 'UniversalOrigin' | 'ExportMessage' | 'LockAsset' | 'UnlockAsset' | 'NoteUnlockable' | 'RequestUnlock' | 'SetFeesMode' | 'SetTopic' | 'ClearTopic' | 'AliasOrigin' | 'UnpaidExecution';
  }

  /** @name XcmV3MultiassetMultiAssets (83) */
  interface XcmV3MultiassetMultiAssets extends Vec<XcmV3MultiAsset> {}

  /** @name XcmV3MultiAsset (85) */
  interface XcmV3MultiAsset extends Struct {
    readonly id: XcmV3MultiassetAssetId;
    readonly fun: XcmV3MultiassetFungibility;
  }

  /** @name XcmV3MultiassetAssetId (86) */
  interface XcmV3MultiassetAssetId extends Enum {
    readonly isConcrete: boolean;
    readonly asConcrete: XcmV3MultiLocation;
    readonly isAbstract: boolean;
    readonly asAbstract: U8aFixed;
    readonly type: 'Concrete' | 'Abstract';
  }

  /** @name XcmV3MultiassetFungibility (87) */
  interface XcmV3MultiassetFungibility extends Enum {
    readonly isFungible: boolean;
    readonly asFungible: Compact<u128>;
    readonly isNonFungible: boolean;
    readonly asNonFungible: XcmV3MultiassetAssetInstance;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV3MultiassetAssetInstance (88) */
  interface XcmV3MultiassetAssetInstance extends Enum {
    readonly isUndefined: boolean;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u128>;
    readonly isArray4: boolean;
    readonly asArray4: U8aFixed;
    readonly isArray8: boolean;
    readonly asArray8: U8aFixed;
    readonly isArray16: boolean;
    readonly asArray16: U8aFixed;
    readonly isArray32: boolean;
    readonly asArray32: U8aFixed;
    readonly type: 'Undefined' | 'Index' | 'Array4' | 'Array8' | 'Array16' | 'Array32';
  }

  /** @name XcmV3Response (91) */
  interface XcmV3Response extends Enum {
    readonly isNull: boolean;
    readonly isAssets: boolean;
    readonly asAssets: XcmV3MultiassetMultiAssets;
    readonly isExecutionResult: boolean;
    readonly asExecutionResult: Option<ITuple<[u32, XcmV3TraitsError]>>;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly isPalletsInfo: boolean;
    readonly asPalletsInfo: Vec<XcmV3PalletInfo>;
    readonly isDispatchResult: boolean;
    readonly asDispatchResult: XcmV3MaybeErrorCode;
    readonly type: 'Null' | 'Assets' | 'ExecutionResult' | 'Version' | 'PalletsInfo' | 'DispatchResult';
  }

  /** @name XcmV3PalletInfo (95) */
  interface XcmV3PalletInfo extends Struct {
    readonly index: Compact<u32>;
    readonly name: Bytes;
    readonly moduleName: Bytes;
    readonly major: Compact<u32>;
    readonly minor: Compact<u32>;
    readonly patch: Compact<u32>;
  }

  /** @name XcmV3MaybeErrorCode (98) */
  interface XcmV3MaybeErrorCode extends Enum {
    readonly isSuccess: boolean;
    readonly isError: boolean;
    readonly asError: Bytes;
    readonly isTruncatedError: boolean;
    readonly asTruncatedError: Bytes;
    readonly type: 'Success' | 'Error' | 'TruncatedError';
  }

  /** @name XcmV2OriginKind (101) */
  interface XcmV2OriginKind extends Enum {
    readonly isNative: boolean;
    readonly isSovereignAccount: boolean;
    readonly isSuperuser: boolean;
    readonly isXcm: boolean;
    readonly type: 'Native' | 'SovereignAccount' | 'Superuser' | 'Xcm';
  }

  /** @name XcmDoubleEncoded (102) */
  interface XcmDoubleEncoded extends Struct {
    readonly encoded: Bytes;
  }

  /** @name XcmV3QueryResponseInfo (103) */
  interface XcmV3QueryResponseInfo extends Struct {
    readonly destination: XcmV3MultiLocation;
    readonly queryId: Compact<u64>;
    readonly maxWeight: SpWeightsWeightV2Weight;
  }

  /** @name XcmV3MultiassetMultiAssetFilter (104) */
  interface XcmV3MultiassetMultiAssetFilter extends Enum {
    readonly isDefinite: boolean;
    readonly asDefinite: XcmV3MultiassetMultiAssets;
    readonly isWild: boolean;
    readonly asWild: XcmV3MultiassetWildMultiAsset;
    readonly type: 'Definite' | 'Wild';
  }

  /** @name XcmV3MultiassetWildMultiAsset (105) */
  interface XcmV3MultiassetWildMultiAsset extends Enum {
    readonly isAll: boolean;
    readonly isAllOf: boolean;
    readonly asAllOf: {
      readonly id: XcmV3MultiassetAssetId;
      readonly fun: XcmV3MultiassetWildFungibility;
    } & Struct;
    readonly isAllCounted: boolean;
    readonly asAllCounted: Compact<u32>;
    readonly isAllOfCounted: boolean;
    readonly asAllOfCounted: {
      readonly id: XcmV3MultiassetAssetId;
      readonly fun: XcmV3MultiassetWildFungibility;
      readonly count: Compact<u32>;
    } & Struct;
    readonly type: 'All' | 'AllOf' | 'AllCounted' | 'AllOfCounted';
  }

  /** @name XcmV3MultiassetWildFungibility (106) */
  interface XcmV3MultiassetWildFungibility extends Enum {
    readonly isFungible: boolean;
    readonly isNonFungible: boolean;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV3WeightLimit (107) */
  interface XcmV3WeightLimit extends Enum {
    readonly isUnlimited: boolean;
    readonly isLimited: boolean;
    readonly asLimited: SpWeightsWeightV2Weight;
    readonly type: 'Unlimited' | 'Limited';
  }

  /** @name XcmVersionedMultiAssets (108) */
  interface XcmVersionedMultiAssets extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2MultiassetMultiAssets;
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiassetMultiAssets;
    readonly type: 'V2' | 'V3';
  }

  /** @name XcmV2MultiassetMultiAssets (109) */
  interface XcmV2MultiassetMultiAssets extends Vec<XcmV2MultiAsset> {}

  /** @name XcmV2MultiAsset (111) */
  interface XcmV2MultiAsset extends Struct {
    readonly id: XcmV2MultiassetAssetId;
    readonly fun: XcmV2MultiassetFungibility;
  }

  /** @name XcmV2MultiassetAssetId (112) */
  interface XcmV2MultiassetAssetId extends Enum {
    readonly isConcrete: boolean;
    readonly asConcrete: XcmV2MultiLocation;
    readonly isAbstract: boolean;
    readonly asAbstract: Bytes;
    readonly type: 'Concrete' | 'Abstract';
  }

  /** @name XcmV2MultiLocation (113) */
  interface XcmV2MultiLocation extends Struct {
    readonly parents: u8;
    readonly interior: XcmV2MultilocationJunctions;
  }

  /** @name XcmV2MultilocationJunctions (114) */
  interface XcmV2MultilocationJunctions extends Enum {
    readonly isHere: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV2Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV2Junction, XcmV2Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
    readonly type: 'Here' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV2Junction (115) */
  interface XcmV2Junction extends Enum {
    readonly isParachain: boolean;
    readonly asParachain: Compact<u32>;
    readonly isAccountId32: boolean;
    readonly asAccountId32: {
      readonly network: XcmV2NetworkId;
      readonly id: U8aFixed;
    } & Struct;
    readonly isAccountIndex64: boolean;
    readonly asAccountIndex64: {
      readonly network: XcmV2NetworkId;
      readonly index: Compact<u64>;
    } & Struct;
    readonly isAccountKey20: boolean;
    readonly asAccountKey20: {
      readonly network: XcmV2NetworkId;
      readonly key: U8aFixed;
    } & Struct;
    readonly isPalletInstance: boolean;
    readonly asPalletInstance: u8;
    readonly isGeneralIndex: boolean;
    readonly asGeneralIndex: Compact<u128>;
    readonly isGeneralKey: boolean;
    readonly asGeneralKey: Bytes;
    readonly isOnlyChild: boolean;
    readonly isPlurality: boolean;
    readonly asPlurality: {
      readonly id: XcmV2BodyId;
      readonly part: XcmV2BodyPart;
    } & Struct;
    readonly type: 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality';
  }

  /** @name XcmV2NetworkId (116) */
  interface XcmV2NetworkId extends Enum {
    readonly isAny: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly type: 'Any' | 'Named' | 'Polkadot' | 'Kusama';
  }

  /** @name XcmV2BodyId (118) */
  interface XcmV2BodyId extends Enum {
    readonly isUnit: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u32>;
    readonly isExecutive: boolean;
    readonly isTechnical: boolean;
    readonly isLegislative: boolean;
    readonly isJudicial: boolean;
    readonly isDefense: boolean;
    readonly isAdministration: boolean;
    readonly isTreasury: boolean;
    readonly type: 'Unit' | 'Named' | 'Index' | 'Executive' | 'Technical' | 'Legislative' | 'Judicial' | 'Defense' | 'Administration' | 'Treasury';
  }

  /** @name XcmV2BodyPart (119) */
  interface XcmV2BodyPart extends Enum {
    readonly isVoice: boolean;
    readonly isMembers: boolean;
    readonly asMembers: {
      readonly count: Compact<u32>;
    } & Struct;
    readonly isFraction: boolean;
    readonly asFraction: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly isAtLeastProportion: boolean;
    readonly asAtLeastProportion: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly isMoreThanProportion: boolean;
    readonly asMoreThanProportion: {
      readonly nom: Compact<u32>;
      readonly denom: Compact<u32>;
    } & Struct;
    readonly type: 'Voice' | 'Members' | 'Fraction' | 'AtLeastProportion' | 'MoreThanProportion';
  }

  /** @name XcmV2MultiassetFungibility (120) */
  interface XcmV2MultiassetFungibility extends Enum {
    readonly isFungible: boolean;
    readonly asFungible: Compact<u128>;
    readonly isNonFungible: boolean;
    readonly asNonFungible: XcmV2MultiassetAssetInstance;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV2MultiassetAssetInstance (121) */
  interface XcmV2MultiassetAssetInstance extends Enum {
    readonly isUndefined: boolean;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u128>;
    readonly isArray4: boolean;
    readonly asArray4: U8aFixed;
    readonly isArray8: boolean;
    readonly asArray8: U8aFixed;
    readonly isArray16: boolean;
    readonly asArray16: U8aFixed;
    readonly isArray32: boolean;
    readonly asArray32: U8aFixed;
    readonly isBlob: boolean;
    readonly asBlob: Bytes;
    readonly type: 'Undefined' | 'Index' | 'Array4' | 'Array8' | 'Array16' | 'Array32' | 'Blob';
  }

  /** @name XcmVersionedMultiLocation (122) */
  interface XcmVersionedMultiLocation extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2MultiLocation;
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiLocation;
    readonly type: 'V2' | 'V3';
  }

  /** @name CumulusPalletXcmEvent (123) */
  interface CumulusPalletXcmEvent extends Enum {
    readonly isInvalidFormat: boolean;
    readonly asInvalidFormat: U8aFixed;
    readonly isUnsupportedVersion: boolean;
    readonly asUnsupportedVersion: U8aFixed;
    readonly isExecutedDownward: boolean;
    readonly asExecutedDownward: ITuple<[U8aFixed, XcmV3TraitsOutcome]>;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward';
  }

  /** @name CumulusPalletDmpQueueEvent (124) */
  interface CumulusPalletDmpQueueEvent extends Enum {
    readonly isInvalidFormat: boolean;
    readonly asInvalidFormat: {
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isUnsupportedVersion: boolean;
    readonly asUnsupportedVersion: {
      readonly messageId: U8aFixed;
    } & Struct;
    readonly isExecutedDownward: boolean;
    readonly asExecutedDownward: {
      readonly messageId: U8aFixed;
      readonly outcome: XcmV3TraitsOutcome;
    } & Struct;
    readonly isWeightExhausted: boolean;
    readonly asWeightExhausted: {
      readonly messageId: U8aFixed;
      readonly remainingWeight: SpWeightsWeightV2Weight;
      readonly requiredWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly messageId: U8aFixed;
      readonly overweightIndex: u64;
      readonly requiredWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly overweightIndex: u64;
      readonly weightUsed: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isMaxMessagesExhausted: boolean;
    readonly asMaxMessagesExhausted: {
      readonly messageId: U8aFixed;
    } & Struct;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward' | 'WeightExhausted' | 'OverweightEnqueued' | 'OverweightServiced' | 'MaxMessagesExhausted';
  }

  /** @name OrmlXtokensModuleEvent (125) */
  interface OrmlXtokensModuleEvent extends Enum {
    readonly isTransferredMultiAssets: boolean;
    readonly asTransferredMultiAssets: {
      readonly sender: AccountId32;
      readonly assets: XcmV3MultiassetMultiAssets;
      readonly fee: XcmV3MultiAsset;
      readonly dest: XcmV3MultiLocation;
    } & Struct;
    readonly type: 'TransferredMultiAssets';
  }

  /** @name OrmlUnknownTokensModuleEvent (126) */
  interface OrmlUnknownTokensModuleEvent extends Enum {
    readonly isDeposited: boolean;
    readonly asDeposited: {
      readonly asset: XcmV3MultiAsset;
      readonly who: XcmV3MultiLocation;
    } & Struct;
    readonly isWithdrawn: boolean;
    readonly asWithdrawn: {
      readonly asset: XcmV3MultiAsset;
      readonly who: XcmV3MultiLocation;
    } & Struct;
    readonly type: 'Deposited' | 'Withdrawn';
  }

  /** @name OrmlTokensModuleEvent (127) */
  interface OrmlTokensModuleEvent extends Enum {
    readonly isEndowed: boolean;
    readonly asEndowed: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isDustLost: boolean;
    readonly asDustLost: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly currencyId: u128;
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isReserved: boolean;
    readonly asReserved: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUnreserved: boolean;
    readonly asUnreserved: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isReserveRepatriated: boolean;
    readonly asReserveRepatriated: {
      readonly currencyId: u128;
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly status: FrameSupportTokensMiscBalanceStatus;
    } & Struct;
    readonly isBalanceSet: boolean;
    readonly asBalanceSet: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly free: u128;
      readonly reserved: u128;
    } & Struct;
    readonly isTotalIssuanceSet: boolean;
    readonly asTotalIssuanceSet: {
      readonly currencyId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isWithdrawn: boolean;
    readonly asWithdrawn: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isSlashed: boolean;
    readonly asSlashed: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly freeAmount: u128;
      readonly reservedAmount: u128;
    } & Struct;
    readonly isDeposited: boolean;
    readonly asDeposited: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isLockSet: boolean;
    readonly asLockSet: {
      readonly lockId: U8aFixed;
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isLockRemoved: boolean;
    readonly asLockRemoved: {
      readonly lockId: U8aFixed;
      readonly currencyId: u128;
      readonly who: AccountId32;
    } & Struct;
    readonly isLocked: boolean;
    readonly asLocked: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isUnlocked: boolean;
    readonly asUnlocked: {
      readonly currencyId: u128;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly type: 'Endowed' | 'DustLost' | 'Transfer' | 'Reserved' | 'Unreserved' | 'ReserveRepatriated' | 'BalanceSet' | 'TotalIssuanceSet' | 'Withdrawn' | 'Slashed' | 'Deposited' | 'LockSet' | 'LockRemoved' | 'Locked' | 'Unlocked';
  }

  /** @name PalletCurrencyFactoryEvent (129) */
  interface PalletCurrencyFactoryEvent extends Enum {
    readonly isRangeCreated: boolean;
    readonly asRangeCreated: {
      readonly range: {
      readonly current: u128;
      readonly end: u128;
    } & Struct;
    } & Struct;
    readonly type: 'RangeCreated';
  }

  /** @name PalletCrowdloanRewardsEvent (131) */
  interface PalletCrowdloanRewardsEvent extends Enum {
    readonly isInitialized: boolean;
    readonly asInitialized: {
      readonly at: u64;
    } & Struct;
    readonly isClaimed: boolean;
    readonly asClaimed: {
      readonly remoteAccount: PalletCrowdloanRewardsModelsRemoteAccount;
      readonly rewardAccount: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isAssociated: boolean;
    readonly asAssociated: {
      readonly remoteAccount: PalletCrowdloanRewardsModelsRemoteAccount;
      readonly rewardAccount: AccountId32;
    } & Struct;
    readonly isOverFunded: boolean;
    readonly asOverFunded: {
      readonly excessFunds: u128;
    } & Struct;
    readonly isRewardsUnlocked: boolean;
    readonly asRewardsUnlocked: {
      readonly at: u64;
    } & Struct;
    readonly isRewardsAdded: boolean;
    readonly asRewardsAdded: {
      readonly additions: Vec<ITuple<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>>;
    } & Struct;
    readonly isRewardsDeleted: boolean;
    readonly asRewardsDeleted: {
      readonly deletions: Vec<PalletCrowdloanRewardsModelsRemoteAccount>;
    } & Struct;
    readonly type: 'Initialized' | 'Claimed' | 'Associated' | 'OverFunded' | 'RewardsUnlocked' | 'RewardsAdded' | 'RewardsDeleted';
  }

  /** @name PalletCrowdloanRewardsModelsRemoteAccount (132) */
  interface PalletCrowdloanRewardsModelsRemoteAccount extends Enum {
    readonly isRelayChain: boolean;
    readonly asRelayChain: AccountId32;
    readonly isEthereum: boolean;
    readonly asEthereum: ComposableSupportEthereumAddress;
    readonly type: 'RelayChain' | 'Ethereum';
  }

  /** @name ComposableSupportEthereumAddress (133) */
  interface ComposableSupportEthereumAddress extends U8aFixed {}

  /** @name PalletVestingModuleEvent (137) */
  interface PalletVestingModuleEvent extends Enum {
    readonly isVestingScheduleAdded: boolean;
    readonly asVestingScheduleAdded: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly asset: u128;
      readonly vestingScheduleId: u128;
      readonly schedule: PalletVestingVestingSchedule;
      readonly scheduleAmount: u128;
    } & Struct;
    readonly isClaimed: boolean;
    readonly asClaimed: {
      readonly who: AccountId32;
      readonly asset: u128;
      readonly vestingScheduleIds: PalletVestingVestingScheduleIdSet;
      readonly lockedAmount: u128;
      readonly claimedAmountPerSchedule: BTreeMap<u128, u128>;
    } & Struct;
    readonly isVestingSchedulesUpdated: boolean;
    readonly asVestingSchedulesUpdated: {
      readonly who: AccountId32;
    } & Struct;
    readonly type: 'VestingScheduleAdded' | 'Claimed' | 'VestingSchedulesUpdated';
  }

  /** @name PalletVestingVestingSchedule (138) */
  interface PalletVestingVestingSchedule extends Struct {
    readonly vestingScheduleId: u128;
    readonly window: PalletVestingVestingWindow;
    readonly periodCount: u32;
    readonly perPeriod: Compact<u128>;
    readonly alreadyClaimed: u128;
  }

  /** @name PalletVestingVestingWindow (139) */
  interface PalletVestingVestingWindow extends Enum {
    readonly isMomentBased: boolean;
    readonly asMomentBased: {
      readonly start: u64;
      readonly period: u64;
    } & Struct;
    readonly isBlockNumberBased: boolean;
    readonly asBlockNumberBased: {
      readonly start: u32;
      readonly period: u32;
    } & Struct;
    readonly type: 'MomentBased' | 'BlockNumberBased';
  }

  /** @name PalletVestingVestingScheduleIdSet (140) */
  interface PalletVestingVestingScheduleIdSet extends Enum {
    readonly isAll: boolean;
    readonly isOne: boolean;
    readonly asOne: u128;
    readonly isMany: boolean;
    readonly asMany: Vec<u128>;
    readonly type: 'All' | 'One' | 'Many';
  }

  /** @name PalletBondedFinanceEvent (147) */
  interface PalletBondedFinanceEvent extends Enum {
    readonly isNewOffer: boolean;
    readonly asNewOffer: {
      readonly offerId: u128;
      readonly beneficiary: AccountId32;
    } & Struct;
    readonly isNewBond: boolean;
    readonly asNewBond: {
      readonly offerId: u128;
      readonly who: AccountId32;
      readonly nbOfBonds: u128;
    } & Struct;
    readonly isOfferCancelled: boolean;
    readonly asOfferCancelled: {
      readonly offerId: u128;
    } & Struct;
    readonly isOfferCompleted: boolean;
    readonly asOfferCompleted: {
      readonly offerId: u128;
    } & Struct;
    readonly type: 'NewOffer' | 'NewBond' | 'OfferCancelled' | 'OfferCompleted';
  }

  /** @name PalletAssetsRegistryEvent (148) */
  interface PalletAssetsRegistryEvent extends Enum {
    readonly isAssetRegistered: boolean;
    readonly asAssetRegistered: {
      readonly assetId: u128;
      readonly location: Option<PrimitivesCurrencyForeignAssetId>;
      readonly assetInfo: ComposableTraitsAssetsAssetInfo;
    } & Struct;
    readonly isAssetUpdated: boolean;
    readonly asAssetUpdated: {
      readonly assetId: u128;
      readonly assetInfo: ComposableTraitsAssetsAssetInfoUpdate;
    } & Struct;
    readonly isAssetLocationUpdated: boolean;
    readonly asAssetLocationUpdated: {
      readonly assetId: u128;
      readonly location: PrimitivesCurrencyForeignAssetId;
    } & Struct;
    readonly isAssetLocationRemoved: boolean;
    readonly asAssetLocationRemoved: {
      readonly assetId: u128;
    } & Struct;
    readonly isMinFeeUpdated: boolean;
    readonly asMinFeeUpdated: {
      readonly targetParachainId: u32;
      readonly foreignAssetId: PrimitivesCurrencyForeignAssetId;
      readonly amount: Option<u128>;
    } & Struct;
    readonly type: 'AssetRegistered' | 'AssetUpdated' | 'AssetLocationUpdated' | 'AssetLocationRemoved' | 'MinFeeUpdated';
  }

  /** @name PrimitivesCurrencyForeignAssetId (150) */
  interface PrimitivesCurrencyForeignAssetId extends Enum {
    readonly isXcm: boolean;
    readonly asXcm: PrimitivesCurrencyVersionedMultiLocation;
    readonly isIbcIcs20: boolean;
    readonly asIbcIcs20: PrimitivesCurrencyPrefixedDenom;
    readonly type: 'Xcm' | 'IbcIcs20';
  }

  /** @name PrimitivesCurrencyVersionedMultiLocation (151) */
  interface PrimitivesCurrencyVersionedMultiLocation extends Enum {
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiLocation;
    readonly type: 'V3';
  }

  /** @name PrimitivesCurrencyPrefixedDenom (152) */
  interface PrimitivesCurrencyPrefixedDenom extends IbcApplicationsTransferDenomPrefixedDenom {}

  /** @name IbcApplicationsTransferDenomPrefixedDenom (153) */
  interface IbcApplicationsTransferDenomPrefixedDenom extends Struct {
    readonly tracePath: IbcApplicationsTransferDenomTracePath;
    readonly baseDenom: Text;
  }

  /** @name IbcApplicationsTransferDenomTracePath (154) */
  interface IbcApplicationsTransferDenomTracePath extends Vec<IbcApplicationsTransferDenomTracePrefix> {}

  /** @name IbcApplicationsTransferDenomTracePrefix (156) */
  interface IbcApplicationsTransferDenomTracePrefix extends Struct {
    readonly portId: Text;
    readonly channelId: Text;
  }

  /** @name ComposableTraitsAssetsAssetInfo (161) */
  interface ComposableTraitsAssetsAssetInfo extends Struct {
    readonly name: Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>;
    readonly symbol: Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>;
    readonly decimals: Option<u8>;
    readonly existentialDeposit: u128;
    readonly ratio: Option<ComposableTraitsCurrencyRational64>;
  }

  /** @name ComposableSupportCollectionsVecBoundedBiBoundedVec (163) */
  interface ComposableSupportCollectionsVecBoundedBiBoundedVec extends Struct {
    readonly inner: Bytes;
  }

  /** @name ComposableTraitsCurrencyRational64 (168) */
  interface ComposableTraitsCurrencyRational64 extends Struct {
    readonly n: u64;
    readonly d: u64;
  }

  /** @name ComposableTraitsAssetsAssetInfoUpdate (169) */
  interface ComposableTraitsAssetsAssetInfoUpdate extends Struct {
    readonly name: {
      readonly isIgnore: boolean;
      readonly isSet: boolean;
      readonly asSet: Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>;
      readonly type: 'Ignore' | 'Set';
    } & Enum;
    readonly symbol: {
      readonly isIgnore: boolean;
      readonly isSet: boolean;
      readonly asSet: Option<ComposableSupportCollectionsVecBoundedBiBoundedVec>;
      readonly type: 'Ignore' | 'Set';
    } & Enum;
    readonly decimals: {
      readonly isIgnore: boolean;
      readonly isSet: boolean;
      readonly asSet: Option<u8>;
      readonly type: 'Ignore' | 'Set';
    } & Enum;
    readonly existentialDeposit: ComposableTraitsStorageUpdateValueU128;
    readonly ratio: ComposableTraitsStorageUpdateValueOption;
  }

  /** @name ComposableTraitsStorageUpdateValueU128 (173) */
  interface ComposableTraitsStorageUpdateValueU128 extends Enum {
    readonly isIgnore: boolean;
    readonly isSet: boolean;
    readonly asSet: u128;
    readonly type: 'Ignore' | 'Set';
  }

  /** @name ComposableTraitsStorageUpdateValueOption (174) */
  interface ComposableTraitsStorageUpdateValueOption extends Enum {
    readonly isIgnore: boolean;
    readonly isSet: boolean;
    readonly asSet: Option<ComposableTraitsCurrencyRational64>;
    readonly type: 'Ignore' | 'Set';
  }

  /** @name PalletPabloEvent (176) */
  interface PalletPabloEvent extends Enum {
    readonly isPoolCreated: boolean;
    readonly asPoolCreated: {
      readonly poolId: u128;
      readonly owner: AccountId32;
      readonly assetWeights: BTreeMap<u128, Permill>;
      readonly lpTokenId: u128;
    } & Struct;
    readonly isLiquidityAdded: boolean;
    readonly asLiquidityAdded: {
      readonly who: AccountId32;
      readonly poolId: u128;
      readonly assetAmounts: BTreeMap<u128, u128>;
      readonly mintedLp: u128;
    } & Struct;
    readonly isLiquidityRemoved: boolean;
    readonly asLiquidityRemoved: {
      readonly who: AccountId32;
      readonly poolId: u128;
      readonly assetAmounts: BTreeMap<u128, u128>;
    } & Struct;
    readonly isSwapped: boolean;
    readonly asSwapped: {
      readonly poolId: u128;
      readonly who: AccountId32;
      readonly baseAsset: u128;
      readonly quoteAsset: u128;
      readonly baseAmount: u128;
      readonly quoteAmount: u128;
      readonly fee: ComposableTraitsDexFee;
    } & Struct;
    readonly isTwapUpdated: boolean;
    readonly asTwapUpdated: {
      readonly poolId: u128;
      readonly timestamp: u64;
      readonly twaps: BTreeMap<u128, u128>;
    } & Struct;
    readonly type: 'PoolCreated' | 'LiquidityAdded' | 'LiquidityRemoved' | 'Swapped' | 'TwapUpdated';
  }

  /** @name ComposableTraitsDexFee (184) */
  interface ComposableTraitsDexFee extends Struct {
    readonly fee: u128;
    readonly lpFee: u128;
    readonly ownerFee: u128;
    readonly protocolFee: u128;
    readonly assetId: u128;
  }

  /** @name PalletOracleEvent (189) */
  interface PalletOracleEvent extends Enum {
    readonly isAssetInfoChange: boolean;
    readonly asAssetInfoChange: ITuple<[u128, Percent, u32, u32, u32, u128, u128]>;
    readonly isSignerSet: boolean;
    readonly asSignerSet: ITuple<[AccountId32, AccountId32]>;
    readonly isStakeAdded: boolean;
    readonly asStakeAdded: ITuple<[AccountId32, u128, u128]>;
    readonly isStakeRemoved: boolean;
    readonly asStakeRemoved: ITuple<[AccountId32, u128, u32]>;
    readonly isStakeReclaimed: boolean;
    readonly asStakeReclaimed: ITuple<[AccountId32, u128]>;
    readonly isPriceSubmitted: boolean;
    readonly asPriceSubmitted: ITuple<[AccountId32, u128, u128]>;
    readonly isUserSlashed: boolean;
    readonly asUserSlashed: ITuple<[AccountId32, u128, u128]>;
    readonly isOracleRewarded: boolean;
    readonly asOracleRewarded: ITuple<[AccountId32, u128, u128]>;
    readonly isRewardingAdjustment: boolean;
    readonly asRewardingAdjustment: u64;
    readonly isAnswerPruned: boolean;
    readonly asAnswerPruned: ITuple<[AccountId32, u128]>;
    readonly isPriceChanged: boolean;
    readonly asPriceChanged: ITuple<[u128, u128]>;
    readonly isSignerRemoved: boolean;
    readonly asSignerRemoved: ITuple<[AccountId32, AccountId32, u128]>;
    readonly type: 'AssetInfoChange' | 'SignerSet' | 'StakeAdded' | 'StakeRemoved' | 'StakeReclaimed' | 'PriceSubmitted' | 'UserSlashed' | 'OracleRewarded' | 'RewardingAdjustment' | 'AnswerPruned' | 'PriceChanged' | 'SignerRemoved';
  }

  /** @name RewardEvent (191) */
  interface RewardEvent extends Enum {
    readonly isDepositStake: boolean;
    readonly asDepositStake: {
      readonly poolId: u128;
      readonly stakeId: AccountId32;
      readonly amount: i128;
    } & Struct;
    readonly isDistributeReward: boolean;
    readonly asDistributeReward: {
      readonly currencyId: u128;
      readonly amount: i128;
    } & Struct;
    readonly isWithdrawStake: boolean;
    readonly asWithdrawStake: {
      readonly poolId: u128;
      readonly stakeId: AccountId32;
      readonly amount: i128;
    } & Struct;
    readonly isWithdrawReward: boolean;
    readonly asWithdrawReward: {
      readonly poolId: u128;
      readonly stakeId: AccountId32;
      readonly currencyId: u128;
      readonly amount: i128;
    } & Struct;
    readonly type: 'DepositStake' | 'DistributeReward' | 'WithdrawStake' | 'WithdrawReward';
  }

  /** @name FarmingEvent (194) */
  interface FarmingEvent extends Enum {
    readonly isRewardScheduleUpdated: boolean;
    readonly asRewardScheduleUpdated: {
      readonly poolCurrencyId: u128;
      readonly rewardCurrencyId: u128;
      readonly periodCount: u32;
      readonly perPeriod: u128;
    } & Struct;
    readonly isRewardDistributed: boolean;
    readonly asRewardDistributed: {
      readonly poolCurrencyId: u128;
      readonly rewardCurrencyId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isRewardClaimed: boolean;
    readonly asRewardClaimed: {
      readonly accountId: AccountId32;
      readonly poolCurrencyId: u128;
      readonly rewardCurrencyId: u128;
      readonly amount: u128;
    } & Struct;
    readonly type: 'RewardScheduleUpdated' | 'RewardDistributed' | 'RewardClaimed';
  }

  /** @name PalletReferendaEvent (195) */
  interface PalletReferendaEvent extends Enum {
    readonly isSubmitted: boolean;
    readonly asSubmitted: {
      readonly index: u32;
      readonly track: u16;
      readonly proposal: FrameSupportPreimagesBounded;
    } & Struct;
    readonly isDecisionDepositPlaced: boolean;
    readonly asDecisionDepositPlaced: {
      readonly index: u32;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isDecisionDepositRefunded: boolean;
    readonly asDecisionDepositRefunded: {
      readonly index: u32;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isDepositSlashed: boolean;
    readonly asDepositSlashed: {
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isDecisionStarted: boolean;
    readonly asDecisionStarted: {
      readonly index: u32;
      readonly track: u16;
      readonly proposal: FrameSupportPreimagesBounded;
      readonly tally: PalletConvictionVotingTally;
    } & Struct;
    readonly isConfirmStarted: boolean;
    readonly asConfirmStarted: {
      readonly index: u32;
    } & Struct;
    readonly isConfirmAborted: boolean;
    readonly asConfirmAborted: {
      readonly index: u32;
    } & Struct;
    readonly isConfirmed: boolean;
    readonly asConfirmed: {
      readonly index: u32;
      readonly tally: PalletConvictionVotingTally;
    } & Struct;
    readonly isApproved: boolean;
    readonly asApproved: {
      readonly index: u32;
    } & Struct;
    readonly isRejected: boolean;
    readonly asRejected: {
      readonly index: u32;
      readonly tally: PalletConvictionVotingTally;
    } & Struct;
    readonly isTimedOut: boolean;
    readonly asTimedOut: {
      readonly index: u32;
      readonly tally: PalletConvictionVotingTally;
    } & Struct;
    readonly isCancelled: boolean;
    readonly asCancelled: {
      readonly index: u32;
      readonly tally: PalletConvictionVotingTally;
    } & Struct;
    readonly isKilled: boolean;
    readonly asKilled: {
      readonly index: u32;
      readonly tally: PalletConvictionVotingTally;
    } & Struct;
    readonly isSubmissionDepositRefunded: boolean;
    readonly asSubmissionDepositRefunded: {
      readonly index: u32;
      readonly who: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isMetadataSet: boolean;
    readonly asMetadataSet: {
      readonly index: u32;
      readonly hash_: H256;
    } & Struct;
    readonly isMetadataCleared: boolean;
    readonly asMetadataCleared: {
      readonly index: u32;
      readonly hash_: H256;
    } & Struct;
    readonly type: 'Submitted' | 'DecisionDepositPlaced' | 'DecisionDepositRefunded' | 'DepositSlashed' | 'DecisionStarted' | 'ConfirmStarted' | 'ConfirmAborted' | 'Confirmed' | 'Approved' | 'Rejected' | 'TimedOut' | 'Cancelled' | 'Killed' | 'SubmissionDepositRefunded' | 'MetadataSet' | 'MetadataCleared';
  }

  /** @name FrameSupportPreimagesBounded (196) */
  interface FrameSupportPreimagesBounded extends Enum {
    readonly isLegacy: boolean;
    readonly asLegacy: {
      readonly hash_: H256;
    } & Struct;
    readonly isInline: boolean;
    readonly asInline: Bytes;
    readonly isLookup: boolean;
    readonly asLookup: {
      readonly hash_: H256;
      readonly len: u32;
    } & Struct;
    readonly type: 'Legacy' | 'Inline' | 'Lookup';
  }

  /** @name FrameSystemCall (198) */
  interface FrameSystemCall extends Enum {
    readonly isRemark: boolean;
    readonly asRemark: {
      readonly remark: Bytes;
    } & Struct;
    readonly isSetHeapPages: boolean;
    readonly asSetHeapPages: {
      readonly pages: u64;
    } & Struct;
    readonly isSetCode: boolean;
    readonly asSetCode: {
      readonly code: Bytes;
    } & Struct;
    readonly isSetCodeWithoutChecks: boolean;
    readonly asSetCodeWithoutChecks: {
      readonly code: Bytes;
    } & Struct;
    readonly isSetStorage: boolean;
    readonly asSetStorage: {
      readonly items: Vec<ITuple<[Bytes, Bytes]>>;
    } & Struct;
    readonly isKillStorage: boolean;
    readonly asKillStorage: {
      readonly keys_: Vec<Bytes>;
    } & Struct;
    readonly isKillPrefix: boolean;
    readonly asKillPrefix: {
      readonly prefix: Bytes;
      readonly subkeys: u32;
    } & Struct;
    readonly isRemarkWithEvent: boolean;
    readonly asRemarkWithEvent: {
      readonly remark: Bytes;
    } & Struct;
    readonly type: 'Remark' | 'SetHeapPages' | 'SetCode' | 'SetCodeWithoutChecks' | 'SetStorage' | 'KillStorage' | 'KillPrefix' | 'RemarkWithEvent';
  }

  /** @name PalletTimestampCall (202) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name PalletSudoCall (203) */
  interface PalletSudoCall extends Enum {
    readonly isSudo: boolean;
    readonly asSudo: {
      readonly call: Call;
    } & Struct;
    readonly isSudoUncheckedWeight: boolean;
    readonly asSudoUncheckedWeight: {
      readonly call: Call;
      readonly weight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isSetKey: boolean;
    readonly asSetKey: {
      readonly new_: MultiAddress;
    } & Struct;
    readonly isSudoAs: boolean;
    readonly asSudoAs: {
      readonly who: MultiAddress;
      readonly call: Call;
    } & Struct;
    readonly type: 'Sudo' | 'SudoUncheckedWeight' | 'SetKey' | 'SudoAs';
  }

  /** @name PalletAssetTxPaymentCall (205) */
  interface PalletAssetTxPaymentCall extends Enum {
    readonly isSetPaymentAsset: boolean;
    readonly asSetPaymentAsset: {
      readonly payer: AccountId32;
      readonly assetId: Option<u128>;
    } & Struct;
    readonly type: 'SetPaymentAsset';
  }

  /** @name PalletIndicesCall (207) */
  interface PalletIndicesCall extends Enum {
    readonly isClaim: boolean;
    readonly asClaim: {
      readonly index: u32;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly new_: MultiAddress;
      readonly index: u32;
    } & Struct;
    readonly isFree: boolean;
    readonly asFree: {
      readonly index: u32;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly new_: MultiAddress;
      readonly index: u32;
      readonly freeze: bool;
    } & Struct;
    readonly isFreeze: boolean;
    readonly asFreeze: {
      readonly index: u32;
    } & Struct;
    readonly type: 'Claim' | 'Transfer' | 'Free' | 'ForceTransfer' | 'Freeze';
  }

  /** @name PalletBalancesCall (208) */
  interface PalletBalancesCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isSetBalance: boolean;
    readonly asSetBalance: {
      readonly who: MultiAddress;
      readonly newFree: Compact<u128>;
      readonly newReserved: Compact<u128>;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isTransferKeepAlive: boolean;
    readonly asTransferKeepAlive: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isTransferAll: boolean;
    readonly asTransferAll: {
      readonly dest: MultiAddress;
      readonly keepAlive: bool;
    } & Struct;
    readonly isForceUnreserve: boolean;
    readonly asForceUnreserve: {
      readonly who: MultiAddress;
      readonly amount: u128;
    } & Struct;
    readonly type: 'Transfer' | 'SetBalance' | 'ForceTransfer' | 'TransferKeepAlive' | 'TransferAll' | 'ForceUnreserve';
  }

  /** @name PalletIdentityCall (209) */
  interface PalletIdentityCall extends Enum {
    readonly isAddRegistrar: boolean;
    readonly asAddRegistrar: {
      readonly account: MultiAddress;
    } & Struct;
    readonly isSetIdentity: boolean;
    readonly asSetIdentity: {
      readonly info: PalletIdentityIdentityInfo;
    } & Struct;
    readonly isSetSubs: boolean;
    readonly asSetSubs: {
      readonly subs: Vec<ITuple<[AccountId32, Data]>>;
    } & Struct;
    readonly isClearIdentity: boolean;
    readonly isRequestJudgement: boolean;
    readonly asRequestJudgement: {
      readonly regIndex: Compact<u32>;
      readonly maxFee: Compact<u128>;
    } & Struct;
    readonly isCancelRequest: boolean;
    readonly asCancelRequest: {
      readonly regIndex: u32;
    } & Struct;
    readonly isSetFee: boolean;
    readonly asSetFee: {
      readonly index: Compact<u32>;
      readonly fee: Compact<u128>;
    } & Struct;
    readonly isSetAccountId: boolean;
    readonly asSetAccountId: {
      readonly index: Compact<u32>;
      readonly new_: MultiAddress;
    } & Struct;
    readonly isSetFields: boolean;
    readonly asSetFields: {
      readonly index: Compact<u32>;
      readonly fields: PalletIdentityBitFlags;
    } & Struct;
    readonly isProvideJudgement: boolean;
    readonly asProvideJudgement: {
      readonly regIndex: Compact<u32>;
      readonly target: MultiAddress;
      readonly judgement: PalletIdentityJudgement;
      readonly identity: H256;
    } & Struct;
    readonly isKillIdentity: boolean;
    readonly asKillIdentity: {
      readonly target: MultiAddress;
    } & Struct;
    readonly isAddSub: boolean;
    readonly asAddSub: {
      readonly sub: MultiAddress;
      readonly data: Data;
    } & Struct;
    readonly isRenameSub: boolean;
    readonly asRenameSub: {
      readonly sub: MultiAddress;
      readonly data: Data;
    } & Struct;
    readonly isRemoveSub: boolean;
    readonly asRemoveSub: {
      readonly sub: MultiAddress;
    } & Struct;
    readonly isQuitSub: boolean;
    readonly type: 'AddRegistrar' | 'SetIdentity' | 'SetSubs' | 'ClearIdentity' | 'RequestJudgement' | 'CancelRequest' | 'SetFee' | 'SetAccountId' | 'SetFields' | 'ProvideJudgement' | 'KillIdentity' | 'AddSub' | 'RenameSub' | 'RemoveSub' | 'QuitSub';
  }

  /** @name PalletIdentityIdentityInfo (210) */
  interface PalletIdentityIdentityInfo extends Struct {
    readonly additional: Vec<ITuple<[Data, Data]>>;
    readonly display: Data;
    readonly legal: Data;
    readonly web: Data;
    readonly riot: Data;
    readonly email: Data;
    readonly pgpFingerprint: Option<U8aFixed>;
    readonly image: Data;
    readonly twitter: Data;
  }

  /** @name PalletIdentityBitFlags (246) */
  interface PalletIdentityBitFlags extends Set {
    readonly isDisplay: boolean;
    readonly isLegal: boolean;
    readonly isWeb: boolean;
    readonly isRiot: boolean;
    readonly isEmail: boolean;
    readonly isPgpFingerprint: boolean;
    readonly isImage: boolean;
    readonly isTwitter: boolean;
  }

  /** @name PalletIdentityIdentityField (247) */
  interface PalletIdentityIdentityField extends Enum {
    readonly isDisplay: boolean;
    readonly isLegal: boolean;
    readonly isWeb: boolean;
    readonly isRiot: boolean;
    readonly isEmail: boolean;
    readonly isPgpFingerprint: boolean;
    readonly isImage: boolean;
    readonly isTwitter: boolean;
    readonly type: 'Display' | 'Legal' | 'Web' | 'Riot' | 'Email' | 'PgpFingerprint' | 'Image' | 'Twitter';
  }

  /** @name PalletIdentityJudgement (248) */
  interface PalletIdentityJudgement extends Enum {
    readonly isUnknown: boolean;
    readonly isFeePaid: boolean;
    readonly asFeePaid: u128;
    readonly isReasonable: boolean;
    readonly isKnownGood: boolean;
    readonly isOutOfDate: boolean;
    readonly isLowQuality: boolean;
    readonly isErroneous: boolean;
    readonly type: 'Unknown' | 'FeePaid' | 'Reasonable' | 'KnownGood' | 'OutOfDate' | 'LowQuality' | 'Erroneous';
  }

  /** @name PalletMultisigCall (249) */
  interface PalletMultisigCall extends Enum {
    readonly isAsMultiThreshold1: boolean;
    readonly asAsMultiThreshold1: {
      readonly otherSignatories: Vec<AccountId32>;
      readonly call: Call;
    } & Struct;
    readonly isAsMulti: boolean;
    readonly asAsMulti: {
      readonly threshold: u16;
      readonly otherSignatories: Vec<AccountId32>;
      readonly maybeTimepoint: Option<PalletMultisigTimepoint>;
      readonly call: Call;
      readonly maxWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isApproveAsMulti: boolean;
    readonly asApproveAsMulti: {
      readonly threshold: u16;
      readonly otherSignatories: Vec<AccountId32>;
      readonly maybeTimepoint: Option<PalletMultisigTimepoint>;
      readonly callHash: U8aFixed;
      readonly maxWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isCancelAsMulti: boolean;
    readonly asCancelAsMulti: {
      readonly threshold: u16;
      readonly otherSignatories: Vec<AccountId32>;
      readonly timepoint: PalletMultisigTimepoint;
      readonly callHash: U8aFixed;
    } & Struct;
    readonly type: 'AsMultiThreshold1' | 'AsMulti' | 'ApproveAsMulti' | 'CancelAsMulti';
  }

  /** @name CumulusPalletParachainSystemCall (251) */
  interface CumulusPalletParachainSystemCall extends Enum {
    readonly isSetValidationData: boolean;
    readonly asSetValidationData: {
      readonly data: CumulusPrimitivesParachainInherentParachainInherentData;
    } & Struct;
    readonly isSudoSendUpwardMessage: boolean;
    readonly asSudoSendUpwardMessage: {
      readonly message: Bytes;
    } & Struct;
    readonly isAuthorizeUpgrade: boolean;
    readonly asAuthorizeUpgrade: {
      readonly codeHash: H256;
    } & Struct;
    readonly isEnactAuthorizedUpgrade: boolean;
    readonly asEnactAuthorizedUpgrade: {
      readonly code: Bytes;
    } & Struct;
    readonly type: 'SetValidationData' | 'SudoSendUpwardMessage' | 'AuthorizeUpgrade' | 'EnactAuthorizedUpgrade';
  }

  /** @name CumulusPrimitivesParachainInherentParachainInherentData (252) */
  interface CumulusPrimitivesParachainInherentParachainInherentData extends Struct {
    readonly validationData: PolkadotPrimitivesV2PersistedValidationData;
    readonly relayChainState: SpTrieStorageProof;
    readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
    readonly horizontalMessages: BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>;
  }

  /** @name PolkadotPrimitivesV2PersistedValidationData (253) */
  interface PolkadotPrimitivesV2PersistedValidationData extends Struct {
    readonly parentHead: Bytes;
    readonly relayParentNumber: u32;
    readonly relayParentStorageRoot: H256;
    readonly maxPovSize: u32;
  }

  /** @name SpTrieStorageProof (255) */
  interface SpTrieStorageProof extends Struct {
    readonly trieNodes: BTreeSet<Bytes>;
  }

  /** @name PolkadotCorePrimitivesInboundDownwardMessage (258) */
  interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
    readonly sentAt: u32;
    readonly msg: Bytes;
  }

  /** @name PolkadotCorePrimitivesInboundHrmpMessage (261) */
  interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
    readonly sentAt: u32;
    readonly data: Bytes;
  }

  /** @name ParachainInfoCall (264) */
  type ParachainInfoCall = Null;

  /** @name PalletCollatorSelectionCall (265) */
  interface PalletCollatorSelectionCall extends Enum {
    readonly isSetInvulnerables: boolean;
    readonly asSetInvulnerables: {
      readonly new_: Vec<AccountId32>;
    } & Struct;
    readonly isSetDesiredCandidates: boolean;
    readonly asSetDesiredCandidates: {
      readonly max: u32;
    } & Struct;
    readonly isSetCandidacyBond: boolean;
    readonly asSetCandidacyBond: {
      readonly bond: u128;
    } & Struct;
    readonly isRegisterAsCandidate: boolean;
    readonly isLeaveIntent: boolean;
    readonly type: 'SetInvulnerables' | 'SetDesiredCandidates' | 'SetCandidacyBond' | 'RegisterAsCandidate' | 'LeaveIntent';
  }

  /** @name PalletSessionCall (266) */
  interface PalletSessionCall extends Enum {
    readonly isSetKeys: boolean;
    readonly asSetKeys: {
      readonly keys_: PicassoRuntimeOpaqueSessionKeys;
      readonly proof: Bytes;
    } & Struct;
    readonly isPurgeKeys: boolean;
    readonly type: 'SetKeys' | 'PurgeKeys';
  }

  /** @name PicassoRuntimeOpaqueSessionKeys (267) */
  interface PicassoRuntimeOpaqueSessionKeys extends Struct {
    readonly aura: SpConsensusAuraSr25519AppSr25519Public;
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (268) */
  interface SpConsensusAuraSr25519AppSr25519Public extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (269) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name PalletCollectiveCall (270) */
  interface PalletCollectiveCall extends Enum {
    readonly isSetMembers: boolean;
    readonly asSetMembers: {
      readonly newMembers: Vec<AccountId32>;
      readonly prime: Option<AccountId32>;
      readonly oldCount: u32;
    } & Struct;
    readonly isExecute: boolean;
    readonly asExecute: {
      readonly proposal: Call;
      readonly lengthBound: Compact<u32>;
    } & Struct;
    readonly isPropose: boolean;
    readonly asPropose: {
      readonly threshold: Compact<u32>;
      readonly proposal: Call;
      readonly lengthBound: Compact<u32>;
    } & Struct;
    readonly isVote: boolean;
    readonly asVote: {
      readonly proposal: H256;
      readonly index: Compact<u32>;
      readonly approve: bool;
    } & Struct;
    readonly isCloseOldWeight: boolean;
    readonly asCloseOldWeight: {
      readonly proposalHash: H256;
      readonly index: Compact<u32>;
      readonly proposalWeightBound: Compact<u64>;
      readonly lengthBound: Compact<u32>;
    } & Struct;
    readonly isDisapproveProposal: boolean;
    readonly asDisapproveProposal: {
      readonly proposalHash: H256;
    } & Struct;
    readonly isClose: boolean;
    readonly asClose: {
      readonly proposalHash: H256;
      readonly index: Compact<u32>;
      readonly proposalWeightBound: SpWeightsWeightV2Weight;
      readonly lengthBound: Compact<u32>;
    } & Struct;
    readonly type: 'SetMembers' | 'Execute' | 'Propose' | 'Vote' | 'CloseOldWeight' | 'DisapproveProposal' | 'Close';
  }

  /** @name PalletMembershipCall (273) */
  interface PalletMembershipCall extends Enum {
    readonly isAddMember: boolean;
    readonly asAddMember: {
      readonly who: MultiAddress;
    } & Struct;
    readonly isRemoveMember: boolean;
    readonly asRemoveMember: {
      readonly who: MultiAddress;
    } & Struct;
    readonly isSwapMember: boolean;
    readonly asSwapMember: {
      readonly remove: MultiAddress;
      readonly add: MultiAddress;
    } & Struct;
    readonly isResetMembers: boolean;
    readonly asResetMembers: {
      readonly members: Vec<AccountId32>;
    } & Struct;
    readonly isChangeKey: boolean;
    readonly asChangeKey: {
      readonly new_: MultiAddress;
    } & Struct;
    readonly isSetPrime: boolean;
    readonly asSetPrime: {
      readonly who: MultiAddress;
    } & Struct;
    readonly isClearPrime: boolean;
    readonly type: 'AddMember' | 'RemoveMember' | 'SwapMember' | 'ResetMembers' | 'ChangeKey' | 'SetPrime' | 'ClearPrime';
  }

  /** @name PalletTreasuryCall (274) */
  interface PalletTreasuryCall extends Enum {
    readonly isProposeSpend: boolean;
    readonly asProposeSpend: {
      readonly value: Compact<u128>;
      readonly beneficiary: MultiAddress;
    } & Struct;
    readonly isRejectProposal: boolean;
    readonly asRejectProposal: {
      readonly proposalId: Compact<u32>;
    } & Struct;
    readonly isApproveProposal: boolean;
    readonly asApproveProposal: {
      readonly proposalId: Compact<u32>;
    } & Struct;
    readonly isSpend: boolean;
    readonly asSpend: {
      readonly amount: Compact<u128>;
      readonly beneficiary: MultiAddress;
    } & Struct;
    readonly isRemoveApproval: boolean;
    readonly asRemoveApproval: {
      readonly proposalId: Compact<u32>;
    } & Struct;
    readonly type: 'ProposeSpend' | 'RejectProposal' | 'ApproveProposal' | 'Spend' | 'RemoveApproval';
  }

  /** @name PalletDemocracyCall (275) */
  interface PalletDemocracyCall extends Enum {
    readonly isPropose: boolean;
    readonly asPropose: {
      readonly proposal: FrameSupportPreimagesBounded;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isSecond: boolean;
    readonly asSecond: {
      readonly proposal: Compact<u32>;
    } & Struct;
    readonly isVote: boolean;
    readonly asVote: {
      readonly refIndex: Compact<u32>;
      readonly vote: PalletDemocracyVoteAccountVote;
    } & Struct;
    readonly isEmergencyCancel: boolean;
    readonly asEmergencyCancel: {
      readonly refIndex: u32;
    } & Struct;
    readonly isExternalPropose: boolean;
    readonly asExternalPropose: {
      readonly proposal: FrameSupportPreimagesBounded;
    } & Struct;
    readonly isExternalProposeMajority: boolean;
    readonly asExternalProposeMajority: {
      readonly proposal: FrameSupportPreimagesBounded;
    } & Struct;
    readonly isExternalProposeDefault: boolean;
    readonly asExternalProposeDefault: {
      readonly proposal: FrameSupportPreimagesBounded;
    } & Struct;
    readonly isFastTrack: boolean;
    readonly asFastTrack: {
      readonly proposalHash: H256;
      readonly votingPeriod: u32;
      readonly delay: u32;
    } & Struct;
    readonly isVetoExternal: boolean;
    readonly asVetoExternal: {
      readonly proposalHash: H256;
    } & Struct;
    readonly isCancelReferendum: boolean;
    readonly asCancelReferendum: {
      readonly refIndex: Compact<u32>;
    } & Struct;
    readonly isDelegate: boolean;
    readonly asDelegate: {
      readonly to: MultiAddress;
      readonly conviction: PalletDemocracyConviction;
      readonly balance: u128;
    } & Struct;
    readonly isUndelegate: boolean;
    readonly isClearPublicProposals: boolean;
    readonly isUnlock: boolean;
    readonly asUnlock: {
      readonly target: MultiAddress;
    } & Struct;
    readonly isRemoveVote: boolean;
    readonly asRemoveVote: {
      readonly index: u32;
    } & Struct;
    readonly isRemoveOtherVote: boolean;
    readonly asRemoveOtherVote: {
      readonly target: MultiAddress;
      readonly index: u32;
    } & Struct;
    readonly isBlacklist: boolean;
    readonly asBlacklist: {
      readonly proposalHash: H256;
      readonly maybeRefIndex: Option<u32>;
    } & Struct;
    readonly isCancelProposal: boolean;
    readonly asCancelProposal: {
      readonly propIndex: Compact<u32>;
    } & Struct;
    readonly isSetMetadata: boolean;
    readonly asSetMetadata: {
      readonly owner: PalletDemocracyMetadataOwner;
      readonly maybeHash: Option<H256>;
    } & Struct;
    readonly type: 'Propose' | 'Second' | 'Vote' | 'EmergencyCancel' | 'ExternalPropose' | 'ExternalProposeMajority' | 'ExternalProposeDefault' | 'FastTrack' | 'VetoExternal' | 'CancelReferendum' | 'Delegate' | 'Undelegate' | 'ClearPublicProposals' | 'Unlock' | 'RemoveVote' | 'RemoveOtherVote' | 'Blacklist' | 'CancelProposal' | 'SetMetadata';
  }

  /** @name PalletDemocracyConviction (276) */
  interface PalletDemocracyConviction extends Enum {
    readonly isNone: boolean;
    readonly isLocked1x: boolean;
    readonly isLocked2x: boolean;
    readonly isLocked3x: boolean;
    readonly isLocked4x: boolean;
    readonly isLocked5x: boolean;
    readonly isLocked6x: boolean;
    readonly type: 'None' | 'Locked1x' | 'Locked2x' | 'Locked3x' | 'Locked4x' | 'Locked5x' | 'Locked6x';
  }

  /** @name PalletSchedulerCall (283) */
  interface PalletSchedulerCall extends Enum {
    readonly isSchedule: boolean;
    readonly asSchedule: {
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly isCancel: boolean;
    readonly asCancel: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isScheduleNamed: boolean;
    readonly asScheduleNamed: {
      readonly id: U8aFixed;
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly isCancelNamed: boolean;
    readonly asCancelNamed: {
      readonly id: U8aFixed;
    } & Struct;
    readonly isScheduleAfter: boolean;
    readonly asScheduleAfter: {
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly isScheduleNamedAfter: boolean;
    readonly asScheduleNamedAfter: {
      readonly id: U8aFixed;
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: Call;
    } & Struct;
    readonly type: 'Schedule' | 'Cancel' | 'ScheduleNamed' | 'CancelNamed' | 'ScheduleAfter' | 'ScheduleNamedAfter';
  }

  /** @name PalletUtilityCall (285) */
  interface PalletUtilityCall extends Enum {
    readonly isBatch: boolean;
    readonly asBatch: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly isAsDerivative: boolean;
    readonly asAsDerivative: {
      readonly index: u16;
      readonly call: Call;
    } & Struct;
    readonly isBatchAll: boolean;
    readonly asBatchAll: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly isDispatchAs: boolean;
    readonly asDispatchAs: {
      readonly asOrigin: PicassoRuntimeOriginCaller;
      readonly call: Call;
    } & Struct;
    readonly isForceBatch: boolean;
    readonly asForceBatch: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly isWithWeight: boolean;
    readonly asWithWeight: {
      readonly call: Call;
      readonly weight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch' | 'WithWeight';
  }

  /** @name PicassoRuntimeOriginCaller (287) */
  interface PicassoRuntimeOriginCaller extends Enum {
    readonly isSystem: boolean;
    readonly asSystem: FrameSupportDispatchRawOrigin;
    readonly isVoid: boolean;
    readonly isCouncil: boolean;
    readonly asCouncil: PalletCollectiveRawOrigin;
    readonly isPolkadotXcm: boolean;
    readonly asPolkadotXcm: PalletXcmOrigin;
    readonly isCumulusXcm: boolean;
    readonly asCumulusXcm: CumulusPalletXcmOrigin;
    readonly isTechnicalCommittee: boolean;
    readonly asTechnicalCommittee: PalletCollectiveRawOrigin;
    readonly isReleaseCommittee: boolean;
    readonly asReleaseCommittee: PalletCollectiveRawOrigin;
    readonly isOrigins: boolean;
    readonly asOrigins: PalletCustomOriginsOrigin;
    readonly type: 'System' | 'Void' | 'Council' | 'PolkadotXcm' | 'CumulusXcm' | 'TechnicalCommittee' | 'ReleaseCommittee' | 'Origins';
  }

  /** @name FrameSupportDispatchRawOrigin (288) */
  interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: 'Root' | 'Signed' | 'None';
  }

  /** @name PalletCollectiveRawOrigin (289) */
  interface PalletCollectiveRawOrigin extends Enum {
    readonly isMembers: boolean;
    readonly asMembers: ITuple<[u32, u32]>;
    readonly isMember: boolean;
    readonly asMember: AccountId32;
    readonly isPhantom: boolean;
    readonly type: 'Members' | 'Member' | 'Phantom';
  }

  /** @name PalletXcmOrigin (292) */
  interface PalletXcmOrigin extends Enum {
    readonly isXcm: boolean;
    readonly asXcm: XcmV3MultiLocation;
    readonly isResponse: boolean;
    readonly asResponse: XcmV3MultiLocation;
    readonly type: 'Xcm' | 'Response';
  }

  /** @name CumulusPalletXcmOrigin (293) */
  interface CumulusPalletXcmOrigin extends Enum {
    readonly isRelay: boolean;
    readonly isSiblingParachain: boolean;
    readonly asSiblingParachain: u32;
    readonly type: 'Relay' | 'SiblingParachain';
  }

  /** @name PalletCustomOriginsOrigin (294) */
  interface PalletCustomOriginsOrigin extends Enum {
    readonly isWhitelistedCaller: boolean;
    readonly type: 'WhitelistedCaller';
  }

  /** @name SpCoreVoid (295) */
  type SpCoreVoid = Null;

  /** @name PalletPreimageCall (296) */
  interface PalletPreimageCall extends Enum {
    readonly isNotePreimage: boolean;
    readonly asNotePreimage: {
      readonly bytes: Bytes;
    } & Struct;
    readonly isUnnotePreimage: boolean;
    readonly asUnnotePreimage: {
      readonly hash_: H256;
    } & Struct;
    readonly isRequestPreimage: boolean;
    readonly asRequestPreimage: {
      readonly hash_: H256;
    } & Struct;
    readonly isUnrequestPreimage: boolean;
    readonly asUnrequestPreimage: {
      readonly hash_: H256;
    } & Struct;
    readonly type: 'NotePreimage' | 'UnnotePreimage' | 'RequestPreimage' | 'UnrequestPreimage';
  }

  /** @name PalletProxyCall (297) */
  interface PalletProxyCall extends Enum {
    readonly isProxy: boolean;
    readonly asProxy: {
      readonly real: MultiAddress;
      readonly forceProxyType: Option<ComposableTraitsAccountProxyProxyType>;
      readonly call: Call;
    } & Struct;
    readonly isAddProxy: boolean;
    readonly asAddProxy: {
      readonly delegate: MultiAddress;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
    } & Struct;
    readonly isRemoveProxy: boolean;
    readonly asRemoveProxy: {
      readonly delegate: MultiAddress;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
    } & Struct;
    readonly isRemoveProxies: boolean;
    readonly isCreatePure: boolean;
    readonly asCreatePure: {
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
      readonly index: u16;
    } & Struct;
    readonly isKillPure: boolean;
    readonly asKillPure: {
      readonly spawner: MultiAddress;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly index: u16;
      readonly height: Compact<u32>;
      readonly extIndex: Compact<u32>;
    } & Struct;
    readonly isAnnounce: boolean;
    readonly asAnnounce: {
      readonly real: MultiAddress;
      readonly callHash: H256;
    } & Struct;
    readonly isRemoveAnnouncement: boolean;
    readonly asRemoveAnnouncement: {
      readonly real: MultiAddress;
      readonly callHash: H256;
    } & Struct;
    readonly isRejectAnnouncement: boolean;
    readonly asRejectAnnouncement: {
      readonly delegate: MultiAddress;
      readonly callHash: H256;
    } & Struct;
    readonly isProxyAnnounced: boolean;
    readonly asProxyAnnounced: {
      readonly delegate: MultiAddress;
      readonly real: MultiAddress;
      readonly forceProxyType: Option<ComposableTraitsAccountProxyProxyType>;
      readonly call: Call;
    } & Struct;
    readonly type: 'Proxy' | 'AddProxy' | 'RemoveProxy' | 'RemoveProxies' | 'CreatePure' | 'KillPure' | 'Announce' | 'RemoveAnnouncement' | 'RejectAnnouncement' | 'ProxyAnnounced';
  }

  /** @name CumulusPalletXcmpQueueCall (299) */
  interface CumulusPalletXcmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isSuspendXcmExecution: boolean;
    readonly isResumeXcmExecution: boolean;
    readonly isUpdateSuspendThreshold: boolean;
    readonly asUpdateSuspendThreshold: {
      readonly new_: u32;
    } & Struct;
    readonly isUpdateDropThreshold: boolean;
    readonly asUpdateDropThreshold: {
      readonly new_: u32;
    } & Struct;
    readonly isUpdateResumeThreshold: boolean;
    readonly asUpdateResumeThreshold: {
      readonly new_: u32;
    } & Struct;
    readonly isUpdateThresholdWeight: boolean;
    readonly asUpdateThresholdWeight: {
      readonly new_: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isUpdateWeightRestrictDecay: boolean;
    readonly asUpdateWeightRestrictDecay: {
      readonly new_: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isUpdateXcmpMaxIndividualWeight: boolean;
    readonly asUpdateXcmpMaxIndividualWeight: {
      readonly new_: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'ServiceOverweight' | 'SuspendXcmExecution' | 'ResumeXcmExecution' | 'UpdateSuspendThreshold' | 'UpdateDropThreshold' | 'UpdateResumeThreshold' | 'UpdateThresholdWeight' | 'UpdateWeightRestrictDecay' | 'UpdateXcmpMaxIndividualWeight';
  }

  /** @name PalletXcmCall (300) */
  interface PalletXcmCall extends Enum {
    readonly isSend: boolean;
    readonly asSend: {
      readonly dest: XcmVersionedMultiLocation;
      readonly message: XcmVersionedXcm;
    } & Struct;
    readonly isTeleportAssets: boolean;
    readonly asTeleportAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
    } & Struct;
    readonly isReserveTransferAssets: boolean;
    readonly asReserveTransferAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
    } & Struct;
    readonly isExecute: boolean;
    readonly asExecute: {
      readonly message: XcmVersionedXcm;
      readonly maxWeight: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isForceXcmVersion: boolean;
    readonly asForceXcmVersion: {
      readonly location: XcmV3MultiLocation;
      readonly xcmVersion: u32;
    } & Struct;
    readonly isForceDefaultXcmVersion: boolean;
    readonly asForceDefaultXcmVersion: {
      readonly maybeXcmVersion: Option<u32>;
    } & Struct;
    readonly isForceSubscribeVersionNotify: boolean;
    readonly asForceSubscribeVersionNotify: {
      readonly location: XcmVersionedMultiLocation;
    } & Struct;
    readonly isForceUnsubscribeVersionNotify: boolean;
    readonly asForceUnsubscribeVersionNotify: {
      readonly location: XcmVersionedMultiLocation;
    } & Struct;
    readonly isLimitedReserveTransferAssets: boolean;
    readonly asLimitedReserveTransferAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
      readonly weightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isLimitedTeleportAssets: boolean;
    readonly asLimitedTeleportAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
      readonly weightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly type: 'Send' | 'TeleportAssets' | 'ReserveTransferAssets' | 'Execute' | 'ForceXcmVersion' | 'ForceDefaultXcmVersion' | 'ForceSubscribeVersionNotify' | 'ForceUnsubscribeVersionNotify' | 'LimitedReserveTransferAssets' | 'LimitedTeleportAssets';
  }

  /** @name XcmVersionedXcm (301) */
  interface XcmVersionedXcm extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2Xcm;
    readonly isV3: boolean;
    readonly asV3: XcmV3Xcm;
    readonly type: 'V2' | 'V3';
  }

  /** @name XcmV2Xcm (302) */
  interface XcmV2Xcm extends Vec<XcmV2Instruction> {}

  /** @name XcmV2Instruction (304) */
  interface XcmV2Instruction extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: XcmV2MultiassetMultiAssets;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: XcmV2MultiassetMultiAssets;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: XcmV2MultiassetMultiAssets;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV2Response;
      readonly maxWeight: Compact<u64>;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: XcmV2MultiassetMultiAssets;
      readonly beneficiary: XcmV2MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: XcmV2MultiassetMultiAssets;
      readonly dest: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV2OriginKind;
      readonly requireWeightAtMost: Compact<u64>;
      readonly call: XcmDoubleEncoded;
    } & Struct;
    readonly isHrmpNewChannelOpenRequest: boolean;
    readonly asHrmpNewChannelOpenRequest: {
      readonly sender: Compact<u32>;
      readonly maxMessageSize: Compact<u32>;
      readonly maxCapacity: Compact<u32>;
    } & Struct;
    readonly isHrmpChannelAccepted: boolean;
    readonly asHrmpChannelAccepted: {
      readonly recipient: Compact<u32>;
    } & Struct;
    readonly isHrmpChannelClosing: boolean;
    readonly asHrmpChannelClosing: {
      readonly initiator: Compact<u32>;
      readonly sender: Compact<u32>;
      readonly recipient: Compact<u32>;
    } & Struct;
    readonly isClearOrigin: boolean;
    readonly isDescendOrigin: boolean;
    readonly asDescendOrigin: XcmV2MultilocationJunctions;
    readonly isReportError: boolean;
    readonly asReportError: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV2MultiLocation;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly beneficiary: XcmV2MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly dest: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: XcmV2MultiassetMultiAssetFilter;
      readonly receive: XcmV2MultiassetMultiAssets;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly reserve: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly dest: XcmV2MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV2MultiLocation;
      readonly assets: XcmV2MultiassetMultiAssetFilter;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV2MultiAsset;
      readonly weightLimit: XcmV2WeightLimit;
    } & Struct;
    readonly isRefundSurplus: boolean;
    readonly isSetErrorHandler: boolean;
    readonly asSetErrorHandler: XcmV2Xcm;
    readonly isSetAppendix: boolean;
    readonly asSetAppendix: XcmV2Xcm;
    readonly isClearError: boolean;
    readonly isClaimAsset: boolean;
    readonly asClaimAsset: {
      readonly assets: XcmV2MultiassetMultiAssets;
      readonly ticket: XcmV2MultiLocation;
    } & Struct;
    readonly isTrap: boolean;
    readonly asTrap: Compact<u64>;
    readonly isSubscribeVersion: boolean;
    readonly asSubscribeVersion: {
      readonly queryId: Compact<u64>;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isUnsubscribeVersion: boolean;
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposited' | 'ReceiveTeleportedAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'ClearOrigin' | 'DescendOrigin' | 'ReportError' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'QueryHolding' | 'BuyExecution' | 'RefundSurplus' | 'SetErrorHandler' | 'SetAppendix' | 'ClearError' | 'ClaimAsset' | 'Trap' | 'SubscribeVersion' | 'UnsubscribeVersion';
  }

  /** @name XcmV2Response (305) */
  interface XcmV2Response extends Enum {
    readonly isNull: boolean;
    readonly isAssets: boolean;
    readonly asAssets: XcmV2MultiassetMultiAssets;
    readonly isExecutionResult: boolean;
    readonly asExecutionResult: Option<ITuple<[u32, XcmV2TraitsError]>>;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly type: 'Null' | 'Assets' | 'ExecutionResult' | 'Version';
  }

  /** @name XcmV2TraitsError (308) */
  interface XcmV2TraitsError extends Enum {
    readonly isOverflow: boolean;
    readonly isUnimplemented: boolean;
    readonly isUntrustedReserveLocation: boolean;
    readonly isUntrustedTeleportLocation: boolean;
    readonly isMultiLocationFull: boolean;
    readonly isMultiLocationNotInvertible: boolean;
    readonly isBadOrigin: boolean;
    readonly isInvalidLocation: boolean;
    readonly isAssetNotFound: boolean;
    readonly isFailedToTransactAsset: boolean;
    readonly isNotWithdrawable: boolean;
    readonly isLocationCannotHold: boolean;
    readonly isExceedsMaxMessageSize: boolean;
    readonly isDestinationUnsupported: boolean;
    readonly isTransport: boolean;
    readonly isUnroutable: boolean;
    readonly isUnknownClaim: boolean;
    readonly isFailedToDecode: boolean;
    readonly isMaxWeightInvalid: boolean;
    readonly isNotHoldingFees: boolean;
    readonly isTooExpensive: boolean;
    readonly isTrap: boolean;
    readonly asTrap: u64;
    readonly isUnhandledXcmVersion: boolean;
    readonly isWeightLimitReached: boolean;
    readonly asWeightLimitReached: u64;
    readonly isBarrier: boolean;
    readonly isWeightNotComputable: boolean;
    readonly type: 'Overflow' | 'Unimplemented' | 'UntrustedReserveLocation' | 'UntrustedTeleportLocation' | 'MultiLocationFull' | 'MultiLocationNotInvertible' | 'BadOrigin' | 'InvalidLocation' | 'AssetNotFound' | 'FailedToTransactAsset' | 'NotWithdrawable' | 'LocationCannotHold' | 'ExceedsMaxMessageSize' | 'DestinationUnsupported' | 'Transport' | 'Unroutable' | 'UnknownClaim' | 'FailedToDecode' | 'MaxWeightInvalid' | 'NotHoldingFees' | 'TooExpensive' | 'Trap' | 'UnhandledXcmVersion' | 'WeightLimitReached' | 'Barrier' | 'WeightNotComputable';
  }

  /** @name XcmV2MultiassetMultiAssetFilter (309) */
  interface XcmV2MultiassetMultiAssetFilter extends Enum {
    readonly isDefinite: boolean;
    readonly asDefinite: XcmV2MultiassetMultiAssets;
    readonly isWild: boolean;
    readonly asWild: XcmV2MultiassetWildMultiAsset;
    readonly type: 'Definite' | 'Wild';
  }

  /** @name XcmV2MultiassetWildMultiAsset (310) */
  interface XcmV2MultiassetWildMultiAsset extends Enum {
    readonly isAll: boolean;
    readonly isAllOf: boolean;
    readonly asAllOf: {
      readonly id: XcmV2MultiassetAssetId;
      readonly fun: XcmV2MultiassetWildFungibility;
    } & Struct;
    readonly type: 'All' | 'AllOf';
  }

  /** @name XcmV2MultiassetWildFungibility (311) */
  interface XcmV2MultiassetWildFungibility extends Enum {
    readonly isFungible: boolean;
    readonly isNonFungible: boolean;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV2WeightLimit (312) */
  interface XcmV2WeightLimit extends Enum {
    readonly isUnlimited: boolean;
    readonly isLimited: boolean;
    readonly asLimited: Compact<u64>;
    readonly type: 'Unlimited' | 'Limited';
  }

  /** @name CumulusPalletXcmCall (321) */
  type CumulusPalletXcmCall = Null;

  /** @name CumulusPalletDmpQueueCall (322) */
  interface CumulusPalletDmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: SpWeightsWeightV2Weight;
    } & Struct;
    readonly type: 'ServiceOverweight';
  }

  /** @name OrmlXtokensModuleCall (323) */
  interface OrmlXtokensModuleCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly currencyId: u128;
      readonly amount: u128;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isTransferMultiasset: boolean;
    readonly asTransferMultiasset: {
      readonly asset: XcmVersionedMultiAsset;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isTransferWithFee: boolean;
    readonly asTransferWithFee: {
      readonly currencyId: u128;
      readonly amount: u128;
      readonly fee: u128;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isTransferMultiassetWithFee: boolean;
    readonly asTransferMultiassetWithFee: {
      readonly asset: XcmVersionedMultiAsset;
      readonly fee: XcmVersionedMultiAsset;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isTransferMulticurrencies: boolean;
    readonly asTransferMulticurrencies: {
      readonly currencies: Vec<ITuple<[u128, u128]>>;
      readonly feeItem: u32;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly isTransferMultiassets: boolean;
    readonly asTransferMultiassets: {
      readonly assets: XcmVersionedMultiAssets;
      readonly feeItem: u32;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeightLimit: XcmV3WeightLimit;
    } & Struct;
    readonly type: 'Transfer' | 'TransferMultiasset' | 'TransferWithFee' | 'TransferMultiassetWithFee' | 'TransferMulticurrencies' | 'TransferMultiassets';
  }

  /** @name XcmVersionedMultiAsset (324) */
  interface XcmVersionedMultiAsset extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2MultiAsset;
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiAsset;
    readonly type: 'V2' | 'V3';
  }

  /** @name OrmlUnknownTokensModuleCall (325) */
  type OrmlUnknownTokensModuleCall = Null;

  /** @name OrmlTokensModuleCall (326) */
  interface OrmlTokensModuleCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly dest: MultiAddress;
      readonly currencyId: u128;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isTransferAll: boolean;
    readonly asTransferAll: {
      readonly dest: MultiAddress;
      readonly currencyId: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isTransferKeepAlive: boolean;
    readonly asTransferKeepAlive: {
      readonly dest: MultiAddress;
      readonly currencyId: u128;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly currencyId: u128;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isSetBalance: boolean;
    readonly asSetBalance: {
      readonly who: MultiAddress;
      readonly currencyId: u128;
      readonly newFree: Compact<u128>;
      readonly newReserved: Compact<u128>;
    } & Struct;
    readonly type: 'Transfer' | 'TransferAll' | 'TransferKeepAlive' | 'ForceTransfer' | 'SetBalance';
  }

  /** @name PalletCurrencyFactoryCall (327) */
  interface PalletCurrencyFactoryCall extends Enum {
    readonly isAddRange: boolean;
    readonly asAddRange: {
      readonly length: u64;
    } & Struct;
    readonly isSetMetadata: boolean;
    readonly asSetMetadata: {
      readonly assetId: u128;
      readonly metadata: ComposableTraitsAssetsBasicAssetMetadata;
    } & Struct;
    readonly type: 'AddRange' | 'SetMetadata';
  }

  /** @name ComposableTraitsAssetsBasicAssetMetadata (328) */
  interface ComposableTraitsAssetsBasicAssetMetadata extends Struct {
    readonly symbol: ComposableSupportCollectionsVecBoundedBiBoundedVec;
    readonly name: ComposableSupportCollectionsVecBoundedBiBoundedVec;
  }

  /** @name PalletCrowdloanRewardsCall (331) */
  interface PalletCrowdloanRewardsCall extends Enum {
    readonly isInitialize: boolean;
    readonly isInitializeAt: boolean;
    readonly asInitializeAt: {
      readonly at: u64;
    } & Struct;
    readonly isPopulate: boolean;
    readonly asPopulate: {
      readonly rewards: Vec<ITuple<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>>;
    } & Struct;
    readonly isAssociate: boolean;
    readonly asAssociate: {
      readonly rewardAccount: AccountId32;
      readonly proof: PalletCrowdloanRewardsModelsProof;
    } & Struct;
    readonly isClaim: boolean;
    readonly isUnlockRewardsFor: boolean;
    readonly asUnlockRewardsFor: {
      readonly rewardAccounts: Vec<AccountId32>;
    } & Struct;
    readonly isAdd: boolean;
    readonly asAdd: {
      readonly additions: Vec<ITuple<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>>;
    } & Struct;
    readonly type: 'Initialize' | 'InitializeAt' | 'Populate' | 'Associate' | 'Claim' | 'UnlockRewardsFor' | 'Add';
  }

  /** @name PalletCrowdloanRewardsModelsProof (332) */
  interface PalletCrowdloanRewardsModelsProof extends Enum {
    readonly isRelayChain: boolean;
    readonly asRelayChain: ITuple<[AccountId32, SpRuntimeMultiSignature]>;
    readonly isEthereum: boolean;
    readonly asEthereum: ComposableSupportEcdsaSignature;
    readonly type: 'RelayChain' | 'Ethereum';
  }

  /** @name SpRuntimeMultiSignature (333) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreEd25519Signature (334) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name SpCoreSr25519Signature (336) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (337) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name ComposableSupportEcdsaSignature (339) */
  interface ComposableSupportEcdsaSignature extends U8aFixed {}

  /** @name PalletVestingModuleCall (340) */
  interface PalletVestingModuleCall extends Enum {
    readonly isClaim: boolean;
    readonly asClaim: {
      readonly asset: u128;
      readonly vestingScheduleIds: PalletVestingVestingScheduleIdSet;
    } & Struct;
    readonly isVestedTransfer: boolean;
    readonly asVestedTransfer: {
      readonly from: MultiAddress;
      readonly beneficiary: MultiAddress;
      readonly asset: u128;
      readonly scheduleInfo: PalletVestingVestingScheduleInfo;
    } & Struct;
    readonly isUpdateVestingSchedules: boolean;
    readonly asUpdateVestingSchedules: {
      readonly who: MultiAddress;
      readonly asset: u128;
      readonly vestingSchedules: Vec<PalletVestingVestingScheduleInfo>;
    } & Struct;
    readonly isClaimFor: boolean;
    readonly asClaimFor: {
      readonly dest: MultiAddress;
      readonly asset: u128;
      readonly vestingScheduleIds: PalletVestingVestingScheduleIdSet;
    } & Struct;
    readonly type: 'Claim' | 'VestedTransfer' | 'UpdateVestingSchedules' | 'ClaimFor';
  }

  /** @name PalletVestingVestingScheduleInfo (341) */
  interface PalletVestingVestingScheduleInfo extends Struct {
    readonly window: PalletVestingVestingWindow;
    readonly periodCount: u32;
    readonly perPeriod: Compact<u128>;
  }

  /** @name PalletBondedFinanceCall (343) */
  interface PalletBondedFinanceCall extends Enum {
    readonly isOffer: boolean;
    readonly asOffer: {
      readonly offer: ComposableTraitsBondedFinanceBondOffer;
      readonly keepAlive: bool;
    } & Struct;
    readonly isBond: boolean;
    readonly asBond: {
      readonly offerId: u128;
      readonly nbOfBonds: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isCancel: boolean;
    readonly asCancel: {
      readonly offerId: u128;
    } & Struct;
    readonly type: 'Offer' | 'Bond' | 'Cancel';
  }

  /** @name ComposableTraitsBondedFinanceBondOffer (344) */
  interface ComposableTraitsBondedFinanceBondOffer extends Struct {
    readonly beneficiary: AccountId32;
    readonly asset: u128;
    readonly bondPrice: u128;
    readonly nbOfBonds: u128;
    readonly maturity: ComposableTraitsBondedFinanceBondDuration;
    readonly reward: ComposableTraitsBondedFinanceBondOfferReward;
  }

  /** @name ComposableTraitsBondedFinanceBondDuration (345) */
  interface ComposableTraitsBondedFinanceBondDuration extends Enum {
    readonly isFinite: boolean;
    readonly asFinite: {
      readonly returnIn: u32;
    } & Struct;
    readonly isInfinite: boolean;
    readonly type: 'Finite' | 'Infinite';
  }

  /** @name ComposableTraitsBondedFinanceBondOfferReward (346) */
  interface ComposableTraitsBondedFinanceBondOfferReward extends Struct {
    readonly asset: u128;
    readonly amount: u128;
    readonly maturity: u32;
  }

  /** @name PalletAssetsRegistryCall (347) */
  interface PalletAssetsRegistryCall extends Enum {
    readonly isRegisterAsset: boolean;
    readonly asRegisterAsset: {
      readonly protocolId: U8aFixed;
      readonly nonce: u64;
      readonly location: Option<PrimitivesCurrencyForeignAssetId>;
      readonly assetInfo: ComposableTraitsAssetsAssetInfo;
    } & Struct;
    readonly isUpdateAsset: boolean;
    readonly asUpdateAsset: {
      readonly assetId: u128;
      readonly assetInfo: ComposableTraitsAssetsAssetInfoUpdate;
    } & Struct;
    readonly isSetMinFee: boolean;
    readonly asSetMinFee: {
      readonly targetParachainId: u32;
      readonly foreignAssetId: PrimitivesCurrencyForeignAssetId;
      readonly amount: Option<u128>;
    } & Struct;
    readonly isUpdateAssetLocation: boolean;
    readonly asUpdateAssetLocation: {
      readonly assetId: u128;
      readonly location: Option<PrimitivesCurrencyForeignAssetId>;
    } & Struct;
    readonly type: 'RegisterAsset' | 'UpdateAsset' | 'SetMinFee' | 'UpdateAssetLocation';
  }

  /** @name PalletPabloCall (348) */
  interface PalletPabloCall extends Enum {
    readonly isCreate: boolean;
    readonly asCreate: {
      readonly pool: PalletPabloPoolInitConfiguration;
    } & Struct;
    readonly isBuy: boolean;
    readonly asBuy: {
      readonly poolId: u128;
      readonly inAssetId: u128;
      readonly outAsset: ComposableTraitsDexAssetAmount;
      readonly keepAlive: bool;
    } & Struct;
    readonly isSwap: boolean;
    readonly asSwap: {
      readonly poolId: u128;
      readonly inAsset: ComposableTraitsDexAssetAmount;
      readonly minReceive: ComposableTraitsDexAssetAmount;
      readonly keepAlive: bool;
    } & Struct;
    readonly isAddLiquidity: boolean;
    readonly asAddLiquidity: {
      readonly poolId: u128;
      readonly assets: BTreeMap<u128, u128>;
      readonly minMintAmount: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isRemoveLiquidity: boolean;
    readonly asRemoveLiquidity: {
      readonly poolId: u128;
      readonly lpAmount: u128;
      readonly minReceive: BTreeMap<u128, u128>;
    } & Struct;
    readonly isEnableTwap: boolean;
    readonly asEnableTwap: {
      readonly poolId: u128;
    } & Struct;
    readonly type: 'Create' | 'Buy' | 'Swap' | 'AddLiquidity' | 'RemoveLiquidity' | 'EnableTwap';
  }

  /** @name PalletPabloPoolInitConfiguration (349) */
  interface PalletPabloPoolInitConfiguration extends Enum {
    readonly isDualAssetConstantProduct: boolean;
    readonly asDualAssetConstantProduct: {
      readonly owner: AccountId32;
      readonly assetsWeights: Vec<ITuple<[u128, Permill]>>;
      readonly fee: Permill;
    } & Struct;
    readonly type: 'DualAssetConstantProduct';
  }

  /** @name ComposableTraitsDexAssetAmount (350) */
  interface ComposableTraitsDexAssetAmount extends Struct {
    readonly assetId: u128;
    readonly amount: u128;
  }

  /** @name PalletOracleCall (351) */
  interface PalletOracleCall extends Enum {
    readonly isAddAssetAndInfo: boolean;
    readonly asAddAssetAndInfo: {
      readonly assetId: u128;
      readonly threshold: Percent;
      readonly minAnswers: u32;
      readonly maxAnswers: u32;
      readonly blockInterval: u32;
      readonly rewardWeight: u128;
      readonly slash: u128;
      readonly emitPriceChanges: bool;
    } & Struct;
    readonly isSetSigner: boolean;
    readonly asSetSigner: {
      readonly who: AccountId32;
      readonly signer: AccountId32;
    } & Struct;
    readonly isAdjustRewards: boolean;
    readonly asAdjustRewards: {
      readonly annualCostPerOracle: u128;
      readonly numIdealOracles: u8;
    } & Struct;
    readonly isAddStake: boolean;
    readonly asAddStake: {
      readonly stake: u128;
    } & Struct;
    readonly isRemoveStake: boolean;
    readonly isReclaimStake: boolean;
    readonly isSubmitPrice: boolean;
    readonly asSubmitPrice: {
      readonly price: u128;
      readonly assetId: u128;
    } & Struct;
    readonly isRemoveSigner: boolean;
    readonly asRemoveSigner: {
      readonly who: AccountId32;
    } & Struct;
    readonly type: 'AddAssetAndInfo' | 'SetSigner' | 'AdjustRewards' | 'AddStake' | 'RemoveStake' | 'ReclaimStake' | 'SubmitPrice' | 'RemoveSigner';
  }

  /** @name PalletAssetsTransactorRouterCall (352) */
  interface PalletAssetsTransactorRouterCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly asset: u128;
      readonly dest: MultiAddress;
      readonly amount: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isTransferNative: boolean;
    readonly asTransferNative: {
      readonly dest: MultiAddress;
      readonly value: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly asset: u128;
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly value: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isForceTransferNative: boolean;
    readonly asForceTransferNative: {
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly value: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isTransferAll: boolean;
    readonly asTransferAll: {
      readonly asset: u128;
      readonly dest: MultiAddress;
      readonly keepAlive: bool;
    } & Struct;
    readonly isTransferAllNative: boolean;
    readonly asTransferAllNative: {
      readonly dest: MultiAddress;
      readonly keepAlive: bool;
    } & Struct;
    readonly isMintInto: boolean;
    readonly asMintInto: {
      readonly assetId: u128;
      readonly dest: MultiAddress;
      readonly amount: u128;
    } & Struct;
    readonly isBurnFrom: boolean;
    readonly asBurnFrom: {
      readonly assetId: u128;
      readonly dest: MultiAddress;
      readonly amount: u128;
    } & Struct;
    readonly type: 'Transfer' | 'TransferNative' | 'ForceTransfer' | 'ForceTransferNative' | 'TransferAll' | 'TransferAllNative' | 'MintInto' | 'BurnFrom';
  }

  /** @name RewardCall (353) */
  type RewardCall = Null;

  /** @name FarmingCall (354) */
  interface FarmingCall extends Enum {
    readonly isUpdateRewardSchedule: boolean;
    readonly asUpdateRewardSchedule: {
      readonly poolCurrencyId: u128;
      readonly rewardCurrencyId: u128;
      readonly periodCount: u32;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isRemoveRewardSchedule: boolean;
    readonly asRemoveRewardSchedule: {
      readonly poolCurrencyId: u128;
      readonly rewardCurrencyId: u128;
    } & Struct;
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly poolCurrencyId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isWithdraw: boolean;
    readonly asWithdraw: {
      readonly poolCurrencyId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isClaim: boolean;
    readonly asClaim: {
      readonly poolCurrencyId: u128;
      readonly rewardCurrencyId: u128;
    } & Struct;
    readonly type: 'UpdateRewardSchedule' | 'RemoveRewardSchedule' | 'Deposit' | 'Withdraw' | 'Claim';
  }

  /** @name PalletReferendaCall (355) */
  interface PalletReferendaCall extends Enum {
    readonly isSubmit: boolean;
    readonly asSubmit: {
      readonly proposalOrigin: PicassoRuntimeOriginCaller;
      readonly proposal: FrameSupportPreimagesBounded;
      readonly enactmentMoment: FrameSupportScheduleDispatchTime;
    } & Struct;
    readonly isPlaceDecisionDeposit: boolean;
    readonly asPlaceDecisionDeposit: {
      readonly index: u32;
    } & Struct;
    readonly isRefundDecisionDeposit: boolean;
    readonly asRefundDecisionDeposit: {
      readonly index: u32;
    } & Struct;
    readonly isCancel: boolean;
    readonly asCancel: {
      readonly index: u32;
    } & Struct;
    readonly isKill: boolean;
    readonly asKill: {
      readonly index: u32;
    } & Struct;
    readonly isNudgeReferendum: boolean;
    readonly asNudgeReferendum: {
      readonly index: u32;
    } & Struct;
    readonly isOneFewerDeciding: boolean;
    readonly asOneFewerDeciding: {
      readonly track: u16;
    } & Struct;
    readonly isRefundSubmissionDeposit: boolean;
    readonly asRefundSubmissionDeposit: {
      readonly index: u32;
    } & Struct;
    readonly isSetMetadata: boolean;
    readonly asSetMetadata: {
      readonly index: u32;
      readonly maybeHash: Option<H256>;
    } & Struct;
    readonly type: 'Submit' | 'PlaceDecisionDeposit' | 'RefundDecisionDeposit' | 'Cancel' | 'Kill' | 'NudgeReferendum' | 'OneFewerDeciding' | 'RefundSubmissionDeposit' | 'SetMetadata';
  }

  /** @name FrameSupportScheduleDispatchTime (356) */
  interface FrameSupportScheduleDispatchTime extends Enum {
    readonly isAt: boolean;
    readonly asAt: u32;
    readonly isAfter: boolean;
    readonly asAfter: u32;
    readonly type: 'At' | 'After';
  }

  /** @name PalletConvictionVotingCall (357) */
  interface PalletConvictionVotingCall extends Enum {
    readonly isVote: boolean;
    readonly asVote: {
      readonly pollIndex: Compact<u32>;
      readonly vote: PalletConvictionVotingVoteAccountVote;
    } & Struct;
    readonly isDelegate: boolean;
    readonly asDelegate: {
      readonly class: u16;
      readonly to: MultiAddress;
      readonly conviction: PalletConvictionVotingConviction;
      readonly balance: u128;
    } & Struct;
    readonly isUndelegate: boolean;
    readonly asUndelegate: {
      readonly class: u16;
    } & Struct;
    readonly isUnlock: boolean;
    readonly asUnlock: {
      readonly class: u16;
      readonly target: MultiAddress;
    } & Struct;
    readonly isRemoveVote: boolean;
    readonly asRemoveVote: {
      readonly class: Option<u16>;
      readonly index: u32;
    } & Struct;
    readonly isRemoveOtherVote: boolean;
    readonly asRemoveOtherVote: {
      readonly target: MultiAddress;
      readonly class: u16;
      readonly index: u32;
    } & Struct;
    readonly type: 'Vote' | 'Delegate' | 'Undelegate' | 'Unlock' | 'RemoveVote' | 'RemoveOtherVote';
  }

  /** @name PalletConvictionVotingVoteAccountVote (358) */
  interface PalletConvictionVotingVoteAccountVote extends Enum {
    readonly isStandard: boolean;
    readonly asStandard: {
      readonly vote: Vote;
      readonly balance: u128;
    } & Struct;
    readonly isSplit: boolean;
    readonly asSplit: {
      readonly aye: u128;
      readonly nay: u128;
    } & Struct;
    readonly isSplitAbstain: boolean;
    readonly asSplitAbstain: {
      readonly aye: u128;
      readonly nay: u128;
      readonly abstain: u128;
    } & Struct;
    readonly type: 'Standard' | 'Split' | 'SplitAbstain';
  }

  /** @name PalletConvictionVotingConviction (360) */
  interface PalletConvictionVotingConviction extends Enum {
    readonly isNone: boolean;
    readonly isLocked1x: boolean;
    readonly isLocked2x: boolean;
    readonly isLocked3x: boolean;
    readonly isLocked4x: boolean;
    readonly isLocked5x: boolean;
    readonly isLocked6x: boolean;
    readonly type: 'None' | 'Locked1x' | 'Locked2x' | 'Locked3x' | 'Locked4x' | 'Locked5x' | 'Locked6x';
  }

  /** @name PalletWhitelistCall (363) */
  interface PalletWhitelistCall extends Enum {
    readonly isWhitelistCall: boolean;
    readonly asWhitelistCall: {
      readonly callHash: H256;
    } & Struct;
    readonly isRemoveWhitelistedCall: boolean;
    readonly asRemoveWhitelistedCall: {
      readonly callHash: H256;
    } & Struct;
    readonly isDispatchWhitelistedCall: boolean;
    readonly asDispatchWhitelistedCall: {
      readonly callHash: H256;
      readonly callEncodedLen: u32;
      readonly callWeightWitness: SpWeightsWeightV2Weight;
    } & Struct;
    readonly isDispatchWhitelistedCallWithPreimage: boolean;
    readonly asDispatchWhitelistedCallWithPreimage: {
      readonly call: Call;
    } & Struct;
    readonly type: 'WhitelistCall' | 'RemoveWhitelistedCall' | 'DispatchWhitelistedCall' | 'DispatchWhitelistedCallWithPreimage';
  }

  /** @name PalletCallFilterCall (364) */
  interface PalletCallFilterCall extends Enum {
    readonly isDisable: boolean;
    readonly asDisable: {
      readonly entry: PalletCallFilterCallFilterEntry;
    } & Struct;
    readonly isEnable: boolean;
    readonly asEnable: {
      readonly entry: PalletCallFilterCallFilterEntry;
    } & Struct;
    readonly type: 'Disable' | 'Enable';
  }

  /** @name PalletCallFilterCallFilterEntry (365) */
  interface PalletCallFilterCallFilterEntry extends Struct {
    readonly palletName: Bytes;
    readonly functionName: Bytes;
  }

  /** @name CommonMaxStringSize (366) */
  type CommonMaxStringSize = Null;

  /** @name PalletCosmwasmCall (368) */
  interface PalletCosmwasmCall extends Enum {
    readonly isUpload: boolean;
    readonly asUpload: {
      readonly code: Bytes;
    } & Struct;
    readonly isInstantiate: boolean;
    readonly asInstantiate: {
      readonly codeIdentifier: PalletCosmwasmCodeIdentifier;
      readonly salt: Bytes;
      readonly admin: Option<AccountId32>;
      readonly label: Bytes;
      readonly funds: BTreeMap<u128, ITuple<[u128, bool]>>;
      readonly gas: u64;
      readonly message: Bytes;
    } & Struct;
    readonly isExecute: boolean;
    readonly asExecute: {
      readonly contract: AccountId32;
      readonly funds: BTreeMap<u128, ITuple<[u128, bool]>>;
      readonly gas: u64;
      readonly message: Bytes;
    } & Struct;
    readonly isMigrate: boolean;
    readonly asMigrate: {
      readonly contract: AccountId32;
      readonly newCodeIdentifier: PalletCosmwasmCodeIdentifier;
      readonly gas: u64;
      readonly message: Bytes;
    } & Struct;
    readonly isUpdateAdmin: boolean;
    readonly asUpdateAdmin: {
      readonly contract: AccountId32;
      readonly newAdmin: Option<AccountId32>;
      readonly gas: u64;
    } & Struct;
    readonly type: 'Upload' | 'Instantiate' | 'Execute' | 'Migrate' | 'UpdateAdmin';
  }

  /** @name PalletCosmwasmCodeIdentifier (370) */
  interface PalletCosmwasmCodeIdentifier extends Enum {
    readonly isCodeId: boolean;
    readonly asCodeId: u64;
    readonly isCodeHash: boolean;
    readonly asCodeHash: U8aFixed;
    readonly type: 'CodeId' | 'CodeHash';
  }

  /** @name PalletIbcCall (379) */
  interface PalletIbcCall extends Enum {
    readonly isDeliver: boolean;
    readonly asDeliver: {
      readonly messages: Vec<PalletIbcAny>;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly params: PalletIbcTransferParams;
      readonly assetId: u128;
      readonly amount: u128;
      readonly memo: Option<Text>;
    } & Struct;
    readonly isUpgradeClient: boolean;
    readonly asUpgradeClient: {
      readonly params: PalletIbcUpgradeParams;
    } & Struct;
    readonly isFreezeClient: boolean;
    readonly asFreezeClient: {
      readonly clientId: Bytes;
      readonly height: u64;
    } & Struct;
    readonly isIncreaseCounters: boolean;
    readonly isAddChannelsToFeelessChannelList: boolean;
    readonly asAddChannelsToFeelessChannelList: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly isRemoveChannelsFromFeelessChannelList: boolean;
    readonly asRemoveChannelsFromFeelessChannelList: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly isSetChildStorage: boolean;
    readonly asSetChildStorage: {
      readonly key: Bytes;
      readonly value: Bytes;
    } & Struct;
    readonly isSubstituteClientState: boolean;
    readonly asSubstituteClientState: {
      readonly clientId: Text;
      readonly height: IbcCoreIcs02ClientHeight;
      readonly clientStateBytes: Bytes;
      readonly consensusStateBytes: Bytes;
    } & Struct;
    readonly type: 'Deliver' | 'Transfer' | 'UpgradeClient' | 'FreezeClient' | 'IncreaseCounters' | 'AddChannelsToFeelessChannelList' | 'RemoveChannelsFromFeelessChannelList' | 'SetChildStorage' | 'SubstituteClientState';
  }

  /** @name PalletIbcAny (381) */
  interface PalletIbcAny extends Struct {
    readonly typeUrl: Text;
    readonly value: Bytes;
  }

  /** @name PalletIbcTransferParams (382) */
  interface PalletIbcTransferParams extends Struct {
    readonly to: PalletIbcMultiAddress;
    readonly sourceChannel: u64;
    readonly timeout: IbcPrimitivesTimeout;
  }

  /** @name PalletIbcMultiAddress (383) */
  interface PalletIbcMultiAddress extends Enum {
    readonly isId: boolean;
    readonly asId: AccountId32;
    readonly isRaw: boolean;
    readonly asRaw: Bytes;
    readonly type: 'Id' | 'Raw';
  }

  /** @name IbcPrimitivesTimeout (384) */
  interface IbcPrimitivesTimeout extends Enum {
    readonly isOffset: boolean;
    readonly asOffset: {
      readonly timestamp: Option<u64>;
      readonly height: Option<u64>;
    } & Struct;
    readonly isAbsolute: boolean;
    readonly asAbsolute: {
      readonly timestamp: Option<u64>;
      readonly height: Option<u64>;
    } & Struct;
    readonly type: 'Offset' | 'Absolute';
  }

  /** @name PalletIbcUpgradeParams (387) */
  interface PalletIbcUpgradeParams extends Struct {
    readonly clientState: Bytes;
    readonly consensusState: Bytes;
  }

  /** @name IbcCoreIcs02ClientHeight (388) */
  interface IbcCoreIcs02ClientHeight extends Struct {
    readonly revisionNumber: u64;
    readonly revisionHeight: u64;
  }

  /** @name PalletIbcIcs20FeePalletCall (389) */
  interface PalletIbcIcs20FeePalletCall extends Enum {
    readonly isSetCharge: boolean;
    readonly asSetCharge: {
      readonly charge: Perbill;
    } & Struct;
    readonly isAddChannelsToFeelessChannelList: boolean;
    readonly asAddChannelsToFeelessChannelList: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly isRemoveChannelsFromFeelessChannelList: boolean;
    readonly asRemoveChannelsFromFeelessChannelList: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly type: 'SetCharge' | 'AddChannelsToFeelessChannelList' | 'RemoveChannelsFromFeelessChannelList';
  }

  /** @name PalletMultihopXcmIbcCall (391) */
  interface PalletMultihopXcmIbcCall extends Enum {
    readonly isAddRoute: boolean;
    readonly asAddRoute: {
      readonly routeId: u128;
      readonly route: Vec<ITuple<[ComposableTraitsXcmMemoChainInfo, Bytes]>>;
    } & Struct;
    readonly type: 'AddRoute';
  }

  /** @name ComposableTraitsXcmMemoChainInfo (394) */
  interface ComposableTraitsXcmMemoChainInfo extends Struct {
    readonly chainId: u32;
    readonly order: u8;
    readonly channelId: u64;
    readonly timestamp: Option<u64>;
    readonly height: Option<u64>;
    readonly retries: Option<u8>;
    readonly timeout: Option<u64>;
    readonly chainHop: ComposableTraitsXcmMemoChainHop;
    readonly paraId: Option<u32>;
  }

  /** @name ComposableTraitsXcmMemoChainHop (395) */
  interface ComposableTraitsXcmMemoChainHop extends Enum {
    readonly isSubstrateIbc: boolean;
    readonly isCosmosIbc: boolean;
    readonly isXcm: boolean;
    readonly type: 'SubstrateIbc' | 'CosmosIbc' | 'Xcm';
  }

  /** @name PalletConvictionVotingTally (399) */
  interface PalletConvictionVotingTally extends Struct {
    readonly ayes: u128;
    readonly nays: u128;
    readonly support: u128;
  }

  /** @name PalletConvictionVotingEvent (400) */
  interface PalletConvictionVotingEvent extends Enum {
    readonly isDelegated: boolean;
    readonly asDelegated: ITuple<[AccountId32, AccountId32]>;
    readonly isUndelegated: boolean;
    readonly asUndelegated: AccountId32;
    readonly type: 'Delegated' | 'Undelegated';
  }

  /** @name PalletWhitelistEvent (402) */
  interface PalletWhitelistEvent extends Enum {
    readonly isCallWhitelisted: boolean;
    readonly asCallWhitelisted: {
      readonly callHash: H256;
    } & Struct;
    readonly isWhitelistedCallRemoved: boolean;
    readonly asWhitelistedCallRemoved: {
      readonly callHash: H256;
    } & Struct;
    readonly isWhitelistedCallDispatched: boolean;
    readonly asWhitelistedCallDispatched: {
      readonly callHash: H256;
      readonly result: Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo>;
    } & Struct;
    readonly type: 'CallWhitelisted' | 'WhitelistedCallRemoved' | 'WhitelistedCallDispatched';
  }

  /** @name FrameSupportDispatchPostDispatchInfo (404) */
  interface FrameSupportDispatchPostDispatchInfo extends Struct {
    readonly actualWeight: Option<SpWeightsWeightV2Weight>;
    readonly paysFee: FrameSupportDispatchPays;
  }

  /** @name SpRuntimeDispatchErrorWithPostInfo (406) */
  interface SpRuntimeDispatchErrorWithPostInfo extends Struct {
    readonly postInfo: FrameSupportDispatchPostDispatchInfo;
    readonly error: SpRuntimeDispatchError;
  }

  /** @name PalletCallFilterEvent (407) */
  interface PalletCallFilterEvent extends Enum {
    readonly isDisabled: boolean;
    readonly asDisabled: {
      readonly entry: PalletCallFilterCallFilterEntry;
    } & Struct;
    readonly isEnabled: boolean;
    readonly asEnabled: {
      readonly entry: PalletCallFilterCallFilterEntry;
    } & Struct;
    readonly type: 'Disabled' | 'Enabled';
  }

  /** @name PalletCosmwasmEvent (408) */
  interface PalletCosmwasmEvent extends Enum {
    readonly isUploaded: boolean;
    readonly asUploaded: {
      readonly codeHash: U8aFixed;
      readonly codeId: u64;
    } & Struct;
    readonly isInstantiated: boolean;
    readonly asInstantiated: {
      readonly contract: AccountId32;
      readonly info: PalletCosmwasmContractInfo;
    } & Struct;
    readonly isExecuted: boolean;
    readonly asExecuted: {
      readonly contract: AccountId32;
      readonly entrypoint: PalletCosmwasmEntryPoint;
      readonly data: Option<Bytes>;
    } & Struct;
    readonly isExecutionFailed: boolean;
    readonly asExecutionFailed: {
      readonly contract: AccountId32;
      readonly entrypoint: PalletCosmwasmEntryPoint;
      readonly error: Bytes;
    } & Struct;
    readonly isEmitted: boolean;
    readonly asEmitted: {
      readonly contract: AccountId32;
      readonly ty: Bytes;
      readonly attributes: Vec<ITuple<[Bytes, Bytes]>>;
    } & Struct;
    readonly isMigrated: boolean;
    readonly asMigrated: {
      readonly contract: AccountId32;
      readonly to: u64;
    } & Struct;
    readonly isAdminUpdated: boolean;
    readonly asAdminUpdated: {
      readonly contract: AccountId32;
      readonly newAdmin: Option<AccountId32>;
    } & Struct;
    readonly type: 'Uploaded' | 'Instantiated' | 'Executed' | 'ExecutionFailed' | 'Emitted' | 'Migrated' | 'AdminUpdated';
  }

  /** @name PalletCosmwasmContractInfo (409) */
  interface PalletCosmwasmContractInfo extends Struct {
    readonly codeId: u64;
    readonly trieId: Bytes;
    readonly instantiator: AccountId32;
    readonly admin: Option<AccountId32>;
    readonly label: Bytes;
  }

  /** @name PalletCosmwasmEntryPoint (411) */
  interface PalletCosmwasmEntryPoint extends Enum {
    readonly isInstantiate: boolean;
    readonly isExecute: boolean;
    readonly isMigrate: boolean;
    readonly isReply: boolean;
    readonly isIbcChannelOpen: boolean;
    readonly isIbcChannelConnect: boolean;
    readonly isIbcChannelClose: boolean;
    readonly isIbcPacketTimeout: boolean;
    readonly isIbcPacketReceive: boolean;
    readonly isIbcPacketAck: boolean;
    readonly type: 'Instantiate' | 'Execute' | 'Migrate' | 'Reply' | 'IbcChannelOpen' | 'IbcChannelConnect' | 'IbcChannelClose' | 'IbcPacketTimeout' | 'IbcPacketReceive' | 'IbcPacketAck';
  }

  /** @name PalletIbcEvent (413) */
  interface PalletIbcEvent extends Enum {
    readonly isEvents: boolean;
    readonly asEvents: {
      readonly events: Vec<Result<PalletIbcEventsIbcEvent, PalletIbcErrorsIbcError>>;
    } & Struct;
    readonly isTokenTransferInitiated: boolean;
    readonly asTokenTransferInitiated: {
      readonly from: Bytes;
      readonly to: Bytes;
      readonly ibcDenom: Bytes;
      readonly localAssetId: Option<u128>;
      readonly amount: u128;
      readonly isSenderSource: bool;
      readonly sourceChannel: Bytes;
      readonly destinationChannel: Bytes;
    } & Struct;
    readonly isChannelOpened: boolean;
    readonly asChannelOpened: {
      readonly channelId: Bytes;
      readonly portId: Bytes;
    } & Struct;
    readonly isParamsUpdated: boolean;
    readonly asParamsUpdated: {
      readonly sendEnabled: bool;
      readonly receiveEnabled: bool;
    } & Struct;
    readonly isTokenTransferCompleted: boolean;
    readonly asTokenTransferCompleted: {
      readonly from: Text;
      readonly to: Text;
      readonly ibcDenom: Bytes;
      readonly localAssetId: Option<u128>;
      readonly amount: u128;
      readonly isSenderSource: bool;
      readonly sourceChannel: Bytes;
      readonly destinationChannel: Bytes;
    } & Struct;
    readonly isTokenReceived: boolean;
    readonly asTokenReceived: {
      readonly from: Text;
      readonly to: Text;
      readonly ibcDenom: Bytes;
      readonly localAssetId: Option<u128>;
      readonly amount: u128;
      readonly isReceiverSource: bool;
      readonly sourceChannel: Bytes;
      readonly destinationChannel: Bytes;
    } & Struct;
    readonly isTokenTransferFailed: boolean;
    readonly asTokenTransferFailed: {
      readonly from: Text;
      readonly to: Text;
      readonly ibcDenom: Bytes;
      readonly localAssetId: Option<u128>;
      readonly amount: u128;
      readonly isSenderSource: bool;
      readonly sourceChannel: Bytes;
      readonly destinationChannel: Bytes;
    } & Struct;
    readonly isTokenTransferTimeout: boolean;
    readonly asTokenTransferTimeout: {
      readonly from: Text;
      readonly to: Text;
      readonly ibcDenom: Bytes;
      readonly localAssetId: Option<u128>;
      readonly amount: u128;
      readonly isSenderSource: bool;
      readonly sourceChannel: Bytes;
      readonly destinationChannel: Bytes;
    } & Struct;
    readonly isOnRecvPacketError: boolean;
    readonly asOnRecvPacketError: {
      readonly msg: Bytes;
    } & Struct;
    readonly isClientUpgradeSet: boolean;
    readonly isClientFrozen: boolean;
    readonly asClientFrozen: {
      readonly clientId: Bytes;
      readonly height: u64;
      readonly revisionNumber: u64;
    } & Struct;
    readonly isAssetAdminUpdated: boolean;
    readonly asAssetAdminUpdated: {
      readonly adminAccount: AccountId32;
    } & Struct;
    readonly isFeeLessChannelIdsAdded: boolean;
    readonly asFeeLessChannelIdsAdded: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly isFeeLessChannelIdsRemoved: boolean;
    readonly asFeeLessChannelIdsRemoved: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly isChargingFeeOnTransferInitiated: boolean;
    readonly asChargingFeeOnTransferInitiated: {
      readonly sequence: u64;
      readonly from: Bytes;
      readonly to: Bytes;
      readonly ibcDenom: Bytes;
      readonly localAssetId: Option<u128>;
      readonly amount: u128;
      readonly isFlatFee: bool;
      readonly sourceChannel: Bytes;
      readonly destinationChannel: Bytes;
    } & Struct;
    readonly isChargingFeeConfirmed: boolean;
    readonly asChargingFeeConfirmed: {
      readonly sequence: u64;
    } & Struct;
    readonly isChargingFeeTimeout: boolean;
    readonly asChargingFeeTimeout: {
      readonly sequence: u64;
    } & Struct;
    readonly isChargingFeeFailedAcknowledgement: boolean;
    readonly asChargingFeeFailedAcknowledgement: {
      readonly sequence: u64;
    } & Struct;
    readonly isChildStateUpdated: boolean;
    readonly isClientStateSubstituted: boolean;
    readonly asClientStateSubstituted: {
      readonly clientId: Text;
      readonly height: IbcCoreIcs02ClientHeight;
    } & Struct;
    readonly isExecuteMemoStarted: boolean;
    readonly asExecuteMemoStarted: {
      readonly accountId: AccountId32;
      readonly memo: Option<Text>;
    } & Struct;
    readonly isExecuteMemoIbcTokenTransferSuccess: boolean;
    readonly asExecuteMemoIbcTokenTransferSuccess: {
      readonly from: AccountId32;
      readonly to: Bytes;
      readonly assetId: u128;
      readonly amount: u128;
      readonly channel: u64;
      readonly nextMemo: Option<Text>;
    } & Struct;
    readonly isExecuteMemoIbcTokenTransferFailedWithReason: boolean;
    readonly asExecuteMemoIbcTokenTransferFailedWithReason: {
      readonly from: AccountId32;
      readonly memo: Text;
      readonly reason: u8;
    } & Struct;
    readonly isExecuteMemoIbcTokenTransferFailed: boolean;
    readonly asExecuteMemoIbcTokenTransferFailed: {
      readonly from: AccountId32;
      readonly to: Bytes;
      readonly assetId: u128;
      readonly amount: u128;
      readonly channel: u64;
      readonly nextMemo: Option<Text>;
    } & Struct;
    readonly isExecuteMemoXcmSuccess: boolean;
    readonly asExecuteMemoXcmSuccess: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly assetId: u128;
      readonly paraId: Option<u32>;
    } & Struct;
    readonly isExecuteMemoXcmFailed: boolean;
    readonly asExecuteMemoXcmFailed: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly assetId: u128;
      readonly paraId: Option<u32>;
    } & Struct;
    readonly type: 'Events' | 'TokenTransferInitiated' | 'ChannelOpened' | 'ParamsUpdated' | 'TokenTransferCompleted' | 'TokenReceived' | 'TokenTransferFailed' | 'TokenTransferTimeout' | 'OnRecvPacketError' | 'ClientUpgradeSet' | 'ClientFrozen' | 'AssetAdminUpdated' | 'FeeLessChannelIdsAdded' | 'FeeLessChannelIdsRemoved' | 'ChargingFeeOnTransferInitiated' | 'ChargingFeeConfirmed' | 'ChargingFeeTimeout' | 'ChargingFeeFailedAcknowledgement' | 'ChildStateUpdated' | 'ClientStateSubstituted' | 'ExecuteMemoStarted' | 'ExecuteMemoIbcTokenTransferSuccess' | 'ExecuteMemoIbcTokenTransferFailedWithReason' | 'ExecuteMemoIbcTokenTransferFailed' | 'ExecuteMemoXcmSuccess' | 'ExecuteMemoXcmFailed';
  }

  /** @name PalletIbcEventsIbcEvent (416) */
  interface PalletIbcEventsIbcEvent extends Enum {
    readonly isNewBlock: boolean;
    readonly asNewBlock: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
    } & Struct;
    readonly isCreateClient: boolean;
    readonly asCreateClient: {
      readonly clientId: Bytes;
      readonly clientType: Bytes;
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly consensusHeight: u64;
      readonly consensusRevisionNumber: u64;
    } & Struct;
    readonly isUpdateClient: boolean;
    readonly asUpdateClient: {
      readonly clientId: Bytes;
      readonly clientType: Bytes;
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly consensusHeight: u64;
      readonly consensusRevisionNumber: u64;
    } & Struct;
    readonly isUpgradeClient: boolean;
    readonly asUpgradeClient: {
      readonly clientId: Bytes;
      readonly clientType: Bytes;
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly consensusHeight: u64;
      readonly consensusRevisionNumber: u64;
    } & Struct;
    readonly isClientMisbehaviour: boolean;
    readonly asClientMisbehaviour: {
      readonly clientId: Bytes;
      readonly clientType: Bytes;
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly consensusHeight: u64;
      readonly consensusRevisionNumber: u64;
    } & Struct;
    readonly isOpenInitConnection: boolean;
    readonly asOpenInitConnection: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly connectionId: Option<Bytes>;
      readonly clientId: Bytes;
      readonly counterpartyConnectionId: Option<Bytes>;
      readonly counterpartyClientId: Bytes;
    } & Struct;
    readonly isOpenConfirmConnection: boolean;
    readonly asOpenConfirmConnection: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly connectionId: Option<Bytes>;
      readonly clientId: Bytes;
      readonly counterpartyConnectionId: Option<Bytes>;
      readonly counterpartyClientId: Bytes;
    } & Struct;
    readonly isOpenTryConnection: boolean;
    readonly asOpenTryConnection: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly connectionId: Option<Bytes>;
      readonly clientId: Bytes;
      readonly counterpartyConnectionId: Option<Bytes>;
      readonly counterpartyClientId: Bytes;
    } & Struct;
    readonly isOpenAckConnection: boolean;
    readonly asOpenAckConnection: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly connectionId: Option<Bytes>;
      readonly clientId: Bytes;
      readonly counterpartyConnectionId: Option<Bytes>;
      readonly counterpartyClientId: Bytes;
    } & Struct;
    readonly isOpenInitChannel: boolean;
    readonly asOpenInitChannel: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Option<Bytes>;
      readonly connectionId: Bytes;
      readonly counterpartyPortId: Bytes;
      readonly counterpartyChannelId: Option<Bytes>;
    } & Struct;
    readonly isOpenConfirmChannel: boolean;
    readonly asOpenConfirmChannel: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Option<Bytes>;
      readonly connectionId: Bytes;
      readonly counterpartyPortId: Bytes;
      readonly counterpartyChannelId: Option<Bytes>;
    } & Struct;
    readonly isOpenTryChannel: boolean;
    readonly asOpenTryChannel: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Option<Bytes>;
      readonly connectionId: Bytes;
      readonly counterpartyPortId: Bytes;
      readonly counterpartyChannelId: Option<Bytes>;
    } & Struct;
    readonly isOpenAckChannel: boolean;
    readonly asOpenAckChannel: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Option<Bytes>;
      readonly connectionId: Bytes;
      readonly counterpartyPortId: Bytes;
      readonly counterpartyChannelId: Option<Bytes>;
    } & Struct;
    readonly isCloseInitChannel: boolean;
    readonly asCloseInitChannel: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Bytes;
      readonly connectionId: Bytes;
      readonly counterpartyPortId: Bytes;
      readonly counterpartyChannelId: Option<Bytes>;
    } & Struct;
    readonly isCloseConfirmChannel: boolean;
    readonly asCloseConfirmChannel: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly channelId: Option<Bytes>;
      readonly portId: Bytes;
      readonly connectionId: Bytes;
      readonly counterpartyPortId: Bytes;
      readonly counterpartyChannelId: Option<Bytes>;
    } & Struct;
    readonly isReceivePacket: boolean;
    readonly asReceivePacket: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Bytes;
      readonly destPort: Bytes;
      readonly destChannel: Bytes;
      readonly sequence: u64;
    } & Struct;
    readonly isSendPacket: boolean;
    readonly asSendPacket: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Bytes;
      readonly destPort: Bytes;
      readonly destChannel: Bytes;
      readonly sequence: u64;
    } & Struct;
    readonly isAcknowledgePacket: boolean;
    readonly asAcknowledgePacket: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Bytes;
      readonly sequence: u64;
    } & Struct;
    readonly isWriteAcknowledgement: boolean;
    readonly asWriteAcknowledgement: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Bytes;
      readonly destPort: Bytes;
      readonly destChannel: Bytes;
      readonly sequence: u64;
    } & Struct;
    readonly isTimeoutPacket: boolean;
    readonly asTimeoutPacket: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Bytes;
      readonly sequence: u64;
    } & Struct;
    readonly isTimeoutOnClosePacket: boolean;
    readonly asTimeoutOnClosePacket: {
      readonly revisionHeight: u64;
      readonly revisionNumber: u64;
      readonly portId: Bytes;
      readonly channelId: Bytes;
      readonly sequence: u64;
    } & Struct;
    readonly isEmpty: boolean;
    readonly isChainError: boolean;
    readonly isAppModule: boolean;
    readonly asAppModule: {
      readonly kind: Bytes;
      readonly moduleId: Bytes;
    } & Struct;
    readonly isPushWasmCode: boolean;
    readonly asPushWasmCode: {
      readonly wasmCodeId: Bytes;
    } & Struct;
    readonly type: 'NewBlock' | 'CreateClient' | 'UpdateClient' | 'UpgradeClient' | 'ClientMisbehaviour' | 'OpenInitConnection' | 'OpenConfirmConnection' | 'OpenTryConnection' | 'OpenAckConnection' | 'OpenInitChannel' | 'OpenConfirmChannel' | 'OpenTryChannel' | 'OpenAckChannel' | 'CloseInitChannel' | 'CloseConfirmChannel' | 'ReceivePacket' | 'SendPacket' | 'AcknowledgePacket' | 'WriteAcknowledgement' | 'TimeoutPacket' | 'TimeoutOnClosePacket' | 'Empty' | 'ChainError' | 'AppModule' | 'PushWasmCode';
  }

  /** @name PalletIbcErrorsIbcError (417) */
  interface PalletIbcErrorsIbcError extends Enum {
    readonly isIcs02Client: boolean;
    readonly asIcs02Client: {
      readonly message: Bytes;
    } & Struct;
    readonly isIcs03Connection: boolean;
    readonly asIcs03Connection: {
      readonly message: Bytes;
    } & Struct;
    readonly isIcs04Channel: boolean;
    readonly asIcs04Channel: {
      readonly message: Bytes;
    } & Struct;
    readonly isIcs20FungibleTokenTransfer: boolean;
    readonly asIcs20FungibleTokenTransfer: {
      readonly message: Bytes;
    } & Struct;
    readonly isUnknownMessageTypeUrl: boolean;
    readonly asUnknownMessageTypeUrl: {
      readonly message: Bytes;
    } & Struct;
    readonly isMalformedMessageBytes: boolean;
    readonly asMalformedMessageBytes: {
      readonly message: Bytes;
    } & Struct;
    readonly type: 'Ics02Client' | 'Ics03Connection' | 'Ics04Channel' | 'Ics20FungibleTokenTransfer' | 'UnknownMessageTypeUrl' | 'MalformedMessageBytes';
  }

  /** @name PalletIbcIcs20FeePalletEvent (419) */
  interface PalletIbcIcs20FeePalletEvent extends Enum {
    readonly isIbcTransferFeeCollected: boolean;
    readonly asIbcTransferFeeCollected: {
      readonly amount: u128;
      readonly assetId: u128;
    } & Struct;
    readonly isFeeLessChannelIdsAdded: boolean;
    readonly asFeeLessChannelIdsAdded: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly isFeeLessChannelIdsRemoved: boolean;
    readonly asFeeLessChannelIdsRemoved: {
      readonly sourceChannel: u64;
      readonly destinationChannel: u64;
    } & Struct;
    readonly type: 'IbcTransferFeeCollected' | 'FeeLessChannelIdsAdded' | 'FeeLessChannelIdsRemoved';
  }

  /** @name PalletMultihopXcmIbcEvent (420) */
  interface PalletMultihopXcmIbcEvent extends Enum {
    readonly isSuccessXcmToIbc: boolean;
    readonly asSuccessXcmToIbc: {
      readonly originAddress: AccountId32;
      readonly to: U8aFixed;
      readonly amount: u128;
      readonly assetId: u128;
      readonly memo: Option<Text>;
    } & Struct;
    readonly isFailedXcmToIbc: boolean;
    readonly asFailedXcmToIbc: {
      readonly originAddress: AccountId32;
      readonly to: U8aFixed;
      readonly amount: u128;
      readonly assetId: u128;
      readonly memo: Option<Text>;
    } & Struct;
    readonly isFailedCallback: boolean;
    readonly asFailedCallback: {
      readonly originAddress: U8aFixed;
      readonly routeId: u128;
      readonly reason: PalletMultihopXcmIbcMultihopEventReason;
    } & Struct;
    readonly isMultihopXcmMemo: boolean;
    readonly asMultihopXcmMemo: {
      readonly reason: PalletMultihopXcmIbcMultihopEventReason;
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly assetId: u128;
      readonly isError: bool;
    } & Struct;
    readonly isFailedMatchLocation: boolean;
    readonly type: 'SuccessXcmToIbc' | 'FailedXcmToIbc' | 'FailedCallback' | 'MultihopXcmMemo' | 'FailedMatchLocation';
  }

  /** @name PalletMultihopXcmIbcMultihopEventReason (421) */
  interface PalletMultihopXcmIbcMultihopEventReason extends Enum {
    readonly isFailedToConvertAddressToBytes: boolean;
    readonly isXcmTransferInitiated: boolean;
    readonly isIncorrectPalletId: boolean;
    readonly isMultiHopRouteDoesNotExist: boolean;
    readonly isMultiHopRouteExistButNotConfigured: boolean;
    readonly isIncorrectCountOfAddresses: boolean;
    readonly isFailedToDeriveCosmosAddressFromBytes: boolean;
    readonly isFailedToDeriveChainNameFromUtf8: boolean;
    readonly isFailedToEncodeBech32Address: boolean;
    readonly isFailedToDecodeDestAccountId: boolean;
    readonly isFailedToDecodeSenderAccountId: boolean;
    readonly isDoesNotSupportNonFungible: boolean;
    readonly isFailedCreateMemo: boolean;
    readonly isFailedToConvertMemoIntoPalletIbcMemoMessageType: boolean;
    readonly type: 'FailedToConvertAddressToBytes' | 'XcmTransferInitiated' | 'IncorrectPalletId' | 'MultiHopRouteDoesNotExist' | 'MultiHopRouteExistButNotConfigured' | 'IncorrectCountOfAddresses' | 'FailedToDeriveCosmosAddressFromBytes' | 'FailedToDeriveChainNameFromUtf8' | 'FailedToEncodeBech32Address' | 'FailedToDecodeDestAccountId' | 'FailedToDecodeSenderAccountId' | 'DoesNotSupportNonFungible' | 'FailedCreateMemo' | 'FailedToConvertMemoIntoPalletIbcMemoMessageType';
  }

  /** @name FrameSystemPhase (422) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (425) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemLimitsBlockWeights (426) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: SpWeightsWeightV2Weight;
    readonly maxBlock: SpWeightsWeightV2Weight;
    readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (427) */
  interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (428) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: SpWeightsWeightV2Weight;
    readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
    readonly maxTotal: Option<SpWeightsWeightV2Weight>;
    readonly reserved: Option<SpWeightsWeightV2Weight>;
  }

  /** @name FrameSystemLimitsBlockLength (429) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportDispatchPerDispatchClassU32;
  }

  /** @name FrameSupportDispatchPerDispatchClassU32 (430) */
  interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name SpWeightsRuntimeDbWeight (431) */
  interface SpWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (432) */
  interface SpVersionRuntimeVersion extends Struct {
    readonly specName: Text;
    readonly implName: Text;
    readonly authoringVersion: u32;
    readonly specVersion: u32;
    readonly implVersion: u32;
    readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
    readonly transactionVersion: u32;
    readonly stateVersion: u8;
  }

  /** @name FrameSystemError (436) */
  interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PalletSudoError (437) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name PalletTransactionPaymentReleases (438) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name PalletIndicesError (440) */
  interface PalletIndicesError extends Enum {
    readonly isNotAssigned: boolean;
    readonly isNotOwner: boolean;
    readonly isInUse: boolean;
    readonly isNotTransfer: boolean;
    readonly isPermanent: boolean;
    readonly type: 'NotAssigned' | 'NotOwner' | 'InUse' | 'NotTransfer' | 'Permanent';
  }

  /** @name PalletBalancesBalanceLock (442) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (443) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (446) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesError (448) */
  interface PalletBalancesError extends Enum {
    readonly isVestingBalance: boolean;
    readonly isLiquidityRestrictions: boolean;
    readonly isInsufficientBalance: boolean;
    readonly isExistentialDeposit: boolean;
    readonly isKeepAlive: boolean;
    readonly isExistingVestingSchedule: boolean;
    readonly isDeadAccount: boolean;
    readonly isTooManyReserves: boolean;
    readonly type: 'VestingBalance' | 'LiquidityRestrictions' | 'InsufficientBalance' | 'ExistentialDeposit' | 'KeepAlive' | 'ExistingVestingSchedule' | 'DeadAccount' | 'TooManyReserves';
  }

  /** @name PalletIdentityRegistration (449) */
  interface PalletIdentityRegistration extends Struct {
    readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
    readonly deposit: u128;
    readonly info: PalletIdentityIdentityInfo;
  }

  /** @name PalletIdentityRegistrarInfo (457) */
  interface PalletIdentityRegistrarInfo extends Struct {
    readonly account: AccountId32;
    readonly fee: u128;
    readonly fields: PalletIdentityBitFlags;
  }

  /** @name PalletIdentityError (459) */
  interface PalletIdentityError extends Enum {
    readonly isTooManySubAccounts: boolean;
    readonly isNotFound: boolean;
    readonly isNotNamed: boolean;
    readonly isEmptyIndex: boolean;
    readonly isFeeChanged: boolean;
    readonly isNoIdentity: boolean;
    readonly isStickyJudgement: boolean;
    readonly isJudgementGiven: boolean;
    readonly isInvalidJudgement: boolean;
    readonly isInvalidIndex: boolean;
    readonly isInvalidTarget: boolean;
    readonly isTooManyFields: boolean;
    readonly isTooManyRegistrars: boolean;
    readonly isAlreadyClaimed: boolean;
    readonly isNotSub: boolean;
    readonly isNotOwned: boolean;
    readonly isJudgementForDifferentIdentity: boolean;
    readonly isJudgementPaymentFailed: boolean;
    readonly type: 'TooManySubAccounts' | 'NotFound' | 'NotNamed' | 'EmptyIndex' | 'FeeChanged' | 'NoIdentity' | 'StickyJudgement' | 'JudgementGiven' | 'InvalidJudgement' | 'InvalidIndex' | 'InvalidTarget' | 'TooManyFields' | 'TooManyRegistrars' | 'AlreadyClaimed' | 'NotSub' | 'NotOwned' | 'JudgementForDifferentIdentity' | 'JudgementPaymentFailed';
  }

  /** @name PalletMultisigMultisig (461) */
  interface PalletMultisigMultisig extends Struct {
    readonly when: PalletMultisigTimepoint;
    readonly deposit: u128;
    readonly depositor: AccountId32;
    readonly approvals: Vec<AccountId32>;
  }

  /** @name PalletMultisigError (463) */
  interface PalletMultisigError extends Enum {
    readonly isMinimumThreshold: boolean;
    readonly isAlreadyApproved: boolean;
    readonly isNoApprovalsNeeded: boolean;
    readonly isTooFewSignatories: boolean;
    readonly isTooManySignatories: boolean;
    readonly isSignatoriesOutOfOrder: boolean;
    readonly isSenderInSignatories: boolean;
    readonly isNotFound: boolean;
    readonly isNotOwner: boolean;
    readonly isNoTimepoint: boolean;
    readonly isWrongTimepoint: boolean;
    readonly isUnexpectedTimepoint: boolean;
    readonly isMaxWeightTooLow: boolean;
    readonly isAlreadyStored: boolean;
    readonly type: 'MinimumThreshold' | 'AlreadyApproved' | 'NoApprovalsNeeded' | 'TooFewSignatories' | 'TooManySignatories' | 'SignatoriesOutOfOrder' | 'SenderInSignatories' | 'NotFound' | 'NotOwner' | 'NoTimepoint' | 'WrongTimepoint' | 'UnexpectedTimepoint' | 'MaxWeightTooLow' | 'AlreadyStored';
  }

  /** @name PolkadotPrimitivesV2UpgradeRestriction (465) */
  interface PolkadotPrimitivesV2UpgradeRestriction extends Enum {
    readonly isPresent: boolean;
    readonly type: 'Present';
  }

  /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (466) */
  interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Struct {
    readonly dmqMqcHead: H256;
    readonly relayDispatchQueueSize: ITuple<[u32, u32]>;
    readonly ingressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV2AbridgedHrmpChannel]>>;
    readonly egressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV2AbridgedHrmpChannel]>>;
  }

  /** @name PolkadotPrimitivesV2AbridgedHrmpChannel (469) */
  interface PolkadotPrimitivesV2AbridgedHrmpChannel extends Struct {
    readonly maxCapacity: u32;
    readonly maxTotalSize: u32;
    readonly maxMessageSize: u32;
    readonly msgCount: u32;
    readonly totalSize: u32;
    readonly mqcHead: Option<H256>;
  }

  /** @name PolkadotPrimitivesV2AbridgedHostConfiguration (470) */
  interface PolkadotPrimitivesV2AbridgedHostConfiguration extends Struct {
    readonly maxCodeSize: u32;
    readonly maxHeadDataSize: u32;
    readonly maxUpwardQueueCount: u32;
    readonly maxUpwardQueueSize: u32;
    readonly maxUpwardMessageSize: u32;
    readonly maxUpwardMessageNumPerCandidate: u32;
    readonly hrmpMaxMessageNumPerCandidate: u32;
    readonly validationUpgradeCooldown: u32;
    readonly validationUpgradeDelay: u32;
  }

  /** @name PolkadotCorePrimitivesOutboundHrmpMessage (476) */
  interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
    readonly recipient: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemError (477) */
  interface CumulusPalletParachainSystemError extends Enum {
    readonly isOverlappingUpgrades: boolean;
    readonly isProhibitedByPolkadot: boolean;
    readonly isTooBig: boolean;
    readonly isValidationDataNotAvailable: boolean;
    readonly isHostConfigurationNotAvailable: boolean;
    readonly isNotScheduled: boolean;
    readonly isNothingAuthorized: boolean;
    readonly isUnauthorized: boolean;
    readonly type: 'OverlappingUpgrades' | 'ProhibitedByPolkadot' | 'TooBig' | 'ValidationDataNotAvailable' | 'HostConfigurationNotAvailable' | 'NotScheduled' | 'NothingAuthorized' | 'Unauthorized';
  }

  /** @name PalletCollatorSelectionCandidateInfo (480) */
  interface PalletCollatorSelectionCandidateInfo extends Struct {
    readonly who: AccountId32;
    readonly deposit: u128;
  }

  /** @name PalletCollatorSelectionError (482) */
  interface PalletCollatorSelectionError extends Enum {
    readonly isTooManyCandidates: boolean;
    readonly isTooFewCandidates: boolean;
    readonly isUnknown: boolean;
    readonly isPermission: boolean;
    readonly isAlreadyCandidate: boolean;
    readonly isNotCandidate: boolean;
    readonly isTooManyInvulnerables: boolean;
    readonly isAlreadyInvulnerable: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isValidatorNotRegistered: boolean;
    readonly type: 'TooManyCandidates' | 'TooFewCandidates' | 'Unknown' | 'Permission' | 'AlreadyCandidate' | 'NotCandidate' | 'TooManyInvulnerables' | 'AlreadyInvulnerable' | 'NoAssociatedValidatorId' | 'ValidatorNotRegistered';
  }

  /** @name SpCoreCryptoKeyTypeId (487) */
  interface SpCoreCryptoKeyTypeId extends U8aFixed {}

  /** @name PalletSessionError (488) */
  interface PalletSessionError extends Enum {
    readonly isInvalidProof: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isDuplicatedKey: boolean;
    readonly isNoKeys: boolean;
    readonly isNoAccount: boolean;
    readonly type: 'InvalidProof' | 'NoAssociatedValidatorId' | 'DuplicatedKey' | 'NoKeys' | 'NoAccount';
  }

  /** @name PalletCollectiveVotes (493) */
  interface PalletCollectiveVotes extends Struct {
    readonly index: u32;
    readonly threshold: u32;
    readonly ayes: Vec<AccountId32>;
    readonly nays: Vec<AccountId32>;
    readonly end: u32;
  }

  /** @name PalletCollectiveError (494) */
  interface PalletCollectiveError extends Enum {
    readonly isNotMember: boolean;
    readonly isDuplicateProposal: boolean;
    readonly isProposalMissing: boolean;
    readonly isWrongIndex: boolean;
    readonly isDuplicateVote: boolean;
    readonly isAlreadyInitialized: boolean;
    readonly isTooEarly: boolean;
    readonly isTooManyProposals: boolean;
    readonly isWrongProposalWeight: boolean;
    readonly isWrongProposalLength: boolean;
    readonly type: 'NotMember' | 'DuplicateProposal' | 'ProposalMissing' | 'WrongIndex' | 'DuplicateVote' | 'AlreadyInitialized' | 'TooEarly' | 'TooManyProposals' | 'WrongProposalWeight' | 'WrongProposalLength';
  }

  /** @name PalletMembershipError (496) */
  interface PalletMembershipError extends Enum {
    readonly isAlreadyMember: boolean;
    readonly isNotMember: boolean;
    readonly isTooManyMembers: boolean;
    readonly type: 'AlreadyMember' | 'NotMember' | 'TooManyMembers';
  }

  /** @name PalletTreasuryProposal (497) */
  interface PalletTreasuryProposal extends Struct {
    readonly proposer: AccountId32;
    readonly value: u128;
    readonly beneficiary: AccountId32;
    readonly bond: u128;
  }

  /** @name FrameSupportPalletId (499) */
  interface FrameSupportPalletId extends U8aFixed {}

  /** @name PalletTreasuryError (500) */
  interface PalletTreasuryError extends Enum {
    readonly isInsufficientProposersBalance: boolean;
    readonly isInvalidIndex: boolean;
    readonly isTooManyApprovals: boolean;
    readonly isInsufficientPermission: boolean;
    readonly isProposalNotApproved: boolean;
    readonly type: 'InsufficientProposersBalance' | 'InvalidIndex' | 'TooManyApprovals' | 'InsufficientPermission' | 'ProposalNotApproved';
  }

  /** @name PalletDemocracyReferendumInfo (506) */
  interface PalletDemocracyReferendumInfo extends Enum {
    readonly isOngoing: boolean;
    readonly asOngoing: PalletDemocracyReferendumStatus;
    readonly isFinished: boolean;
    readonly asFinished: {
      readonly approved: bool;
      readonly end: u32;
    } & Struct;
    readonly type: 'Ongoing' | 'Finished';
  }

  /** @name PalletDemocracyReferendumStatus (507) */
  interface PalletDemocracyReferendumStatus extends Struct {
    readonly end: u32;
    readonly proposal: FrameSupportPreimagesBounded;
    readonly threshold: PalletDemocracyVoteThreshold;
    readonly delay: u32;
    readonly tally: PalletDemocracyTally;
  }

  /** @name PalletDemocracyTally (508) */
  interface PalletDemocracyTally extends Struct {
    readonly ayes: u128;
    readonly nays: u128;
    readonly turnout: u128;
  }

  /** @name PalletDemocracyVoteVoting (509) */
  interface PalletDemocracyVoteVoting extends Enum {
    readonly isDirect: boolean;
    readonly asDirect: {
      readonly votes: Vec<ITuple<[u32, PalletDemocracyVoteAccountVote]>>;
      readonly delegations: PalletDemocracyDelegations;
      readonly prior: PalletDemocracyVotePriorLock;
    } & Struct;
    readonly isDelegating: boolean;
    readonly asDelegating: {
      readonly balance: u128;
      readonly target: AccountId32;
      readonly conviction: PalletDemocracyConviction;
      readonly delegations: PalletDemocracyDelegations;
      readonly prior: PalletDemocracyVotePriorLock;
    } & Struct;
    readonly type: 'Direct' | 'Delegating';
  }

  /** @name PalletDemocracyDelegations (513) */
  interface PalletDemocracyDelegations extends Struct {
    readonly votes: u128;
    readonly capital: u128;
  }

  /** @name PalletDemocracyVotePriorLock (514) */
  interface PalletDemocracyVotePriorLock extends ITuple<[u32, u128]> {}

  /** @name PalletDemocracyError (517) */
  interface PalletDemocracyError extends Enum {
    readonly isValueLow: boolean;
    readonly isProposalMissing: boolean;
    readonly isAlreadyCanceled: boolean;
    readonly isDuplicateProposal: boolean;
    readonly isProposalBlacklisted: boolean;
    readonly isNotSimpleMajority: boolean;
    readonly isInvalidHash: boolean;
    readonly isNoProposal: boolean;
    readonly isAlreadyVetoed: boolean;
    readonly isReferendumInvalid: boolean;
    readonly isNoneWaiting: boolean;
    readonly isNotVoter: boolean;
    readonly isNoPermission: boolean;
    readonly isAlreadyDelegating: boolean;
    readonly isInsufficientFunds: boolean;
    readonly isNotDelegating: boolean;
    readonly isVotesExist: boolean;
    readonly isInstantNotAllowed: boolean;
    readonly isNonsense: boolean;
    readonly isWrongUpperBound: boolean;
    readonly isMaxVotesReached: boolean;
    readonly isTooMany: boolean;
    readonly isVotingPeriodLow: boolean;
    readonly isPreimageNotExist: boolean;
    readonly type: 'ValueLow' | 'ProposalMissing' | 'AlreadyCanceled' | 'DuplicateProposal' | 'ProposalBlacklisted' | 'NotSimpleMajority' | 'InvalidHash' | 'NoProposal' | 'AlreadyVetoed' | 'ReferendumInvalid' | 'NoneWaiting' | 'NotVoter' | 'NoPermission' | 'AlreadyDelegating' | 'InsufficientFunds' | 'NotDelegating' | 'VotesExist' | 'InstantNotAllowed' | 'Nonsense' | 'WrongUpperBound' | 'MaxVotesReached' | 'TooMany' | 'VotingPeriodLow' | 'PreimageNotExist';
  }

  /** @name PalletSchedulerScheduled (525) */
  interface PalletSchedulerScheduled extends Struct {
    readonly maybeId: Option<U8aFixed>;
    readonly priority: u8;
    readonly call: FrameSupportPreimagesBounded;
    readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
    readonly origin: PicassoRuntimeOriginCaller;
  }

  /** @name PalletSchedulerError (527) */
  interface PalletSchedulerError extends Enum {
    readonly isFailedToSchedule: boolean;
    readonly isNotFound: boolean;
    readonly isTargetBlockNumberInPast: boolean;
    readonly isRescheduleNoChange: boolean;
    readonly isNamed: boolean;
    readonly type: 'FailedToSchedule' | 'NotFound' | 'TargetBlockNumberInPast' | 'RescheduleNoChange' | 'Named';
  }

  /** @name PalletUtilityError (528) */
  interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: 'TooManyCalls';
  }

  /** @name PalletPreimageRequestStatus (529) */
  interface PalletPreimageRequestStatus extends Enum {
    readonly isUnrequested: boolean;
    readonly asUnrequested: {
      readonly deposit: ITuple<[AccountId32, u128]>;
      readonly len: u32;
    } & Struct;
    readonly isRequested: boolean;
    readonly asRequested: {
      readonly deposit: Option<ITuple<[AccountId32, u128]>>;
      readonly count: u32;
      readonly len: Option<u32>;
    } & Struct;
    readonly type: 'Unrequested' | 'Requested';
  }

  /** @name PalletPreimageError (534) */
  interface PalletPreimageError extends Enum {
    readonly isTooBig: boolean;
    readonly isAlreadyNoted: boolean;
    readonly isNotAuthorized: boolean;
    readonly isNotNoted: boolean;
    readonly isRequested: boolean;
    readonly isNotRequested: boolean;
    readonly type: 'TooBig' | 'AlreadyNoted' | 'NotAuthorized' | 'NotNoted' | 'Requested' | 'NotRequested';
  }

  /** @name PalletProxyProxyDefinition (537) */
  interface PalletProxyProxyDefinition extends Struct {
    readonly delegate: AccountId32;
    readonly proxyType: ComposableTraitsAccountProxyProxyType;
    readonly delay: u32;
  }

  /** @name PalletProxyAnnouncement (541) */
  interface PalletProxyAnnouncement extends Struct {
    readonly real: AccountId32;
    readonly callHash: H256;
    readonly height: u32;
  }

  /** @name PalletProxyError (543) */
  interface PalletProxyError extends Enum {
    readonly isTooMany: boolean;
    readonly isNotFound: boolean;
    readonly isNotProxy: boolean;
    readonly isUnproxyable: boolean;
    readonly isDuplicate: boolean;
    readonly isNoPermission: boolean;
    readonly isUnannounced: boolean;
    readonly isNoSelfProxy: boolean;
    readonly type: 'TooMany' | 'NotFound' | 'NotProxy' | 'Unproxyable' | 'Duplicate' | 'NoPermission' | 'Unannounced' | 'NoSelfProxy';
  }

  /** @name CumulusPalletXcmpQueueInboundChannelDetails (545) */
  interface CumulusPalletXcmpQueueInboundChannelDetails extends Struct {
    readonly sender: u32;
    readonly state: CumulusPalletXcmpQueueInboundState;
    readonly messageMetadata: Vec<ITuple<[u32, PolkadotParachainPrimitivesXcmpMessageFormat]>>;
  }

  /** @name CumulusPalletXcmpQueueInboundState (546) */
  interface CumulusPalletXcmpQueueInboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name PolkadotParachainPrimitivesXcmpMessageFormat (549) */
  interface PolkadotParachainPrimitivesXcmpMessageFormat extends Enum {
    readonly isConcatenatedVersionedXcm: boolean;
    readonly isConcatenatedEncodedBlob: boolean;
    readonly isSignals: boolean;
    readonly type: 'ConcatenatedVersionedXcm' | 'ConcatenatedEncodedBlob' | 'Signals';
  }

  /** @name CumulusPalletXcmpQueueOutboundChannelDetails (552) */
  interface CumulusPalletXcmpQueueOutboundChannelDetails extends Struct {
    readonly recipient: u32;
    readonly state: CumulusPalletXcmpQueueOutboundState;
    readonly signalsExist: bool;
    readonly firstIndex: u16;
    readonly lastIndex: u16;
  }

  /** @name CumulusPalletXcmpQueueOutboundState (553) */
  interface CumulusPalletXcmpQueueOutboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name CumulusPalletXcmpQueueQueueConfigData (555) */
  interface CumulusPalletXcmpQueueQueueConfigData extends Struct {
    readonly suspendThreshold: u32;
    readonly dropThreshold: u32;
    readonly resumeThreshold: u32;
    readonly thresholdWeight: SpWeightsWeightV2Weight;
    readonly weightRestrictDecay: SpWeightsWeightV2Weight;
    readonly xcmpMaxIndividualWeight: SpWeightsWeightV2Weight;
  }

  /** @name CumulusPalletXcmpQueueError (557) */
  interface CumulusPalletXcmpQueueError extends Enum {
    readonly isFailedToSend: boolean;
    readonly isBadXcmOrigin: boolean;
    readonly isBadXcm: boolean;
    readonly isBadOverweightIndex: boolean;
    readonly isWeightOverLimit: boolean;
    readonly type: 'FailedToSend' | 'BadXcmOrigin' | 'BadXcm' | 'BadOverweightIndex' | 'WeightOverLimit';
  }

  /** @name PalletXcmQueryStatus (558) */
  interface PalletXcmQueryStatus extends Enum {
    readonly isPending: boolean;
    readonly asPending: {
      readonly responder: XcmVersionedMultiLocation;
      readonly maybeMatchQuerier: Option<XcmVersionedMultiLocation>;
      readonly maybeNotify: Option<ITuple<[u8, u8]>>;
      readonly timeout: u32;
    } & Struct;
    readonly isVersionNotifier: boolean;
    readonly asVersionNotifier: {
      readonly origin: XcmVersionedMultiLocation;
      readonly isActive: bool;
    } & Struct;
    readonly isReady: boolean;
    readonly asReady: {
      readonly response: XcmVersionedResponse;
      readonly at: u32;
    } & Struct;
    readonly type: 'Pending' | 'VersionNotifier' | 'Ready';
  }

  /** @name XcmVersionedResponse (562) */
  interface XcmVersionedResponse extends Enum {
    readonly isV2: boolean;
    readonly asV2: XcmV2Response;
    readonly isV3: boolean;
    readonly asV3: XcmV3Response;
    readonly type: 'V2' | 'V3';
  }

  /** @name PalletXcmVersionMigrationStage (568) */
  interface PalletXcmVersionMigrationStage extends Enum {
    readonly isMigrateSupportedVersion: boolean;
    readonly isMigrateVersionNotifiers: boolean;
    readonly isNotifyCurrentTargets: boolean;
    readonly asNotifyCurrentTargets: Option<Bytes>;
    readonly isMigrateAndNotifyOldTargets: boolean;
    readonly type: 'MigrateSupportedVersion' | 'MigrateVersionNotifiers' | 'NotifyCurrentTargets' | 'MigrateAndNotifyOldTargets';
  }

  /** @name XcmVersionedAssetId (570) */
  interface XcmVersionedAssetId extends Enum {
    readonly isV3: boolean;
    readonly asV3: XcmV3MultiassetAssetId;
    readonly type: 'V3';
  }

  /** @name PalletXcmRemoteLockedFungibleRecord (571) */
  interface PalletXcmRemoteLockedFungibleRecord extends Struct {
    readonly amount: u128;
    readonly owner: XcmVersionedMultiLocation;
    readonly locker: XcmVersionedMultiLocation;
    readonly users: u32;
  }

  /** @name PalletXcmError (575) */
  interface PalletXcmError extends Enum {
    readonly isUnreachable: boolean;
    readonly isSendFailure: boolean;
    readonly isFiltered: boolean;
    readonly isUnweighableMessage: boolean;
    readonly isDestinationNotInvertible: boolean;
    readonly isEmpty: boolean;
    readonly isCannotReanchor: boolean;
    readonly isTooManyAssets: boolean;
    readonly isInvalidOrigin: boolean;
    readonly isBadVersion: boolean;
    readonly isBadLocation: boolean;
    readonly isNoSubscription: boolean;
    readonly isAlreadySubscribed: boolean;
    readonly isInvalidAsset: boolean;
    readonly isLowBalance: boolean;
    readonly isTooManyLocks: boolean;
    readonly isAccountNotSovereign: boolean;
    readonly isFeesNotMet: boolean;
    readonly isLockNotFound: boolean;
    readonly isInUse: boolean;
    readonly type: 'Unreachable' | 'SendFailure' | 'Filtered' | 'UnweighableMessage' | 'DestinationNotInvertible' | 'Empty' | 'CannotReanchor' | 'TooManyAssets' | 'InvalidOrigin' | 'BadVersion' | 'BadLocation' | 'NoSubscription' | 'AlreadySubscribed' | 'InvalidAsset' | 'LowBalance' | 'TooManyLocks' | 'AccountNotSovereign' | 'FeesNotMet' | 'LockNotFound' | 'InUse';
  }

  /** @name CumulusPalletXcmError (576) */
  type CumulusPalletXcmError = Null;

  /** @name CumulusPalletDmpQueueConfigData (577) */
  interface CumulusPalletDmpQueueConfigData extends Struct {
    readonly maxIndividual: SpWeightsWeightV2Weight;
  }

  /** @name CumulusPalletDmpQueuePageIndexData (578) */
  interface CumulusPalletDmpQueuePageIndexData extends Struct {
    readonly beginUsed: u32;
    readonly endUsed: u32;
    readonly overweightCount: u64;
  }

  /** @name CumulusPalletDmpQueueError (581) */
  interface CumulusPalletDmpQueueError extends Enum {
    readonly isUnknown: boolean;
    readonly isOverLimit: boolean;
    readonly type: 'Unknown' | 'OverLimit';
  }

  /** @name OrmlXtokensModuleError (582) */
  interface OrmlXtokensModuleError extends Enum {
    readonly isAssetHasNoReserve: boolean;
    readonly isNotCrossChainTransfer: boolean;
    readonly isInvalidDest: boolean;
    readonly isNotCrossChainTransferableCurrency: boolean;
    readonly isUnweighableMessage: boolean;
    readonly isXcmExecutionFailed: boolean;
    readonly isCannotReanchor: boolean;
    readonly isInvalidAncestry: boolean;
    readonly isInvalidAsset: boolean;
    readonly isDestinationNotInvertible: boolean;
    readonly isBadVersion: boolean;
    readonly isDistinctReserveForAssetAndFee: boolean;
    readonly isZeroFee: boolean;
    readonly isZeroAmount: boolean;
    readonly isTooManyAssetsBeingSent: boolean;
    readonly isAssetIndexNonExistent: boolean;
    readonly isFeeNotEnough: boolean;
    readonly isNotSupportedMultiLocation: boolean;
    readonly isMinXcmFeeNotDefined: boolean;
    readonly type: 'AssetHasNoReserve' | 'NotCrossChainTransfer' | 'InvalidDest' | 'NotCrossChainTransferableCurrency' | 'UnweighableMessage' | 'XcmExecutionFailed' | 'CannotReanchor' | 'InvalidAncestry' | 'InvalidAsset' | 'DestinationNotInvertible' | 'BadVersion' | 'DistinctReserveForAssetAndFee' | 'ZeroFee' | 'ZeroAmount' | 'TooManyAssetsBeingSent' | 'AssetIndexNonExistent' | 'FeeNotEnough' | 'NotSupportedMultiLocation' | 'MinXcmFeeNotDefined';
  }

  /** @name OrmlUnknownTokensModuleError (585) */
  interface OrmlUnknownTokensModuleError extends Enum {
    readonly isBalanceTooLow: boolean;
    readonly isBalanceOverflow: boolean;
    readonly isUnhandledAsset: boolean;
    readonly type: 'BalanceTooLow' | 'BalanceOverflow' | 'UnhandledAsset';
  }

  /** @name OrmlTokensBalanceLock (588) */
  interface OrmlTokensBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name OrmlTokensAccountData (590) */
  interface OrmlTokensAccountData extends Struct {
    readonly free: u128;
    readonly reserved: u128;
    readonly frozen: u128;
  }

  /** @name OrmlTokensReserveData (592) */
  interface OrmlTokensReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name OrmlTokensModuleError (594) */
  interface OrmlTokensModuleError extends Enum {
    readonly isBalanceTooLow: boolean;
    readonly isAmountIntoBalanceFailed: boolean;
    readonly isLiquidityRestrictions: boolean;
    readonly isMaxLocksExceeded: boolean;
    readonly isKeepAlive: boolean;
    readonly isExistentialDeposit: boolean;
    readonly isDeadAccount: boolean;
    readonly isTooManyReserves: boolean;
    readonly type: 'BalanceTooLow' | 'AmountIntoBalanceFailed' | 'LiquidityRestrictions' | 'MaxLocksExceeded' | 'KeepAlive' | 'ExistentialDeposit' | 'DeadAccount' | 'TooManyReserves';
  }

  /** @name PalletCurrencyFactoryRanges (595) */
  interface PalletCurrencyFactoryRanges extends Struct {
    readonly ranges: Vec<{
      readonly current: u128;
      readonly end: u128;
    } & Struct>;
  }

  /** @name PalletCurrencyFactoryError (598) */
  interface PalletCurrencyFactoryError extends Enum {
    readonly isAssetNotFound: boolean;
    readonly type: 'AssetNotFound';
  }

  /** @name PalletCrowdloanRewardsModelsReward (599) */
  interface PalletCrowdloanRewardsModelsReward extends Struct {
    readonly total: u128;
    readonly claimed: u128;
    readonly vestingPeriod: u64;
  }

  /** @name PalletCrowdloanRewardsError (600) */
  interface PalletCrowdloanRewardsError extends Enum {
    readonly isNotInitialized: boolean;
    readonly isAlreadyInitialized: boolean;
    readonly isBackToTheFuture: boolean;
    readonly isRewardsNotFunded: boolean;
    readonly isInvalidProof: boolean;
    readonly isInvalidClaim: boolean;
    readonly isNothingToClaim: boolean;
    readonly isNotAssociated: boolean;
    readonly isAlreadyAssociated: boolean;
    readonly isNotClaimableYet: boolean;
    readonly isUnexpectedRewardAmount: boolean;
    readonly type: 'NotInitialized' | 'AlreadyInitialized' | 'BackToTheFuture' | 'RewardsNotFunded' | 'InvalidProof' | 'InvalidClaim' | 'NothingToClaim' | 'NotAssociated' | 'AlreadyAssociated' | 'NotClaimableYet' | 'UnexpectedRewardAmount';
  }

  /** @name PalletVestingModuleError (605) */
  interface PalletVestingModuleError extends Enum {
    readonly isZeroVestingPeriod: boolean;
    readonly isZeroVestingPeriodCount: boolean;
    readonly isInsufficientBalanceToLock: boolean;
    readonly isTooManyVestingSchedules: boolean;
    readonly isAmountLow: boolean;
    readonly isMaxVestingSchedulesExceeded: boolean;
    readonly isTryingToSelfVest: boolean;
    readonly isVestingScheduleNotFound: boolean;
    readonly type: 'ZeroVestingPeriod' | 'ZeroVestingPeriodCount' | 'InsufficientBalanceToLock' | 'TooManyVestingSchedules' | 'AmountLow' | 'MaxVestingSchedulesExceeded' | 'TryingToSelfVest' | 'VestingScheduleNotFound';
  }

  /** @name PalletBondedFinanceError (607) */
  interface PalletBondedFinanceError extends Enum {
    readonly isBondOfferNotFound: boolean;
    readonly isInvalidBondOffer: boolean;
    readonly isOfferCompleted: boolean;
    readonly isInvalidNumberOfBonds: boolean;
    readonly type: 'BondOfferNotFound' | 'InvalidBondOffer' | 'OfferCompleted' | 'InvalidNumberOfBonds';
  }

  /** @name PalletAssetsRegistryError (609) */
  interface PalletAssetsRegistryError extends Enum {
    readonly isAssetNotFound: boolean;
    readonly isAssetAlreadyRegistered: boolean;
    readonly isAssetLocationIsNone: boolean;
    readonly isStringExceedsMaxLength: boolean;
    readonly isLocationIsUsed: boolean;
    readonly type: 'AssetNotFound' | 'AssetAlreadyRegistered' | 'AssetLocationIsNone' | 'StringExceedsMaxLength' | 'LocationIsUsed';
  }

  /** @name PalletPabloPoolConfiguration (610) */
  interface PalletPabloPoolConfiguration extends Enum {
    readonly isDualAssetConstantProduct: boolean;
    readonly asDualAssetConstantProduct: ComposableTraitsDexBasicPoolInfo;
    readonly type: 'DualAssetConstantProduct';
  }

  /** @name ComposableTraitsDexBasicPoolInfo (611) */
  interface ComposableTraitsDexBasicPoolInfo extends Struct {
    readonly owner: AccountId32;
    readonly assetsWeights: BTreeMap<u128, Permill>;
    readonly lpToken: u128;
    readonly feeConfig: ComposableTraitsDexFeeConfig;
  }

  /** @name ComposableTraitsDexFeeConfig (613) */
  interface ComposableTraitsDexFeeConfig extends Struct {
    readonly feeRate: Permill;
    readonly ownerFeeRate: Permill;
    readonly protocolFeeRate: Permill;
  }

  /** @name PalletPabloTimeWeightedAveragePrice (614) */
  interface PalletPabloTimeWeightedAveragePrice extends Struct {
    readonly timestamp: u64;
    readonly basePriceCumulative: u128;
    readonly quotePriceCumulative: u128;
    readonly baseTwap: u128;
    readonly quoteTwap: u128;
  }

  /** @name PalletPabloPriceCumulative (615) */
  interface PalletPabloPriceCumulative extends Struct {
    readonly timestamp: u64;
    readonly basePriceCumulative: u128;
    readonly quotePriceCumulative: u128;
  }

  /** @name PalletPabloError (616) */
  interface PalletPabloError extends Enum {
    readonly isPoolNotFound: boolean;
    readonly isNotEnoughLiquidity: boolean;
    readonly isNotEnoughLpToken: boolean;
    readonly isPairMismatch: boolean;
    readonly isAssetNotFound: boolean;
    readonly isMustBeOwner: boolean;
    readonly isInvalidSaleState: boolean;
    readonly isInvalidAmount: boolean;
    readonly isInvalidAsset: boolean;
    readonly isCannotRespectMinimumRequested: boolean;
    readonly isAssetAmountMustBePositiveNumber: boolean;
    readonly isInvalidPair: boolean;
    readonly isInvalidFees: boolean;
    readonly isAmpFactorMustBeGreaterThanZero: boolean;
    readonly isMissingAmount: boolean;
    readonly isMissingMinExpectedAmount: boolean;
    readonly isMoreThanTwoAssetsNotYetSupported: boolean;
    readonly isNoLpTokenForLbp: boolean;
    readonly isNoXTokenForLbp: boolean;
    readonly isWeightsMustBeNonZero: boolean;
    readonly isWeightsMustSumToOne: boolean;
    readonly isStakingPoolConfigError: boolean;
    readonly isIncorrectAssetAmounts: boolean;
    readonly isUnsupportedOperation: boolean;
    readonly isInitialDepositCannotBeZero: boolean;
    readonly isInitialDepositMustContainAllAssets: boolean;
    readonly isMinAmountsMustContainAtLeastOneAsset: boolean;
    readonly isMustDepositMinimumOneAsset: boolean;
    readonly isCannotSwapSameAsset: boolean;
    readonly isCannotBuyAssetWithItself: boolean;
    readonly isIncorrectPoolConfig: boolean;
    readonly type: 'PoolNotFound' | 'NotEnoughLiquidity' | 'NotEnoughLpToken' | 'PairMismatch' | 'AssetNotFound' | 'MustBeOwner' | 'InvalidSaleState' | 'InvalidAmount' | 'InvalidAsset' | 'CannotRespectMinimumRequested' | 'AssetAmountMustBePositiveNumber' | 'InvalidPair' | 'InvalidFees' | 'AmpFactorMustBeGreaterThanZero' | 'MissingAmount' | 'MissingMinExpectedAmount' | 'MoreThanTwoAssetsNotYetSupported' | 'NoLpTokenForLbp' | 'NoXTokenForLbp' | 'WeightsMustBeNonZero' | 'WeightsMustSumToOne' | 'StakingPoolConfigError' | 'IncorrectAssetAmounts' | 'UnsupportedOperation' | 'InitialDepositCannotBeZero' | 'InitialDepositMustContainAllAssets' | 'MinAmountsMustContainAtLeastOneAsset' | 'MustDepositMinimumOneAsset' | 'CannotSwapSameAsset' | 'CannotBuyAssetWithItself' | 'IncorrectPoolConfig';
  }

  /** @name ComposableTraitsOracleRewardTracker (617) */
  interface ComposableTraitsOracleRewardTracker extends Struct {
    readonly period: u64;
    readonly start: u64;
    readonly totalAlreadyRewarded: u128;
    readonly currentBlockReward: u128;
    readonly totalRewardWeight: u128;
  }

  /** @name PalletOracleWithdraw (618) */
  interface PalletOracleWithdraw extends Struct {
    readonly stake: u128;
    readonly unlockBlock: u32;
  }

  /** @name ComposableTraitsOraclePrice (619) */
  interface ComposableTraitsOraclePrice extends Struct {
    readonly price: u128;
    readonly block: u32;
  }

  /** @name PalletOraclePrePrice (623) */
  interface PalletOraclePrePrice extends Struct {
    readonly price: u128;
    readonly block: u32;
    readonly who: AccountId32;
  }

  /** @name PalletOracleAssetInfo (625) */
  interface PalletOracleAssetInfo extends Struct {
    readonly threshold: Percent;
    readonly minAnswers: u32;
    readonly maxAnswers: u32;
    readonly blockInterval: u32;
    readonly rewardWeight: u128;
    readonly slash: u128;
    readonly emitPriceChanges: bool;
  }

  /** @name PalletOracleError (626) */
  interface PalletOracleError extends Enum {
    readonly isUnknown: boolean;
    readonly isNoPermission: boolean;
    readonly isNoStake: boolean;
    readonly isStakeLocked: boolean;
    readonly isNotEnoughStake: boolean;
    readonly isNotEnoughFunds: boolean;
    readonly isInvalidAssetId: boolean;
    readonly isAlreadySubmitted: boolean;
    readonly isMaxPrices: boolean;
    readonly isPriceNotRequested: boolean;
    readonly isUnsetSigner: boolean;
    readonly isAlreadySet: boolean;
    readonly isUnsetController: boolean;
    readonly isControllerUsed: boolean;
    readonly isSignerUsed: boolean;
    readonly isAvoidPanic: boolean;
    readonly isExceedMaxAnswers: boolean;
    readonly isInvalidMinAnswers: boolean;
    readonly isMaxAnswersLessThanMinAnswers: boolean;
    readonly isExceedThreshold: boolean;
    readonly isExceedAssetsCount: boolean;
    readonly isPriceNotFound: boolean;
    readonly isExceedStake: boolean;
    readonly isMustSumTo100: boolean;
    readonly isDepthTooLarge: boolean;
    readonly isArithmeticError: boolean;
    readonly isBlockIntervalLength: boolean;
    readonly isTransferError: boolean;
    readonly isMaxHistory: boolean;
    readonly isMaxPrePrices: boolean;
    readonly isNoRewardTrackerSet: boolean;
    readonly isAnnualRewardLessThanAlreadyRewarded: boolean;
    readonly type: 'Unknown' | 'NoPermission' | 'NoStake' | 'StakeLocked' | 'NotEnoughStake' | 'NotEnoughFunds' | 'InvalidAssetId' | 'AlreadySubmitted' | 'MaxPrices' | 'PriceNotRequested' | 'UnsetSigner' | 'AlreadySet' | 'UnsetController' | 'ControllerUsed' | 'SignerUsed' | 'AvoidPanic' | 'ExceedMaxAnswers' | 'InvalidMinAnswers' | 'MaxAnswersLessThanMinAnswers' | 'ExceedThreshold' | 'ExceedAssetsCount' | 'PriceNotFound' | 'ExceedStake' | 'MustSumTo100' | 'DepthTooLarge' | 'ArithmeticError' | 'BlockIntervalLength' | 'TransferError' | 'MaxHistory' | 'MaxPrePrices' | 'NoRewardTrackerSet' | 'AnnualRewardLessThanAlreadyRewarded';
  }

  /** @name PalletAssetsTransactorRouterError (627) */
  interface PalletAssetsTransactorRouterError extends Enum {
    readonly isCannotSetNewCurrencyToRegistry: boolean;
    readonly isInvalidCurrency: boolean;
    readonly type: 'CannotSetNewCurrencyToRegistry' | 'InvalidCurrency';
  }

  /** @name RewardError (634) */
  interface RewardError extends Enum {
    readonly isTryIntoIntError: boolean;
    readonly isInsufficientFunds: boolean;
    readonly isZeroTotalStake: boolean;
    readonly isMaxRewardCurrencies: boolean;
    readonly type: 'TryIntoIntError' | 'InsufficientFunds' | 'ZeroTotalStake' | 'MaxRewardCurrencies';
  }

  /** @name FarmingRewardSchedule (635) */
  interface FarmingRewardSchedule extends Struct {
    readonly periodCount: u32;
    readonly perPeriod: Compact<u128>;
  }

  /** @name FarmingError (636) */
  interface FarmingError extends Enum {
    readonly isInsufficientStake: boolean;
    readonly type: 'InsufficientStake';
  }

  /** @name PalletReferendaReferendumInfo (637) */
  interface PalletReferendaReferendumInfo extends Enum {
    readonly isOngoing: boolean;
    readonly asOngoing: PalletReferendaReferendumStatus;
    readonly isApproved: boolean;
    readonly asApproved: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
    readonly isRejected: boolean;
    readonly asRejected: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
    readonly isCancelled: boolean;
    readonly asCancelled: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
    readonly isTimedOut: boolean;
    readonly asTimedOut: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
    readonly isKilled: boolean;
    readonly asKilled: u32;
    readonly type: 'Ongoing' | 'Approved' | 'Rejected' | 'Cancelled' | 'TimedOut' | 'Killed';
  }

  /** @name PalletReferendaReferendumStatus (638) */
  interface PalletReferendaReferendumStatus extends Struct {
    readonly track: u16;
    readonly origin: PicassoRuntimeOriginCaller;
    readonly proposal: FrameSupportPreimagesBounded;
    readonly enactment: FrameSupportScheduleDispatchTime;
    readonly submitted: u32;
    readonly submissionDeposit: PalletReferendaDeposit;
    readonly decisionDeposit: Option<PalletReferendaDeposit>;
    readonly deciding: Option<PalletReferendaDecidingStatus>;
    readonly tally: PalletConvictionVotingTally;
    readonly inQueue: bool;
    readonly alarm: Option<ITuple<[u32, ITuple<[u32, u32]>]>>;
  }

  /** @name PalletReferendaDeposit (639) */
  interface PalletReferendaDeposit extends Struct {
    readonly who: AccountId32;
    readonly amount: u128;
  }

  /** @name PalletReferendaDecidingStatus (642) */
  interface PalletReferendaDecidingStatus extends Struct {
    readonly since: u32;
    readonly confirming: Option<u32>;
  }

  /** @name PalletReferendaTrackInfo (650) */
  interface PalletReferendaTrackInfo extends Struct {
    readonly name: Text;
    readonly maxDeciding: u32;
    readonly decisionDeposit: u128;
    readonly preparePeriod: u32;
    readonly decisionPeriod: u32;
    readonly confirmPeriod: u32;
    readonly minEnactmentPeriod: u32;
    readonly minApproval: PalletReferendaCurve;
    readonly minSupport: PalletReferendaCurve;
  }

  /** @name PalletReferendaCurve (651) */
  interface PalletReferendaCurve extends Enum {
    readonly isLinearDecreasing: boolean;
    readonly asLinearDecreasing: {
      readonly length: Perbill;
      readonly floor: Perbill;
      readonly ceil: Perbill;
    } & Struct;
    readonly isSteppedDecreasing: boolean;
    readonly asSteppedDecreasing: {
      readonly begin: Perbill;
      readonly end: Perbill;
      readonly step: Perbill;
      readonly period: Perbill;
    } & Struct;
    readonly isReciprocal: boolean;
    readonly asReciprocal: {
      readonly factor: i64;
      readonly xOffset: i64;
      readonly yOffset: i64;
    } & Struct;
    readonly type: 'LinearDecreasing' | 'SteppedDecreasing' | 'Reciprocal';
  }

  /** @name PalletReferendaError (654) */
  interface PalletReferendaError extends Enum {
    readonly isNotOngoing: boolean;
    readonly isHasDeposit: boolean;
    readonly isBadTrack: boolean;
    readonly isFull: boolean;
    readonly isQueueEmpty: boolean;
    readonly isBadReferendum: boolean;
    readonly isNothingToDo: boolean;
    readonly isNoTrack: boolean;
    readonly isUnfinished: boolean;
    readonly isNoPermission: boolean;
    readonly isNoDeposit: boolean;
    readonly isBadStatus: boolean;
    readonly isPreimageNotExist: boolean;
    readonly type: 'NotOngoing' | 'HasDeposit' | 'BadTrack' | 'Full' | 'QueueEmpty' | 'BadReferendum' | 'NothingToDo' | 'NoTrack' | 'Unfinished' | 'NoPermission' | 'NoDeposit' | 'BadStatus' | 'PreimageNotExist';
  }

  /** @name PalletConvictionVotingVoteVoting (656) */
  interface PalletConvictionVotingVoteVoting extends Enum {
    readonly isCasting: boolean;
    readonly asCasting: PalletConvictionVotingVoteCasting;
    readonly isDelegating: boolean;
    readonly asDelegating: PalletConvictionVotingVoteDelegating;
    readonly type: 'Casting' | 'Delegating';
  }

  /** @name PalletConvictionVotingVoteCasting (657) */
  interface PalletConvictionVotingVoteCasting extends Struct {
    readonly votes: Vec<ITuple<[u32, PalletConvictionVotingVoteAccountVote]>>;
    readonly delegations: PalletConvictionVotingDelegations;
    readonly prior: PalletConvictionVotingVotePriorLock;
  }

  /** @name PalletConvictionVotingDelegations (661) */
  interface PalletConvictionVotingDelegations extends Struct {
    readonly votes: u128;
    readonly capital: u128;
  }

  /** @name PalletConvictionVotingVotePriorLock (662) */
  interface PalletConvictionVotingVotePriorLock extends ITuple<[u32, u128]> {}

  /** @name PalletConvictionVotingVoteDelegating (663) */
  interface PalletConvictionVotingVoteDelegating extends Struct {
    readonly balance: u128;
    readonly target: AccountId32;
    readonly conviction: PalletConvictionVotingConviction;
    readonly delegations: PalletConvictionVotingDelegations;
    readonly prior: PalletConvictionVotingVotePriorLock;
  }

  /** @name PalletConvictionVotingError (667) */
  interface PalletConvictionVotingError extends Enum {
    readonly isNotOngoing: boolean;
    readonly isNotVoter: boolean;
    readonly isNoPermission: boolean;
    readonly isNoPermissionYet: boolean;
    readonly isAlreadyDelegating: boolean;
    readonly isAlreadyVoting: boolean;
    readonly isInsufficientFunds: boolean;
    readonly isNotDelegating: boolean;
    readonly isNonsense: boolean;
    readonly isMaxVotesReached: boolean;
    readonly isClassNeeded: boolean;
    readonly isBadClass: boolean;
    readonly type: 'NotOngoing' | 'NotVoter' | 'NoPermission' | 'NoPermissionYet' | 'AlreadyDelegating' | 'AlreadyVoting' | 'InsufficientFunds' | 'NotDelegating' | 'Nonsense' | 'MaxVotesReached' | 'ClassNeeded' | 'BadClass';
  }

  /** @name PalletWhitelistError (669) */
  interface PalletWhitelistError extends Enum {
    readonly isUnavailablePreImage: boolean;
    readonly isUndecodableCall: boolean;
    readonly isInvalidCallWeightWitness: boolean;
    readonly isCallIsNotWhitelisted: boolean;
    readonly isCallAlreadyWhitelisted: boolean;
    readonly type: 'UnavailablePreImage' | 'UndecodableCall' | 'InvalidCallWeightWitness' | 'CallIsNotWhitelisted' | 'CallAlreadyWhitelisted';
  }

  /** @name PalletCallFilterError (670) */
  interface PalletCallFilterError extends Enum {
    readonly isCannotDisable: boolean;
    readonly isInvalidString: boolean;
    readonly type: 'CannotDisable' | 'InvalidString';
  }

  /** @name PalletCosmwasmCodeInfo (672) */
  interface PalletCosmwasmCodeInfo extends Struct {
    readonly creator: AccountId32;
    readonly pristineCodeHash: U8aFixed;
    readonly instrumentationVersion: u16;
    readonly refcount: u32;
    readonly ibcCapable: bool;
  }

  /** @name PalletCosmwasmInstrumentCostRules (673) */
  interface PalletCosmwasmInstrumentCostRules extends Struct {
    readonly i64const: u32;
    readonly f64const: u32;
    readonly i64load: u32;
    readonly f64load: u32;
    readonly i64store: u32;
    readonly f64store: u32;
    readonly i64eq: u32;
    readonly i64eqz: u32;
    readonly i64ne: u32;
    readonly i64lts: u32;
    readonly i64gts: u32;
    readonly i64les: u32;
    readonly i64ges: u32;
    readonly i64clz: u32;
    readonly i64ctz: u32;
    readonly i64popcnt: u32;
    readonly i64add: u32;
    readonly i64sub: u32;
    readonly i64mul: u32;
    readonly i64divs: u32;
    readonly i64divu: u32;
    readonly i64rems: u32;
    readonly i64and: u32;
    readonly i64or: u32;
    readonly i64xor: u32;
    readonly i64shl: u32;
    readonly i64shrs: u32;
    readonly i64rotl: u32;
    readonly i64rotr: u32;
    readonly i32wrapi64: u32;
    readonly i64extendsi32: u32;
    readonly f64eq: u32;
    readonly f64ne: u32;
    readonly f64lt: u32;
    readonly f64gt: u32;
    readonly f64le: u32;
    readonly f64ge: u32;
    readonly f64abs: u32;
    readonly f64neg: u32;
    readonly f64ceil: u32;
    readonly f64floor: u32;
    readonly f64trunc: u32;
    readonly f64nearest: u32;
    readonly f64sqrt: u32;
    readonly f64add: u32;
    readonly f64sub: u32;
    readonly f64mul: u32;
    readonly f64div: u32;
    readonly f64min: u32;
    readonly f64max: u32;
    readonly f64copysign: u32;
    readonly select: u32;
    readonly if: u32;
    readonly else: u32;
    readonly getlocal: u32;
    readonly setlocal: u32;
    readonly teelocal: u32;
    readonly setglobal: u32;
    readonly getglobal: u32;
    readonly currentmemory: u32;
    readonly growmemory: u32;
    readonly br: u32;
    readonly brif: u32;
    readonly brtable: u32;
    readonly brtablePerElem: u32;
    readonly call: u32;
    readonly callIndirect: u32;
  }

  /** @name PalletCosmwasmError (674) */
  interface PalletCosmwasmError extends Enum {
    readonly isInstrumentation: boolean;
    readonly isVmCreation: boolean;
    readonly isContractHasNoInfo: boolean;
    readonly isCodeDecoding: boolean;
    readonly isCodeValidation: boolean;
    readonly isCodeEncoding: boolean;
    readonly isCodeInstrumentation: boolean;
    readonly isInstrumentedCodeIsTooBig: boolean;
    readonly isCodeAlreadyExists: boolean;
    readonly isCodeNotFound: boolean;
    readonly isContractAlreadyExists: boolean;
    readonly isContractNotFound: boolean;
    readonly isSubstrateDispatch: boolean;
    readonly isAssetConversion: boolean;
    readonly isTransferFailed: boolean;
    readonly isLabelTooBig: boolean;
    readonly isUnknownDenom: boolean;
    readonly isStackOverflow: boolean;
    readonly isNotEnoughFundsForUpload: boolean;
    readonly isNonceOverflow: boolean;
    readonly isRefcountOverflow: boolean;
    readonly isVmDepthOverflow: boolean;
    readonly isSignatureVerificationError: boolean;
    readonly isIteratorIdOverflow: boolean;
    readonly isIteratorNotFound: boolean;
    readonly isIteratorValueNotFound: boolean;
    readonly isNotAuthorized: boolean;
    readonly isNotImplemented: boolean;
    readonly isUnsupported: boolean;
    readonly isExecuteDeserialize: boolean;
    readonly isIbc: boolean;
    readonly isFailedToSerialize: boolean;
    readonly isOutOfGas: boolean;
    readonly isInvalidGasCheckpoint: boolean;
    readonly isInvalidSalt: boolean;
    readonly isInvalidAccount: boolean;
    readonly isInterpreter: boolean;
    readonly isVirtualMachine: boolean;
    readonly isAccountConversionFailure: boolean;
    readonly isAborted: boolean;
    readonly isReadOnlyViolation: boolean;
    readonly isRpc: boolean;
    readonly isPrecompile: boolean;
    readonly isQueryDeserialize: boolean;
    readonly isExecuteSerialize: boolean;
    readonly type: 'Instrumentation' | 'VmCreation' | 'ContractHasNoInfo' | 'CodeDecoding' | 'CodeValidation' | 'CodeEncoding' | 'CodeInstrumentation' | 'InstrumentedCodeIsTooBig' | 'CodeAlreadyExists' | 'CodeNotFound' | 'ContractAlreadyExists' | 'ContractNotFound' | 'SubstrateDispatch' | 'AssetConversion' | 'TransferFailed' | 'LabelTooBig' | 'UnknownDenom' | 'StackOverflow' | 'NotEnoughFundsForUpload' | 'NonceOverflow' | 'RefcountOverflow' | 'VmDepthOverflow' | 'SignatureVerificationError' | 'IteratorIdOverflow' | 'IteratorNotFound' | 'IteratorValueNotFound' | 'NotAuthorized' | 'NotImplemented' | 'Unsupported' | 'ExecuteDeserialize' | 'Ibc' | 'FailedToSerialize' | 'OutOfGas' | 'InvalidGasCheckpoint' | 'InvalidSalt' | 'InvalidAccount' | 'Interpreter' | 'VirtualMachine' | 'AccountConversionFailure' | 'Aborted' | 'ReadOnlyViolation' | 'Rpc' | 'Precompile' | 'QueryDeserialize' | 'ExecuteSerialize';
  }

  /** @name PalletIbcLightClientProtocol (683) */
  interface PalletIbcLightClientProtocol extends Enum {
    readonly isBeefy: boolean;
    readonly isGrandpa: boolean;
    readonly type: 'Beefy' | 'Grandpa';
  }

  /** @name PalletIbcError (684) */
  interface PalletIbcError extends Enum {
    readonly isProcessingError: boolean;
    readonly isDecodingError: boolean;
    readonly isEncodingError: boolean;
    readonly isProofGenerationError: boolean;
    readonly isConsensusStateNotFound: boolean;
    readonly isChannelNotFound: boolean;
    readonly isClientStateNotFound: boolean;
    readonly isConnectionNotFound: boolean;
    readonly isPacketCommitmentNotFound: boolean;
    readonly isPacketReceiptNotFound: boolean;
    readonly isPacketAcknowledgmentNotFound: boolean;
    readonly isSendPacketError: boolean;
    readonly isInvalidChannelId: boolean;
    readonly isInvalidPortId: boolean;
    readonly isOther: boolean;
    readonly isInvalidRoute: boolean;
    readonly isInvalidMessageType: boolean;
    readonly isTransferInternals: boolean;
    readonly isTransferSerde: boolean;
    readonly isTransferOther: boolean;
    readonly isTransferProtocol: boolean;
    readonly isTransferSend: boolean;
    readonly isUtf8Error: boolean;
    readonly isInvalidAssetId: boolean;
    readonly isPrefixedDenomParse: boolean;
    readonly isInvalidAmount: boolean;
    readonly isInvalidTimestamp: boolean;
    readonly isFailedToGetRevisionNumber: boolean;
    readonly isInvalidParams: boolean;
    readonly isChannelInitError: boolean;
    readonly isTimestampAndHeightNotFound: boolean;
    readonly isChannelEscrowAddress: boolean;
    readonly isWriteAckError: boolean;
    readonly isClientUpdateNotFound: boolean;
    readonly isClientFreezeFailed: boolean;
    readonly isAccessDenied: boolean;
    readonly isRateLimiter: boolean;
    readonly isFailedSendFeeToAccount: boolean;
    readonly isOriginAddress: boolean;
    readonly type: 'ProcessingError' | 'DecodingError' | 'EncodingError' | 'ProofGenerationError' | 'ConsensusStateNotFound' | 'ChannelNotFound' | 'ClientStateNotFound' | 'ConnectionNotFound' | 'PacketCommitmentNotFound' | 'PacketReceiptNotFound' | 'PacketAcknowledgmentNotFound' | 'SendPacketError' | 'InvalidChannelId' | 'InvalidPortId' | 'Other' | 'InvalidRoute' | 'InvalidMessageType' | 'TransferInternals' | 'TransferSerde' | 'TransferOther' | 'TransferProtocol' | 'TransferSend' | 'Utf8Error' | 'InvalidAssetId' | 'PrefixedDenomParse' | 'InvalidAmount' | 'InvalidTimestamp' | 'FailedToGetRevisionNumber' | 'InvalidParams' | 'ChannelInitError' | 'TimestampAndHeightNotFound' | 'ChannelEscrowAddress' | 'WriteAckError' | 'ClientUpdateNotFound' | 'ClientFreezeFailed' | 'AccessDenied' | 'RateLimiter' | 'FailedSendFeeToAccount' | 'OriginAddress';
  }

  /** @name PalletMultihopXcmIbcError (685) */
  interface PalletMultihopXcmIbcError extends Enum {
    readonly isIncorrectAddress: boolean;
    readonly asIncorrectAddress: {
      readonly chainId: u8;
    } & Struct;
    readonly isIncorrectChainName: boolean;
    readonly asIncorrectChainName: {
      readonly chainId: u8;
    } & Struct;
    readonly isFailedToEncodeBech32Address: boolean;
    readonly asFailedToEncodeBech32Address: {
      readonly chainId: u8;
    } & Struct;
    readonly isIncorrectMultiLocation: boolean;
    readonly isXcmDepositFailed: boolean;
    readonly isMultiHopRouteDoesNotExist: boolean;
    readonly isDoesNotSupportNonFungible: boolean;
    readonly isIncorrectCountOfAddresses: boolean;
    readonly isFailedToConstructMemo: boolean;
    readonly isFailedToDecodeAccountId: boolean;
    readonly type: 'IncorrectAddress' | 'IncorrectChainName' | 'FailedToEncodeBech32Address' | 'IncorrectMultiLocation' | 'XcmDepositFailed' | 'MultiHopRouteDoesNotExist' | 'DoesNotSupportNonFungible' | 'IncorrectCountOfAddresses' | 'FailedToConstructMemo' | 'FailedToDecodeAccountId';
  }

  /** @name FrameSystemExtensionsCheckNonZeroSender (688) */
  type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (689) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (690) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (691) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (694) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (695) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletAssetTxPaymentChargeAssetTxPayment (696) */
  interface PalletAssetTxPaymentChargeAssetTxPayment extends Struct {
    readonly tip: Compact<u128>;
    readonly assetId: Option<u128>;
  }

  /** @name PicassoRuntimeRuntime (697) */
  type PicassoRuntimeRuntime = Null;

} // declare module
