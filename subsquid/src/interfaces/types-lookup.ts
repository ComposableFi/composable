// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type { Data } from '@polkadot/types';
import type { BTreeMap, BTreeSet, Bytes, Compact, Enum, Null, Option, Result, Set, Struct, Text, U8aFixed, Vec, WrapperKeepOpaque, bool, i128, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { Vote } from '@polkadot/types/interfaces/elections';
import type { AccountId32, Call, H256, MultiAddress, Perbill, Percent, Permill, Perquintill } from '@polkadot/types/interfaces/runtime';
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

  /** @name FrameSupportWeightsPerDispatchClassU64 (7) */
  interface FrameSupportWeightsPerDispatchClassU64 extends Struct {
    readonly normal: u64;
    readonly operational: u64;
    readonly mandatory: u64;
  }

  /** @name SpRuntimeDigest (11) */
  interface SpRuntimeDigest extends Struct {
    readonly logs: Vec<SpRuntimeDigestDigestItem>;
  }

  /** @name SpRuntimeDigestDigestItem (13) */
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

  /** @name FrameSystemEventRecord (16) */
  interface FrameSystemEventRecord extends Struct {
    readonly phase: FrameSystemPhase;
    readonly event: Event;
    readonly topics: Vec<H256>;
  }

  /** @name FrameSystemEvent (18) */
  interface FrameSystemEvent extends Enum {
    readonly isExtrinsicSuccess: boolean;
    readonly asExtrinsicSuccess: {
      readonly dispatchInfo: FrameSupportWeightsDispatchInfo;
    } & Struct;
    readonly isExtrinsicFailed: boolean;
    readonly asExtrinsicFailed: {
      readonly dispatchError: SpRuntimeDispatchError;
      readonly dispatchInfo: FrameSupportWeightsDispatchInfo;
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

  /** @name FrameSupportWeightsDispatchInfo (19) */
  interface FrameSupportWeightsDispatchInfo extends Struct {
    readonly weight: u64;
    readonly class: FrameSupportWeightsDispatchClass;
    readonly paysFee: FrameSupportWeightsPays;
  }

  /** @name FrameSupportWeightsDispatchClass (20) */
  interface FrameSupportWeightsDispatchClass extends Enum {
    readonly isNormal: boolean;
    readonly isOperational: boolean;
    readonly isMandatory: boolean;
    readonly type: 'Normal' | 'Operational' | 'Mandatory';
  }

  /** @name FrameSupportWeightsPays (21) */
  interface FrameSupportWeightsPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: 'Yes' | 'No';
  }

  /** @name SpRuntimeDispatchError (22) */
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
    readonly asArithmetic: SpRuntimeArithmeticError;
    readonly isTransactional: boolean;
    readonly asTransactional: SpRuntimeTransactionalError;
    readonly type: 'Other' | 'CannotLookup' | 'BadOrigin' | 'Module' | 'ConsumerRemaining' | 'NoProviders' | 'TooManyConsumers' | 'Token' | 'Arithmetic' | 'Transactional';
  }

  /** @name SpRuntimeModuleError (23) */
  interface SpRuntimeModuleError extends Struct {
    readonly index: u8;
    readonly error: U8aFixed;
  }

  /** @name SpRuntimeTokenError (24) */
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

  /** @name SpRuntimeArithmeticError (25) */
  interface SpRuntimeArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
  }

  /** @name SpRuntimeTransactionalError (26) */
  interface SpRuntimeTransactionalError extends Enum {
    readonly isLimitReached: boolean;
    readonly isNoLayer: boolean;
    readonly type: 'LimitReached' | 'NoLayer';
  }

  /** @name PalletSudoEvent (27) */
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

  /** @name PalletTransactionPaymentEvent (31) */
  interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: 'TransactionFeePaid';
  }

  /** @name PalletIndicesEvent (32) */
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

  /** @name PalletBalancesEvent (33) */
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

  /** @name FrameSupportTokensMiscBalanceStatus (34) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletIdentityEvent (35) */
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

  /** @name PalletMultisigEvent (36) */
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

  /** @name PalletMultisigTimepoint (37) */
  interface PalletMultisigTimepoint extends Struct {
    readonly height: u32;
    readonly index: u32;
  }

  /** @name CumulusPalletParachainSystemEvent (38) */
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
      readonly weightUsed: u64;
      readonly dmqHead: H256;
    } & Struct;
    readonly type: 'ValidationFunctionStored' | 'ValidationFunctionApplied' | 'ValidationFunctionDiscarded' | 'UpgradeAuthorized' | 'DownwardMessagesReceived' | 'DownwardMessagesProcessed';
  }

  /** @name PalletCollatorSelectionEvent (39) */
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

  /** @name PalletSessionEvent (41) */
  interface PalletSessionEvent extends Enum {
    readonly isNewSession: boolean;
    readonly asNewSession: {
      readonly sessionIndex: u32;
    } & Struct;
    readonly type: 'NewSession';
  }

  /** @name PalletCollectiveEvent (42) */
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

  /** @name PalletMembershipEvent (44) */
  interface PalletMembershipEvent extends Enum {
    readonly isMemberAdded: boolean;
    readonly isMemberRemoved: boolean;
    readonly isMembersSwapped: boolean;
    readonly isMembersReset: boolean;
    readonly isKeyChanged: boolean;
    readonly isDummy: boolean;
    readonly type: 'MemberAdded' | 'MemberRemoved' | 'MembersSwapped' | 'MembersReset' | 'KeyChanged' | 'Dummy';
  }

  /** @name PalletTreasuryEvent (45) */
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
    readonly type: 'Proposed' | 'Spending' | 'Awarded' | 'Rejected' | 'Burnt' | 'Rollover' | 'Deposit' | 'SpendApproved';
  }

  /** @name PalletDemocracyEvent (46) */
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
      readonly depositors: Vec<AccountId32>;
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
    readonly isExecuted: boolean;
    readonly asExecuted: {
      readonly refIndex: u32;
      readonly result: Result<Null, SpRuntimeDispatchError>;
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
    readonly isPreimageNoted: boolean;
    readonly asPreimageNoted: {
      readonly proposalHash: H256;
      readonly who: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isPreimageUsed: boolean;
    readonly asPreimageUsed: {
      readonly proposalHash: H256;
      readonly provider: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isPreimageInvalid: boolean;
    readonly asPreimageInvalid: {
      readonly proposalHash: H256;
      readonly refIndex: u32;
    } & Struct;
    readonly isPreimageMissing: boolean;
    readonly asPreimageMissing: {
      readonly proposalHash: H256;
      readonly refIndex: u32;
    } & Struct;
    readonly isPreimageReaped: boolean;
    readonly asPreimageReaped: {
      readonly proposalHash: H256;
      readonly provider: AccountId32;
      readonly deposit: u128;
      readonly reaper: AccountId32;
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
    readonly type: 'Proposed' | 'Tabled' | 'ExternalTabled' | 'Started' | 'Passed' | 'NotPassed' | 'Cancelled' | 'Executed' | 'Delegated' | 'Undelegated' | 'Vetoed' | 'PreimageNoted' | 'PreimageUsed' | 'PreimageInvalid' | 'PreimageMissing' | 'PreimageReaped' | 'Blacklisted' | 'Voted' | 'Seconded' | 'ProposalCanceled';
  }

  /** @name PalletDemocracyVoteThreshold (47) */
  interface PalletDemocracyVoteThreshold extends Enum {
    readonly isSuperMajorityApprove: boolean;
    readonly isSuperMajorityAgainst: boolean;
    readonly isSimpleMajority: boolean;
    readonly type: 'SuperMajorityApprove' | 'SuperMajorityAgainst' | 'SimpleMajority';
  }

  /** @name PalletDemocracyVoteAccountVote (48) */
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

  /** @name PalletSchedulerEvent (52) */
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
      readonly id: Option<Bytes>;
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isCallLookupFailed: boolean;
    readonly asCallLookupFailed: {
      readonly task: ITuple<[u32, u32]>;
      readonly id: Option<Bytes>;
      readonly error: FrameSupportScheduleLookupError;
    } & Struct;
    readonly type: 'Scheduled' | 'Canceled' | 'Dispatched' | 'CallLookupFailed';
  }

  /** @name FrameSupportScheduleLookupError (55) */
  interface FrameSupportScheduleLookupError extends Enum {
    readonly isUnknown: boolean;
    readonly isBadFormat: boolean;
    readonly type: 'Unknown' | 'BadFormat';
  }

  /** @name PalletUtilityEvent (56) */
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

  /** @name PalletPreimageEvent (57) */
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

  /** @name PalletAccountProxyEvent (58) */
  interface PalletAccountProxyEvent extends Enum {
    readonly isProxyExecuted: boolean;
    readonly asProxyExecuted: {
      readonly result: Result<Null, SpRuntimeDispatchError>;
    } & Struct;
    readonly isAnonymousCreated: boolean;
    readonly asAnonymousCreated: {
      readonly anonymous: AccountId32;
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
    readonly type: 'ProxyExecuted' | 'AnonymousCreated' | 'Announced' | 'ProxyAdded' | 'ProxyRemoved';
  }

  /** @name ComposableTraitsAccountProxyProxyType (59) */
  interface ComposableTraitsAccountProxyProxyType extends Enum {
    readonly isAny: boolean;
    readonly isGovernance: boolean;
    readonly isCancelProxy: boolean;
    readonly type: 'Any' | 'Governance' | 'CancelProxy';
  }

  /** @name CumulusPalletXcmpQueueEvent (61) */
  interface CumulusPalletXcmpQueueEvent extends Enum {
    readonly isSuccess: boolean;
    readonly asSuccess: {
      readonly messageHash: Option<H256>;
      readonly weight: u64;
    } & Struct;
    readonly isFail: boolean;
    readonly asFail: {
      readonly messageHash: Option<H256>;
      readonly error: XcmV2TraitsError;
      readonly weight: u64;
    } & Struct;
    readonly isBadVersion: boolean;
    readonly asBadVersion: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isBadFormat: boolean;
    readonly asBadFormat: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isUpwardMessageSent: boolean;
    readonly asUpwardMessageSent: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isXcmpMessageSent: boolean;
    readonly asXcmpMessageSent: {
      readonly messageHash: Option<H256>;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly sender: u32;
      readonly sentAt: u32;
      readonly index: u64;
      readonly required: u64;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly index: u64;
      readonly used: u64;
    } & Struct;
    readonly type: 'Success' | 'Fail' | 'BadVersion' | 'BadFormat' | 'UpwardMessageSent' | 'XcmpMessageSent' | 'OverweightEnqueued' | 'OverweightServiced';
  }

  /** @name XcmV2TraitsError (63) */
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

  /** @name PalletXcmEvent (65) */
  interface PalletXcmEvent extends Enum {
    readonly isAttempted: boolean;
    readonly asAttempted: XcmV2TraitsOutcome;
    readonly isSent: boolean;
    readonly asSent: ITuple<[XcmV1MultiLocation, XcmV1MultiLocation, XcmV2Xcm]>;
    readonly isUnexpectedResponse: boolean;
    readonly asUnexpectedResponse: ITuple<[XcmV1MultiLocation, u64]>;
    readonly isResponseReady: boolean;
    readonly asResponseReady: ITuple<[u64, XcmV2Response]>;
    readonly isNotified: boolean;
    readonly asNotified: ITuple<[u64, u8, u8]>;
    readonly isNotifyOverweight: boolean;
    readonly asNotifyOverweight: ITuple<[u64, u8, u8, u64, u64]>;
    readonly isNotifyDispatchError: boolean;
    readonly asNotifyDispatchError: ITuple<[u64, u8, u8]>;
    readonly isNotifyDecodeFailed: boolean;
    readonly asNotifyDecodeFailed: ITuple<[u64, u8, u8]>;
    readonly isInvalidResponder: boolean;
    readonly asInvalidResponder: ITuple<[XcmV1MultiLocation, u64, Option<XcmV1MultiLocation>]>;
    readonly isInvalidResponderVersion: boolean;
    readonly asInvalidResponderVersion: ITuple<[XcmV1MultiLocation, u64]>;
    readonly isResponseTaken: boolean;
    readonly asResponseTaken: u64;
    readonly isAssetsTrapped: boolean;
    readonly asAssetsTrapped: ITuple<[H256, XcmV1MultiLocation, XcmVersionedMultiAssets]>;
    readonly isVersionChangeNotified: boolean;
    readonly asVersionChangeNotified: ITuple<[XcmV1MultiLocation, u32]>;
    readonly isSupportedVersionChanged: boolean;
    readonly asSupportedVersionChanged: ITuple<[XcmV1MultiLocation, u32]>;
    readonly isNotifyTargetSendFail: boolean;
    readonly asNotifyTargetSendFail: ITuple<[XcmV1MultiLocation, u64, XcmV2TraitsError]>;
    readonly isNotifyTargetMigrationFail: boolean;
    readonly asNotifyTargetMigrationFail: ITuple<[XcmVersionedMultiLocation, u64]>;
    readonly type: 'Attempted' | 'Sent' | 'UnexpectedResponse' | 'ResponseReady' | 'Notified' | 'NotifyOverweight' | 'NotifyDispatchError' | 'NotifyDecodeFailed' | 'InvalidResponder' | 'InvalidResponderVersion' | 'ResponseTaken' | 'AssetsTrapped' | 'VersionChangeNotified' | 'SupportedVersionChanged' | 'NotifyTargetSendFail' | 'NotifyTargetMigrationFail';
  }

  /** @name XcmV2TraitsOutcome (66) */
  interface XcmV2TraitsOutcome extends Enum {
    readonly isComplete: boolean;
    readonly asComplete: u64;
    readonly isIncomplete: boolean;
    readonly asIncomplete: ITuple<[u64, XcmV2TraitsError]>;
    readonly isError: boolean;
    readonly asError: XcmV2TraitsError;
    readonly type: 'Complete' | 'Incomplete' | 'Error';
  }

  /** @name XcmV1MultiLocation (67) */
  interface XcmV1MultiLocation extends Struct {
    readonly parents: u8;
    readonly interior: XcmV1MultilocationJunctions;
  }

  /** @name XcmV1MultilocationJunctions (68) */
  interface XcmV1MultilocationJunctions extends Enum {
    readonly isHere: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV1Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV1Junction, XcmV1Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction, XcmV1Junction]>;
    readonly type: 'Here' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV1Junction (69) */
  interface XcmV1Junction extends Enum {
    readonly isParachain: boolean;
    readonly asParachain: Compact<u32>;
    readonly isAccountId32: boolean;
    readonly asAccountId32: {
      readonly network: XcmV0JunctionNetworkId;
      readonly id: U8aFixed;
    } & Struct;
    readonly isAccountIndex64: boolean;
    readonly asAccountIndex64: {
      readonly network: XcmV0JunctionNetworkId;
      readonly index: Compact<u64>;
    } & Struct;
    readonly isAccountKey20: boolean;
    readonly asAccountKey20: {
      readonly network: XcmV0JunctionNetworkId;
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
      readonly id: XcmV0JunctionBodyId;
      readonly part: XcmV0JunctionBodyPart;
    } & Struct;
    readonly type: 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality';
  }

  /** @name XcmV0JunctionNetworkId (71) */
  interface XcmV0JunctionNetworkId extends Enum {
    readonly isAny: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly type: 'Any' | 'Named' | 'Polkadot' | 'Kusama';
  }

  /** @name XcmV0JunctionBodyId (76) */
  interface XcmV0JunctionBodyId extends Enum {
    readonly isUnit: boolean;
    readonly isNamed: boolean;
    readonly asNamed: Bytes;
    readonly isIndex: boolean;
    readonly asIndex: Compact<u32>;
    readonly isExecutive: boolean;
    readonly isTechnical: boolean;
    readonly isLegislative: boolean;
    readonly isJudicial: boolean;
    readonly type: 'Unit' | 'Named' | 'Index' | 'Executive' | 'Technical' | 'Legislative' | 'Judicial';
  }

  /** @name XcmV0JunctionBodyPart (77) */
  interface XcmV0JunctionBodyPart extends Enum {
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

  /** @name XcmV2Xcm (78) */
  interface XcmV2Xcm extends Vec<XcmV2Instruction> {}

  /** @name XcmV2Instruction (80) */
  interface XcmV2Instruction extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: XcmV1MultiassetMultiAssets;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: XcmV1MultiassetMultiAssets;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: XcmV1MultiassetMultiAssets;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV2Response;
      readonly maxWeight: Compact<u64>;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly beneficiary: XcmV1MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly dest: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV0OriginKind;
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
    readonly asDescendOrigin: XcmV1MultilocationJunctions;
    readonly isReportError: boolean;
    readonly asReportError: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV1MultiLocation;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly beneficiary: XcmV1MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: Compact<u32>;
      readonly dest: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: XcmV1MultiassetMultiAssetFilter;
      readonly receive: XcmV1MultiassetMultiAssets;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly reserve: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly dest: XcmV1MultiLocation;
      readonly xcm: XcmV2Xcm;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV1MultiLocation;
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV1MultiAsset;
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
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly ticket: XcmV1MultiLocation;
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

  /** @name XcmV1MultiassetMultiAssets (81) */
  interface XcmV1MultiassetMultiAssets extends Vec<XcmV1MultiAsset> {}

  /** @name XcmV1MultiAsset (83) */
  interface XcmV1MultiAsset extends Struct {
    readonly id: XcmV1MultiassetAssetId;
    readonly fun: XcmV1MultiassetFungibility;
  }

  /** @name XcmV1MultiassetAssetId (84) */
  interface XcmV1MultiassetAssetId extends Enum {
    readonly isConcrete: boolean;
    readonly asConcrete: XcmV1MultiLocation;
    readonly isAbstract: boolean;
    readonly asAbstract: Bytes;
    readonly type: 'Concrete' | 'Abstract';
  }

  /** @name XcmV1MultiassetFungibility (85) */
  interface XcmV1MultiassetFungibility extends Enum {
    readonly isFungible: boolean;
    readonly asFungible: Compact<u128>;
    readonly isNonFungible: boolean;
    readonly asNonFungible: XcmV1MultiassetAssetInstance;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV1MultiassetAssetInstance (86) */
  interface XcmV1MultiassetAssetInstance extends Enum {
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

  /** @name XcmV2Response (89) */
  interface XcmV2Response extends Enum {
    readonly isNull: boolean;
    readonly isAssets: boolean;
    readonly asAssets: XcmV1MultiassetMultiAssets;
    readonly isExecutionResult: boolean;
    readonly asExecutionResult: Option<ITuple<[u32, XcmV2TraitsError]>>;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly type: 'Null' | 'Assets' | 'ExecutionResult' | 'Version';
  }

  /** @name XcmV0OriginKind (92) */
  interface XcmV0OriginKind extends Enum {
    readonly isNative: boolean;
    readonly isSovereignAccount: boolean;
    readonly isSuperuser: boolean;
    readonly isXcm: boolean;
    readonly type: 'Native' | 'SovereignAccount' | 'Superuser' | 'Xcm';
  }

  /** @name XcmDoubleEncoded (93) */
  interface XcmDoubleEncoded extends Struct {
    readonly encoded: Bytes;
  }

  /** @name XcmV1MultiassetMultiAssetFilter (94) */
  interface XcmV1MultiassetMultiAssetFilter extends Enum {
    readonly isDefinite: boolean;
    readonly asDefinite: XcmV1MultiassetMultiAssets;
    readonly isWild: boolean;
    readonly asWild: XcmV1MultiassetWildMultiAsset;
    readonly type: 'Definite' | 'Wild';
  }

  /** @name XcmV1MultiassetWildMultiAsset (95) */
  interface XcmV1MultiassetWildMultiAsset extends Enum {
    readonly isAll: boolean;
    readonly isAllOf: boolean;
    readonly asAllOf: {
      readonly id: XcmV1MultiassetAssetId;
      readonly fun: XcmV1MultiassetWildFungibility;
    } & Struct;
    readonly type: 'All' | 'AllOf';
  }

  /** @name XcmV1MultiassetWildFungibility (96) */
  interface XcmV1MultiassetWildFungibility extends Enum {
    readonly isFungible: boolean;
    readonly isNonFungible: boolean;
    readonly type: 'Fungible' | 'NonFungible';
  }

  /** @name XcmV2WeightLimit (97) */
  interface XcmV2WeightLimit extends Enum {
    readonly isUnlimited: boolean;
    readonly isLimited: boolean;
    readonly asLimited: Compact<u64>;
    readonly type: 'Unlimited' | 'Limited';
  }

  /** @name XcmVersionedMultiAssets (99) */
  interface XcmVersionedMultiAssets extends Enum {
    readonly isV0: boolean;
    readonly asV0: Vec<XcmV0MultiAsset>;
    readonly isV1: boolean;
    readonly asV1: XcmV1MultiassetMultiAssets;
    readonly type: 'V0' | 'V1';
  }

  /** @name XcmV0MultiAsset (101) */
  interface XcmV0MultiAsset extends Enum {
    readonly isNone: boolean;
    readonly isAll: boolean;
    readonly isAllFungible: boolean;
    readonly isAllNonFungible: boolean;
    readonly isAllAbstractFungible: boolean;
    readonly asAllAbstractFungible: {
      readonly id: Bytes;
    } & Struct;
    readonly isAllAbstractNonFungible: boolean;
    readonly asAllAbstractNonFungible: {
      readonly class: Bytes;
    } & Struct;
    readonly isAllConcreteFungible: boolean;
    readonly asAllConcreteFungible: {
      readonly id: XcmV0MultiLocation;
    } & Struct;
    readonly isAllConcreteNonFungible: boolean;
    readonly asAllConcreteNonFungible: {
      readonly class: XcmV0MultiLocation;
    } & Struct;
    readonly isAbstractFungible: boolean;
    readonly asAbstractFungible: {
      readonly id: Bytes;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isAbstractNonFungible: boolean;
    readonly asAbstractNonFungible: {
      readonly class: Bytes;
      readonly instance: XcmV1MultiassetAssetInstance;
    } & Struct;
    readonly isConcreteFungible: boolean;
    readonly asConcreteFungible: {
      readonly id: XcmV0MultiLocation;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isConcreteNonFungible: boolean;
    readonly asConcreteNonFungible: {
      readonly class: XcmV0MultiLocation;
      readonly instance: XcmV1MultiassetAssetInstance;
    } & Struct;
    readonly type: 'None' | 'All' | 'AllFungible' | 'AllNonFungible' | 'AllAbstractFungible' | 'AllAbstractNonFungible' | 'AllConcreteFungible' | 'AllConcreteNonFungible' | 'AbstractFungible' | 'AbstractNonFungible' | 'ConcreteFungible' | 'ConcreteNonFungible';
  }

  /** @name XcmV0MultiLocation (102) */
  interface XcmV0MultiLocation extends Enum {
    readonly isNull: boolean;
    readonly isX1: boolean;
    readonly asX1: XcmV0Junction;
    readonly isX2: boolean;
    readonly asX2: ITuple<[XcmV0Junction, XcmV0Junction]>;
    readonly isX3: boolean;
    readonly asX3: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX4: boolean;
    readonly asX4: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX5: boolean;
    readonly asX5: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX6: boolean;
    readonly asX6: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX7: boolean;
    readonly asX7: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly isX8: boolean;
    readonly asX8: ITuple<[XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction, XcmV0Junction]>;
    readonly type: 'Null' | 'X1' | 'X2' | 'X3' | 'X4' | 'X5' | 'X6' | 'X7' | 'X8';
  }

  /** @name XcmV0Junction (103) */
  interface XcmV0Junction extends Enum {
    readonly isParent: boolean;
    readonly isParachain: boolean;
    readonly asParachain: Compact<u32>;
    readonly isAccountId32: boolean;
    readonly asAccountId32: {
      readonly network: XcmV0JunctionNetworkId;
      readonly id: U8aFixed;
    } & Struct;
    readonly isAccountIndex64: boolean;
    readonly asAccountIndex64: {
      readonly network: XcmV0JunctionNetworkId;
      readonly index: Compact<u64>;
    } & Struct;
    readonly isAccountKey20: boolean;
    readonly asAccountKey20: {
      readonly network: XcmV0JunctionNetworkId;
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
      readonly id: XcmV0JunctionBodyId;
      readonly part: XcmV0JunctionBodyPart;
    } & Struct;
    readonly type: 'Parent' | 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality';
  }

  /** @name XcmVersionedMultiLocation (104) */
  interface XcmVersionedMultiLocation extends Enum {
    readonly isV0: boolean;
    readonly asV0: XcmV0MultiLocation;
    readonly isV1: boolean;
    readonly asV1: XcmV1MultiLocation;
    readonly type: 'V0' | 'V1';
  }

  /** @name CumulusPalletXcmEvent (105) */
  interface CumulusPalletXcmEvent extends Enum {
    readonly isInvalidFormat: boolean;
    readonly asInvalidFormat: U8aFixed;
    readonly isUnsupportedVersion: boolean;
    readonly asUnsupportedVersion: U8aFixed;
    readonly isExecutedDownward: boolean;
    readonly asExecutedDownward: ITuple<[U8aFixed, XcmV2TraitsOutcome]>;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward';
  }

  /** @name CumulusPalletDmpQueueEvent (106) */
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
      readonly outcome: XcmV2TraitsOutcome;
    } & Struct;
    readonly isWeightExhausted: boolean;
    readonly asWeightExhausted: {
      readonly messageId: U8aFixed;
      readonly remainingWeight: u64;
      readonly requiredWeight: u64;
    } & Struct;
    readonly isOverweightEnqueued: boolean;
    readonly asOverweightEnqueued: {
      readonly messageId: U8aFixed;
      readonly overweightIndex: u64;
      readonly requiredWeight: u64;
    } & Struct;
    readonly isOverweightServiced: boolean;
    readonly asOverweightServiced: {
      readonly overweightIndex: u64;
      readonly weightUsed: u64;
    } & Struct;
    readonly type: 'InvalidFormat' | 'UnsupportedVersion' | 'ExecutedDownward' | 'WeightExhausted' | 'OverweightEnqueued' | 'OverweightServiced';
  }

  /** @name OrmlXtokensModuleEvent (107) */
  interface OrmlXtokensModuleEvent extends Enum {
    readonly isTransferredMultiAssets: boolean;
    readonly asTransferredMultiAssets: {
      readonly sender: AccountId32;
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly fee: XcmV1MultiAsset;
      readonly dest: XcmV1MultiLocation;
    } & Struct;
    readonly type: 'TransferredMultiAssets';
  }

  /** @name OrmlUnknownTokensModuleEvent (108) */
  interface OrmlUnknownTokensModuleEvent extends Enum {
    readonly isDeposited: boolean;
    readonly asDeposited: {
      readonly asset: XcmV1MultiAsset;
      readonly who: XcmV1MultiLocation;
    } & Struct;
    readonly isWithdrawn: boolean;
    readonly asWithdrawn: {
      readonly asset: XcmV1MultiAsset;
      readonly who: XcmV1MultiLocation;
    } & Struct;
    readonly type: 'Deposited' | 'Withdrawn';
  }

  /** @name OrmlTokensModuleEvent (109) */
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
    readonly type: 'Endowed' | 'DustLost' | 'Transfer' | 'Reserved' | 'Unreserved' | 'ReserveRepatriated' | 'BalanceSet' | 'TotalIssuanceSet' | 'Withdrawn' | 'Slashed' | 'Deposited' | 'LockSet' | 'LockRemoved';
  }

  /** @name PalletOracleEvent (111) */
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
    readonly type: 'AssetInfoChange' | 'SignerSet' | 'StakeAdded' | 'StakeRemoved' | 'StakeReclaimed' | 'PriceSubmitted' | 'UserSlashed' | 'OracleRewarded' | 'RewardingAdjustment' | 'AnswerPruned' | 'PriceChanged';
  }

  /** @name PalletCurrencyFactoryEvent (113) */
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

  /** @name PalletVaultEvent (115) */
  interface PalletVaultEvent extends Enum {
    readonly isVaultCreated: boolean;
    readonly asVaultCreated: {
      readonly id: u64;
    } & Struct;
    readonly isDeposited: boolean;
    readonly asDeposited: {
      readonly account: AccountId32;
      readonly assetAmount: u128;
      readonly lpAmount: u128;
    } & Struct;
    readonly isLiquidateStrategy: boolean;
    readonly asLiquidateStrategy: {
      readonly account: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isWithdrawn: boolean;
    readonly asWithdrawn: {
      readonly account: AccountId32;
      readonly lpAmount: u128;
      readonly assetAmount: u128;
    } & Struct;
    readonly isEmergencyShutdown: boolean;
    readonly asEmergencyShutdown: {
      readonly vault: u64;
    } & Struct;
    readonly isVaultStarted: boolean;
    readonly asVaultStarted: {
      readonly vault: u64;
    } & Struct;
    readonly type: 'VaultCreated' | 'Deposited' | 'LiquidateStrategy' | 'Withdrawn' | 'EmergencyShutdown' | 'VaultStarted';
  }

  /** @name PalletAssetsRegistryEvent (116) */
  interface PalletAssetsRegistryEvent extends Enum {
    readonly isAssetRegistered: boolean;
    readonly asAssetRegistered: {
      readonly assetId: u128;
      readonly location: ComposableTraitsXcmAssetsXcmAssetLocation;
    } & Struct;
    readonly isAssetUpdated: boolean;
    readonly asAssetUpdated: {
      readonly assetId: u128;
      readonly location: ComposableTraitsXcmAssetsXcmAssetLocation;
    } & Struct;
    readonly isMinFeeUpdated: boolean;
    readonly asMinFeeUpdated: {
      readonly targetParachainId: u32;
      readonly foreignAssetId: ComposableTraitsXcmAssetsXcmAssetLocation;
      readonly amount: Option<u128>;
    } & Struct;
    readonly type: 'AssetRegistered' | 'AssetUpdated' | 'MinFeeUpdated';
  }

  /** @name ComposableTraitsXcmAssetsXcmAssetLocation (117) */
  interface ComposableTraitsXcmAssetsXcmAssetLocation extends XcmV1MultiLocation {}

  /** @name PalletGovernanceRegistryEvent (119) */
  interface PalletGovernanceRegistryEvent extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly assetId: u128;
      readonly value: AccountId32;
    } & Struct;
    readonly isGrantRoot: boolean;
    readonly asGrantRoot: {
      readonly assetId: u128;
    } & Struct;
    readonly isRemove: boolean;
    readonly asRemove: {
      readonly assetId: u128;
    } & Struct;
    readonly type: 'Set' | 'GrantRoot' | 'Remove';
  }

  /** @name PalletCrowdloanRewardsEvent (120) */
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
    readonly type: 'Initialized' | 'Claimed' | 'Associated' | 'OverFunded';
  }

  /** @name PalletCrowdloanRewardsModelsRemoteAccount (121) */
  interface PalletCrowdloanRewardsModelsRemoteAccount extends Enum {
    readonly isRelayChain: boolean;
    readonly asRelayChain: AccountId32;
    readonly isEthereum: boolean;
    readonly asEthereum: ComposableSupportEthereumAddress;
    readonly type: 'RelayChain' | 'Ethereum';
  }

  /** @name ComposableSupportEthereumAddress (122) */
  interface ComposableSupportEthereumAddress extends U8aFixed {}

  /** @name PalletVestingModuleEvent (123) */
  interface PalletVestingModuleEvent extends Enum {
    readonly isVestingScheduleAdded: boolean;
    readonly asVestingScheduleAdded: {
      readonly from: AccountId32;
      readonly to: AccountId32;
      readonly asset: u128;
      readonly vestingScheduleId: u128;
      readonly schedule: ComposableTraitsVestingVestingSchedule;
      readonly scheduleAmount: u128;
    } & Struct;
    readonly isClaimed: boolean;
    readonly asClaimed: {
      readonly who: AccountId32;
      readonly asset: u128;
      readonly vestingScheduleIds: ComposableTraitsVestingVestingScheduleIdSet;
      readonly lockedAmount: u128;
      readonly claimedAmountPerSchedule: BTreeMap<u128, u128>;
    } & Struct;
    readonly isVestingSchedulesUpdated: boolean;
    readonly asVestingSchedulesUpdated: {
      readonly who: AccountId32;
    } & Struct;
    readonly type: 'VestingScheduleAdded' | 'Claimed' | 'VestingSchedulesUpdated';
  }

  /** @name ComposableTraitsVestingVestingSchedule (124) */
  interface ComposableTraitsVestingVestingSchedule extends Struct {
    readonly vestingScheduleId: u128;
    readonly window: ComposableTraitsVestingVestingWindow;
    readonly periodCount: u32;
    readonly perPeriod: Compact<u128>;
    readonly alreadyClaimed: u128;
  }

  /** @name ComposableTraitsVestingVestingWindow (125) */
  interface ComposableTraitsVestingVestingWindow extends Enum {
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

  /** @name ComposableTraitsVestingVestingScheduleIdSet (126) */
  interface ComposableTraitsVestingVestingScheduleIdSet extends Enum {
    readonly isAll: boolean;
    readonly isOne: boolean;
    readonly asOne: u128;
    readonly isMany: boolean;
    readonly asMany: Vec<u128>;
    readonly type: 'All' | 'One' | 'Many';
  }

  /** @name PalletBondedFinanceEvent (133) */
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

  /** @name PalletDutchAuctionEvent (134) */
  interface PalletDutchAuctionEvent extends Enum {
    readonly isOrderAdded: boolean;
    readonly asOrderAdded: {
      readonly orderId: u128;
      readonly order: PalletDutchAuctionSellOrder;
    } & Struct;
    readonly isOrderTaken: boolean;
    readonly asOrderTaken: {
      readonly orderId: u128;
      readonly taken: u128;
    } & Struct;
    readonly isOrderRemoved: boolean;
    readonly asOrderRemoved: {
      readonly orderId: u128;
    } & Struct;
    readonly isConfigurationAdded: boolean;
    readonly asConfigurationAdded: {
      readonly configurationId: u128;
      readonly configuration: ComposableTraitsTimeTimeReleaseFunction;
    } & Struct;
    readonly type: 'OrderAdded' | 'OrderTaken' | 'OrderRemoved' | 'ConfigurationAdded';
  }

  /** @name PalletDutchAuctionSellOrder (135) */
  interface PalletDutchAuctionSellOrder extends Struct {
    readonly fromTo: AccountId32;
    readonly order: ComposableTraitsDefiSellCurrencyId;
    readonly configuration: ComposableTraitsTimeTimeReleaseFunction;
    readonly context: PalletDutchAuctionEdContext;
    readonly totalAmountReceived: u128;
  }

  /** @name PalletDutchAuctionEdContext (136) */
  interface PalletDutchAuctionEdContext extends Struct {
    readonly addedAt: u64;
    readonly deposit: u128;
  }

  /** @name ComposableTraitsTimeTimeReleaseFunction (137) */
  interface ComposableTraitsTimeTimeReleaseFunction extends Enum {
    readonly isLinearDecrease: boolean;
    readonly asLinearDecrease: ComposableTraitsTimeLinearDecrease;
    readonly isStairstepExponentialDecrease: boolean;
    readonly asStairstepExponentialDecrease: ComposableTraitsTimeStairstepExponentialDecrease;
    readonly type: 'LinearDecrease' | 'StairstepExponentialDecrease';
  }

  /** @name ComposableTraitsTimeLinearDecrease (138) */
  interface ComposableTraitsTimeLinearDecrease extends Struct {
    readonly total: u64;
  }

  /** @name ComposableTraitsTimeStairstepExponentialDecrease (139) */
  interface ComposableTraitsTimeStairstepExponentialDecrease extends Struct {
    readonly step: u64;
    readonly cut: Permill;
  }

  /** @name ComposableTraitsDefiSellCurrencyId (141) */
  interface ComposableTraitsDefiSellCurrencyId extends Struct {
    readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly take: ComposableTraitsDefiTake;
  }

  /** @name ComposableTraitsDefiCurrencyPairCurrencyId (142) */
  interface ComposableTraitsDefiCurrencyPairCurrencyId extends Struct {
    readonly base: u128;
    readonly quote: u128;
  }

  /** @name ComposableTraitsDefiTake (143) */
  interface ComposableTraitsDefiTake extends Struct {
    readonly amount: u128;
    readonly limit: u128;
  }

  /** @name PalletMosaicEvent (145) */
  interface PalletMosaicEvent extends Enum {
    readonly isRelayerSet: boolean;
    readonly asRelayerSet: {
      readonly relayer: AccountId32;
    } & Struct;
    readonly isRelayerRotated: boolean;
    readonly asRelayerRotated: {
      readonly ttl: u32;
      readonly accountId: AccountId32;
    } & Struct;
    readonly isBudgetUpdated: boolean;
    readonly asBudgetUpdated: {
      readonly assetId: u128;
      readonly amount: u128;
      readonly decay: PalletMosaicDecayBudgetPenaltyDecayer;
    } & Struct;
    readonly isNetworksUpdated: boolean;
    readonly asNetworksUpdated: {
      readonly networkId: u32;
      readonly networkInfo: PalletMosaicNetworkInfo;
    } & Struct;
    readonly isTransferOut: boolean;
    readonly asTransferOut: {
      readonly id: H256;
      readonly to: ComposableSupportEthereumAddress;
      readonly assetId: u128;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
      readonly amount: u128;
      readonly swapToNative: bool;
      readonly sourceUserAccount: AccountId32;
      readonly ammSwapInfo: Option<PalletMosaicAmmSwapInfo>;
      readonly minimumAmountOut: u128;
    } & Struct;
    readonly isStaleTxClaimed: boolean;
    readonly asStaleTxClaimed: {
      readonly to: AccountId32;
      readonly by: AccountId32;
      readonly assetId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isTransferInto: boolean;
    readonly asTransferInto: {
      readonly id: H256;
      readonly to: AccountId32;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
      readonly assetId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isTransferIntoRescined: boolean;
    readonly asTransferIntoRescined: {
      readonly account: AccountId32;
      readonly amount: u128;
      readonly assetId: u128;
    } & Struct;
    readonly isPartialTransferAccepted: boolean;
    readonly asPartialTransferAccepted: {
      readonly from: AccountId32;
      readonly assetId: u128;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
      readonly amount: u128;
    } & Struct;
    readonly isTransferAccepted: boolean;
    readonly asTransferAccepted: {
      readonly from: AccountId32;
      readonly assetId: u128;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
      readonly amount: u128;
    } & Struct;
    readonly isTransferClaimed: boolean;
    readonly asTransferClaimed: {
      readonly by: AccountId32;
      readonly to: AccountId32;
      readonly assetId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isAssetMappingCreated: boolean;
    readonly asAssetMappingCreated: {
      readonly assetId: u128;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
    } & Struct;
    readonly isAssetMappingUpdated: boolean;
    readonly asAssetMappingUpdated: {
      readonly assetId: u128;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
    } & Struct;
    readonly isAssetMappingDeleted: boolean;
    readonly asAssetMappingDeleted: {
      readonly assetId: u128;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
    } & Struct;
    readonly type: 'RelayerSet' | 'RelayerRotated' | 'BudgetUpdated' | 'NetworksUpdated' | 'TransferOut' | 'StaleTxClaimed' | 'TransferInto' | 'TransferIntoRescined' | 'PartialTransferAccepted' | 'TransferAccepted' | 'TransferClaimed' | 'AssetMappingCreated' | 'AssetMappingUpdated' | 'AssetMappingDeleted';
  }

  /** @name PalletMosaicDecayBudgetPenaltyDecayer (146) */
  interface PalletMosaicDecayBudgetPenaltyDecayer extends Enum {
    readonly isLinear: boolean;
    readonly asLinear: PalletMosaicDecayLinearDecay;
    readonly type: 'Linear';
  }

  /** @name PalletMosaicDecayLinearDecay (147) */
  interface PalletMosaicDecayLinearDecay extends Struct {
    readonly factor: u128;
  }

  /** @name PalletMosaicNetworkInfo (148) */
  interface PalletMosaicNetworkInfo extends Struct {
    readonly enabled: bool;
    readonly minTransferSize: u128;
    readonly maxTransferSize: u128;
  }

  /** @name CommonMosaicRemoteAssetId (149) */
  interface CommonMosaicRemoteAssetId extends Enum {
    readonly isEthereumTokenAddress: boolean;
    readonly asEthereumTokenAddress: U8aFixed;
    readonly type: 'EthereumTokenAddress';
  }

  /** @name PalletMosaicAmmSwapInfo (151) */
  interface PalletMosaicAmmSwapInfo extends Struct {
    readonly destinationTokenOutAddress: ComposableSupportEthereumAddress;
    readonly destinationAmm: PalletMosaicRemoteAmm;
    readonly minimumAmountOut: u128;
  }

  /** @name PalletMosaicRemoteAmm (152) */
  interface PalletMosaicRemoteAmm extends Struct {
    readonly networkId: u32;
    readonly ammId: u128;
  }

  /** @name PalletLiquidationsEvent (153) */
  interface PalletLiquidationsEvent extends Enum {
    readonly isPositionWasSentToLiquidation: boolean;
    readonly type: 'PositionWasSentToLiquidation';
  }

  /** @name PalletLendingEvent (154) */
  interface PalletLendingEvent extends Enum {
    readonly isMarketCreated: boolean;
    readonly asMarketCreated: {
      readonly marketId: u32;
      readonly vaultId: u64;
      readonly manager: AccountId32;
      readonly currencyPair: ComposableTraitsDefiCurrencyPairCurrencyId;
    } & Struct;
    readonly isMarketUpdated: boolean;
    readonly asMarketUpdated: {
      readonly marketId: u32;
      readonly input: ComposableTraitsLendingUpdateInput;
    } & Struct;
    readonly isCollateralDeposited: boolean;
    readonly asCollateralDeposited: {
      readonly sender: AccountId32;
      readonly marketId: u32;
      readonly amount: u128;
    } & Struct;
    readonly isCollateralWithdrawn: boolean;
    readonly asCollateralWithdrawn: {
      readonly sender: AccountId32;
      readonly marketId: u32;
      readonly amount: u128;
    } & Struct;
    readonly isBorrowed: boolean;
    readonly asBorrowed: {
      readonly sender: AccountId32;
      readonly marketId: u32;
      readonly amount: u128;
    } & Struct;
    readonly isBorrowRepaid: boolean;
    readonly asBorrowRepaid: {
      readonly sender: AccountId32;
      readonly marketId: u32;
      readonly beneficiary: AccountId32;
      readonly amount: u128;
    } & Struct;
    readonly isLiquidationInitiated: boolean;
    readonly asLiquidationInitiated: {
      readonly marketId: u32;
      readonly borrowers: Vec<AccountId32>;
    } & Struct;
    readonly isMayGoUnderCollateralizedSoon: boolean;
    readonly asMayGoUnderCollateralizedSoon: {
      readonly marketId: u32;
      readonly account: AccountId32;
    } & Struct;
    readonly type: 'MarketCreated' | 'MarketUpdated' | 'CollateralDeposited' | 'CollateralWithdrawn' | 'Borrowed' | 'BorrowRepaid' | 'LiquidationInitiated' | 'MayGoUnderCollateralizedSoon';
  }

  /** @name ComposableTraitsLendingUpdateInput (156) */
  interface ComposableTraitsLendingUpdateInput extends Struct {
    readonly collateralFactor: u128;
    readonly underCollateralizedWarnPercent: Percent;
    readonly liquidators: Vec<u32>;
    readonly maxPriceAge: u32;
  }

  /** @name PalletPabloEvent (158) */
  interface PalletPabloEvent extends Enum {
    readonly isPoolCreated: boolean;
    readonly asPoolCreated: {
      readonly poolId: u128;
      readonly owner: AccountId32;
      readonly assets: ComposableTraitsDefiCurrencyPairCurrencyId;
    } & Struct;
    readonly isPoolDeleted: boolean;
    readonly asPoolDeleted: {
      readonly poolId: u128;
      readonly baseAmount: u128;
      readonly quoteAmount: u128;
    } & Struct;
    readonly isLiquidityAdded: boolean;
    readonly asLiquidityAdded: {
      readonly who: AccountId32;
      readonly poolId: u128;
      readonly baseAmount: u128;
      readonly quoteAmount: u128;
      readonly mintedLp: u128;
    } & Struct;
    readonly isLiquidityRemoved: boolean;
    readonly asLiquidityRemoved: {
      readonly who: AccountId32;
      readonly poolId: u128;
      readonly baseAmount: u128;
      readonly quoteAmount: u128;
      readonly totalIssuance: u128;
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
    readonly type: 'PoolCreated' | 'PoolDeleted' | 'LiquidityAdded' | 'LiquidityRemoved' | 'Swapped' | 'TwapUpdated';
  }

  /** @name ComposableTraitsDexFee (159) */
  interface ComposableTraitsDexFee extends Struct {
    readonly fee: u128;
    readonly lpFee: u128;
    readonly ownerFee: u128;
    readonly protocolFee: u128;
    readonly assetId: u128;
  }

  /** @name PalletDexRouterEvent (163) */
  interface PalletDexRouterEvent extends Enum {
    readonly isRouteAdded: boolean;
    readonly asRouteAdded: {
      readonly xAssetId: u128;
      readonly yAssetId: u128;
      readonly route: Vec<u128>;
    } & Struct;
    readonly isRouteDeleted: boolean;
    readonly asRouteDeleted: {
      readonly xAssetId: u128;
      readonly yAssetId: u128;
      readonly route: Vec<u128>;
    } & Struct;
    readonly isRouteUpdated: boolean;
    readonly asRouteUpdated: {
      readonly xAssetId: u128;
      readonly yAssetId: u128;
      readonly oldRoute: Vec<u128>;
      readonly updatedRoute: Vec<u128>;
    } & Struct;
    readonly type: 'RouteAdded' | 'RouteDeleted' | 'RouteUpdated';
  }

  /** @name PalletFnftEvent (164) */
  interface PalletFnftEvent extends Enum {
    readonly isFinancialNftCollectionCreated: boolean;
    readonly asFinancialNftCollectionCreated: {
      readonly collectionId: u128;
      readonly who: AccountId32;
      readonly admin: AccountId32;
    } & Struct;
    readonly isFinancialNftCreated: boolean;
    readonly asFinancialNftCreated: {
      readonly collectionId: u128;
      readonly instanceId: u64;
    } & Struct;
    readonly isFinancialNftBurned: boolean;
    readonly asFinancialNftBurned: {
      readonly collectionId: u128;
      readonly instanceId: u64;
    } & Struct;
    readonly isFinancialNftTransferred: boolean;
    readonly asFinancialNftTransferred: {
      readonly collectionId: u128;
      readonly instanceId: u64;
      readonly to: AccountId32;
    } & Struct;
    readonly type: 'FinancialNftCollectionCreated' | 'FinancialNftCreated' | 'FinancialNftBurned' | 'FinancialNftTransferred';
  }

  /** @name PalletStakingRewardsEvent (165) */
  interface PalletStakingRewardsEvent extends Enum {
    readonly isRewardPoolCreated: boolean;
    readonly asRewardPoolCreated: {
      readonly poolId: u128;
      readonly owner: AccountId32;
      readonly endBlock: u32;
    } & Struct;
    readonly isStaked: boolean;
    readonly asStaked: {
      readonly poolId: u128;
      readonly owner: AccountId32;
      readonly amount: u128;
      readonly durationPreset: u64;
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
      readonly rewardMultiplier: u64;
      readonly keepAlive: bool;
    } & Struct;
    readonly isClaimed: boolean;
    readonly asClaimed: {
      readonly owner: AccountId32;
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
    } & Struct;
    readonly isStakeAmountExtended: boolean;
    readonly asStakeAmountExtended: {
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
      readonly amount: u128;
    } & Struct;
    readonly isUnstaked: boolean;
    readonly asUnstaked: {
      readonly owner: AccountId32;
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
      readonly slash: Option<u128>;
    } & Struct;
    readonly isSplitPosition: boolean;
    readonly asSplitPosition: {
      readonly positions: Vec<ITuple<[u128, u64, u128]>>;
    } & Struct;
    readonly isRewardTransferred: boolean;
    readonly asRewardTransferred: {
      readonly from: AccountId32;
      readonly poolId: u128;
      readonly rewardCurrency: u128;
      readonly rewardIncrement: u128;
    } & Struct;
    readonly isRewardAccumulationHookError: boolean;
    readonly asRewardAccumulationHookError: {
      readonly poolId: u128;
      readonly assetId: u128;
      readonly error: PalletStakingRewardsRewardAccumulationHookError;
    } & Struct;
    readonly isMaxRewardsAccumulated: boolean;
    readonly asMaxRewardsAccumulated: {
      readonly poolId: u128;
      readonly assetId: u128;
    } & Struct;
    readonly isRewardPoolUpdated: boolean;
    readonly asRewardPoolUpdated: {
      readonly poolId: u128;
    } & Struct;
    readonly isRewardsPotIncreased: boolean;
    readonly asRewardsPotIncreased: {
      readonly poolId: u128;
      readonly assetId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isUnstakeRewardSlashed: boolean;
    readonly asUnstakeRewardSlashed: {
      readonly poolId: u128;
      readonly owner: AccountId32;
      readonly fnftInstanceId: u64;
      readonly rewardAssetId: u128;
      readonly amountSlashed: u128;
    } & Struct;
    readonly type: 'RewardPoolCreated' | 'Staked' | 'Claimed' | 'StakeAmountExtended' | 'Unstaked' | 'SplitPosition' | 'RewardTransferred' | 'RewardAccumulationHookError' | 'MaxRewardsAccumulated' | 'RewardPoolUpdated' | 'RewardsPotIncreased' | 'UnstakeRewardSlashed';
  }

  /** @name PalletStakingRewardsRewardAccumulationHookError (169) */
  interface PalletStakingRewardsRewardAccumulationHookError extends Enum {
    readonly isBackToTheFuture: boolean;
    readonly isRewardsPotEmpty: boolean;
    readonly type: 'BackToTheFuture' | 'RewardsPotEmpty';
  }

  /** @name PalletCallFilterEvent (170) */
  interface PalletCallFilterEvent extends Enum {
    readonly isDisabled: boolean;
    readonly asDisabled: {
      readonly entry: ComposableTraitsCallFilterCallFilterEntry;
    } & Struct;
    readonly isEnabled: boolean;
    readonly asEnabled: {
      readonly entry: ComposableTraitsCallFilterCallFilterEntry;
    } & Struct;
    readonly type: 'Disabled' | 'Enabled';
  }

  /** @name ComposableTraitsCallFilterCallFilterEntry (171) */
  interface ComposableTraitsCallFilterCallFilterEntry extends Struct {
    readonly palletName: Bytes;
    readonly functionName: Bytes;
  }

  /** @name CommonMaxStringSize (172) */
  type CommonMaxStringSize = Null;

  /** @name PalletIbcPingEvent (174) */
  interface PalletIbcPingEvent extends Enum {
    readonly isPacketSent: boolean;
    readonly isChannelOpened: boolean;
    readonly asChannelOpened: {
      readonly channelId: Bytes;
      readonly portId: Bytes;
    } & Struct;
    readonly type: 'PacketSent' | 'ChannelOpened';
  }

  /** @name IbcTransferEvent (175) */
  interface IbcTransferEvent extends Enum {
    readonly isTokenTransferInitiated: boolean;
    readonly asTokenTransferInitiated: {
      readonly from: AccountId32;
      readonly to: Bytes;
      readonly amount: u128;
    } & Struct;
    readonly isChannelOpened: boolean;
    readonly asChannelOpened: {
      readonly channelId: Bytes;
      readonly portId: Bytes;
    } & Struct;
    readonly isPalletParamsUpdated: boolean;
    readonly asPalletParamsUpdated: {
      readonly sendEnabled: bool;
      readonly receiveEnabled: bool;
    } & Struct;
    readonly type: 'TokenTransferInitiated' | 'ChannelOpened' | 'PalletParamsUpdated';
  }

  /** @name PalletIbcEvent (176) */
  interface PalletIbcEvent extends Enum {
    readonly isIbcEvents: boolean;
    readonly asIbcEvents: {
      readonly events: Vec<PalletIbcEventsIbcEvent>;
    } & Struct;
    readonly isIbcErrors: boolean;
    readonly asIbcErrors: {
      readonly errors: Vec<PalletIbcErrorsIbcError>;
    } & Struct;
    readonly type: 'IbcEvents' | 'IbcErrors';
  }

  /** @name PalletIbcEventsIbcEvent (178) */
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
    readonly type: 'NewBlock' | 'CreateClient' | 'UpdateClient' | 'UpgradeClient' | 'ClientMisbehaviour' | 'OpenInitConnection' | 'OpenConfirmConnection' | 'OpenTryConnection' | 'OpenAckConnection' | 'OpenInitChannel' | 'OpenConfirmChannel' | 'OpenTryChannel' | 'OpenAckChannel' | 'CloseInitChannel' | 'CloseConfirmChannel' | 'ReceivePacket' | 'SendPacket' | 'AcknowledgePacket' | 'WriteAcknowledgement' | 'TimeoutPacket' | 'TimeoutOnClosePacket' | 'Empty' | 'ChainError' | 'AppModule';
  }

  /** @name PalletIbcErrorsIbcError (180) */
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

  /** @name PalletCosmwasmEvent (181) */
  interface PalletCosmwasmEvent extends Enum {
    readonly isUploaded: boolean;
    readonly asUploaded: {
      readonly codeHash: H256;
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
    readonly type: 'Uploaded' | 'Instantiated' | 'Executed' | 'ExecutionFailed' | 'Emitted';
  }

  /** @name PalletCosmwasmContractInfo (182) */
  interface PalletCosmwasmContractInfo extends Struct {
    readonly codeId: u64;
    readonly trieId: Bytes;
    readonly instantiator: AccountId32;
    readonly admin: Option<AccountId32>;
    readonly label: Bytes;
  }

  /** @name PalletCosmwasmEntryPoint (185) */
  interface PalletCosmwasmEntryPoint extends Enum {
    readonly isInstantiate: boolean;
    readonly isExecute: boolean;
    readonly isMigrate: boolean;
    readonly isReply: boolean;
    readonly isSudo: boolean;
    readonly isQuery: boolean;
    readonly type: 'Instantiate' | 'Execute' | 'Migrate' | 'Reply' | 'Sudo' | 'Query';
  }

  /** @name FrameSystemPhase (188) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (191) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (193) */
  interface FrameSystemCall extends Enum {
    readonly isFillBlock: boolean;
    readonly asFillBlock: {
      readonly ratio: Perbill;
    } & Struct;
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
    readonly type: 'FillBlock' | 'Remark' | 'SetHeapPages' | 'SetCode' | 'SetCodeWithoutChecks' | 'SetStorage' | 'KillStorage' | 'KillPrefix' | 'RemarkWithEvent';
  }

  /** @name FrameSystemLimitsBlockWeights (196) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (197) */
  interface FrameSupportWeightsPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (198) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (200) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (201) */
  interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (202) */
  interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (203) */
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

  /** @name FrameSystemError (207) */
  interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PalletTimestampCall (208) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name PalletSudoCall (209) */
  interface PalletSudoCall extends Enum {
    readonly isSudo: boolean;
    readonly asSudo: {
      readonly call: Call;
    } & Struct;
    readonly isSudoUncheckedWeight: boolean;
    readonly asSudoUncheckedWeight: {
      readonly call: Call;
      readonly weight: u64;
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

  /** @name PalletAssetTxPaymentCall (211) */
  interface PalletAssetTxPaymentCall extends Enum {
    readonly isSetPaymentAsset: boolean;
    readonly asSetPaymentAsset: {
      readonly payer: AccountId32;
      readonly assetId: Option<u128>;
    } & Struct;
    readonly type: 'SetPaymentAsset';
  }

  /** @name PalletIndicesCall (213) */
  interface PalletIndicesCall extends Enum {
    readonly isClaim: boolean;
    readonly asClaim: {
      readonly index: u32;
    } & Struct;
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly new_: AccountId32;
      readonly index: u32;
    } & Struct;
    readonly isFree: boolean;
    readonly asFree: {
      readonly index: u32;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly new_: AccountId32;
      readonly index: u32;
      readonly freeze: bool;
    } & Struct;
    readonly isFreeze: boolean;
    readonly asFreeze: {
      readonly index: u32;
    } & Struct;
    readonly type: 'Claim' | 'Transfer' | 'Free' | 'ForceTransfer' | 'Freeze';
  }

  /** @name PalletBalancesCall (214) */
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

  /** @name PalletIdentityCall (216) */
  interface PalletIdentityCall extends Enum {
    readonly isAddRegistrar: boolean;
    readonly asAddRegistrar: {
      readonly account: AccountId32;
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
      readonly new_: AccountId32;
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

  /** @name PalletIdentityIdentityInfo (217) */
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

  /** @name PalletIdentityBitFlags (253) */
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

  /** @name PalletIdentityIdentityField (254) */
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

  /** @name PalletIdentityJudgement (255) */
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

  /** @name PalletMultisigCall (256) */
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
      readonly call: WrapperKeepOpaque<Call>;
      readonly storeCall: bool;
      readonly maxWeight: u64;
    } & Struct;
    readonly isApproveAsMulti: boolean;
    readonly asApproveAsMulti: {
      readonly threshold: u16;
      readonly otherSignatories: Vec<AccountId32>;
      readonly maybeTimepoint: Option<PalletMultisigTimepoint>;
      readonly callHash: U8aFixed;
      readonly maxWeight: u64;
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

  /** @name CumulusPalletParachainSystemCall (259) */
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

  /** @name CumulusPrimitivesParachainInherentParachainInherentData (260) */
  interface CumulusPrimitivesParachainInherentParachainInherentData extends Struct {
    readonly validationData: PolkadotPrimitivesV2PersistedValidationData;
    readonly relayChainState: SpTrieStorageProof;
    readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
    readonly horizontalMessages: BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>;
  }

  /** @name PolkadotPrimitivesV2PersistedValidationData (261) */
  interface PolkadotPrimitivesV2PersistedValidationData extends Struct {
    readonly parentHead: Bytes;
    readonly relayParentNumber: u32;
    readonly relayParentStorageRoot: H256;
    readonly maxPovSize: u32;
  }

  /** @name SpTrieStorageProof (263) */
  interface SpTrieStorageProof extends Struct {
    readonly trieNodes: BTreeSet<Bytes>;
  }

  /** @name PolkadotCorePrimitivesInboundDownwardMessage (266) */
  interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
    readonly sentAt: u32;
    readonly msg: Bytes;
  }

  /** @name PolkadotCorePrimitivesInboundHrmpMessage (269) */
  interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
    readonly sentAt: u32;
    readonly data: Bytes;
  }

  /** @name ParachainInfoCall (272) */
  type ParachainInfoCall = Null;

  /** @name PalletAuthorshipCall (273) */
  interface PalletAuthorshipCall extends Enum {
    readonly isSetUncles: boolean;
    readonly asSetUncles: {
      readonly newUncles: Vec<SpRuntimeHeader>;
    } & Struct;
    readonly type: 'SetUncles';
  }

  /** @name SpRuntimeHeader (275) */
  interface SpRuntimeHeader extends Struct {
    readonly parentHash: H256;
    readonly number: Compact<u32>;
    readonly stateRoot: H256;
    readonly extrinsicsRoot: H256;
    readonly digest: SpRuntimeDigest;
  }

  /** @name SpRuntimeBlakeTwo256 (276) */
  type SpRuntimeBlakeTwo256 = Null;

  /** @name PalletCollatorSelectionCall (277) */
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

  /** @name PalletSessionCall (278) */
  interface PalletSessionCall extends Enum {
    readonly isSetKeys: boolean;
    readonly asSetKeys: {
      readonly keys_: DaliRuntimeOpaqueSessionKeys;
      readonly proof: Bytes;
    } & Struct;
    readonly isPurgeKeys: boolean;
    readonly type: 'SetKeys' | 'PurgeKeys';
  }

  /** @name DaliRuntimeOpaqueSessionKeys (279) */
  interface DaliRuntimeOpaqueSessionKeys extends Struct {
    readonly aura: SpConsensusAuraSr25519AppSr25519Public;
  }

  /** @name SpConsensusAuraSr25519AppSr25519Public (280) */
  interface SpConsensusAuraSr25519AppSr25519Public extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (281) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name PalletCollectiveCall (282) */
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
    readonly isClose: boolean;
    readonly asClose: {
      readonly proposalHash: H256;
      readonly index: Compact<u32>;
      readonly proposalWeightBound: Compact<u64>;
      readonly lengthBound: Compact<u32>;
    } & Struct;
    readonly isDisapproveProposal: boolean;
    readonly asDisapproveProposal: {
      readonly proposalHash: H256;
    } & Struct;
    readonly type: 'SetMembers' | 'Execute' | 'Propose' | 'Vote' | 'Close' | 'DisapproveProposal';
  }

  /** @name PalletMembershipCall (283) */
  interface PalletMembershipCall extends Enum {
    readonly isAddMember: boolean;
    readonly asAddMember: {
      readonly who: AccountId32;
    } & Struct;
    readonly isRemoveMember: boolean;
    readonly asRemoveMember: {
      readonly who: AccountId32;
    } & Struct;
    readonly isSwapMember: boolean;
    readonly asSwapMember: {
      readonly remove: AccountId32;
      readonly add: AccountId32;
    } & Struct;
    readonly isResetMembers: boolean;
    readonly asResetMembers: {
      readonly members: Vec<AccountId32>;
    } & Struct;
    readonly isChangeKey: boolean;
    readonly asChangeKey: {
      readonly new_: AccountId32;
    } & Struct;
    readonly isSetPrime: boolean;
    readonly asSetPrime: {
      readonly who: AccountId32;
    } & Struct;
    readonly isClearPrime: boolean;
    readonly type: 'AddMember' | 'RemoveMember' | 'SwapMember' | 'ResetMembers' | 'ChangeKey' | 'SetPrime' | 'ClearPrime';
  }

  /** @name PalletTreasuryCall (284) */
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

  /** @name PalletDemocracyCall (285) */
  interface PalletDemocracyCall extends Enum {
    readonly isPropose: boolean;
    readonly asPropose: {
      readonly proposalHash: H256;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isSecond: boolean;
    readonly asSecond: {
      readonly proposal: Compact<u32>;
      readonly secondsUpperBound: Compact<u32>;
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
      readonly proposalHash: H256;
    } & Struct;
    readonly isExternalProposeMajority: boolean;
    readonly asExternalProposeMajority: {
      readonly proposalHash: H256;
    } & Struct;
    readonly isExternalProposeDefault: boolean;
    readonly asExternalProposeDefault: {
      readonly proposalHash: H256;
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
    readonly isCancelQueued: boolean;
    readonly asCancelQueued: {
      readonly which: u32;
    } & Struct;
    readonly isDelegate: boolean;
    readonly asDelegate: {
      readonly to: AccountId32;
      readonly conviction: PalletDemocracyConviction;
      readonly balance: u128;
    } & Struct;
    readonly isUndelegate: boolean;
    readonly isClearPublicProposals: boolean;
    readonly isNotePreimage: boolean;
    readonly asNotePreimage: {
      readonly encodedProposal: Bytes;
    } & Struct;
    readonly isNotePreimageOperational: boolean;
    readonly asNotePreimageOperational: {
      readonly encodedProposal: Bytes;
    } & Struct;
    readonly isNoteImminentPreimage: boolean;
    readonly asNoteImminentPreimage: {
      readonly encodedProposal: Bytes;
    } & Struct;
    readonly isNoteImminentPreimageOperational: boolean;
    readonly asNoteImminentPreimageOperational: {
      readonly encodedProposal: Bytes;
    } & Struct;
    readonly isReapPreimage: boolean;
    readonly asReapPreimage: {
      readonly proposalHash: H256;
      readonly proposalLenUpperBound: Compact<u32>;
    } & Struct;
    readonly isUnlock: boolean;
    readonly asUnlock: {
      readonly target: AccountId32;
    } & Struct;
    readonly isRemoveVote: boolean;
    readonly asRemoveVote: {
      readonly index: u32;
    } & Struct;
    readonly isRemoveOtherVote: boolean;
    readonly asRemoveOtherVote: {
      readonly target: AccountId32;
      readonly index: u32;
    } & Struct;
    readonly isEnactProposal: boolean;
    readonly asEnactProposal: {
      readonly proposalHash: H256;
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
    readonly type: 'Propose' | 'Second' | 'Vote' | 'EmergencyCancel' | 'ExternalPropose' | 'ExternalProposeMajority' | 'ExternalProposeDefault' | 'FastTrack' | 'VetoExternal' | 'CancelReferendum' | 'CancelQueued' | 'Delegate' | 'Undelegate' | 'ClearPublicProposals' | 'NotePreimage' | 'NotePreimageOperational' | 'NoteImminentPreimage' | 'NoteImminentPreimageOperational' | 'ReapPreimage' | 'Unlock' | 'RemoveVote' | 'RemoveOtherVote' | 'EnactProposal' | 'Blacklist' | 'CancelProposal';
  }

  /** @name PalletDemocracyConviction (286) */
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

  /** @name PalletSchedulerCall (290) */
  interface PalletSchedulerCall extends Enum {
    readonly isSchedule: boolean;
    readonly asSchedule: {
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly isCancel: boolean;
    readonly asCancel: {
      readonly when: u32;
      readonly index: u32;
    } & Struct;
    readonly isScheduleNamed: boolean;
    readonly asScheduleNamed: {
      readonly id: Bytes;
      readonly when: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly isCancelNamed: boolean;
    readonly asCancelNamed: {
      readonly id: Bytes;
    } & Struct;
    readonly isScheduleAfter: boolean;
    readonly asScheduleAfter: {
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly isScheduleNamedAfter: boolean;
    readonly asScheduleNamedAfter: {
      readonly id: Bytes;
      readonly after: u32;
      readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
      readonly priority: u8;
      readonly call: FrameSupportScheduleMaybeHashed;
    } & Struct;
    readonly type: 'Schedule' | 'Cancel' | 'ScheduleNamed' | 'CancelNamed' | 'ScheduleAfter' | 'ScheduleNamedAfter';
  }

  /** @name FrameSupportScheduleMaybeHashed (292) */
  interface FrameSupportScheduleMaybeHashed extends Enum {
    readonly isValue: boolean;
    readonly asValue: Call;
    readonly isHash: boolean;
    readonly asHash: H256;
    readonly type: 'Value' | 'Hash';
  }

  /** @name PalletUtilityCall (293) */
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
      readonly asOrigin: DaliRuntimeOriginCaller;
      readonly call: Call;
    } & Struct;
    readonly isForceBatch: boolean;
    readonly asForceBatch: {
      readonly calls: Vec<Call>;
    } & Struct;
    readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch';
  }

  /** @name DaliRuntimeOriginCaller (295) */
  interface DaliRuntimeOriginCaller extends Enum {
    readonly isSystem: boolean;
    readonly asSystem: FrameSupportDispatchRawOrigin;
    readonly isVoid: boolean;
    readonly isCouncil: boolean;
    readonly asCouncil: PalletCollectiveRawOrigin;
    readonly isRelayerXcm: boolean;
    readonly asRelayerXcm: PalletXcmOrigin;
    readonly isCumulusXcm: boolean;
    readonly asCumulusXcm: CumulusPalletXcmOrigin;
    readonly isTechnicalCollective: boolean;
    readonly asTechnicalCollective: PalletCollectiveRawOrigin;
    readonly type: 'System' | 'Void' | 'Council' | 'RelayerXcm' | 'CumulusXcm' | 'TechnicalCollective';
  }

  /** @name FrameSupportDispatchRawOrigin (296) */
  interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: 'Root' | 'Signed' | 'None';
  }

  /** @name PalletCollectiveRawOrigin (297) */
  interface PalletCollectiveRawOrigin extends Enum {
    readonly isMembers: boolean;
    readonly asMembers: ITuple<[u32, u32]>;
    readonly isMember: boolean;
    readonly asMember: AccountId32;
    readonly isPhantom: boolean;
    readonly type: 'Members' | 'Member' | 'Phantom';
  }

  /** @name PalletXcmOrigin (299) */
  interface PalletXcmOrigin extends Enum {
    readonly isXcm: boolean;
    readonly asXcm: XcmV1MultiLocation;
    readonly isResponse: boolean;
    readonly asResponse: XcmV1MultiLocation;
    readonly type: 'Xcm' | 'Response';
  }

  /** @name CumulusPalletXcmOrigin (300) */
  interface CumulusPalletXcmOrigin extends Enum {
    readonly isRelay: boolean;
    readonly isSiblingParachain: boolean;
    readonly asSiblingParachain: u32;
    readonly type: 'Relay' | 'SiblingParachain';
  }

  /** @name SpCoreVoid (301) */
  type SpCoreVoid = Null;

  /** @name PalletPreimageCall (302) */
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

  /** @name PalletAccountProxyCall (303) */
  interface PalletAccountProxyCall extends Enum {
    readonly isProxy: boolean;
    readonly asProxy: {
      readonly real: AccountId32;
      readonly forceProxyType: Option<ComposableTraitsAccountProxyProxyType>;
      readonly call: Call;
    } & Struct;
    readonly isAddProxy: boolean;
    readonly asAddProxy: {
      readonly delegate: AccountId32;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
    } & Struct;
    readonly isRemoveProxy: boolean;
    readonly asRemoveProxy: {
      readonly delegate: AccountId32;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
    } & Struct;
    readonly isRemoveProxies: boolean;
    readonly isAnonymous: boolean;
    readonly asAnonymous: {
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly delay: u32;
      readonly index: u16;
    } & Struct;
    readonly isKillAnonymous: boolean;
    readonly asKillAnonymous: {
      readonly spawner: AccountId32;
      readonly proxyType: ComposableTraitsAccountProxyProxyType;
      readonly index: u16;
      readonly height: Compact<u32>;
      readonly extIndex: Compact<u32>;
    } & Struct;
    readonly isAnnounce: boolean;
    readonly asAnnounce: {
      readonly real: AccountId32;
      readonly callHash: H256;
    } & Struct;
    readonly isRemoveAnnouncement: boolean;
    readonly asRemoveAnnouncement: {
      readonly real: AccountId32;
      readonly callHash: H256;
    } & Struct;
    readonly isRejectAnnouncement: boolean;
    readonly asRejectAnnouncement: {
      readonly delegate: AccountId32;
      readonly callHash: H256;
    } & Struct;
    readonly isProxyAnnounced: boolean;
    readonly asProxyAnnounced: {
      readonly delegate: AccountId32;
      readonly real: AccountId32;
      readonly forceProxyType: Option<ComposableTraitsAccountProxyProxyType>;
      readonly call: Call;
    } & Struct;
    readonly type: 'Proxy' | 'AddProxy' | 'RemoveProxy' | 'RemoveProxies' | 'Anonymous' | 'KillAnonymous' | 'Announce' | 'RemoveAnnouncement' | 'RejectAnnouncement' | 'ProxyAnnounced';
  }

  /** @name CumulusPalletXcmpQueueCall (305) */
  interface CumulusPalletXcmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: u64;
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
      readonly new_: u64;
    } & Struct;
    readonly isUpdateWeightRestrictDecay: boolean;
    readonly asUpdateWeightRestrictDecay: {
      readonly new_: u64;
    } & Struct;
    readonly isUpdateXcmpMaxIndividualWeight: boolean;
    readonly asUpdateXcmpMaxIndividualWeight: {
      readonly new_: u64;
    } & Struct;
    readonly type: 'ServiceOverweight' | 'SuspendXcmExecution' | 'ResumeXcmExecution' | 'UpdateSuspendThreshold' | 'UpdateDropThreshold' | 'UpdateResumeThreshold' | 'UpdateThresholdWeight' | 'UpdateWeightRestrictDecay' | 'UpdateXcmpMaxIndividualWeight';
  }

  /** @name PalletXcmCall (306) */
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
      readonly maxWeight: u64;
    } & Struct;
    readonly isForceXcmVersion: boolean;
    readonly asForceXcmVersion: {
      readonly location: XcmV1MultiLocation;
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
      readonly weightLimit: XcmV2WeightLimit;
    } & Struct;
    readonly isLimitedTeleportAssets: boolean;
    readonly asLimitedTeleportAssets: {
      readonly dest: XcmVersionedMultiLocation;
      readonly beneficiary: XcmVersionedMultiLocation;
      readonly assets: XcmVersionedMultiAssets;
      readonly feeAssetItem: u32;
      readonly weightLimit: XcmV2WeightLimit;
    } & Struct;
    readonly type: 'Send' | 'TeleportAssets' | 'ReserveTransferAssets' | 'Execute' | 'ForceXcmVersion' | 'ForceDefaultXcmVersion' | 'ForceSubscribeVersionNotify' | 'ForceUnsubscribeVersionNotify' | 'LimitedReserveTransferAssets' | 'LimitedTeleportAssets';
  }

  /** @name XcmVersionedXcm (307) */
  interface XcmVersionedXcm extends Enum {
    readonly isV0: boolean;
    readonly asV0: XcmV0Xcm;
    readonly isV1: boolean;
    readonly asV1: XcmV1Xcm;
    readonly isV2: boolean;
    readonly asV2: XcmV2Xcm;
    readonly type: 'V0' | 'V1' | 'V2';
  }

  /** @name XcmV0Xcm (308) */
  interface XcmV0Xcm extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isReserveAssetDeposit: boolean;
    readonly asReserveAssetDeposit: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isTeleportAsset: boolean;
    readonly asTeleportAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV0Response;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV0OriginKind;
      readonly requireWeightAtMost: u64;
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
    readonly isRelayedFrom: boolean;
    readonly asRelayedFrom: {
      readonly who: XcmV0MultiLocation;
      readonly message: XcmV0Xcm;
    } & Struct;
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposit' | 'TeleportAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'RelayedFrom';
  }

  /** @name XcmV0Order (310) */
  interface XcmV0Order extends Enum {
    readonly isNull: boolean;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: Vec<XcmV0MultiAsset>;
      readonly receive: Vec<XcmV0MultiAsset>;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly reserve: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: Vec<XcmV0MultiAsset>;
      readonly dest: XcmV0MultiLocation;
      readonly effects: Vec<XcmV0Order>;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV0MultiLocation;
      readonly assets: Vec<XcmV0MultiAsset>;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV0MultiAsset;
      readonly weight: u64;
      readonly debt: u64;
      readonly haltOnError: bool;
      readonly xcm: Vec<XcmV0Xcm>;
    } & Struct;
    readonly type: 'Null' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'QueryHolding' | 'BuyExecution';
  }

  /** @name XcmV0Response (312) */
  interface XcmV0Response extends Enum {
    readonly isAssets: boolean;
    readonly asAssets: Vec<XcmV0MultiAsset>;
    readonly type: 'Assets';
  }

  /** @name XcmV1Xcm (313) */
  interface XcmV1Xcm extends Enum {
    readonly isWithdrawAsset: boolean;
    readonly asWithdrawAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isReserveAssetDeposited: boolean;
    readonly asReserveAssetDeposited: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isReceiveTeleportedAsset: boolean;
    readonly asReceiveTeleportedAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isQueryResponse: boolean;
    readonly asQueryResponse: {
      readonly queryId: Compact<u64>;
      readonly response: XcmV1Response;
    } & Struct;
    readonly isTransferAsset: boolean;
    readonly asTransferAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly beneficiary: XcmV1MultiLocation;
    } & Struct;
    readonly isTransferReserveAsset: boolean;
    readonly asTransferReserveAsset: {
      readonly assets: XcmV1MultiassetMultiAssets;
      readonly dest: XcmV1MultiLocation;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isTransact: boolean;
    readonly asTransact: {
      readonly originType: XcmV0OriginKind;
      readonly requireWeightAtMost: u64;
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
    readonly isRelayedFrom: boolean;
    readonly asRelayedFrom: {
      readonly who: XcmV1MultilocationJunctions;
      readonly message: XcmV1Xcm;
    } & Struct;
    readonly isSubscribeVersion: boolean;
    readonly asSubscribeVersion: {
      readonly queryId: Compact<u64>;
      readonly maxResponseWeight: Compact<u64>;
    } & Struct;
    readonly isUnsubscribeVersion: boolean;
    readonly type: 'WithdrawAsset' | 'ReserveAssetDeposited' | 'ReceiveTeleportedAsset' | 'QueryResponse' | 'TransferAsset' | 'TransferReserveAsset' | 'Transact' | 'HrmpNewChannelOpenRequest' | 'HrmpChannelAccepted' | 'HrmpChannelClosing' | 'RelayedFrom' | 'SubscribeVersion' | 'UnsubscribeVersion';
  }

  /** @name XcmV1Order (315) */
  interface XcmV1Order extends Enum {
    readonly isNoop: boolean;
    readonly isDepositAsset: boolean;
    readonly asDepositAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: u32;
      readonly beneficiary: XcmV1MultiLocation;
    } & Struct;
    readonly isDepositReserveAsset: boolean;
    readonly asDepositReserveAsset: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly maxAssets: u32;
      readonly dest: XcmV1MultiLocation;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isExchangeAsset: boolean;
    readonly asExchangeAsset: {
      readonly give: XcmV1MultiassetMultiAssetFilter;
      readonly receive: XcmV1MultiassetMultiAssets;
    } & Struct;
    readonly isInitiateReserveWithdraw: boolean;
    readonly asInitiateReserveWithdraw: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly reserve: XcmV1MultiLocation;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isInitiateTeleport: boolean;
    readonly asInitiateTeleport: {
      readonly assets: XcmV1MultiassetMultiAssetFilter;
      readonly dest: XcmV1MultiLocation;
      readonly effects: Vec<XcmV1Order>;
    } & Struct;
    readonly isQueryHolding: boolean;
    readonly asQueryHolding: {
      readonly queryId: Compact<u64>;
      readonly dest: XcmV1MultiLocation;
      readonly assets: XcmV1MultiassetMultiAssetFilter;
    } & Struct;
    readonly isBuyExecution: boolean;
    readonly asBuyExecution: {
      readonly fees: XcmV1MultiAsset;
      readonly weight: u64;
      readonly debt: u64;
      readonly haltOnError: bool;
      readonly instructions: Vec<XcmV1Xcm>;
    } & Struct;
    readonly type: 'Noop' | 'DepositAsset' | 'DepositReserveAsset' | 'ExchangeAsset' | 'InitiateReserveWithdraw' | 'InitiateTeleport' | 'QueryHolding' | 'BuyExecution';
  }

  /** @name XcmV1Response (317) */
  interface XcmV1Response extends Enum {
    readonly isAssets: boolean;
    readonly asAssets: XcmV1MultiassetMultiAssets;
    readonly isVersion: boolean;
    readonly asVersion: u32;
    readonly type: 'Assets' | 'Version';
  }

  /** @name CumulusPalletXcmCall (331) */
  type CumulusPalletXcmCall = Null;

  /** @name CumulusPalletDmpQueueCall (332) */
  interface CumulusPalletDmpQueueCall extends Enum {
    readonly isServiceOverweight: boolean;
    readonly asServiceOverweight: {
      readonly index: u64;
      readonly weightLimit: u64;
    } & Struct;
    readonly type: 'ServiceOverweight';
  }

  /** @name OrmlXtokensModuleCall (333) */
  interface OrmlXtokensModuleCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly currencyId: u128;
      readonly amount: u128;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeight: u64;
    } & Struct;
    readonly isTransferMultiasset: boolean;
    readonly asTransferMultiasset: {
      readonly asset: XcmVersionedMultiAsset;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeight: u64;
    } & Struct;
    readonly isTransferWithFee: boolean;
    readonly asTransferWithFee: {
      readonly currencyId: u128;
      readonly amount: u128;
      readonly fee: u128;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeight: u64;
    } & Struct;
    readonly isTransferMultiassetWithFee: boolean;
    readonly asTransferMultiassetWithFee: {
      readonly asset: XcmVersionedMultiAsset;
      readonly fee: XcmVersionedMultiAsset;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeight: u64;
    } & Struct;
    readonly isTransferMulticurrencies: boolean;
    readonly asTransferMulticurrencies: {
      readonly currencies: Vec<ITuple<[u128, u128]>>;
      readonly feeItem: u32;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeight: u64;
    } & Struct;
    readonly isTransferMultiassets: boolean;
    readonly asTransferMultiassets: {
      readonly assets: XcmVersionedMultiAssets;
      readonly feeItem: u32;
      readonly dest: XcmVersionedMultiLocation;
      readonly destWeight: u64;
    } & Struct;
    readonly type: 'Transfer' | 'TransferMultiasset' | 'TransferWithFee' | 'TransferMultiassetWithFee' | 'TransferMulticurrencies' | 'TransferMultiassets';
  }

  /** @name XcmVersionedMultiAsset (334) */
  interface XcmVersionedMultiAsset extends Enum {
    readonly isV0: boolean;
    readonly asV0: XcmV0MultiAsset;
    readonly isV1: boolean;
    readonly asV1: XcmV1MultiAsset;
    readonly type: 'V0' | 'V1';
  }

  /** @name OrmlUnknownTokensModuleCall (337) */
  type OrmlUnknownTokensModuleCall = Null;

  /** @name OrmlTokensModuleCall (338) */
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

  /** @name PalletOracleCall (339) */
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
    readonly type: 'AddAssetAndInfo' | 'SetSigner' | 'AdjustRewards' | 'AddStake' | 'RemoveStake' | 'ReclaimStake' | 'SubmitPrice';
  }

  /** @name PalletCurrencyFactoryCall (340) */
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

  /** @name ComposableTraitsAssetsBasicAssetMetadata (341) */
  interface ComposableTraitsAssetsBasicAssetMetadata extends Struct {
    readonly symbol: ComposableSupportCollectionsVecBoundedBiBoundedVec;
    readonly name: ComposableSupportCollectionsVecBoundedBiBoundedVec;
  }

  /** @name ComposableSupportCollectionsVecBoundedBiBoundedVec (342) */
  interface ComposableSupportCollectionsVecBoundedBiBoundedVec extends Struct {
    readonly inner: Bytes;
  }

  /** @name PalletVaultCall (344) */
  interface PalletVaultCall extends Enum {
    readonly isCreate: boolean;
    readonly asCreate: {
      readonly vault: ComposableTraitsVaultVaultConfig;
      readonly depositAmount: u128;
    } & Struct;
    readonly isClaimSurcharge: boolean;
    readonly asClaimSurcharge: {
      readonly dest: u64;
      readonly address: Option<AccountId32>;
    } & Struct;
    readonly isAddSurcharge: boolean;
    readonly asAddSurcharge: {
      readonly dest: u64;
      readonly amount: u128;
    } & Struct;
    readonly isDeleteTombstoned: boolean;
    readonly asDeleteTombstoned: {
      readonly dest: u64;
      readonly address: Option<AccountId32>;
    } & Struct;
    readonly isDeposit: boolean;
    readonly asDeposit: {
      readonly vault: u64;
      readonly assetAmount: u128;
    } & Struct;
    readonly isWithdraw: boolean;
    readonly asWithdraw: {
      readonly vault: u64;
      readonly lpAmount: u128;
    } & Struct;
    readonly isEmergencyShutdown: boolean;
    readonly asEmergencyShutdown: {
      readonly vault: u64;
    } & Struct;
    readonly isStart: boolean;
    readonly asStart: {
      readonly vault: u64;
    } & Struct;
    readonly isLiquidateStrategy: boolean;
    readonly asLiquidateStrategy: {
      readonly vaultIdx: u64;
      readonly strategyAccountId: AccountId32;
    } & Struct;
    readonly type: 'Create' | 'ClaimSurcharge' | 'AddSurcharge' | 'DeleteTombstoned' | 'Deposit' | 'Withdraw' | 'EmergencyShutdown' | 'Start' | 'LiquidateStrategy';
  }

  /** @name ComposableTraitsVaultVaultConfig (345) */
  interface ComposableTraitsVaultVaultConfig extends Struct {
    readonly assetId: u128;
    readonly reserved: Perquintill;
    readonly manager: AccountId32;
    readonly strategies: BTreeMap<AccountId32, Perquintill>;
  }

  /** @name PalletAssetsRegistryCall (350) */
  interface PalletAssetsRegistryCall extends Enum {
    readonly isRegisterAsset: boolean;
    readonly asRegisterAsset: {
      readonly location: ComposableTraitsXcmAssetsXcmAssetLocation;
      readonly ed: u128;
      readonly ratio: Option<u128>;
      readonly decimals: Option<u32>;
    } & Struct;
    readonly isUpdateAsset: boolean;
    readonly asUpdateAsset: {
      readonly assetId: u128;
      readonly location: ComposableTraitsXcmAssetsXcmAssetLocation;
      readonly ratio: Option<u128>;
      readonly decimals: Option<u32>;
    } & Struct;
    readonly isSetMinFee: boolean;
    readonly asSetMinFee: {
      readonly targetParachainId: u32;
      readonly foreignAssetId: ComposableTraitsXcmAssetsXcmAssetLocation;
      readonly amount: Option<u128>;
    } & Struct;
    readonly type: 'RegisterAsset' | 'UpdateAsset' | 'SetMinFee';
  }

  /** @name PalletGovernanceRegistryCall (352) */
  interface PalletGovernanceRegistryCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly assetId: u128;
      readonly value: AccountId32;
    } & Struct;
    readonly isGrantRoot: boolean;
    readonly asGrantRoot: {
      readonly assetId: u128;
    } & Struct;
    readonly isRemove: boolean;
    readonly asRemove: {
      readonly assetId: u128;
    } & Struct;
    readonly type: 'Set' | 'GrantRoot' | 'Remove';
  }

  /** @name PalletAssetsCall (353) */
  interface PalletAssetsCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly asset: u128;
      readonly dest: MultiAddress;
      readonly amount: Compact<u128>;
      readonly keepAlive: bool;
    } & Struct;
    readonly isTransferNative: boolean;
    readonly asTransferNative: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
      readonly keepAlive: bool;
    } & Struct;
    readonly isForceTransfer: boolean;
    readonly asForceTransfer: {
      readonly asset: u128;
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
      readonly keepAlive: bool;
    } & Struct;
    readonly isForceTransferNative: boolean;
    readonly asForceTransferNative: {
      readonly source: MultiAddress;
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
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
    readonly isMintInitialize: boolean;
    readonly asMintInitialize: {
      readonly amount: Compact<u128>;
      readonly dest: MultiAddress;
    } & Struct;
    readonly isMintInitializeWithGovernance: boolean;
    readonly asMintInitializeWithGovernance: {
      readonly amount: Compact<u128>;
      readonly governanceOrigin: MultiAddress;
      readonly dest: MultiAddress;
    } & Struct;
    readonly isMintInto: boolean;
    readonly asMintInto: {
      readonly assetId: u128;
      readonly dest: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly isBurnFrom: boolean;
    readonly asBurnFrom: {
      readonly assetId: u128;
      readonly dest: MultiAddress;
      readonly amount: Compact<u128>;
    } & Struct;
    readonly type: 'Transfer' | 'TransferNative' | 'ForceTransfer' | 'ForceTransferNative' | 'TransferAll' | 'TransferAllNative' | 'MintInitialize' | 'MintInitializeWithGovernance' | 'MintInto' | 'BurnFrom';
  }

  /** @name PalletCrowdloanRewardsCall (354) */
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
    readonly type: 'Initialize' | 'InitializeAt' | 'Populate' | 'Associate' | 'Claim';
  }

  /** @name PalletCrowdloanRewardsModelsProof (357) */
  interface PalletCrowdloanRewardsModelsProof extends Enum {
    readonly isRelayChain: boolean;
    readonly asRelayChain: ITuple<[AccountId32, SpRuntimeMultiSignature]>;
    readonly isEthereum: boolean;
    readonly asEthereum: ComposableSupportEcdsaSignature;
    readonly type: 'RelayChain' | 'Ethereum';
  }

  /** @name SpRuntimeMultiSignature (358) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreEd25519Signature (359) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name SpCoreSr25519Signature (361) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (362) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name ComposableSupportEcdsaSignature (364) */
  interface ComposableSupportEcdsaSignature extends U8aFixed {}

  /** @name PalletVestingModuleCall (365) */
  interface PalletVestingModuleCall extends Enum {
    readonly isClaim: boolean;
    readonly asClaim: {
      readonly asset: u128;
      readonly vestingScheduleIds: ComposableTraitsVestingVestingScheduleIdSet;
    } & Struct;
    readonly isVestedTransfer: boolean;
    readonly asVestedTransfer: {
      readonly from: MultiAddress;
      readonly beneficiary: MultiAddress;
      readonly asset: u128;
      readonly scheduleInfo: ComposableTraitsVestingVestingScheduleInfo;
    } & Struct;
    readonly isUpdateVestingSchedules: boolean;
    readonly asUpdateVestingSchedules: {
      readonly who: MultiAddress;
      readonly asset: u128;
      readonly vestingSchedules: Vec<ComposableTraitsVestingVestingSchedule>;
    } & Struct;
    readonly isClaimFor: boolean;
    readonly asClaimFor: {
      readonly dest: MultiAddress;
      readonly asset: u128;
      readonly vestingScheduleIds: ComposableTraitsVestingVestingScheduleIdSet;
    } & Struct;
    readonly type: 'Claim' | 'VestedTransfer' | 'UpdateVestingSchedules' | 'ClaimFor';
  }

  /** @name ComposableTraitsVestingVestingScheduleInfo (366) */
  interface ComposableTraitsVestingVestingScheduleInfo extends Struct {
    readonly window: ComposableTraitsVestingVestingWindow;
    readonly periodCount: u32;
    readonly perPeriod: Compact<u128>;
  }

  /** @name PalletBondedFinanceCall (368) */
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

  /** @name ComposableTraitsBondedFinanceBondOffer (369) */
  interface ComposableTraitsBondedFinanceBondOffer extends Struct {
    readonly beneficiary: AccountId32;
    readonly asset: u128;
    readonly bondPrice: u128;
    readonly nbOfBonds: u128;
    readonly maturity: ComposableTraitsBondedFinanceBondDuration;
    readonly reward: ComposableTraitsBondedFinanceBondOfferReward;
  }

  /** @name ComposableTraitsBondedFinanceBondDuration (370) */
  interface ComposableTraitsBondedFinanceBondDuration extends Enum {
    readonly isFinite: boolean;
    readonly asFinite: {
      readonly returnIn: u32;
    } & Struct;
    readonly isInfinite: boolean;
    readonly type: 'Finite' | 'Infinite';
  }

  /** @name ComposableTraitsBondedFinanceBondOfferReward (371) */
  interface ComposableTraitsBondedFinanceBondOfferReward extends Struct {
    readonly asset: u128;
    readonly amount: u128;
    readonly maturity: u32;
  }

  /** @name PalletDutchAuctionCall (372) */
  interface PalletDutchAuctionCall extends Enum {
    readonly isAddConfiguration: boolean;
    readonly asAddConfiguration: {
      readonly configurationId: u128;
      readonly configuration: ComposableTraitsTimeTimeReleaseFunction;
    } & Struct;
    readonly isAsk: boolean;
    readonly asAsk: {
      readonly order: ComposableTraitsDefiSellCurrencyId;
      readonly configuration: ComposableTraitsTimeTimeReleaseFunction;
    } & Struct;
    readonly isTake: boolean;
    readonly asTake: {
      readonly orderId: u128;
      readonly take: ComposableTraitsDefiTake;
    } & Struct;
    readonly isLiquidate: boolean;
    readonly asLiquidate: {
      readonly orderId: u128;
    } & Struct;
    readonly isXcmSell: boolean;
    readonly asXcmSell: {
      readonly request: ComposableTraitsXcmXcmSellRequest;
    } & Struct;
    readonly type: 'AddConfiguration' | 'Ask' | 'Take' | 'Liquidate' | 'XcmSell';
  }

  /** @name ComposableTraitsXcmXcmSellRequest (373) */
  interface ComposableTraitsXcmXcmSellRequest extends Struct {
    readonly orderId: u64;
    readonly fromTo: U8aFixed;
    readonly order: ComposableTraitsDefiSellU128;
    readonly configuration: u128;
  }

  /** @name ComposableTraitsDefiSellU128 (374) */
  interface ComposableTraitsDefiSellU128 extends Struct {
    readonly pair: ComposableTraitsDefiCurrencyPairU128;
    readonly take: ComposableTraitsDefiTake;
  }

  /** @name ComposableTraitsDefiCurrencyPairU128 (375) */
  interface ComposableTraitsDefiCurrencyPairU128 extends Struct {
    readonly base: u128;
    readonly quote: u128;
  }

  /** @name PalletMosaicCall (376) */
  interface PalletMosaicCall extends Enum {
    readonly isSetRelayer: boolean;
    readonly asSetRelayer: {
      readonly relayer: AccountId32;
    } & Struct;
    readonly isRotateRelayer: boolean;
    readonly asRotateRelayer: {
      readonly new_: AccountId32;
      readonly validatedTtl: u32;
    } & Struct;
    readonly isSetNetwork: boolean;
    readonly asSetNetwork: {
      readonly networkId: u32;
      readonly networkInfo: PalletMosaicNetworkInfo;
    } & Struct;
    readonly isSetBudget: boolean;
    readonly asSetBudget: {
      readonly assetId: u128;
      readonly amount: u128;
      readonly decay: PalletMosaicDecayBudgetPenaltyDecayer;
    } & Struct;
    readonly isTransferTo: boolean;
    readonly asTransferTo: {
      readonly networkId: u32;
      readonly assetId: u128;
      readonly address: ComposableSupportEthereumAddress;
      readonly amount: u128;
      readonly minimumAmountOut: u128;
      readonly swapToNative: bool;
      readonly sourceUserAccount: AccountId32;
      readonly ammSwapInfo: Option<PalletMosaicAmmSwapInfo>;
      readonly keepAlive: bool;
    } & Struct;
    readonly isAcceptTransfer: boolean;
    readonly asAcceptTransfer: {
      readonly from: AccountId32;
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
      readonly amount: u128;
    } & Struct;
    readonly isClaimStaleTo: boolean;
    readonly asClaimStaleTo: {
      readonly assetId: u128;
      readonly to: AccountId32;
    } & Struct;
    readonly isTimelockedMint: boolean;
    readonly asTimelockedMint: {
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
      readonly to: AccountId32;
      readonly amount: u128;
      readonly lockTime: u32;
      readonly id: H256;
    } & Struct;
    readonly isSetTimelockDuration: boolean;
    readonly asSetTimelockDuration: {
      readonly period: u32;
    } & Struct;
    readonly isRescindTimelockedMint: boolean;
    readonly asRescindTimelockedMint: {
      readonly networkId: u32;
      readonly remoteAssetId: CommonMosaicRemoteAssetId;
      readonly account: AccountId32;
      readonly untrustedAmount: u128;
    } & Struct;
    readonly isClaimTo: boolean;
    readonly asClaimTo: {
      readonly assetId: u128;
      readonly to: AccountId32;
    } & Struct;
    readonly isUpdateAssetMapping: boolean;
    readonly asUpdateAssetMapping: {
      readonly assetId: u128;
      readonly networkId: u32;
      readonly remoteAssetId: Option<CommonMosaicRemoteAssetId>;
    } & Struct;
    readonly isAddRemoteAmmId: boolean;
    readonly asAddRemoteAmmId: {
      readonly networkId: u32;
      readonly ammId: u128;
    } & Struct;
    readonly isRemoveRemoteAmmId: boolean;
    readonly asRemoveRemoteAmmId: {
      readonly networkId: u32;
      readonly ammId: u128;
    } & Struct;
    readonly type: 'SetRelayer' | 'RotateRelayer' | 'SetNetwork' | 'SetBudget' | 'TransferTo' | 'AcceptTransfer' | 'ClaimStaleTo' | 'TimelockedMint' | 'SetTimelockDuration' | 'RescindTimelockedMint' | 'ClaimTo' | 'UpdateAssetMapping' | 'AddRemoteAmmId' | 'RemoveRemoteAmmId';
  }

  /** @name PalletLiquidationsCall (378) */
  interface PalletLiquidationsCall extends Enum {
    readonly isAddLiquidationStrategy: boolean;
    readonly asAddLiquidationStrategy: {
      readonly configuration: PalletLiquidationsLiquidationStrategyConfiguration;
    } & Struct;
    readonly isSell: boolean;
    readonly asSell: {
      readonly order: ComposableTraitsDefiSellCurrencyId;
      readonly configuration: Vec<u32>;
    } & Struct;
    readonly type: 'AddLiquidationStrategy' | 'Sell';
  }

  /** @name PalletLiquidationsLiquidationStrategyConfiguration (379) */
  interface PalletLiquidationsLiquidationStrategyConfiguration extends Enum {
    readonly isDutchAuction: boolean;
    readonly asDutchAuction: ComposableTraitsTimeTimeReleaseFunction;
    readonly isPablo: boolean;
    readonly asPablo: {
      readonly slippage: Perquintill;
    } & Struct;
    readonly isXcm: boolean;
    readonly asXcm: ComposableTraitsXcmXcmSellRequestTransactConfiguration;
    readonly type: 'DutchAuction' | 'Pablo' | 'Xcm';
  }

  /** @name ComposableTraitsXcmXcmSellRequestTransactConfiguration (380) */
  interface ComposableTraitsXcmXcmSellRequestTransactConfiguration extends Struct {
    readonly location: ComposableTraitsXcmXcmTransactConfiguration;
    readonly configurationId: u128;
    readonly fee: u128;
  }

  /** @name ComposableTraitsXcmXcmTransactConfiguration (381) */
  interface ComposableTraitsXcmXcmTransactConfiguration extends Struct {
    readonly parachainId: u32;
    readonly methodId: ComposableTraitsXcmCumulusMethodId;
  }

  /** @name ComposableTraitsXcmCumulusMethodId (382) */
  interface ComposableTraitsXcmCumulusMethodId extends Struct {
    readonly palletInstance: u8;
    readonly methodId: u8;
  }

  /** @name PalletLendingCall (383) */
  interface PalletLendingCall extends Enum {
    readonly isCreateMarket: boolean;
    readonly asCreateMarket: {
      readonly input: ComposableTraitsLendingCreateInput;
      readonly keepAlive: bool;
    } & Struct;
    readonly isUpdateMarket: boolean;
    readonly asUpdateMarket: {
      readonly marketId: u32;
      readonly input: ComposableTraitsLendingUpdateInput;
    } & Struct;
    readonly isDepositCollateral: boolean;
    readonly asDepositCollateral: {
      readonly marketId: u32;
      readonly amount: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isWithdrawCollateral: boolean;
    readonly asWithdrawCollateral: {
      readonly marketId: u32;
      readonly amount: u128;
    } & Struct;
    readonly isBorrow: boolean;
    readonly asBorrow: {
      readonly marketId: u32;
      readonly amountToBorrow: u128;
    } & Struct;
    readonly isRepayBorrow: boolean;
    readonly asRepayBorrow: {
      readonly marketId: u32;
      readonly beneficiary: AccountId32;
      readonly amount: ComposableTraitsLendingRepayStrategy;
      readonly keepAlive: bool;
    } & Struct;
    readonly isLiquidate: boolean;
    readonly asLiquidate: {
      readonly marketId: u32;
      readonly borrowers: Vec<AccountId32>;
    } & Struct;
    readonly type: 'CreateMarket' | 'UpdateMarket' | 'DepositCollateral' | 'WithdrawCollateral' | 'Borrow' | 'RepayBorrow' | 'Liquidate';
  }

  /** @name ComposableTraitsLendingCreateInput (384) */
  interface ComposableTraitsLendingCreateInput extends Struct {
    readonly updatable: ComposableTraitsLendingUpdateInput;
    readonly currencyPair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly reservedFactor: Perquintill;
    readonly interestRateModel: ComposableTraitsLendingMathInterestRateModel;
  }

  /** @name ComposableTraitsLendingMathInterestRateModel (385) */
  interface ComposableTraitsLendingMathInterestRateModel extends Enum {
    readonly isJump: boolean;
    readonly asJump: ComposableTraitsLendingMathJumpModel;
    readonly isCurve: boolean;
    readonly asCurve: ComposableTraitsLendingMathCurveModel;
    readonly isDynamicPIDController: boolean;
    readonly asDynamicPIDController: ComposableTraitsLendingMathDynamicPIDControllerModel;
    readonly isDoubleExponent: boolean;
    readonly asDoubleExponent: ComposableTraitsLendingMathDoubleExponentModel;
    readonly type: 'Jump' | 'Curve' | 'DynamicPIDController' | 'DoubleExponent';
  }

  /** @name ComposableTraitsLendingMathJumpModel (386) */
  interface ComposableTraitsLendingMathJumpModel extends Struct {
    readonly baseRate: u128;
    readonly jumpRate: u128;
    readonly fullRate: u128;
    readonly targetUtilization: Percent;
  }

  /** @name ComposableTraitsLendingMathCurveModel (387) */
  interface ComposableTraitsLendingMathCurveModel extends Struct {
    readonly baseRate: u128;
  }

  /** @name ComposableTraitsLendingMathDynamicPIDControllerModel (388) */
  interface ComposableTraitsLendingMathDynamicPIDControllerModel extends Struct {
    readonly proportionalParameter: i128;
    readonly integralParameter: i128;
    readonly derivativeParameter: i128;
    readonly previousErrorValue: i128;
    readonly previousIntegralTerm: i128;
    readonly previousInterestRate: u128;
    readonly targetUtilization: u128;
  }

  /** @name ComposableTraitsLendingMathDoubleExponentModel (391) */
  interface ComposableTraitsLendingMathDoubleExponentModel extends Struct {
    readonly coefficients: U8aFixed;
  }

  /** @name ComposableTraitsLendingRepayStrategy (392) */
  interface ComposableTraitsLendingRepayStrategy extends Enum {
    readonly isTotalDebt: boolean;
    readonly isPartialAmount: boolean;
    readonly asPartialAmount: u128;
    readonly type: 'TotalDebt' | 'PartialAmount';
  }

  /** @name PalletPabloCall (394) */
  interface PalletPabloCall extends Enum {
    readonly isCreate: boolean;
    readonly asCreate: {
      readonly pool: PalletPabloPoolInitConfiguration;
    } & Struct;
    readonly isBuy: boolean;
    readonly asBuy: {
      readonly poolId: u128;
      readonly assetId: u128;
      readonly amount: u128;
      readonly minReceive: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isSell: boolean;
    readonly asSell: {
      readonly poolId: u128;
      readonly assetId: u128;
      readonly amount: u128;
      readonly minReceive: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isSwap: boolean;
    readonly asSwap: {
      readonly poolId: u128;
      readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly quoteAmount: u128;
      readonly minReceive: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isAddLiquidity: boolean;
    readonly asAddLiquidity: {
      readonly poolId: u128;
      readonly baseAmount: u128;
      readonly quoteAmount: u128;
      readonly minMintAmount: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isRemoveLiquidity: boolean;
    readonly asRemoveLiquidity: {
      readonly poolId: u128;
      readonly lpAmount: u128;
      readonly minBaseAmount: u128;
      readonly minQuoteAmount: u128;
    } & Struct;
    readonly isEnableTwap: boolean;
    readonly asEnableTwap: {
      readonly poolId: u128;
    } & Struct;
    readonly type: 'Create' | 'Buy' | 'Sell' | 'Swap' | 'AddLiquidity' | 'RemoveLiquidity' | 'EnableTwap';
  }

  /** @name PalletPabloPoolInitConfiguration (395) */
  interface PalletPabloPoolInitConfiguration extends Enum {
    readonly isStableSwap: boolean;
    readonly asStableSwap: {
      readonly owner: AccountId32;
      readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly amplificationCoefficient: u16;
      readonly fee: Permill;
    } & Struct;
    readonly isConstantProduct: boolean;
    readonly asConstantProduct: {
      readonly owner: AccountId32;
      readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly fee: Permill;
      readonly baseWeight: Permill;
    } & Struct;
    readonly isLiquidityBootstrapping: boolean;
    readonly asLiquidityBootstrapping: ComposableTraitsDexLiquidityBootstrappingPoolInfo;
    readonly type: 'StableSwap' | 'ConstantProduct' | 'LiquidityBootstrapping';
  }

  /** @name ComposableTraitsDexLiquidityBootstrappingPoolInfo (396) */
  interface ComposableTraitsDexLiquidityBootstrappingPoolInfo extends Struct {
    readonly owner: AccountId32;
    readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly sale: ComposableTraitsDexSale;
    readonly feeConfig: ComposableTraitsDexFeeConfig;
  }

  /** @name ComposableTraitsDexSale (397) */
  interface ComposableTraitsDexSale extends Struct {
    readonly start: u32;
    readonly end: u32;
    readonly initialWeight: Permill;
    readonly finalWeight: Permill;
  }

  /** @name ComposableTraitsDexFeeConfig (398) */
  interface ComposableTraitsDexFeeConfig extends Struct {
    readonly feeRate: Permill;
    readonly ownerFeeRate: Permill;
    readonly protocolFeeRate: Permill;
  }

  /** @name PalletDexRouterCall (399) */
  interface PalletDexRouterCall extends Enum {
    readonly isUpdateRoute: boolean;
    readonly asUpdateRoute: {
      readonly assetPair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly route: Option<Vec<u128>>;
    } & Struct;
    readonly isExchange: boolean;
    readonly asExchange: {
      readonly assetPair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly amount: u128;
      readonly minReceive: u128;
    } & Struct;
    readonly isSell: boolean;
    readonly asSell: {
      readonly assetPair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly amount: u128;
      readonly minReceive: u128;
    } & Struct;
    readonly isBuy: boolean;
    readonly asBuy: {
      readonly assetPair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly amount: u128;
      readonly minReceive: u128;
    } & Struct;
    readonly isAddLiquidity: boolean;
    readonly asAddLiquidity: {
      readonly assetPair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly baseAmount: u128;
      readonly quoteAmount: u128;
      readonly minMintAmount: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly isRemoveLiquidity: boolean;
    readonly asRemoveLiquidity: {
      readonly assetPair: ComposableTraitsDefiCurrencyPairCurrencyId;
      readonly lpAmount: u128;
      readonly minBaseAmount: u128;
      readonly minQuoteAmount: u128;
    } & Struct;
    readonly type: 'UpdateRoute' | 'Exchange' | 'Sell' | 'Buy' | 'AddLiquidity' | 'RemoveLiquidity';
  }

  /** @name PalletStakingRewardsCall (402) */
  interface PalletStakingRewardsCall extends Enum {
    readonly isCreateRewardPool: boolean;
    readonly asCreateRewardPool: {
      readonly poolConfig: ComposableTraitsStakingRewardPoolConfiguration;
    } & Struct;
    readonly isStake: boolean;
    readonly asStake: {
      readonly poolId: u128;
      readonly amount: u128;
      readonly durationPreset: u64;
    } & Struct;
    readonly isExtend: boolean;
    readonly asExtend: {
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
      readonly amount: u128;
    } & Struct;
    readonly isUnstake: boolean;
    readonly asUnstake: {
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
    } & Struct;
    readonly isSplit: boolean;
    readonly asSplit: {
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
      readonly ratio: Permill;
    } & Struct;
    readonly isUpdateRewardsPool: boolean;
    readonly asUpdateRewardsPool: {
      readonly poolId: u128;
      readonly rewardUpdates: BTreeMap<u128, ComposableTraitsStakingRewardUpdate>;
    } & Struct;
    readonly isClaim: boolean;
    readonly asClaim: {
      readonly fnftCollectionId: u128;
      readonly fnftInstanceId: u64;
    } & Struct;
    readonly isAddToRewardsPot: boolean;
    readonly asAddToRewardsPot: {
      readonly poolId: u128;
      readonly assetId: u128;
      readonly amount: u128;
      readonly keepAlive: bool;
    } & Struct;
    readonly type: 'CreateRewardPool' | 'Stake' | 'Extend' | 'Unstake' | 'Split' | 'UpdateRewardsPool' | 'Claim' | 'AddToRewardsPot';
  }

  /** @name ComposableTraitsStakingRewardPoolConfiguration (403) */
  interface ComposableTraitsStakingRewardPoolConfiguration extends Enum {
    readonly isRewardRateBasedIncentive: boolean;
    readonly asRewardRateBasedIncentive: {
      readonly owner: AccountId32;
      readonly assetId: u128;
      readonly startBlock: u32;
      readonly endBlock: u32;
      readonly rewardConfigs: BTreeMap<u128, ComposableTraitsStakingRewardConfig>;
      readonly lock: ComposableTraitsStakingLockLockConfig;
      readonly shareAssetId: u128;
      readonly financialNftAssetId: u128;
      readonly minimumStakingAmount: u128;
    } & Struct;
    readonly type: 'RewardRateBasedIncentive';
  }

  /** @name ComposableTraitsStakingRewardConfig (405) */
  interface ComposableTraitsStakingRewardConfig extends Struct {
    readonly maxRewards: u128;
    readonly rewardRate: ComposableTraitsStakingRewardRate;
  }

  /** @name ComposableTraitsStakingRewardRate (406) */
  interface ComposableTraitsStakingRewardRate extends Struct {
    readonly period: ComposableTraitsStakingRewardRatePeriod;
    readonly amount: u128;
  }

  /** @name ComposableTraitsStakingRewardRatePeriod (407) */
  interface ComposableTraitsStakingRewardRatePeriod extends Enum {
    readonly isPerSecond: boolean;
    readonly type: 'PerSecond';
  }

  /** @name ComposableTraitsStakingLockLockConfig (411) */
  interface ComposableTraitsStakingLockLockConfig extends Struct {
    readonly durationPresets: BTreeMap<u64, u64>;
    readonly unlockPenalty: Perbill;
  }

  /** @name ComposableTraitsStakingRewardUpdate (417) */
  interface ComposableTraitsStakingRewardUpdate extends Struct {
    readonly rewardRate: ComposableTraitsStakingRewardRate;
  }

  /** @name PalletCallFilterCall (421) */
  interface PalletCallFilterCall extends Enum {
    readonly isDisable: boolean;
    readonly asDisable: {
      readonly entry: ComposableTraitsCallFilterCallFilterEntry;
    } & Struct;
    readonly isEnable: boolean;
    readonly asEnable: {
      readonly entry: ComposableTraitsCallFilterCallFilterEntry;
    } & Struct;
    readonly type: 'Disable' | 'Enable';
  }

  /** @name PalletIbcPingCall (422) */
  interface PalletIbcPingCall extends Enum {
    readonly isOpenChannel: boolean;
    readonly asOpenChannel: {
      readonly params: IbcTraitOpenChannelParams;
    } & Struct;
    readonly isSendPing: boolean;
    readonly asSendPing: {
      readonly params: PalletIbcPingSendPingParams;
    } & Struct;
    readonly type: 'OpenChannel' | 'SendPing';
  }

  /** @name IbcTraitOpenChannelParams (423) */
  interface IbcTraitOpenChannelParams extends Struct {
    readonly order: u8;
    readonly connectionId: Bytes;
    readonly counterpartyPortId: Bytes;
    readonly version: Bytes;
  }

  /** @name PalletIbcPingSendPingParams (424) */
  interface PalletIbcPingSendPingParams extends Struct {
    readonly data: Bytes;
    readonly timeoutHeight: u64;
    readonly timeoutTimestamp: u64;
    readonly channelId: Bytes;
    readonly destPortId: Bytes;
    readonly destChannelId: Bytes;
  }

  /** @name IbcTransferCall (425) */
  interface IbcTransferCall extends Enum {
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly params: IbcTransferTransferParams;
      readonly assetId: u128;
      readonly amount: u128;
    } & Struct;
    readonly isOpenChannel: boolean;
    readonly asOpenChannel: {
      readonly params: IbcTraitOpenChannelParams;
    } & Struct;
    readonly isSetPalletParams: boolean;
    readonly asSetPalletParams: {
      readonly params: IbcTransferPalletParams;
    } & Struct;
    readonly type: 'Transfer' | 'OpenChannel' | 'SetPalletParams';
  }

  /** @name IbcTransferTransferParams (426) */
  interface IbcTransferTransferParams extends Struct {
    readonly to: Bytes;
    readonly sourceChannel: Bytes;
    readonly timeoutTimestamp: u64;
    readonly timeoutHeight: u64;
    readonly revisionNumber: Option<u64>;
  }

  /** @name IbcTransferPalletParams (427) */
  interface IbcTransferPalletParams extends Struct {
    readonly sendEnabled: bool;
    readonly receiveEnabled: bool;
  }

  /** @name PalletIbcCall (428) */
  interface PalletIbcCall extends Enum {
    readonly isDeliver: boolean;
    readonly asDeliver: {
      readonly messages: Vec<PalletIbcAny>;
    } & Struct;
    readonly isCreateClient: boolean;
    readonly asCreateClient: {
      readonly msg: PalletIbcAny;
    } & Struct;
    readonly isInitiateConnection: boolean;
    readonly asInitiateConnection: {
      readonly params: PalletIbcConnectionParams;
    } & Struct;
    readonly type: 'Deliver' | 'CreateClient' | 'InitiateConnection';
  }

  /** @name PalletIbcAny (430) */
  interface PalletIbcAny extends Struct {
    readonly typeUrl: Bytes;
    readonly value: Bytes;
  }

  /** @name PalletIbcConnectionParams (431) */
  interface PalletIbcConnectionParams extends Struct {
    readonly version: ITuple<[Bytes, Vec<Bytes>]>;
    readonly clientId: Bytes;
    readonly counterpartyClientId: Bytes;
    readonly commitmentPrefix: Bytes;
    readonly delayPeriod: u64;
  }

  /** @name PalletCosmwasmCall (433) */
  interface PalletCosmwasmCall extends Enum {
    readonly isUpload: boolean;
    readonly asUpload: {
      readonly code: Bytes;
    } & Struct;
    readonly isInstantiate: boolean;
    readonly asInstantiate: {
      readonly codeId: u64;
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
    readonly type: 'Upload' | 'Instantiate' | 'Execute';
  }

  /** @name PalletSudoError (442) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name PalletTransactionPaymentReleases (444) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name PalletIndicesError (446) */
  interface PalletIndicesError extends Enum {
    readonly isNotAssigned: boolean;
    readonly isNotOwner: boolean;
    readonly isInUse: boolean;
    readonly isNotTransfer: boolean;
    readonly isPermanent: boolean;
    readonly type: 'NotAssigned' | 'NotOwner' | 'InUse' | 'NotTransfer' | 'Permanent';
  }

  /** @name PalletBalancesBalanceLock (448) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (449) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (452) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (454) */
  interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: 'V100' | 'V200';
  }

  /** @name PalletBalancesError (455) */
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

  /** @name PalletIdentityRegistration (456) */
  interface PalletIdentityRegistration extends Struct {
    readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
    readonly deposit: u128;
    readonly info: PalletIdentityIdentityInfo;
  }

  /** @name PalletIdentityRegistrarInfo (464) */
  interface PalletIdentityRegistrarInfo extends Struct {
    readonly account: AccountId32;
    readonly fee: u128;
    readonly fields: PalletIdentityBitFlags;
  }

  /** @name PalletIdentityError (466) */
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
    readonly type: 'TooManySubAccounts' | 'NotFound' | 'NotNamed' | 'EmptyIndex' | 'FeeChanged' | 'NoIdentity' | 'StickyJudgement' | 'JudgementGiven' | 'InvalidJudgement' | 'InvalidIndex' | 'InvalidTarget' | 'TooManyFields' | 'TooManyRegistrars' | 'AlreadyClaimed' | 'NotSub' | 'NotOwned';
  }

  /** @name PalletMultisigMultisig (468) */
  interface PalletMultisigMultisig extends Struct {
    readonly when: PalletMultisigTimepoint;
    readonly deposit: u128;
    readonly depositor: AccountId32;
    readonly approvals: Vec<AccountId32>;
  }

  /** @name PalletMultisigError (470) */
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

  /** @name PolkadotPrimitivesV2UpgradeRestriction (472) */
  interface PolkadotPrimitivesV2UpgradeRestriction extends Enum {
    readonly isPresent: boolean;
    readonly type: 'Present';
  }

  /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (473) */
  interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Struct {
    readonly dmqMqcHead: H256;
    readonly relayDispatchQueueSize: ITuple<[u32, u32]>;
    readonly ingressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV2AbridgedHrmpChannel]>>;
    readonly egressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV2AbridgedHrmpChannel]>>;
  }

  /** @name PolkadotPrimitivesV2AbridgedHrmpChannel (476) */
  interface PolkadotPrimitivesV2AbridgedHrmpChannel extends Struct {
    readonly maxCapacity: u32;
    readonly maxTotalSize: u32;
    readonly maxMessageSize: u32;
    readonly msgCount: u32;
    readonly totalSize: u32;
    readonly mqcHead: Option<H256>;
  }

  /** @name PolkadotPrimitivesV2AbridgedHostConfiguration (477) */
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

  /** @name PolkadotCorePrimitivesOutboundHrmpMessage (483) */
  interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
    readonly recipient: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemError (484) */
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

  /** @name PalletAuthorshipUncleEntryItem (486) */
  interface PalletAuthorshipUncleEntryItem extends Enum {
    readonly isInclusionHeight: boolean;
    readonly asInclusionHeight: u32;
    readonly isUncle: boolean;
    readonly asUncle: ITuple<[H256, Option<AccountId32>]>;
    readonly type: 'InclusionHeight' | 'Uncle';
  }

  /** @name PalletAuthorshipError (488) */
  interface PalletAuthorshipError extends Enum {
    readonly isInvalidUncleParent: boolean;
    readonly isUnclesAlreadySet: boolean;
    readonly isTooManyUncles: boolean;
    readonly isGenesisUncle: boolean;
    readonly isTooHighUncle: boolean;
    readonly isUncleAlreadyIncluded: boolean;
    readonly isOldUncle: boolean;
    readonly type: 'InvalidUncleParent' | 'UnclesAlreadySet' | 'TooManyUncles' | 'GenesisUncle' | 'TooHighUncle' | 'UncleAlreadyIncluded' | 'OldUncle';
  }

  /** @name PalletCollatorSelectionCandidateInfo (491) */
  interface PalletCollatorSelectionCandidateInfo extends Struct {
    readonly who: AccountId32;
    readonly deposit: u128;
  }

  /** @name PalletCollatorSelectionError (493) */
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

  /** @name SpCoreCryptoKeyTypeId (497) */
  interface SpCoreCryptoKeyTypeId extends U8aFixed {}

  /** @name PalletSessionError (498) */
  interface PalletSessionError extends Enum {
    readonly isInvalidProof: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isDuplicatedKey: boolean;
    readonly isNoKeys: boolean;
    readonly isNoAccount: boolean;
    readonly type: 'InvalidProof' | 'NoAssociatedValidatorId' | 'DuplicatedKey' | 'NoKeys' | 'NoAccount';
  }

  /** @name PalletCollectiveVotes (503) */
  interface PalletCollectiveVotes extends Struct {
    readonly index: u32;
    readonly threshold: u32;
    readonly ayes: Vec<AccountId32>;
    readonly nays: Vec<AccountId32>;
    readonly end: u32;
  }

  /** @name PalletCollectiveError (504) */
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

  /** @name PalletMembershipError (506) */
  interface PalletMembershipError extends Enum {
    readonly isAlreadyMember: boolean;
    readonly isNotMember: boolean;
    readonly isTooManyMembers: boolean;
    readonly type: 'AlreadyMember' | 'NotMember' | 'TooManyMembers';
  }

  /** @name PalletTreasuryProposal (507) */
  interface PalletTreasuryProposal extends Struct {
    readonly proposer: AccountId32;
    readonly value: u128;
    readonly beneficiary: AccountId32;
    readonly bond: u128;
  }

  /** @name FrameSupportPalletId (509) */
  interface FrameSupportPalletId extends U8aFixed {}

  /** @name PalletTreasuryError (510) */
  interface PalletTreasuryError extends Enum {
    readonly isInsufficientProposersBalance: boolean;
    readonly isInvalidIndex: boolean;
    readonly isTooManyApprovals: boolean;
    readonly isInsufficientPermission: boolean;
    readonly isProposalNotApproved: boolean;
    readonly type: 'InsufficientProposersBalance' | 'InvalidIndex' | 'TooManyApprovals' | 'InsufficientPermission' | 'ProposalNotApproved';
  }

  /** @name PalletDemocracyPreimageStatus (514) */
  interface PalletDemocracyPreimageStatus extends Enum {
    readonly isMissing: boolean;
    readonly asMissing: u32;
    readonly isAvailable: boolean;
    readonly asAvailable: {
      readonly data: Bytes;
      readonly provider: AccountId32;
      readonly deposit: u128;
      readonly since: u32;
      readonly expiry: Option<u32>;
    } & Struct;
    readonly type: 'Missing' | 'Available';
  }

  /** @name PalletDemocracyReferendumInfo (515) */
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

  /** @name PalletDemocracyReferendumStatus (516) */
  interface PalletDemocracyReferendumStatus extends Struct {
    readonly end: u32;
    readonly proposalHash: H256;
    readonly threshold: PalletDemocracyVoteThreshold;
    readonly delay: u32;
    readonly tally: PalletDemocracyTally;
  }

  /** @name PalletDemocracyTally (517) */
  interface PalletDemocracyTally extends Struct {
    readonly ayes: u128;
    readonly nays: u128;
    readonly turnout: u128;
  }

  /** @name PalletDemocracyVoteVoting (518) */
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

  /** @name PalletDemocracyDelegations (521) */
  interface PalletDemocracyDelegations extends Struct {
    readonly votes: u128;
    readonly capital: u128;
  }

  /** @name PalletDemocracyVotePriorLock (522) */
  interface PalletDemocracyVotePriorLock extends ITuple<[u32, u128]> {}

  /** @name PalletDemocracyReleases (525) */
  interface PalletDemocracyReleases extends Enum {
    readonly isV1: boolean;
    readonly type: 'V1';
  }

  /** @name PalletDemocracyError (526) */
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
    readonly isDuplicatePreimage: boolean;
    readonly isNotImminent: boolean;
    readonly isTooEarly: boolean;
    readonly isImminent: boolean;
    readonly isPreimageMissing: boolean;
    readonly isReferendumInvalid: boolean;
    readonly isPreimageInvalid: boolean;
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
    readonly isTooManyProposals: boolean;
    readonly isVotingPeriodLow: boolean;
    readonly type: 'ValueLow' | 'ProposalMissing' | 'AlreadyCanceled' | 'DuplicateProposal' | 'ProposalBlacklisted' | 'NotSimpleMajority' | 'InvalidHash' | 'NoProposal' | 'AlreadyVetoed' | 'DuplicatePreimage' | 'NotImminent' | 'TooEarly' | 'Imminent' | 'PreimageMissing' | 'ReferendumInvalid' | 'PreimageInvalid' | 'NoneWaiting' | 'NotVoter' | 'NoPermission' | 'AlreadyDelegating' | 'InsufficientFunds' | 'NotDelegating' | 'VotesExist' | 'InstantNotAllowed' | 'Nonsense' | 'WrongUpperBound' | 'MaxVotesReached' | 'TooManyProposals' | 'VotingPeriodLow';
  }

  /** @name PalletSchedulerScheduledV3 (531) */
  interface PalletSchedulerScheduledV3 extends Struct {
    readonly maybeId: Option<Bytes>;
    readonly priority: u8;
    readonly call: FrameSupportScheduleMaybeHashed;
    readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
    readonly origin: DaliRuntimeOriginCaller;
  }

  /** @name PalletSchedulerError (532) */
  interface PalletSchedulerError extends Enum {
    readonly isFailedToSchedule: boolean;
    readonly isNotFound: boolean;
    readonly isTargetBlockNumberInPast: boolean;
    readonly isRescheduleNoChange: boolean;
    readonly type: 'FailedToSchedule' | 'NotFound' | 'TargetBlockNumberInPast' | 'RescheduleNoChange';
  }

  /** @name PalletUtilityError (533) */
  interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: 'TooManyCalls';
  }

  /** @name PalletPreimageRequestStatus (534) */
  interface PalletPreimageRequestStatus extends Enum {
    readonly isUnrequested: boolean;
    readonly asUnrequested: Option<ITuple<[AccountId32, u128]>>;
    readonly isRequested: boolean;
    readonly asRequested: u32;
    readonly type: 'Unrequested' | 'Requested';
  }

  /** @name PalletPreimageError (538) */
  interface PalletPreimageError extends Enum {
    readonly isTooLarge: boolean;
    readonly isAlreadyNoted: boolean;
    readonly isNotAuthorized: boolean;
    readonly isNotNoted: boolean;
    readonly isRequested: boolean;
    readonly isNotRequested: boolean;
    readonly type: 'TooLarge' | 'AlreadyNoted' | 'NotAuthorized' | 'NotNoted' | 'Requested' | 'NotRequested';
  }

  /** @name ComposableTraitsAccountProxyProxyDefinition (541) */
  interface ComposableTraitsAccountProxyProxyDefinition extends Struct {
    readonly delegate: AccountId32;
    readonly proxyType: ComposableTraitsAccountProxyProxyType;
    readonly delay: u32;
  }

  /** @name PalletAccountProxyAnnouncement (545) */
  interface PalletAccountProxyAnnouncement extends Struct {
    readonly real: AccountId32;
    readonly callHash: H256;
    readonly height: u32;
  }

  /** @name PalletAccountProxyError (547) */
  interface PalletAccountProxyError extends Enum {
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

  /** @name CumulusPalletXcmpQueueInboundChannelDetails (549) */
  interface CumulusPalletXcmpQueueInboundChannelDetails extends Struct {
    readonly sender: u32;
    readonly state: CumulusPalletXcmpQueueInboundState;
    readonly messageMetadata: Vec<ITuple<[u32, PolkadotParachainPrimitivesXcmpMessageFormat]>>;
  }

  /** @name CumulusPalletXcmpQueueInboundState (550) */
  interface CumulusPalletXcmpQueueInboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name PolkadotParachainPrimitivesXcmpMessageFormat (553) */
  interface PolkadotParachainPrimitivesXcmpMessageFormat extends Enum {
    readonly isConcatenatedVersionedXcm: boolean;
    readonly isConcatenatedEncodedBlob: boolean;
    readonly isSignals: boolean;
    readonly type: 'ConcatenatedVersionedXcm' | 'ConcatenatedEncodedBlob' | 'Signals';
  }

  /** @name CumulusPalletXcmpQueueOutboundChannelDetails (556) */
  interface CumulusPalletXcmpQueueOutboundChannelDetails extends Struct {
    readonly recipient: u32;
    readonly state: CumulusPalletXcmpQueueOutboundState;
    readonly signalsExist: bool;
    readonly firstIndex: u16;
    readonly lastIndex: u16;
  }

  /** @name CumulusPalletXcmpQueueOutboundState (557) */
  interface CumulusPalletXcmpQueueOutboundState extends Enum {
    readonly isOk: boolean;
    readonly isSuspended: boolean;
    readonly type: 'Ok' | 'Suspended';
  }

  /** @name CumulusPalletXcmpQueueQueueConfigData (559) */
  interface CumulusPalletXcmpQueueQueueConfigData extends Struct {
    readonly suspendThreshold: u32;
    readonly dropThreshold: u32;
    readonly resumeThreshold: u32;
    readonly thresholdWeight: u64;
    readonly weightRestrictDecay: u64;
    readonly xcmpMaxIndividualWeight: u64;
  }

  /** @name CumulusPalletXcmpQueueError (561) */
  interface CumulusPalletXcmpQueueError extends Enum {
    readonly isFailedToSend: boolean;
    readonly isBadXcmOrigin: boolean;
    readonly isBadXcm: boolean;
    readonly isBadOverweightIndex: boolean;
    readonly isWeightOverLimit: boolean;
    readonly type: 'FailedToSend' | 'BadXcmOrigin' | 'BadXcm' | 'BadOverweightIndex' | 'WeightOverLimit';
  }

  /** @name PalletXcmQueryStatus (562) */
  interface PalletXcmQueryStatus extends Enum {
    readonly isPending: boolean;
    readonly asPending: {
      readonly responder: XcmVersionedMultiLocation;
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

  /** @name XcmVersionedResponse (565) */
  interface XcmVersionedResponse extends Enum {
    readonly isV0: boolean;
    readonly asV0: XcmV0Response;
    readonly isV1: boolean;
    readonly asV1: XcmV1Response;
    readonly isV2: boolean;
    readonly asV2: XcmV2Response;
    readonly type: 'V0' | 'V1' | 'V2';
  }

  /** @name PalletXcmVersionMigrationStage (571) */
  interface PalletXcmVersionMigrationStage extends Enum {
    readonly isMigrateSupportedVersion: boolean;
    readonly isMigrateVersionNotifiers: boolean;
    readonly isNotifyCurrentTargets: boolean;
    readonly asNotifyCurrentTargets: Option<Bytes>;
    readonly isMigrateAndNotifyOldTargets: boolean;
    readonly type: 'MigrateSupportedVersion' | 'MigrateVersionNotifiers' | 'NotifyCurrentTargets' | 'MigrateAndNotifyOldTargets';
  }

  /** @name PalletXcmError (572) */
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
    readonly type: 'Unreachable' | 'SendFailure' | 'Filtered' | 'UnweighableMessage' | 'DestinationNotInvertible' | 'Empty' | 'CannotReanchor' | 'TooManyAssets' | 'InvalidOrigin' | 'BadVersion' | 'BadLocation' | 'NoSubscription' | 'AlreadySubscribed';
  }

  /** @name CumulusPalletXcmError (573) */
  type CumulusPalletXcmError = Null;

  /** @name CumulusPalletDmpQueueConfigData (574) */
  interface CumulusPalletDmpQueueConfigData extends Struct {
    readonly maxIndividual: u64;
  }

  /** @name CumulusPalletDmpQueuePageIndexData (575) */
  interface CumulusPalletDmpQueuePageIndexData extends Struct {
    readonly beginUsed: u32;
    readonly endUsed: u32;
    readonly overweightCount: u64;
  }

  /** @name CumulusPalletDmpQueueError (578) */
  interface CumulusPalletDmpQueueError extends Enum {
    readonly isUnknown: boolean;
    readonly isOverLimit: boolean;
    readonly type: 'Unknown' | 'OverLimit';
  }

  /** @name OrmlXtokensModuleError (579) */
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

  /** @name OrmlUnknownTokensModuleError (582) */
  interface OrmlUnknownTokensModuleError extends Enum {
    readonly isBalanceTooLow: boolean;
    readonly isBalanceOverflow: boolean;
    readonly isUnhandledAsset: boolean;
    readonly type: 'BalanceTooLow' | 'BalanceOverflow' | 'UnhandledAsset';
  }

  /** @name OrmlTokensBalanceLock (585) */
  interface OrmlTokensBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name OrmlTokensAccountData (587) */
  interface OrmlTokensAccountData extends Struct {
    readonly free: u128;
    readonly reserved: u128;
    readonly frozen: u128;
  }

  /** @name OrmlTokensReserveData (589) */
  interface OrmlTokensReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name OrmlTokensModuleError (591) */
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

  /** @name ComposableTraitsOracleRewardTracker (592) */
  interface ComposableTraitsOracleRewardTracker extends Struct {
    readonly period: u64;
    readonly start: u64;
    readonly totalAlreadyRewarded: u128;
    readonly currentBlockReward: u128;
    readonly totalRewardWeight: u128;
  }

  /** @name PalletOracleWithdraw (593) */
  interface PalletOracleWithdraw extends Struct {
    readonly stake: u128;
    readonly unlockBlock: u32;
  }

  /** @name ComposableTraitsOraclePrice (594) */
  interface ComposableTraitsOraclePrice extends Struct {
    readonly price: u128;
    readonly block: u32;
  }

  /** @name PalletOraclePrePrice (598) */
  interface PalletOraclePrePrice extends Struct {
    readonly price: u128;
    readonly block: u32;
    readonly who: AccountId32;
  }

  /** @name PalletOracleAssetInfo (600) */
  interface PalletOracleAssetInfo extends Struct {
    readonly threshold: Percent;
    readonly minAnswers: u32;
    readonly maxAnswers: u32;
    readonly blockInterval: u32;
    readonly rewardWeight: u128;
    readonly slash: u128;
    readonly emitPriceChanges: bool;
  }

  /** @name PalletOracleError (601) */
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

  /** @name PalletCurrencyFactoryRanges (602) */
  interface PalletCurrencyFactoryRanges extends Struct {
    readonly ranges: Vec<{
      readonly current: u128;
      readonly end: u128;
    } & Struct>;
  }

  /** @name PalletCurrencyFactoryError (605) */
  interface PalletCurrencyFactoryError extends Enum {
    readonly isAssetNotFound: boolean;
    readonly type: 'AssetNotFound';
  }

  /** @name PalletVaultModelsVaultInfo (606) */
  interface PalletVaultModelsVaultInfo extends Struct {
    readonly assetId: u128;
    readonly lpTokenId: u128;
    readonly manager: AccountId32;
    readonly deposit: ComposableTraitsVaultDeposit;
    readonly capabilities: PalletVaultCapabilities;
  }

  /** @name ComposableTraitsVaultDeposit (607) */
  interface ComposableTraitsVaultDeposit extends Enum {
    readonly isExistential: boolean;
    readonly isRent: boolean;
    readonly asRent: {
      readonly amount: u128;
      readonly at: u32;
    } & Struct;
    readonly type: 'Existential' | 'Rent';
  }

  /** @name PalletVaultCapabilities (608) */
  interface PalletVaultCapabilities extends Struct {
    readonly bits: u32;
  }

  /** @name PalletVaultModelsStrategyOverview (610) */
  interface PalletVaultModelsStrategyOverview extends Struct {
    readonly allocation: Perquintill;
    readonly balance: u128;
    readonly lifetimeWithdrawn: u128;
    readonly lifetimeDeposited: u128;
  }

  /** @name PalletVaultError (611) */
  interface PalletVaultError extends Enum {
    readonly isAccountIsNotManager: boolean;
    readonly isCannotCreateAsset: boolean;
    readonly isTransferFromFailed: boolean;
    readonly isMintFailed: boolean;
    readonly isInsufficientLpTokens: boolean;
    readonly isVaultDoesNotExist: boolean;
    readonly isNoFreeVaultAllocation: boolean;
    readonly isAllocationMustSumToOne: boolean;
    readonly isTooManyStrategies: boolean;
    readonly isInsufficientFunds: boolean;
    readonly isAmountMustGteMinimumDeposit: boolean;
    readonly isAmountMustGteMinimumWithdrawal: boolean;
    readonly isNotEnoughLiquidity: boolean;
    readonly isInsufficientCreationDeposit: boolean;
    readonly isInvalidSurchargeClaim: boolean;
    readonly isNotVaultLpToken: boolean;
    readonly isDepositsHalted: boolean;
    readonly isWithdrawalsHalted: boolean;
    readonly isOnlyManagerCanDoThisOperation: boolean;
    readonly isInvalidDeletionClaim: boolean;
    readonly isVaultNotTombstoned: boolean;
    readonly isTombstoneDurationNotExceeded: boolean;
    readonly isInvalidAddSurcharge: boolean;
    readonly type: 'AccountIsNotManager' | 'CannotCreateAsset' | 'TransferFromFailed' | 'MintFailed' | 'InsufficientLpTokens' | 'VaultDoesNotExist' | 'NoFreeVaultAllocation' | 'AllocationMustSumToOne' | 'TooManyStrategies' | 'InsufficientFunds' | 'AmountMustGteMinimumDeposit' | 'AmountMustGteMinimumWithdrawal' | 'NotEnoughLiquidity' | 'InsufficientCreationDeposit' | 'InvalidSurchargeClaim' | 'NotVaultLpToken' | 'DepositsHalted' | 'WithdrawalsHalted' | 'OnlyManagerCanDoThisOperation' | 'InvalidDeletionClaim' | 'VaultNotTombstoned' | 'TombstoneDurationNotExceeded' | 'InvalidAddSurcharge';
  }

  /** @name ComposableTraitsXcmAssetsForeignMetadata (612) */
  interface ComposableTraitsXcmAssetsForeignMetadata extends Struct {
    readonly decimals: Option<u32>;
    readonly location: ComposableTraitsXcmAssetsXcmAssetLocation;
  }

  /** @name PalletAssetsRegistryError (614) */
  interface PalletAssetsRegistryError extends Enum {
    readonly isAssetNotFound: boolean;
    readonly isForeignAssetAlreadyRegistered: boolean;
    readonly type: 'AssetNotFound' | 'ForeignAssetAlreadyRegistered';
  }

  /** @name ComposableTraitsGovernanceSignedRawOrigin (615) */
  interface ComposableTraitsGovernanceSignedRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly type: 'Root' | 'Signed';
  }

  /** @name PalletGovernanceRegistryError (616) */
  interface PalletGovernanceRegistryError extends Enum {
    readonly isNoneError: boolean;
    readonly type: 'NoneError';
  }

  /** @name PalletAssetsError (617) */
  interface PalletAssetsError extends Enum {
    readonly isCannotSetNewCurrencyToRegistry: boolean;
    readonly isInvalidCurrency: boolean;
    readonly type: 'CannotSetNewCurrencyToRegistry' | 'InvalidCurrency';
  }

  /** @name PalletCrowdloanRewardsModelsReward (618) */
  interface PalletCrowdloanRewardsModelsReward extends Struct {
    readonly total: u128;
    readonly claimed: u128;
    readonly vestingPeriod: u64;
  }

  /** @name PalletCrowdloanRewardsError (619) */
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
    readonly type: 'NotInitialized' | 'AlreadyInitialized' | 'BackToTheFuture' | 'RewardsNotFunded' | 'InvalidProof' | 'InvalidClaim' | 'NothingToClaim' | 'NotAssociated' | 'AlreadyAssociated' | 'NotClaimableYet';
  }

  /** @name PalletVestingModuleError (624) */
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

  /** @name PalletBondedFinanceError (626) */
  interface PalletBondedFinanceError extends Enum {
    readonly isBondOfferNotFound: boolean;
    readonly isInvalidBondOffer: boolean;
    readonly isOfferCompleted: boolean;
    readonly isInvalidNumberOfBonds: boolean;
    readonly type: 'BondOfferNotFound' | 'InvalidBondOffer' | 'OfferCompleted' | 'InvalidNumberOfBonds';
  }

  /** @name PalletDutchAuctionTakeOrder (629) */
  interface PalletDutchAuctionTakeOrder extends Struct {
    readonly fromTo: AccountId32;
    readonly take: ComposableTraitsDefiTake;
  }

  /** @name PalletDutchAuctionError (630) */
  interface PalletDutchAuctionError extends Enum {
    readonly isRequestedOrderDoesNotExists: boolean;
    readonly isOrderParametersIsInvalid: boolean;
    readonly isTakeParametersIsInvalid: boolean;
    readonly isTakeLimitDoesNotSatisfyOrder: boolean;
    readonly isOrderNotFound: boolean;
    readonly isTakeOrderDidNotHappen: boolean;
    readonly isNotEnoughNativeCurrencyToPayForAuction: boolean;
    readonly isXcmCannotDecodeRemoteParametersToLocalRepresentations: boolean;
    readonly isXcmCannotFindLocalIdentifiersAsDecodedFromRemote: boolean;
    readonly isXcmNotFoundConfigurationById: boolean;
    readonly type: 'RequestedOrderDoesNotExists' | 'OrderParametersIsInvalid' | 'TakeParametersIsInvalid' | 'TakeLimitDoesNotSatisfyOrder' | 'OrderNotFound' | 'TakeOrderDidNotHappen' | 'NotEnoughNativeCurrencyToPayForAuction' | 'XcmCannotDecodeRemoteParametersToLocalRepresentations' | 'XcmCannotFindLocalIdentifiersAsDecodedFromRemote' | 'XcmNotFoundConfigurationById';
  }

  /** @name PalletMosaicRelayerStaleRelayer (631) */
  interface PalletMosaicRelayerStaleRelayer extends Struct {
    readonly relayer: PalletMosaicRelayerRelayerConfig;
  }

  /** @name PalletMosaicRelayerRelayerConfig (632) */
  interface PalletMosaicRelayerRelayerConfig extends Struct {
    readonly current: AccountId32;
    readonly next: Option<PalletMosaicRelayerNext>;
  }

  /** @name PalletMosaicRelayerNext (634) */
  interface PalletMosaicRelayerNext extends Struct {
    readonly ttl: u32;
    readonly account: AccountId32;
  }

  /** @name PalletMosaicAssetInfo (635) */
  interface PalletMosaicAssetInfo extends Struct {
    readonly lastMintBlock: u32;
    readonly budget: u128;
    readonly penalty: u128;
    readonly penaltyDecayer: PalletMosaicDecayBudgetPenaltyDecayer;
  }

  /** @name PalletMosaicError (640) */
  interface PalletMosaicError extends Enum {
    readonly isRelayerNotSet: boolean;
    readonly isBadTTL: boolean;
    readonly isBadTimelockPeriod: boolean;
    readonly isUnsupportedAsset: boolean;
    readonly isNetworkDisabled: boolean;
    readonly isUnsupportedNetwork: boolean;
    readonly isOverflow: boolean;
    readonly isNoStaleTransactions: boolean;
    readonly isInsufficientBudget: boolean;
    readonly isExceedsMaxTransferSize: boolean;
    readonly isBelowMinTransferSize: boolean;
    readonly isNoClaimableTx: boolean;
    readonly isTxStillLocked: boolean;
    readonly isNoOutgoingTx: boolean;
    readonly isAmountMismatch: boolean;
    readonly isAssetNotMapped: boolean;
    readonly isRemoteAmmIdNotFound: boolean;
    readonly isRemoteAmmIdAlreadyExists: boolean;
    readonly isDestinationAmmIdNotWhitelisted: boolean;
    readonly type: 'RelayerNotSet' | 'BadTTL' | 'BadTimelockPeriod' | 'UnsupportedAsset' | 'NetworkDisabled' | 'UnsupportedNetwork' | 'Overflow' | 'NoStaleTransactions' | 'InsufficientBudget' | 'ExceedsMaxTransferSize' | 'BelowMinTransferSize' | 'NoClaimableTx' | 'TxStillLocked' | 'NoOutgoingTx' | 'AmountMismatch' | 'AssetNotMapped' | 'RemoteAmmIdNotFound' | 'RemoteAmmIdAlreadyExists' | 'DestinationAmmIdNotWhitelisted';
  }

  /** @name PalletLiquidationsError (641) */
  interface PalletLiquidationsError extends Enum {
    readonly isNoLiquidationEngineFound: boolean;
    readonly isInvalidLiquidationStrategiesVector: boolean;
    readonly isOnlyDutchAuctionStrategyIsImplemented: boolean;
    readonly type: 'NoLiquidationEngineFound' | 'InvalidLiquidationStrategiesVector' | 'OnlyDutchAuctionStrategyIsImplemented';
  }

  /** @name ComposableTraitsLendingMarketConfig (642) */
  interface ComposableTraitsLendingMarketConfig extends Struct {
    readonly manager: AccountId32;
    readonly borrowAssetVault: u64;
    readonly collateralAsset: u128;
    readonly maxPriceAge: u32;
    readonly collateralFactor: u128;
    readonly interestRateModel: ComposableTraitsLendingMathInterestRateModel;
    readonly underCollateralizedWarnPercent: Percent;
    readonly liquidators: Vec<u32>;
  }

  /** @name PalletLendingError (644) */
  interface PalletLendingError extends Enum {
    readonly isMarketDoesNotExist: boolean;
    readonly isAccountCollateralAbsent: boolean;
    readonly isInvalidCollateralFactor: boolean;
    readonly isMarketIsClosing: boolean;
    readonly isInvalidTimestampOnBorrowRequest: boolean;
    readonly isNotEnoughCollateralToWithdraw: boolean;
    readonly isWouldGoUnderCollateralized: boolean;
    readonly isNotEnoughCollateralToBorrow: boolean;
    readonly isCannotCalculateBorrowRate: boolean;
    readonly isBorrowAndRepayInSameBlockIsNotSupported: boolean;
    readonly isBorrowDoesNotExist: boolean;
    readonly isExceedLendingCount: boolean;
    readonly isBorrowLimitCalculationFailed: boolean;
    readonly isUnauthorized: boolean;
    readonly isInitialMarketVolumeIncorrect: boolean;
    readonly isCannotRepayZeroBalance: boolean;
    readonly isCannotRepayMoreThanTotalDebt: boolean;
    readonly isBorrowRentDoesNotExist: boolean;
    readonly isPriceTooOld: boolean;
    readonly isCannotIncreaseCollateralFactorOfOpenMarket: boolean;
    readonly isCannotBorrowFromMarketWithUnbalancedVault: boolean;
    readonly type: 'MarketDoesNotExist' | 'AccountCollateralAbsent' | 'InvalidCollateralFactor' | 'MarketIsClosing' | 'InvalidTimestampOnBorrowRequest' | 'NotEnoughCollateralToWithdraw' | 'WouldGoUnderCollateralized' | 'NotEnoughCollateralToBorrow' | 'CannotCalculateBorrowRate' | 'BorrowAndRepayInSameBlockIsNotSupported' | 'BorrowDoesNotExist' | 'ExceedLendingCount' | 'BorrowLimitCalculationFailed' | 'Unauthorized' | 'InitialMarketVolumeIncorrect' | 'CannotRepayZeroBalance' | 'CannotRepayMoreThanTotalDebt' | 'BorrowRentDoesNotExist' | 'PriceTooOld' | 'CannotIncreaseCollateralFactorOfOpenMarket' | 'CannotBorrowFromMarketWithUnbalancedVault';
  }

  /** @name PalletPabloPoolConfiguration (645) */
  interface PalletPabloPoolConfiguration extends Enum {
    readonly isStableSwap: boolean;
    readonly asStableSwap: ComposableTraitsDexStableSwapPoolInfo;
    readonly isConstantProduct: boolean;
    readonly asConstantProduct: ComposableTraitsDexConstantProductPoolInfo;
    readonly isLiquidityBootstrapping: boolean;
    readonly asLiquidityBootstrapping: ComposableTraitsDexLiquidityBootstrappingPoolInfo;
    readonly type: 'StableSwap' | 'ConstantProduct' | 'LiquidityBootstrapping';
  }

  /** @name ComposableTraitsDexStableSwapPoolInfo (646) */
  interface ComposableTraitsDexStableSwapPoolInfo extends Struct {
    readonly owner: AccountId32;
    readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly lpToken: u128;
    readonly amplificationCoefficient: u16;
    readonly feeConfig: ComposableTraitsDexFeeConfig;
  }

  /** @name ComposableTraitsDexConstantProductPoolInfo (647) */
  interface ComposableTraitsDexConstantProductPoolInfo extends Struct {
    readonly owner: AccountId32;
    readonly pair: ComposableTraitsDefiCurrencyPairCurrencyId;
    readonly lpToken: u128;
    readonly feeConfig: ComposableTraitsDexFeeConfig;
    readonly baseWeight: Permill;
    readonly quoteWeight: Permill;
  }

  /** @name PalletPabloTimeWeightedAveragePrice (648) */
  interface PalletPabloTimeWeightedAveragePrice extends Struct {
    readonly timestamp: u64;
    readonly basePriceCumulative: u128;
    readonly quotePriceCumulative: u128;
    readonly baseTwap: u128;
    readonly quoteTwap: u128;
  }

  /** @name PalletPabloPriceCumulative (649) */
  interface PalletPabloPriceCumulative extends Struct {
    readonly timestamp: u64;
    readonly basePriceCumulative: u128;
    readonly quotePriceCumulative: u128;
  }

  /** @name PalletPabloError (650) */
  interface PalletPabloError extends Enum {
    readonly isPoolNotFound: boolean;
    readonly isNotEnoughLiquidity: boolean;
    readonly isNotEnoughLpToken: boolean;
    readonly isPairMismatch: boolean;
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
    readonly type: 'PoolNotFound' | 'NotEnoughLiquidity' | 'NotEnoughLpToken' | 'PairMismatch' | 'MustBeOwner' | 'InvalidSaleState' | 'InvalidAmount' | 'InvalidAsset' | 'CannotRespectMinimumRequested' | 'AssetAmountMustBePositiveNumber' | 'InvalidPair' | 'InvalidFees' | 'AmpFactorMustBeGreaterThanZero' | 'MissingAmount' | 'MissingMinExpectedAmount' | 'MoreThanTwoAssetsNotYetSupported' | 'NoLpTokenForLbp' | 'NoXTokenForLbp' | 'WeightsMustBeNonZero' | 'WeightsMustSumToOne' | 'StakingPoolConfigError';
  }

  /** @name ComposableTraitsDexDexRoute (652) */
  interface ComposableTraitsDexDexRoute extends Enum {
    readonly isDirect: boolean;
    readonly asDirect: Vec<u128>;
    readonly type: 'Direct';
  }

  /** @name DaliRuntimeMaxHopsCount (653) */
  type DaliRuntimeMaxHopsCount = Null;

  /** @name PalletDexRouterError (654) */
  interface PalletDexRouterError extends Enum {
    readonly isMaxHopsExceeded: boolean;
    readonly isNoRouteFound: boolean;
    readonly isUnexpectedNodeFoundWhileValidation: boolean;
    readonly isCanNotRespectMinAmountRequested: boolean;
    readonly isUnsupportedOperation: boolean;
    readonly isLoopSuspectedInRouteUpdate: boolean;
    readonly type: 'MaxHopsExceeded' | 'NoRouteFound' | 'UnexpectedNodeFoundWhileValidation' | 'CanNotRespectMinAmountRequested' | 'UnsupportedOperation' | 'LoopSuspectedInRouteUpdate';
  }

  /** @name PalletFnftError (661) */
  interface PalletFnftError extends Enum {
    readonly isCollectionAlreadyExists: boolean;
    readonly isInstanceAlreadyExists: boolean;
    readonly isCollectionNotFound: boolean;
    readonly isInstanceNotFound: boolean;
    readonly isMustBeOwner: boolean;
    readonly type: 'CollectionAlreadyExists' | 'InstanceAlreadyExists' | 'CollectionNotFound' | 'InstanceNotFound' | 'MustBeOwner';
  }

  /** @name ComposableTraitsStakingRewardPool (662) */
  interface ComposableTraitsStakingRewardPool extends Struct {
    readonly owner: AccountId32;
    readonly assetId: u128;
    readonly rewards: BTreeMap<u128, ComposableTraitsStakingReward>;
    readonly totalShares: u128;
    readonly claimedShares: u128;
    readonly startBlock: u32;
    readonly endBlock: u32;
    readonly lock: ComposableTraitsStakingLockLockConfig;
    readonly shareAssetId: u128;
    readonly financialNftAssetId: u128;
    readonly minimumStakingAmount: u128;
  }

  /** @name ComposableTraitsStakingReward (664) */
  interface ComposableTraitsStakingReward extends Struct {
    readonly totalRewards: u128;
    readonly claimedRewards: u128;
    readonly totalDilutionAdjustment: u128;
    readonly maxRewards: u128;
    readonly rewardRate: ComposableTraitsStakingRewardRate;
    readonly lastUpdatedTimestamp: u64;
  }

  /** @name ComposableTraitsStakingStake (668) */
  interface ComposableTraitsStakingStake extends Struct {
    readonly fnftInstanceId: u64;
    readonly rewardPoolId: u128;
    readonly stake: u128;
    readonly share: u128;
    readonly reductions: BTreeMap<u128, u128>;
    readonly lock: ComposableTraitsStakingLock;
  }

  /** @name ComposableTraitsStakingLock (671) */
  interface ComposableTraitsStakingLock extends Struct {
    readonly startedAt: u64;
    readonly duration: u64;
    readonly unlockPenalty: Perbill;
  }

  /** @name PalletStakingRewardsError (672) */
  interface PalletStakingRewardsError extends Enum {
    readonly isRewardConfigProblem: boolean;
    readonly isInvalidAssetId: boolean;
    readonly isRewardsPoolAlreadyExists: boolean;
    readonly isNoDurationPresetsConfigured: boolean;
    readonly isTooManyRewardAssetTypes: boolean;
    readonly isStartBlockMustBeAfterCurrentBlock: boolean;
    readonly isEndBlockMustBeAfterStartBlock: boolean;
    readonly isUnimplementedRewardPoolConfiguration: boolean;
    readonly isRewardsPoolNotFound: boolean;
    readonly isRewardsPoolHasNotStarted: boolean;
    readonly isReductionConfigProblem: boolean;
    readonly isNotEnoughAssets: boolean;
    readonly isStakeNotFound: boolean;
    readonly isMaxRewardLimitReached: boolean;
    readonly isOnlyStakeOwnerCanInteractWithStake: boolean;
    readonly isRewardAssetNotFound: boolean;
    readonly isBackToTheFuture: boolean;
    readonly isRewardsPotEmpty: boolean;
    readonly isFnftNotFound: boolean;
    readonly isNoDurationPresetsProvided: boolean;
    readonly isSlashedAmountTooLow: boolean;
    readonly isSlashedMinimumStakingAmountTooLow: boolean;
    readonly isStakedAmountTooLow: boolean;
    readonly isStakedAmountTooLowAfterSplit: boolean;
    readonly type: 'RewardConfigProblem' | 'InvalidAssetId' | 'RewardsPoolAlreadyExists' | 'NoDurationPresetsConfigured' | 'TooManyRewardAssetTypes' | 'StartBlockMustBeAfterCurrentBlock' | 'EndBlockMustBeAfterStartBlock' | 'UnimplementedRewardPoolConfiguration' | 'RewardsPoolNotFound' | 'RewardsPoolHasNotStarted' | 'ReductionConfigProblem' | 'NotEnoughAssets' | 'StakeNotFound' | 'MaxRewardLimitReached' | 'OnlyStakeOwnerCanInteractWithStake' | 'RewardAssetNotFound' | 'BackToTheFuture' | 'RewardsPotEmpty' | 'FnftNotFound' | 'NoDurationPresetsProvided' | 'SlashedAmountTooLow' | 'SlashedMinimumStakingAmountTooLow' | 'StakedAmountTooLow' | 'StakedAmountTooLowAfterSplit';
  }

  /** @name PalletCallFilterError (673) */
  interface PalletCallFilterError extends Enum {
    readonly isCannotDisable: boolean;
    readonly isInvalidString: boolean;
    readonly type: 'CannotDisable' | 'InvalidString';
  }

  /** @name PalletIbcPingError (674) */
  interface PalletIbcPingError extends Enum {
    readonly isInvalidParams: boolean;
    readonly isChannelInitError: boolean;
    readonly isPacketSendError: boolean;
    readonly type: 'InvalidParams' | 'ChannelInitError' | 'PacketSendError';
  }

  /** @name IbcTransferError (675) */
  interface IbcTransferError extends Enum {
    readonly isTransferFailed: boolean;
    readonly isUtf8Error: boolean;
    readonly isInvalidAssetId: boolean;
    readonly isInvalidIbcDenom: boolean;
    readonly isInvalidAmount: boolean;
    readonly isInvalidTimestamp: boolean;
    readonly isFailedToGetRevisionNumber: boolean;
    readonly isInvalidParams: boolean;
    readonly isChannelInitError: boolean;
    readonly type: 'TransferFailed' | 'Utf8Error' | 'InvalidAssetId' | 'InvalidIbcDenom' | 'InvalidAmount' | 'InvalidTimestamp' | 'FailedToGetRevisionNumber' | 'InvalidParams' | 'ChannelInitError';
  }

  /** @name PalletIbcIbcConsensusState (677) */
  interface PalletIbcIbcConsensusState extends Struct {
    readonly timestamp: u64;
    readonly commitmentRoot: Bytes;
  }

  /** @name PalletIbcError (681) */
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
    readonly isOther: boolean;
    readonly isInvalidRoute: boolean;
    readonly isInvalidMessageType: boolean;
    readonly type: 'ProcessingError' | 'DecodingError' | 'EncodingError' | 'ProofGenerationError' | 'ConsensusStateNotFound' | 'ChannelNotFound' | 'ClientStateNotFound' | 'ConnectionNotFound' | 'PacketCommitmentNotFound' | 'PacketReceiptNotFound' | 'PacketAcknowledgmentNotFound' | 'SendPacketError' | 'Other' | 'InvalidRoute' | 'InvalidMessageType';
  }

  /** @name PalletCosmwasmCodeInfo (683) */
  interface PalletCosmwasmCodeInfo extends Struct {
    readonly creator: AccountId32;
    readonly pristineCodeHash: H256;
    readonly instrumentationVersion: u16;
    readonly refcount: u32;
  }

  /** @name PalletCosmwasmError (684) */
  interface PalletCosmwasmError extends Enum {
    readonly isInstrumentation: boolean;
    readonly isVmCreation: boolean;
    readonly isContractTrapped: boolean;
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
    readonly isTransferFailed: boolean;
    readonly isChargeGas: boolean;
    readonly isRefundGas: boolean;
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
    readonly type: 'Instrumentation' | 'VmCreation' | 'ContractTrapped' | 'ContractHasNoInfo' | 'CodeDecoding' | 'CodeValidation' | 'CodeEncoding' | 'CodeInstrumentation' | 'InstrumentedCodeIsTooBig' | 'CodeAlreadyExists' | 'CodeNotFound' | 'ContractAlreadyExists' | 'ContractNotFound' | 'TransferFailed' | 'ChargeGas' | 'RefundGas' | 'LabelTooBig' | 'UnknownDenom' | 'StackOverflow' | 'NotEnoughFundsForUpload' | 'NonceOverflow' | 'RefcountOverflow' | 'VmDepthOverflow' | 'SignatureVerificationError' | 'IteratorIdOverflow' | 'IteratorNotFound';
  }

  /** @name FrameSystemExtensionsCheckNonZeroSender (687) */
  type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (688) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (689) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (690) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (693) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (694) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletAssetTxPaymentChargeAssetTxPayment (695) */
  interface PalletAssetTxPaymentChargeAssetTxPayment extends Struct {
    readonly tip: Compact<u128>;
    readonly assetId: Option<u128>;
  }

  /** @name DaliRuntimeRuntime (696) */
  type DaliRuntimeRuntime = Null;

} // declare module
