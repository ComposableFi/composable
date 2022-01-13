// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api/types';

declare module '@polkadot/api/types/errors' {
  export interface AugmentedErrors<ApiType> {
    assetRegistry: {
      /**
       * Asset is already registered.
       **/
      AssetAlreadyRegistered: AugmentedError<ApiType>;
      /**
       * Invalid asset name or symbol.
       **/
      AssetNotFound: AugmentedError<ApiType>;
      /**
       * Asset ID is not registered in the asset-registry.
       **/
      AssetNotRegistered: AugmentedError<ApiType>;
      /**
       * Incorrect number of assets provided to create shared asset.
       **/
      InvalidSharedAssetLen: AugmentedError<ApiType>;
      /**
       * Asset Id is not available. This only happens when it reaches the MAX value of given id type.
       **/
      NoIdAvailable: AugmentedError<ApiType>;
      /**
       * Invalid asset name or symbol.
       **/
      TooLong: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    authorship: {
      /**
       * The uncle is genesis.
       **/
      GenesisUncle: AugmentedError<ApiType>;
      /**
       * The uncle parent not in the chain.
       **/
      InvalidUncleParent: AugmentedError<ApiType>;
      /**
       * The uncle isn't recent enough to be included.
       **/
      OldUncle: AugmentedError<ApiType>;
      /**
       * The uncle is too high in chain.
       **/
      TooHighUncle: AugmentedError<ApiType>;
      /**
       * Too many uncles.
       **/
      TooManyUncles: AugmentedError<ApiType>;
      /**
       * The uncle is already included.
       **/
      UncleAlreadyIncluded: AugmentedError<ApiType>;
      /**
       * Uncles already set in the block.
       **/
      UnclesAlreadySet: AugmentedError<ApiType>;
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
       * Balance too low to send value
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
    cumulusXcm: {
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    currencies: {
      /**
       * Unable to convert the Amount type into Balance.
       **/
      AmountIntoBalanceFailed: AugmentedError<ApiType>;
      /**
       * Balance is too low.
       **/
      BalanceTooLow: AugmentedError<ApiType>;
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
       * Preimage already noted
       **/
      DuplicatePreimage: AugmentedError<ApiType>;
      /**
       * Proposal already made
       **/
      DuplicateProposal: AugmentedError<ApiType>;
      /**
       * Imminent
       **/
      Imminent: AugmentedError<ApiType>;
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
       * Not imminent
       **/
      NotImminent: AugmentedError<ApiType>;
      /**
       * Next external proposal not simple majority
       **/
      NotSimpleMajority: AugmentedError<ApiType>;
      /**
       * The given account did not vote on the referendum.
       **/
      NotVoter: AugmentedError<ApiType>;
      /**
       * Invalid preimage
       **/
      PreimageInvalid: AugmentedError<ApiType>;
      /**
       * Preimage not found
       **/
      PreimageMissing: AugmentedError<ApiType>;
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
       * Too early
       **/
      TooEarly: AugmentedError<ApiType>;
      /**
       * Maximum number of proposals reached.
       **/
      TooManyProposals: AugmentedError<ApiType>;
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
    duster: {
      /**
       * Account is excluded from dusting.
       **/
      AccountBlacklisted: AugmentedError<ApiType>;
      /**
       * Account is not present in the non-dustable list.
       **/
      AccountNotBlacklisted: AugmentedError<ApiType>;
      /**
       * The balance is sufficient to keep account open.
       **/
      BalanceSufficient: AugmentedError<ApiType>;
      /**
       * The balance is zero.
       **/
      ZeroBalance: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    elections: {
      /**
       * Duplicated candidate submission.
       **/
      DuplicatedCandidate: AugmentedError<ApiType>;
      /**
       * Candidate does not have enough funds.
       **/
      InsufficientCandidateFunds: AugmentedError<ApiType>;
      /**
       * The renouncing origin presented a wrong `Renouncing` parameter.
       **/
      InvalidRenouncing: AugmentedError<ApiType>;
      /**
       * Prediction regarding replacement after member removal is wrong.
       **/
      InvalidReplacement: AugmentedError<ApiType>;
      /**
       * The provided count of number of votes is incorrect.
       **/
      InvalidVoteCount: AugmentedError<ApiType>;
      /**
       * The provided count of number of candidates is incorrect.
       **/
      InvalidWitnessData: AugmentedError<ApiType>;
      /**
       * Cannot vote with stake less than minimum balance.
       **/
      LowBalance: AugmentedError<ApiType>;
      /**
       * Cannot vote more than maximum allowed.
       **/
      MaximumVotesExceeded: AugmentedError<ApiType>;
      /**
       * Member cannot re-submit candidacy.
       **/
      MemberSubmit: AugmentedError<ApiType>;
      /**
       * Must be a voter.
       **/
      MustBeVoter: AugmentedError<ApiType>;
      /**
       * Not a member.
       **/
      NotMember: AugmentedError<ApiType>;
      /**
       * Must vote for at least one candidate.
       **/
      NoVotes: AugmentedError<ApiType>;
      /**
       * Cannot report self.
       **/
      ReportSelf: AugmentedError<ApiType>;
      /**
       * Runner cannot re-submit candidacy.
       **/
      RunnerUpSubmit: AugmentedError<ApiType>;
      /**
       * Cannot vote more than candidates.
       **/
      TooManyVotes: AugmentedError<ApiType>;
      /**
       * Voter can not pay voting bond.
       **/
      UnableToPayBond: AugmentedError<ApiType>;
      /**
       * Cannot vote when no candidates or members exist.
       **/
      UnableToVote: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    exchange: {
      /**
       * Insufficient asset balance.
       **/
      InsufficientAssetBalance: AugmentedError<ApiType>;
      /**
       * Trade amount is too low.
       **/
      MinimumTradeLimitNotReached: AugmentedError<ApiType>;
      /**
       * Token pool does not exist.
       **/
      TokenPoolNotFound: AugmentedError<ApiType>;
      /**
       * Given trading limit has been exceeded (buy).
       **/
      TradeAmountExceededLimit: AugmentedError<ApiType>;
      /**
       * Given trading limit has not been reached (sell).
       **/
      TradeAmountNotReachedLimit: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      ZeroSpotPrice: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    lbp: {
      /**
       * Trading limit reached
       **/
      AssetBalanceLimitExceeded: AugmentedError<ApiType>;
      /**
       * Liquidity being added should not be zero
       **/
      CannotAddZeroLiquidity: AugmentedError<ApiType>;
      /**
       * Pool assets can not be the same
       **/
      CannotCreatePoolWithSameAssets: AugmentedError<ApiType>;
      /**
       * Invalid fee amount
       **/
      FeeAmountInvalid: AugmentedError<ApiType>;
      /**
       * Asset balance too low
       **/
      InsufficientAssetBalance: AugmentedError<ApiType>;
      /**
       * Liquidity has not reached the required minimum.
       **/
      InsufficientLiquidity: AugmentedError<ApiType>;
      /**
       * Amount is less than minimum trading limit.
       **/
      InsufficientTradingAmount: AugmentedError<ApiType>;
      /**
       * Invalid block range
       **/
      InvalidBlockRange: AugmentedError<ApiType>;
      /**
       * Weight set is out of range
       **/
      InvalidWeight: AugmentedError<ApiType>;
      /**
       * Trade amount is too high
       **/
      MaxInRatioExceeded: AugmentedError<ApiType>;
      /**
       * Trade amount is too high
       **/
      MaxOutRatioExceeded: AugmentedError<ApiType>;
      /**
       * Sale duration is too long
       **/
      MaxSaleDurationExceeded: AugmentedError<ApiType>;
      /**
       * Nothing to update
       **/
      NothingToUpdate: AugmentedError<ApiType>;
      /**
       * Account is not a pool owner
       **/
      NotOwner: AugmentedError<ApiType>;
      /**
       * An unexpected integer overflow occurred
       **/
      Overflow: AugmentedError<ApiType>;
      /**
       * Pool has been already created
       **/
      PoolAlreadyExists: AugmentedError<ApiType>;
      /**
       * Pool does not exist
       **/
      PoolNotFound: AugmentedError<ApiType>;
      /**
       * Sale is not running
       **/
      SaleIsNotRunning: AugmentedError<ApiType>;
      /**
       * Sale is still in progress
       **/
      SaleNotEnded: AugmentedError<ApiType>;
      /**
       * Sale already started
       **/
      SaleStarted: AugmentedError<ApiType>;
      /**
       * Calculation error
       **/
      WeightCalculationError: AugmentedError<ApiType>;
      /**
       * Can not perform a trade with zero amount
       **/
      ZeroAmount: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    multiTransactionPayment: {
      /**
       * Currency is already in the list of accepted currencies.
       **/
      AlreadyAccepted: AugmentedError<ApiType>;
      /**
       * It is not allowed to add Core Asset as accepted currency. Core asset is accepted by design.
       **/
      CoreAssetNotAllowed: AugmentedError<ApiType>;
      /**
       * Fallback price was not found.
       **/
      FallbackPriceNotFound: AugmentedError<ApiType>;
      /**
       * Math overflow
       **/
      Overflow: AugmentedError<ApiType>;
      /**
       * Selected currency is not supported.
       **/
      UnsupportedCurrency: AugmentedError<ApiType>;
      /**
       * Account balance should be non-zero.
       **/
      ZeroBalance: AugmentedError<ApiType>;
      /**
       * Fallback price cannot be zero.
       **/
      ZeroPrice: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    nft: {
      /**
       * A user cannot buy already owned token
       **/
      CannotBuyOwnToken: AugmentedError<ApiType>;
      /**
       * A token still owned by class owner
       **/
      CannotSellPoolToken: AugmentedError<ApiType>;
      /**
       * A token can not be transferred to self
       **/
      CannotSendToSelf: AugmentedError<ApiType>;
      /**
       * The class does not exist
       **/
      ClassNotFound: AugmentedError<ApiType>;
      /**
       * Metadata exceed the allowed length
       **/
      MetadataTooLong: AugmentedError<ApiType>;
      /**
       * The token class is not empty
       **/
      NonZeroIssuance: AugmentedError<ApiType>;
      /**
       * Class wasn't created as a pool
       **/
      NotAPool: AugmentedError<ApiType>;
      /**
       * Not the class owner
       **/
      NotClassOwner: AugmentedError<ApiType>;
      /**
       * Not the token owner
       **/
      NotTokenOwner: AugmentedError<ApiType>;
      /**
       * Token has been already bought from a pool
       **/
      TokenAlreadyHasAnOwner: AugmentedError<ApiType>;
      /**
       * Token is currently locked
       **/
      TokenLocked: AugmentedError<ApiType>;
      /**
       * The token does not exist
       **/
      TokenNotFound: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    ormlNft: {
      /**
       * Can not destroy class
       * Total issuance is not 0
       **/
      CannotDestroyClass: AugmentedError<ApiType>;
      /**
       * Class not found
       **/
      ClassNotFound: AugmentedError<ApiType>;
      /**
       * Failed because the Maximum amount of metadata was exceeded
       **/
      MaxMetadataExceeded: AugmentedError<ApiType>;
      /**
       * No available class ID
       **/
      NoAvailableClassId: AugmentedError<ApiType>;
      /**
       * No available token ID
       **/
      NoAvailableTokenId: AugmentedError<ApiType>;
      /**
       * The operator is not the owner of the token and has no permission
       **/
      NoPermission: AugmentedError<ApiType>;
      /**
       * Token(ClassId, TokenId) not found
       **/
      TokenNotFound: AugmentedError<ApiType>;
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
       * The message execution fails the filter.
       **/
      Filtered: AugmentedError<ApiType>;
      /**
       * Origin is invalid for sending.
       **/
      InvalidOrigin: AugmentedError<ApiType>;
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
    scheduler: {
      /**
       * Failed to schedule a call
       **/
      FailedToSchedule: AugmentedError<ApiType>;
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
    tips: {
      /**
       * The tip was already found/started.
       **/
      AlreadyKnown: AugmentedError<ApiType>;
      /**
       * The account attempting to retract the tip is not the finder of the tip.
       **/
      NotFinder: AugmentedError<ApiType>;
      /**
       * The tip cannot be claimed/closed because it's still in the countdown period.
       **/
      Premature: AugmentedError<ApiType>;
      /**
       * The reason given is just too big.
       **/
      ReasonTooBig: AugmentedError<ApiType>;
      /**
       * The tip cannot be claimed/closed because there are not enough tippers yet.
       **/
      StillOpen: AugmentedError<ApiType>;
      /**
       * The tip hash is unknown.
       **/
      UnknownTip: AugmentedError<ApiType>;
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
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    treasury: {
      /**
       * Proposer's balance is too low.
       **/
      InsufficientProposersBalance: AugmentedError<ApiType>;
      /**
       * No proposal or bounty at that index.
       **/
      InvalidIndex: AugmentedError<ApiType>;
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
    xcmpQueue: {
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
       * Could not get ancestry of asset reserve location.
       **/
      InvalidAncestry: AugmentedError<ApiType>;
      /**
       * Invalid transfer destination.
       **/
      InvalidDest: AugmentedError<ApiType>;
      /**
       * Not cross-chain transfer.
       **/
      NotCrossChainTransfer: AugmentedError<ApiType>;
      /**
       * Currency is not cross-chain transferable.
       **/
      NotCrossChainTransferableCurrency: AugmentedError<ApiType>;
      /**
       * Not fungible asset.
       **/
      NotFungible: AugmentedError<ApiType>;
      /**
       * The message's weight could not be determined.
       **/
      UnweighableMessage: AugmentedError<ApiType>;
      /**
       * XCM execution failed.
       **/
      XcmExecutionFailed: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    xyk: {
      /**
       * Overflow
       **/
      AddAssetAmountInvalid: AugmentedError<ApiType>;
      /**
       * Asset amount has exceeded given limit.
       **/
      AssetAmountExceededLimit: AugmentedError<ApiType>;
      /**
       * Asset amount has not reached given limit.
       **/
      AssetAmountNotReachedLimit: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      BuyAssetAmountInvalid: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      CannotApplyDiscount: AugmentedError<ApiType>;
      /**
       * It is not allowed to create a pool between same assets.
       **/
      CannotCreatePoolWithSameAssets: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      CreatePoolAssetAmountInvalid: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      FeeAmountInvalid: AugmentedError<ApiType>;
      /**
       * Asset balance is not sufficient.
       **/
      InsufficientAssetBalance: AugmentedError<ApiType>;
      /**
       * Liquidity has not reached the required minimum.
       **/
      InsufficientLiquidity: AugmentedError<ApiType>;
      /**
       * Not enough core asset liquidity in the pool.
       **/
      InsufficientNativeCurrencyBalance: AugmentedError<ApiType>;
      /**
       * Not enough asset liquidity in the pool.
       **/
      InsufficientPoolAssetBalance: AugmentedError<ApiType>;
      /**
       * Amount is less than min trading limit.
       **/
      InsufficientTradingAmount: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      InvalidLiquidityAmount: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      InvalidMintedLiquidity: AugmentedError<ApiType>;
      /**
       * Max fraction of pool to sell in single transaction has been exceeded.
       **/
      MaxInRatioExceeded: AugmentedError<ApiType>;
      /**
       * Max fraction of pool to buy in single transaction has been exceeded.
       **/
      MaxOutRatioExceeded: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      Overflow: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      RemoveAssetAmountInvalid: AugmentedError<ApiType>;
      /**
       * Overflow
       **/
      SellAssetAmountInvalid: AugmentedError<ApiType>;
      /**
       * Liquidity pool for given assets already exists.
       **/
      TokenPoolAlreadyExists: AugmentedError<ApiType>;
      /**
       * Liquidity pool for given assets does not exist.
       **/
      TokenPoolNotFound: AugmentedError<ApiType>;
      /**
       * It is not allowed to create a pool with zero initial price.
       **/
      ZeroInitialPrice: AugmentedError<ApiType>;
      /**
       * Liquidity is zero.
       **/
      ZeroLiquidity: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
  }

  export interface DecoratedErrors<ApiType extends ApiTypes> extends AugmentedErrors<ApiType> {
    [key: string]: ModuleErrors<ApiType>;
  }
}
