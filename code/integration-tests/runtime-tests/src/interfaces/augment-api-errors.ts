// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
  interface AugmentedErrors<ApiType extends ApiTypes> {
    assetsRegistry: {
      AssetAlreadyRegistered: AugmentedError<ApiType>;
      AssetLocationIsNone: AugmentedError<ApiType>;
      AssetNotFound: AugmentedError<ApiType>;
      LocationIsUsed: AugmentedError<ApiType>;
      StringExceedsMaxLength: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    assetsTransactorRouter: {
      CannotSetNewCurrencyToRegistry: AugmentedError<ApiType>;
      InvalidCurrency: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    balances: {
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * A vesting schedule already exists for this account
       **/
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /**
       * Balance too low to send value.
       **/
      InsufficientBalance: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Account liquidity restrictions prevent withdrawal
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Number of named reserves exceed MaxReserves
       **/
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Vesting balance too high to send value
       **/
      VestingBalance: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    bondedFinance: {
      /**
       * The offer could not be found.
       **/
      BondOfferNotFound: AugmentedError<ApiType>;
      /**
       * Someone tried  to submit an invalid offer.
       **/
      InvalidBondOffer: AugmentedError<ApiType>;
      /**
       * Someone tried to bond with an invalid number of nb_of_bonds.
       **/
      InvalidNumberOfBonds: AugmentedError<ApiType>;
      /**
       * Someone tried to bond an already completed offer.
       **/
      OfferCompleted: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    callFilter: {
      /**
       * We tried to disable an extrinsic that cannot be disabled.
       **/
      CannotDisable: AugmentedError<ApiType>;
      /**
       * The pallet name is not a valid UTF8 string.
       **/
      InvalidString: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    collatorSelection: {
      /**
       * User is already a candidate
       **/
      AlreadyCandidate: AugmentedError<ApiType>;
      /**
       * User is already an Invulnerable
       **/
      AlreadyInvulnerable: AugmentedError<ApiType>;
      /**
       * Account has no associated validator ID
       **/
      NoAssociatedValidatorId: AugmentedError<ApiType>;
      /**
       * User is not a candidate
       **/
      NotCandidate: AugmentedError<ApiType>;
      /**
       * Permission issue
       **/
      Permission: AugmentedError<ApiType>;
      /**
       * Too few candidates
       **/
      TooFewCandidates: AugmentedError<ApiType>;
      /**
       * Too many candidates
       **/
      TooManyCandidates: AugmentedError<ApiType>;
      /**
       * Too many invulnerables
       **/
      TooManyInvulnerables: AugmentedError<ApiType>;
      /**
       * Unknown error
       **/
      Unknown: AugmentedError<ApiType>;
      /**
       * Validator ID is not yet registered
       **/
      ValidatorNotRegistered: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    convictionVoting: {
      /**
       * The account is already delegating.
       **/
      AlreadyDelegating: AugmentedError<ApiType>;
      /**
       * The account currently has votes attached to it and the operation cannot succeed until
       * these are removed, either through `unvote` or `reap_vote`.
       **/
      AlreadyVoting: AugmentedError<ApiType>;
      /**
       * The class ID supplied is invalid.
       **/
      BadClass: AugmentedError<ApiType>;
      /**
       * The class must be supplied since it is not easily determinable from the state.
       **/
      ClassNeeded: AugmentedError<ApiType>;
      /**
       * Too high a balance was provided that the account cannot afford.
       **/
      InsufficientFunds: AugmentedError<ApiType>;
      /**
       * Maximum number of votes reached.
       **/
      MaxVotesReached: AugmentedError<ApiType>;
      /**
       * Delegation to oneself makes no sense.
       **/
      Nonsense: AugmentedError<ApiType>;
      /**
       * The actor has no permission to conduct the action.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * The actor has no permission to conduct the action right now but will do in the future.
       **/
      NoPermissionYet: AugmentedError<ApiType>;
      /**
       * The account is not currently delegating.
       **/
      NotDelegating: AugmentedError<ApiType>;
      /**
       * Poll is not ongoing.
       **/
      NotOngoing: AugmentedError<ApiType>;
      /**
       * The given account did not vote on the poll.
       **/
      NotVoter: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    cosmwasm: {
      Aborted: AugmentedError<ApiType>;
      AccountConversionFailure: AugmentedError<ApiType>;
      AssetConversion: AugmentedError<ApiType>;
      CodeAlreadyExists: AugmentedError<ApiType>;
      CodeDecoding: AugmentedError<ApiType>;
      CodeEncoding: AugmentedError<ApiType>;
      CodeInstrumentation: AugmentedError<ApiType>;
      CodeNotFound: AugmentedError<ApiType>;
      CodeValidation: AugmentedError<ApiType>;
      ContractAlreadyExists: AugmentedError<ApiType>;
      ContractHasNoInfo: AugmentedError<ApiType>;
      ContractNotFound: AugmentedError<ApiType>;
      ExecuteDeserialize: AugmentedError<ApiType>;
      ExecuteSerialize: AugmentedError<ApiType>;
      FailedToSerialize: AugmentedError<ApiType>;
      Ibc: AugmentedError<ApiType>;
      Instrumentation: AugmentedError<ApiType>;
      InstrumentedCodeIsTooBig: AugmentedError<ApiType>;
      Interpreter: AugmentedError<ApiType>;
      InvalidAccount: AugmentedError<ApiType>;
      InvalidGasCheckpoint: AugmentedError<ApiType>;
      InvalidSalt: AugmentedError<ApiType>;
      IteratorIdOverflow: AugmentedError<ApiType>;
      IteratorNotFound: AugmentedError<ApiType>;
      IteratorValueNotFound: AugmentedError<ApiType>;
      LabelTooBig: AugmentedError<ApiType>;
      NonceOverflow: AugmentedError<ApiType>;
      NotAuthorized: AugmentedError<ApiType>;
      NotEnoughFundsForUpload: AugmentedError<ApiType>;
      NotImplemented: AugmentedError<ApiType>;
      OutOfGas: AugmentedError<ApiType>;
      Precompile: AugmentedError<ApiType>;
      QueryDeserialize: AugmentedError<ApiType>;
      ReadOnlyViolation: AugmentedError<ApiType>;
      RefcountOverflow: AugmentedError<ApiType>;
      Rpc: AugmentedError<ApiType>;
      SignatureVerificationError: AugmentedError<ApiType>;
      StackOverflow: AugmentedError<ApiType>;
      SubstrateDispatch: AugmentedError<ApiType>;
      TransferFailed: AugmentedError<ApiType>;
      UnknownDenom: AugmentedError<ApiType>;
      Unsupported: AugmentedError<ApiType>;
      VirtualMachine: AugmentedError<ApiType>;
      VmCreation: AugmentedError<ApiType>;
      VMDepthOverflow: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    council: {
      /**
       * Members are already initialized!
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * Duplicate proposals not allowed
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * Duplicate vote ignored
       **/
      DuplicateVote: AugmentedError<ApiType>;
      /**
       * Account is not a member
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Proposal must exist
       **/
      ProposalMissing: AugmentedError<ApiType>;
      /**
       * The close call was made too early, before the end of the voting.
       **/
      TooEarly: AugmentedError<ApiType>;
      /**
       * There can only be a maximum of `MaxProposals` active proposals.
       **/
      TooManyProposals: AugmentedError<ApiType>;
      /**
       * Mismatched index
       **/
      WrongIndex: AugmentedError<ApiType>;
      /**
       * The given length bound for the proposal was too low.
       **/
      WrongProposalLength: AugmentedError<ApiType>;
      /**
       * The given weight bound for the proposal was too low.
       **/
      WrongProposalWeight: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    councilMembership: {
      /**
       * Already a member.
       **/
      AlreadyMember: AugmentedError<ApiType>;
      /**
       * Not a member.
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Too many members.
       **/
      TooManyMembers: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    crowdloanRewards: {
      AlreadyAssociated: AugmentedError<ApiType>;
      AlreadyInitialized: AugmentedError<ApiType>;
      BackToTheFuture: AugmentedError<ApiType>;
      InvalidClaim: AugmentedError<ApiType>;
      InvalidProof: AugmentedError<ApiType>;
      NotAssociated: AugmentedError<ApiType>;
      NotClaimableYet: AugmentedError<ApiType>;
      NothingToClaim: AugmentedError<ApiType>;
      NotInitialized: AugmentedError<ApiType>;
      RewardsNotFunded: AugmentedError<ApiType>;
      /**
       * Returned by `delete` if the provided expected reward mismatches the actual reward.
       **/
      UnexpectedRewardAmount: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    cumulusXcm: {
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    currencyFactory: {
      AssetNotFound: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    democracy: {
      /**
       * Cannot cancel the same proposal twice
       **/
      AlreadyCanceled: AugmentedError<ApiType>;
      /**
       * The account is already delegating.
       **/
      AlreadyDelegating: AugmentedError<ApiType>;
      /**
       * Identity may not veto a proposal twice
       **/
      AlreadyVetoed: AugmentedError<ApiType>;
      /**
       * Proposal already made
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * The instant referendum origin is currently disallowed.
       **/
      InstantNotAllowed: AugmentedError<ApiType>;
      /**
       * Too high a balance was provided that the account cannot afford.
       **/
      InsufficientFunds: AugmentedError<ApiType>;
      /**
       * Invalid hash
       **/
      InvalidHash: AugmentedError<ApiType>;
      /**
       * Maximum number of votes reached.
       **/
      MaxVotesReached: AugmentedError<ApiType>;
      /**
       * No proposals waiting
       **/
      NoneWaiting: AugmentedError<ApiType>;
      /**
       * Delegation to oneself makes no sense.
       **/
      Nonsense: AugmentedError<ApiType>;
      /**
       * The actor has no permission to conduct the action.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * No external proposal
       **/
      NoProposal: AugmentedError<ApiType>;
      /**
       * The account is not currently delegating.
       **/
      NotDelegating: AugmentedError<ApiType>;
      /**
       * Next external proposal not simple majority
       **/
      NotSimpleMajority: AugmentedError<ApiType>;
      /**
       * The given account did not vote on the referendum.
       **/
      NotVoter: AugmentedError<ApiType>;
      /**
       * The preimage does not exist.
       **/
      PreimageNotExist: AugmentedError<ApiType>;
      /**
       * Proposal still blacklisted
       **/
      ProposalBlacklisted: AugmentedError<ApiType>;
      /**
       * Proposal does not exist
       **/
      ProposalMissing: AugmentedError<ApiType>;
      /**
       * Vote given for invalid referendum
       **/
      ReferendumInvalid: AugmentedError<ApiType>;
      /**
       * Maximum number of items reached.
       **/
      TooMany: AugmentedError<ApiType>;
      /**
       * Value too low
       **/
      ValueLow: AugmentedError<ApiType>;
      /**
       * The account currently has votes attached to it and the operation cannot succeed until
       * these are removed, either through `unvote` or `reap_vote`.
       **/
      VotesExist: AugmentedError<ApiType>;
      /**
       * Voting period too low
       **/
      VotingPeriodLow: AugmentedError<ApiType>;
      /**
       * Invalid upper bound.
       **/
      WrongUpperBound: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    dmpQueue: {
      /**
       * The amount of weight given is possibly not enough for executing the message.
       **/
      OverLimit: AugmentedError<ApiType>;
      /**
       * The message index given is unknown.
       **/
      Unknown: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    farming: {
      InsufficientStake: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    farmingRewards: {
      /**
       * Balance not sufficient to withdraw stake.
       **/
      InsufficientFunds: AugmentedError<ApiType>;
      /**
       * Maximum rewards currencies reached.
       **/
      MaxRewardCurrencies: AugmentedError<ApiType>;
      /**
       * Unable to convert value.
       **/
      TryIntoIntError: AugmentedError<ApiType>;
      /**
       * Cannot distribute rewards without stake.
       **/
      ZeroTotalStake: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    ibc: {
      /**
       * Access denied
       **/
      AccessDenied: AugmentedError<ApiType>;
      /**
       * Failed to derive channel escrow address
       **/
      ChannelEscrowAddress: AugmentedError<ApiType>;
      /**
       * Error opening channel
       **/
      ChannelInitError: AugmentedError<ApiType>;
      /**
       * Channel not found
       **/
      ChannelNotFound: AugmentedError<ApiType>;
      /**
       * Error Freezing client
       **/
      ClientFreezeFailed: AugmentedError<ApiType>;
      /**
       * Client state not found
       **/
      ClientStateNotFound: AugmentedError<ApiType>;
      /**
       * Client update time and height not found
       **/
      ClientUpdateNotFound: AugmentedError<ApiType>;
      /**
       * Connection not found
       **/
      ConnectionNotFound: AugmentedError<ApiType>;
      /**
       * Client consensus state not found for height
       **/
      ConsensusStateNotFound: AugmentedError<ApiType>;
      /**
       * Error decoding some type
       **/
      DecodingError: AugmentedError<ApiType>;
      /**
       * Error encoding some type
       **/
      EncodingError: AugmentedError<ApiType>;
      FailedSendFeeToAccount: AugmentedError<ApiType>;
      /**
       * Unable to get client revision number
       **/
      FailedToGetRevisionNumber: AugmentedError<ApiType>;
      /**
       * Invalid amount
       **/
      InvalidAmount: AugmentedError<ApiType>;
      /**
       * Invalid asset id
       **/
      InvalidAssetId: AugmentedError<ApiType>;
      /**
       * Invalid channel id
       **/
      InvalidChannelId: AugmentedError<ApiType>;
      /**
       * Invalid message for extrinsic
       **/
      InvalidMessageType: AugmentedError<ApiType>;
      /**
       * Invalid params passed
       **/
      InvalidParams: AugmentedError<ApiType>;
      /**
       * Invalid port id
       **/
      InvalidPortId: AugmentedError<ApiType>;
      /**
       * Invalid route
       **/
      InvalidRoute: AugmentedError<ApiType>;
      /**
       * Invalid timestamp
       **/
      InvalidTimestamp: AugmentedError<ApiType>;
      OriginAddress: AugmentedError<ApiType>;
      /**
       * Other forms of errors
       **/
      Other: AugmentedError<ApiType>;
      /**
       * Packet Acknowledgment wasn't found
       **/
      PacketAcknowledgmentNotFound: AugmentedError<ApiType>;
      /**
       * Packet commitment wasn't found
       **/
      PacketCommitmentNotFound: AugmentedError<ApiType>;
      /**
       * Packet receipt wasn't found
       **/
      PacketReceiptNotFound: AugmentedError<ApiType>;
      /**
       * Invalid Ibc denom
       **/
      PrefixedDenomParse: AugmentedError<ApiType>;
      /**
       * Error processing ibc messages
       **/
      ProcessingError: AugmentedError<ApiType>;
      /**
       * Error generating trie proof
       **/
      ProofGenerationError: AugmentedError<ApiType>;
      RateLimiter: AugmentedError<ApiType>;
      /**
       * Error constructing packet
       **/
      SendPacketError: AugmentedError<ApiType>;
      /**
       * Latest height and timestamp for a client not found
       **/
      TimestampAndHeightNotFound: AugmentedError<ApiType>;
      /**
       * The interchain token transfer was not successfully initiated
       **/
      TransferInternals: AugmentedError<ApiType>;
      TransferOther: AugmentedError<ApiType>;
      TransferProtocol: AugmentedError<ApiType>;
      TransferSend: AugmentedError<ApiType>;
      TransferSerde: AugmentedError<ApiType>;
      /**
       * Error Decoding utf8 bytes
       **/
      Utf8Error: AugmentedError<ApiType>;
      /**
       * Error writing acknowledgement to storage
       **/
      WriteAckError: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    identity: {
      /**
       * Account ID is already named.
       **/
      AlreadyClaimed: AugmentedError<ApiType>;
      /**
       * Empty index.
       **/
      EmptyIndex: AugmentedError<ApiType>;
      /**
       * Fee is changed.
       **/
      FeeChanged: AugmentedError<ApiType>;
      /**
       * The index is invalid.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Invalid judgement.
       **/
      InvalidJudgement: AugmentedError<ApiType>;
      /**
       * The target is invalid.
       **/
      InvalidTarget: AugmentedError<ApiType>;
      /**
       * The provided judgement was for a different identity.
       **/
      JudgementForDifferentIdentity: AugmentedError<ApiType>;
      /**
       * Judgement given.
       **/
      JudgementGiven: AugmentedError<ApiType>;
      /**
       * Error that occurs when there is an issue paying for judgement.
       **/
      JudgementPaymentFailed: AugmentedError<ApiType>;
      /**
       * No identity found.
       **/
      NoIdentity: AugmentedError<ApiType>;
      /**
       * Account isn't found.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Account isn't named.
       **/
      NotNamed: AugmentedError<ApiType>;
      /**
       * Sub-account isn't owned by sender.
       **/
      NotOwned: AugmentedError<ApiType>;
      /**
       * Sender is not a sub-account.
       **/
      NotSub: AugmentedError<ApiType>;
      /**
       * Sticky judgement.
       **/
      StickyJudgement: AugmentedError<ApiType>;
      /**
       * Too many additional fields.
       **/
      TooManyFields: AugmentedError<ApiType>;
      /**
       * Maximum amount of registrars reached. Cannot add any more.
       **/
      TooManyRegistrars: AugmentedError<ApiType>;
      /**
       * Too many subs-accounts.
       **/
      TooManySubAccounts: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    indices: {
      /**
       * The index was not available.
       **/
      InUse: AugmentedError<ApiType>;
      /**
       * The index was not already assigned.
       **/
      NotAssigned: AugmentedError<ApiType>;
      /**
       * The index is assigned to another account.
       **/
      NotOwner: AugmentedError<ApiType>;
      /**
       * The source and destination accounts are identical.
       **/
      NotTransfer: AugmentedError<ApiType>;
      /**
       * The index is permanent and may not be freed/changed.
       **/
      Permanent: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    multisig: {
      /**
       * Call is already approved by this signatory.
       **/
      AlreadyApproved: AugmentedError<ApiType>;
      /**
       * The data to be stored is already stored.
       **/
      AlreadyStored: AugmentedError<ApiType>;
      /**
       * The maximum weight information provided was too low.
       **/
      MaxWeightTooLow: AugmentedError<ApiType>;
      /**
       * Threshold must be 2 or greater.
       **/
      MinimumThreshold: AugmentedError<ApiType>;
      /**
       * Call doesn't need any (more) approvals.
       **/
      NoApprovalsNeeded: AugmentedError<ApiType>;
      /**
       * Multisig operation not found when attempting to cancel.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * No timepoint was given, yet the multisig operation is already underway.
       **/
      NoTimepoint: AugmentedError<ApiType>;
      /**
       * Only the account that originally created the multisig is able to cancel it.
       **/
      NotOwner: AugmentedError<ApiType>;
      /**
       * The sender was contained in the other signatories; it shouldn't be.
       **/
      SenderInSignatories: AugmentedError<ApiType>;
      /**
       * The signatories were provided out of order; they should be ordered.
       **/
      SignatoriesOutOfOrder: AugmentedError<ApiType>;
      /**
       * There are too few signatories in the list.
       **/
      TooFewSignatories: AugmentedError<ApiType>;
      /**
       * There are too many signatories in the list.
       **/
      TooManySignatories: AugmentedError<ApiType>;
      /**
       * A timepoint was given, yet no multisig operation is underway.
       **/
      UnexpectedTimepoint: AugmentedError<ApiType>;
      /**
       * A different timepoint was given to the multisig operation that is underway.
       **/
      WrongTimepoint: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    openGovBalances: {
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * A vesting schedule already exists for this account
       **/
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /**
       * Balance too low to send value.
       **/
      InsufficientBalance: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Account liquidity restrictions prevent withdrawal
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Number of named reserves exceed MaxReserves
       **/
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Vesting balance too high to send value
       **/
      VestingBalance: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    oracle: {
      /**
       * Signer has already been set
       **/
      AlreadySet: AugmentedError<ApiType>;
      /**
       * Price already submitted
       **/
      AlreadySubmitted: AugmentedError<ApiType>;
      /**
       * Annual rewarding cost too high
       **/
      AnnualRewardLessThanAlreadyRewarded: AugmentedError<ApiType>;
      ArithmeticError: AugmentedError<ApiType>;
      /**
       * Error avoids a panic
       **/
      AvoidPanic: AugmentedError<ApiType>;
      /**
       * Block interval is less then stale price
       **/
      BlockIntervalLength: AugmentedError<ApiType>;
      /**
       * This controller is already in use
       **/
      ControllerUsed: AugmentedError<ApiType>;
      /**
       * Too many weighted averages requested
       **/
      DepthTooLarge: AugmentedError<ApiType>;
      /**
       * Asset count exceeded
       **/
      ExceedAssetsCount: AugmentedError<ApiType>;
      /**
       * Max answers have been exceeded
       **/
      ExceedMaxAnswers: AugmentedError<ApiType>;
      /**
       * Stake exceeded
       **/
      ExceedStake: AugmentedError<ApiType>;
      /**
       * Threshold exceeded
       **/
      ExceedThreshold: AugmentedError<ApiType>;
      /**
       * Invalid asset id
       **/
      InvalidAssetId: AugmentedError<ApiType>;
      /**
       * Invalid min answers
       **/
      InvalidMinAnswers: AugmentedError<ApiType>;
      MaxAnswersLessThanMinAnswers: AugmentedError<ApiType>;
      MaxHistory: AugmentedError<ApiType>;
      MaxPrePrices: AugmentedError<ApiType>;
      /**
       * Max prices already reached
       **/
      MaxPrices: AugmentedError<ApiType>;
      /**
       * Price weight must sum to 100
       **/
      MustSumTo100: AugmentedError<ApiType>;
      /**
       * No Permission
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * Rewarding has not started
       **/
      NoRewardTrackerSet: AugmentedError<ApiType>;
      /**
       * No stake for oracle
       **/
      NoStake: AugmentedError<ApiType>;
      /**
       * Not Enough Funds to complete action
       **/
      NotEnoughFunds: AugmentedError<ApiType>;
      /**
       * Not enough oracle stake for action
       **/
      NotEnoughStake: AugmentedError<ApiType>;
      /**
       * Price not found
       **/
      PriceNotFound: AugmentedError<ApiType>;
      /**
       * Price has not been requested
       **/
      PriceNotRequested: AugmentedError<ApiType>;
      /**
       * This signer is already in use
       **/
      SignerUsed: AugmentedError<ApiType>;
      /**
       * Stake is locked try again later
       **/
      StakeLocked: AugmentedError<ApiType>;
      /**
       * There was an error transferring
       **/
      TransferError: AugmentedError<ApiType>;
      /**
       * Unknown
       **/
      Unknown: AugmentedError<ApiType>;
      /**
       * No controller has been set
       **/
      UnsetController: AugmentedError<ApiType>;
      /**
       * Signer has not been set
       **/
      UnsetSigner: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    pablo: {
      AmpFactorMustBeGreaterThanZero: AugmentedError<ApiType>;
      AssetAmountMustBePositiveNumber: AugmentedError<ApiType>;
      AssetNotFound: AugmentedError<ApiType>;
      /**
       * Cannot buy an asset with itself.
       **/
      CannotBuyAssetWithItself: AugmentedError<ApiType>;
      CannotRespectMinimumRequested: AugmentedError<ApiType>;
      /**
       * Cannot swap an asset with itself.
       **/
      CannotSwapSameAsset: AugmentedError<ApiType>;
      IncorrectAssetAmounts: AugmentedError<ApiType>;
      IncorrectPoolConfig: AugmentedError<ApiType>;
      InitialDepositCannotBeZero: AugmentedError<ApiType>;
      InitialDepositMustContainAllAssets: AugmentedError<ApiType>;
      InvalidAmount: AugmentedError<ApiType>;
      InvalidAsset: AugmentedError<ApiType>;
      InvalidFees: AugmentedError<ApiType>;
      InvalidPair: AugmentedError<ApiType>;
      InvalidSaleState: AugmentedError<ApiType>;
      /**
       * The `min_amounts` map passed to `remove_liquidity` must contain at least one asset.
       **/
      MinAmountsMustContainAtLeastOneAsset: AugmentedError<ApiType>;
      MissingAmount: AugmentedError<ApiType>;
      MissingMinExpectedAmount: AugmentedError<ApiType>;
      MoreThanTwoAssetsNotYetSupported: AugmentedError<ApiType>;
      MustBeOwner: AugmentedError<ApiType>;
      /**
       * The `assets` map passed to `add_liquidity` must contain at least one asset.
       **/
      MustDepositMinimumOneAsset: AugmentedError<ApiType>;
      NoLpTokenForLbp: AugmentedError<ApiType>;
      NotEnoughLiquidity: AugmentedError<ApiType>;
      NotEnoughLpToken: AugmentedError<ApiType>;
      NoXTokenForLbp: AugmentedError<ApiType>;
      PairMismatch: AugmentedError<ApiType>;
      PoolNotFound: AugmentedError<ApiType>;
      StakingPoolConfigError: AugmentedError<ApiType>;
      UnsupportedOperation: AugmentedError<ApiType>;
      WeightsMustBeNonZero: AugmentedError<ApiType>;
      WeightsMustSumToOne: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    palletMultihopXcmIbc: {
      DoesNotSupportNonFungible: AugmentedError<ApiType>;
      FailedToConstructMemo: AugmentedError<ApiType>;
      FailedToDecodeAccountId: AugmentedError<ApiType>;
      FailedToEncodeBech32Address: AugmentedError<ApiType>;
      IncorrectAddress: AugmentedError<ApiType>;
      IncorrectChainName: AugmentedError<ApiType>;
      IncorrectCountOfAddresses: AugmentedError<ApiType>;
      IncorrectMultiLocation: AugmentedError<ApiType>;
      MultiHopRouteDoesNotExist: AugmentedError<ApiType>;
      XcmDepositFailed: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    parachainSystem: {
      /**
       * The inherent which supplies the host configuration did not run this block
       **/
      HostConfigurationNotAvailable: AugmentedError<ApiType>;
      /**
       * No code upgrade has been authorized.
       **/
      NothingAuthorized: AugmentedError<ApiType>;
      /**
       * No validation function upgrade is currently scheduled.
       **/
      NotScheduled: AugmentedError<ApiType>;
      /**
       * Attempt to upgrade validation function while existing upgrade pending
       **/
      OverlappingUpgrades: AugmentedError<ApiType>;
      /**
       * Polkadot currently prohibits this parachain from upgrading its validation function
       **/
      ProhibitedByPolkadot: AugmentedError<ApiType>;
      /**
       * The supplied validation function has compiled into a blob larger than Polkadot is
       * willing to run
       **/
      TooBig: AugmentedError<ApiType>;
      /**
       * The given code upgrade has not been authorized.
       **/
      Unauthorized: AugmentedError<ApiType>;
      /**
       * The inherent which supplies the validation data did not run this block
       **/
      ValidationDataNotAvailable: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    polkadotXcm: {
      /**
       * The given account is not an identifiable sovereign account for any location.
       **/
      AccountNotSovereign: AugmentedError<ApiType>;
      /**
       * The location is invalid since it already has a subscription from us.
       **/
      AlreadySubscribed: AugmentedError<ApiType>;
      /**
       * The given location could not be used (e.g. because it cannot be expressed in the
       * desired version of XCM).
       **/
      BadLocation: AugmentedError<ApiType>;
      /**
       * The version of the `Versioned` value used is not able to be interpreted.
       **/
      BadVersion: AugmentedError<ApiType>;
      /**
       * Could not re-anchor the assets to declare the fees for the destination chain.
       **/
      CannotReanchor: AugmentedError<ApiType>;
      /**
       * The destination `MultiLocation` provided cannot be inverted.
       **/
      DestinationNotInvertible: AugmentedError<ApiType>;
      /**
       * The assets to be sent are empty.
       **/
      Empty: AugmentedError<ApiType>;
      /**
       * The operation required fees to be paid which the initiator could not meet.
       **/
      FeesNotMet: AugmentedError<ApiType>;
      /**
       * The message execution fails the filter.
       **/
      Filtered: AugmentedError<ApiType>;
      /**
       * The unlock operation cannot succeed because there are still users of the lock.
       **/
      InUse: AugmentedError<ApiType>;
      /**
       * Invalid asset for the operation.
       **/
      InvalidAsset: AugmentedError<ApiType>;
      /**
       * Origin is invalid for sending.
       **/
      InvalidOrigin: AugmentedError<ApiType>;
      /**
       * A remote lock with the corresponding data could not be found.
       **/
      LockNotFound: AugmentedError<ApiType>;
      /**
       * The owner does not own (all) of the asset that they wish to do the operation on.
       **/
      LowBalance: AugmentedError<ApiType>;
      /**
       * The referenced subscription could not be found.
       **/
      NoSubscription: AugmentedError<ApiType>;
      /**
       * There was some other issue (i.e. not to do with routing) in sending the message. Perhaps
       * a lack of space for buffering the message.
       **/
      SendFailure: AugmentedError<ApiType>;
      /**
       * Too many assets have been attempted for transfer.
       **/
      TooManyAssets: AugmentedError<ApiType>;
      /**
       * The asset owner has too many locks on the asset.
       **/
      TooManyLocks: AugmentedError<ApiType>;
      /**
       * The desired destination was unreachable, generally because there is a no way of routing
       * to it.
       **/
      Unreachable: AugmentedError<ApiType>;
      /**
       * The message's weight could not be determined.
       **/
      UnweighableMessage: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    preimage: {
      /**
       * Preimage has already been noted on-chain.
       **/
      AlreadyNoted: AugmentedError<ApiType>;
      /**
       * The user is not authorized to perform this action.
       **/
      NotAuthorized: AugmentedError<ApiType>;
      /**
       * The preimage cannot be removed since it has not yet been noted.
       **/
      NotNoted: AugmentedError<ApiType>;
      /**
       * The preimage request cannot be removed since no outstanding requests exist.
       **/
      NotRequested: AugmentedError<ApiType>;
      /**
       * A preimage may not be removed when there are outstanding requests.
       **/
      Requested: AugmentedError<ApiType>;
      /**
       * Preimage is too large to store on-chain.
       **/
      TooBig: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    proxy: {
      /**
       * Account is already a proxy.
       **/
      Duplicate: AugmentedError<ApiType>;
      /**
       * Call may not be made by proxy because it may escalate its privileges.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * Cannot add self as proxy.
       **/
      NoSelfProxy: AugmentedError<ApiType>;
      /**
       * Proxy registration not found.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Sender is not a proxy of the account to be proxied.
       **/
      NotProxy: AugmentedError<ApiType>;
      /**
       * There are too many proxies registered or too many announcements pending.
       **/
      TooMany: AugmentedError<ApiType>;
      /**
       * Announcement, if made at all, was made too recently.
       **/
      Unannounced: AugmentedError<ApiType>;
      /**
       * A call which is incompatible with the proxy type's filter was attempted.
       **/
      Unproxyable: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    referenda: {
      /**
       * The referendum index provided is invalid in this context.
       **/
      BadReferendum: AugmentedError<ApiType>;
      /**
       * The referendum status is invalid for this operation.
       **/
      BadStatus: AugmentedError<ApiType>;
      /**
       * The track identifier given was invalid.
       **/
      BadTrack: AugmentedError<ApiType>;
      /**
       * There are already a full complement of referenda in progress for this track.
       **/
      Full: AugmentedError<ApiType>;
      /**
       * Referendum's decision deposit is already paid.
       **/
      HasDeposit: AugmentedError<ApiType>;
      /**
       * The deposit cannot be refunded since none was made.
       **/
      NoDeposit: AugmentedError<ApiType>;
      /**
       * The deposit refunder is not the depositor.
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * There was nothing to do in the advancement.
       **/
      NothingToDo: AugmentedError<ApiType>;
      /**
       * Referendum is not ongoing.
       **/
      NotOngoing: AugmentedError<ApiType>;
      /**
       * No track exists for the proposal origin.
       **/
      NoTrack: AugmentedError<ApiType>;
      /**
       * The preimage does not exist.
       **/
      PreimageNotExist: AugmentedError<ApiType>;
      /**
       * The queue of the track is empty.
       **/
      QueueEmpty: AugmentedError<ApiType>;
      /**
       * Any deposit cannot be refunded until after the decision is over.
       **/
      Unfinished: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    releaseCommittee: {
      /**
       * Members are already initialized!
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * Duplicate proposals not allowed
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * Duplicate vote ignored
       **/
      DuplicateVote: AugmentedError<ApiType>;
      /**
       * Account is not a member
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Proposal must exist
       **/
      ProposalMissing: AugmentedError<ApiType>;
      /**
       * The close call was made too early, before the end of the voting.
       **/
      TooEarly: AugmentedError<ApiType>;
      /**
       * There can only be a maximum of `MaxProposals` active proposals.
       **/
      TooManyProposals: AugmentedError<ApiType>;
      /**
       * Mismatched index
       **/
      WrongIndex: AugmentedError<ApiType>;
      /**
       * The given length bound for the proposal was too low.
       **/
      WrongProposalLength: AugmentedError<ApiType>;
      /**
       * The given weight bound for the proposal was too low.
       **/
      WrongProposalWeight: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    releaseMembership: {
      /**
       * Already a member.
       **/
      AlreadyMember: AugmentedError<ApiType>;
      /**
       * Not a member.
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Too many members.
       **/
      TooManyMembers: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    scheduler: {
      /**
       * Failed to schedule a call
       **/
      FailedToSchedule: AugmentedError<ApiType>;
      /**
       * Attempt to use a non-named function on a named task.
       **/
      Named: AugmentedError<ApiType>;
      /**
       * Cannot find the scheduled call.
       **/
      NotFound: AugmentedError<ApiType>;
      /**
       * Reschedule failed because it does not change scheduled time.
       **/
      RescheduleNoChange: AugmentedError<ApiType>;
      /**
       * Given target block number is in the past.
       **/
      TargetBlockNumberInPast: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    session: {
      /**
       * Registered duplicate key.
       **/
      DuplicatedKey: AugmentedError<ApiType>;
      /**
       * Invalid ownership proof.
       **/
      InvalidProof: AugmentedError<ApiType>;
      /**
       * Key setting account is not live, so it's impossible to associate keys.
       **/
      NoAccount: AugmentedError<ApiType>;
      /**
       * No associated validator ID for account.
       **/
      NoAssociatedValidatorId: AugmentedError<ApiType>;
      /**
       * No keys are associated with this account.
       **/
      NoKeys: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    sudo: {
      /**
       * Sender must be the Sudo account
       **/
      RequireSudo: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    system: {
      /**
       * The origin filter prevent the call to be dispatched.
       **/
      CallFiltered: AugmentedError<ApiType>;
      /**
       * Failed to extract the runtime version from the new runtime.
       * 
       * Either calling `Core_version` or decoding `RuntimeVersion` failed.
       **/
      FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
      /**
       * The name of specification does not match between the current runtime
       * and the new runtime.
       **/
      InvalidSpecName: AugmentedError<ApiType>;
      /**
       * Suicide called when the account has non-default composite data.
       **/
      NonDefaultComposite: AugmentedError<ApiType>;
      /**
       * There is a non-zero reference count preventing the account from being purged.
       **/
      NonZeroRefCount: AugmentedError<ApiType>;
      /**
       * The specification version is not allowed to decrease between the current runtime
       * and the new runtime.
       **/
      SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    technicalCommittee: {
      /**
       * Members are already initialized!
       **/
      AlreadyInitialized: AugmentedError<ApiType>;
      /**
       * Duplicate proposals not allowed
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * Duplicate vote ignored
       **/
      DuplicateVote: AugmentedError<ApiType>;
      /**
       * Account is not a member
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Proposal must exist
       **/
      ProposalMissing: AugmentedError<ApiType>;
      /**
       * The close call was made too early, before the end of the voting.
       **/
      TooEarly: AugmentedError<ApiType>;
      /**
       * There can only be a maximum of `MaxProposals` active proposals.
       **/
      TooManyProposals: AugmentedError<ApiType>;
      /**
       * Mismatched index
       **/
      WrongIndex: AugmentedError<ApiType>;
      /**
       * The given length bound for the proposal was too low.
       **/
      WrongProposalLength: AugmentedError<ApiType>;
      /**
       * The given weight bound for the proposal was too low.
       **/
      WrongProposalWeight: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    technicalCommitteeMembership: {
      /**
       * Already a member.
       **/
      AlreadyMember: AugmentedError<ApiType>;
      /**
       * Not a member.
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Too many members.
       **/
      TooManyMembers: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    tokens: {
      /**
       * Cannot convert Amount into Balance type
       **/
      AmountIntoBalanceFailed: AugmentedError<ApiType>;
      /**
       * The balance is too low
       **/
      BalanceTooLow: AugmentedError<ApiType>;
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Failed because liquidity restrictions due to locking
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Failed because the maximum locks was exceeded
       **/
      MaxLocksExceeded: AugmentedError<ApiType>;
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    treasury: {
      /**
       * The spend origin is valid but the amount it is allowed to spend is lower than the
       * amount to be spent.
       **/
      InsufficientPermission: AugmentedError<ApiType>;
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
      /**
       * Proposal has not been approved.
       **/
      ProposalNotApproved: AugmentedError<ApiType>;
      /**
       * Too many approvals in the queue.
       **/
      TooManyApprovals: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    unknownTokens: {
      /**
       * The operation will cause balance to overflow.
       **/
      BalanceOverflow: AugmentedError<ApiType>;
      /**
       * The balance is too low.
       **/
      BalanceTooLow: AugmentedError<ApiType>;
      /**
       * Unhandled asset.
       **/
      UnhandledAsset: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    utility: {
      /**
       * Too many calls batched.
       **/
      TooManyCalls: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    vesting: {
      /**
       * The vested transfer amount is too low
       **/
      AmountLow: AugmentedError<ApiType>;
      /**
       * Insufficient amount of balance to lock
       **/
      InsufficientBalanceToLock: AugmentedError<ApiType>;
      /**
       * Failed because the maximum vesting schedules was exceeded
       **/
      MaxVestingSchedulesExceeded: AugmentedError<ApiType>;
      /**
       * This account have too many vesting schedules
       **/
      TooManyVestingSchedules: AugmentedError<ApiType>;
      /**
       * Trying to vest to ourselves
       **/
      TryingToSelfVest: AugmentedError<ApiType>;
      /**
       * There is no vesting schedule with a given id
       **/
      VestingScheduleNotFound: AugmentedError<ApiType>;
      /**
       * Vesting period is zero
       **/
      ZeroVestingPeriod: AugmentedError<ApiType>;
      /**
       * Number of vests is zero
       **/
      ZeroVestingPeriodCount: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    whitelist: {
      /**
       * The call was already whitelisted; No-Op.
       **/
      CallAlreadyWhitelisted: AugmentedError<ApiType>;
      /**
       * The call was not whitelisted.
       **/
      CallIsNotWhitelisted: AugmentedError<ApiType>;
      /**
       * The weight of the decoded call was higher than the witness.
       **/
      InvalidCallWeightWitness: AugmentedError<ApiType>;
      /**
       * The preimage of the call hash could not be loaded.
       **/
      UnavailablePreImage: AugmentedError<ApiType>;
      /**
       * The call could not be decoded.
       **/
      UndecodableCall: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    xcmpQueue: {
      /**
       * Bad overweight index.
       **/
      BadOverweightIndex: AugmentedError<ApiType>;
      /**
       * Bad XCM data.
       **/
      BadXcm: AugmentedError<ApiType>;
      /**
       * Bad XCM origin.
       **/
      BadXcmOrigin: AugmentedError<ApiType>;
      /**
       * Failed to send XCM message.
       **/
      FailedToSend: AugmentedError<ApiType>;
      /**
       * Provided weight is possibly not enough to execute the message.
       **/
      WeightOverLimit: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    xTokens: {
      /**
       * Asset has no reserve location.
       **/
      AssetHasNoReserve: AugmentedError<ApiType>;
      /**
       * The specified index does not exist in a MultiAssets struct.
       **/
      AssetIndexNonExistent: AugmentedError<ApiType>;
      /**
       * The version of the `Versioned` value used is not able to be
       * interpreted.
       **/
      BadVersion: AugmentedError<ApiType>;
      /**
       * Could not re-anchor the assets to declare the fees for the
       * destination chain.
       **/
      CannotReanchor: AugmentedError<ApiType>;
      /**
       * The destination `MultiLocation` provided cannot be inverted.
       **/
      DestinationNotInvertible: AugmentedError<ApiType>;
      /**
       * We tried sending distinct asset and fee but they have different
       * reserve chains.
       **/
      DistinctReserveForAssetAndFee: AugmentedError<ApiType>;
      /**
       * Fee is not enough.
       **/
      FeeNotEnough: AugmentedError<ApiType>;
      /**
       * Could not get ancestry of asset reserve location.
       **/
      InvalidAncestry: AugmentedError<ApiType>;
      /**
       * The MultiAsset is invalid.
       **/
      InvalidAsset: AugmentedError<ApiType>;
      /**
       * Invalid transfer destination.
       **/
      InvalidDest: AugmentedError<ApiType>;
      /**
       * MinXcmFee not registered for certain reserve location
       **/
      MinXcmFeeNotDefined: AugmentedError<ApiType>;
      /**
       * Not cross-chain transfer.
       **/
      NotCrossChainTransfer: AugmentedError<ApiType>;
      /**
       * Currency is not cross-chain transferable.
       **/
      NotCrossChainTransferableCurrency: AugmentedError<ApiType>;
      /**
       * Not supported MultiLocation
       **/
      NotSupportedMultiLocation: AugmentedError<ApiType>;
      /**
       * The number of assets to be sent is over the maximum.
       **/
      TooManyAssetsBeingSent: AugmentedError<ApiType>;
      /**
       * The message's weight could not be determined.
       **/
      UnweighableMessage: AugmentedError<ApiType>;
      /**
       * XCM execution failed.
       **/
      XcmExecutionFailed: AugmentedError<ApiType>;
      /**
       * The transfering asset amount is zero.
       **/
      ZeroAmount: AugmentedError<ApiType>;
      /**
       * The fee is zero.
       **/
      ZeroFee: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
  } // AugmentedErrors
} // declare module
