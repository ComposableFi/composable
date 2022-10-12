// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
  interface AugmentedErrors<ApiType extends ApiTypes> {
    assets: {
      CannotSetNewCurrencyToRegistry: AugmentedError<ApiType>;
      InvalidCurrency: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    assetsRegistry: {
      AssetNotFound: AugmentedError<ApiType>;
      ForeignAssetAlreadyRegistered: AugmentedError<ApiType>;
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
    cosmwasm: {
      ChargeGas: AugmentedError<ApiType>;
      CodeAlreadyExists: AugmentedError<ApiType>;
      CodeDecoding: AugmentedError<ApiType>;
      CodeEncoding: AugmentedError<ApiType>;
      CodeInstrumentation: AugmentedError<ApiType>;
      CodeNotFound: AugmentedError<ApiType>;
      CodeValidation: AugmentedError<ApiType>;
      ContractAlreadyExists: AugmentedError<ApiType>;
      ContractHasNoInfo: AugmentedError<ApiType>;
      ContractNotFound: AugmentedError<ApiType>;
      ContractTrapped: AugmentedError<ApiType>;
      Instrumentation: AugmentedError<ApiType>;
      InstrumentedCodeIsTooBig: AugmentedError<ApiType>;
      IteratorIdOverflow: AugmentedError<ApiType>;
      IteratorNotFound: AugmentedError<ApiType>;
      LabelTooBig: AugmentedError<ApiType>;
      NonceOverflow: AugmentedError<ApiType>;
      NotEnoughFundsForUpload: AugmentedError<ApiType>;
      RefcountOverflow: AugmentedError<ApiType>;
      RefundGas: AugmentedError<ApiType>;
      SignatureVerificationError: AugmentedError<ApiType>;
      StackOverflow: AugmentedError<ApiType>;
      TransferFailed: AugmentedError<ApiType>;
      UnknownDenom: AugmentedError<ApiType>;
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
    dexRouter: {
      /**
       * Can not respect minimum amount requested.
       **/
      CanNotRespectMinAmountRequested: AugmentedError<ApiType>;
      /**
       * Route with possible loop is not allowed.
       **/
      LoopSuspectedInRouteUpdate: AugmentedError<ApiType>;
      /**
       * Number of hops in route exceeded maximum limit.
       **/
      MaxHopsExceeded: AugmentedError<ApiType>;
      /**
       * For given asset pair no route found.
       **/
      NoRouteFound: AugmentedError<ApiType>;
      /**
       * Unexpected node found while route validation.
       **/
      UnexpectedNodeFoundWhileValidation: AugmentedError<ApiType>;
      /**
       * Unsupported operation.
       **/
      UnsupportedOperation: AugmentedError<ApiType>;
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
    dutchAuction: {
      NotEnoughNativeCurrencyToPayForAuction: AugmentedError<ApiType>;
      OrderNotFound: AugmentedError<ApiType>;
      OrderParametersIsInvalid: AugmentedError<ApiType>;
      RequestedOrderDoesNotExists: AugmentedError<ApiType>;
      TakeLimitDoesNotSatisfyOrder: AugmentedError<ApiType>;
      TakeOrderDidNotHappen: AugmentedError<ApiType>;
      TakeParametersIsInvalid: AugmentedError<ApiType>;
      /**
       * errors trying to decode and parse XCM input
       **/
      XcmCannotDecodeRemoteParametersToLocalRepresentations: AugmentedError<ApiType>;
      XcmCannotFindLocalIdentifiersAsDecodedFromRemote: AugmentedError<ApiType>;
      XcmNotFoundConfigurationById: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    fnft: {
      CollectionAlreadyExists: AugmentedError<ApiType>;
      CollectionNotFound: AugmentedError<ApiType>;
      InstanceAlreadyExists: AugmentedError<ApiType>;
      InstanceNotFound: AugmentedError<ApiType>;
      MustBeOwner: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    governanceRegistry: {
      /**
       * Not found
       **/
      NoneError: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    ibc: {
      /**
       * Channel not found
       **/
      ChannelNotFound: AugmentedError<ApiType>;
      /**
       * Client state not found
       **/
      ClientStateNotFound: AugmentedError<ApiType>;
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
      /**
       * Invalid message for extrinsic
       **/
      InvalidMessageType: AugmentedError<ApiType>;
      /**
       * Invalid route
       **/
      InvalidRoute: AugmentedError<ApiType>;
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
       * Error processing ibc messages
       **/
      ProcessingError: AugmentedError<ApiType>;
      /**
       * Error generating trie proof
       **/
      ProofGenerationError: AugmentedError<ApiType>;
      /**
       * Error constructing packet
       **/
      SendPacketError: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    ibcPing: {
      /**
       * Error opening channel
       **/
      ChannelInitError: AugmentedError<ApiType>;
      /**
       * Invalid params passed
       **/
      InvalidParams: AugmentedError<ApiType>;
      /**
       * Error registering packet
       **/
      PacketSendError: AugmentedError<ApiType>;
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
       * Judgement given.
       **/
      JudgementGiven: AugmentedError<ApiType>;
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
    lending: {
      /**
       * Account did not deposit any collateral to particular market.
       **/
      AccountCollateralAbsent: AugmentedError<ApiType>;
      /**
       * Borrow and repay in the same block are not allowed.
       * Flashloans are not supported by the pallet.
       **/
      BorrowAndRepayInSameBlockIsNotSupported: AugmentedError<ApiType>;
      /**
       * User tried to repay non-existent loan.
       **/
      BorrowDoesNotExist: AugmentedError<ApiType>;
      /**
       * Borrow limit for particular borrower was not calculated
       * due to arithmetic error.
       **/
      BorrowLimitCalculationFailed: AugmentedError<ApiType>;
      /**
       * Account did not pay any rent to particular market.
       **/
      BorrowRentDoesNotExist: AugmentedError<ApiType>;
      CannotBorrowFromMarketWithUnbalancedVault: AugmentedError<ApiType>;
      /**
       * Borrow rate can not be calculated.
       **/
      CannotCalculateBorrowRate: AugmentedError<ApiType>;
      CannotIncreaseCollateralFactorOfOpenMarket: AugmentedError<ApiType>;
      /**
       * Cannot repay more than total amount of debt when partially repaying.
       **/
      CannotRepayMoreThanTotalDebt: AugmentedError<ApiType>;
      /**
       * A market with a borrow balance of `0` was attempted to be repaid.
       **/
      CannotRepayZeroBalance: AugmentedError<ApiType>;
      /**
       * Market can not be created since
       * allowed number of markets was exceeded.
       **/
      ExceedLendingCount: AugmentedError<ApiType>;
      /**
       * Market manager has to deposit initial amount of borrow asset into the market account.
       * Initial amount is denominated in normalized currency and calculated based on data
       * from Oracle. The error is emitted if calculated amount is incorrect.
       **/
      InitialMarketVolumeIncorrect: AugmentedError<ApiType>;
      /**
       * Invalid collateral factor was provided.
       * Collateral factor value must be more than one.
       **/
      InvalidCollateralFactor: AugmentedError<ApiType>;
      InvalidTimestampOnBorrowRequest: AugmentedError<ApiType>;
      /**
       * The market could not be found.
       **/
      MarketDoesNotExist: AugmentedError<ApiType>;
      MarketIsClosing: AugmentedError<ApiType>;
      /**
       * User has provided not sufficient amount of collateral.
       **/
      NotEnoughCollateralToBorrow: AugmentedError<ApiType>;
      /**
       * When user try to withdraw money beyond what is available.
       **/
      NotEnoughCollateralToWithdraw: AugmentedError<ApiType>;
      /**
       * Block number of provided price is out of allowed tolerance.
       **/
      PriceTooOld: AugmentedError<ApiType>;
      /**
       * Attempted to update a market owned by someone else.
       **/
      Unauthorized: AugmentedError<ApiType>;
      /**
       * The market would go under collateralized if the requested amount of collateral was
       * withdrawn.
       **/
      WouldGoUnderCollateralized: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    liquidations: {
      InvalidLiquidationStrategiesVector: AugmentedError<ApiType>;
      NoLiquidationEngineFound: AugmentedError<ApiType>;
      OnlyDutchAuctionStrategyIsImplemented: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    mosaic: {
      AmountMismatch: AugmentedError<ApiType>;
      AssetNotMapped: AugmentedError<ApiType>;
      BadTimelockPeriod: AugmentedError<ApiType>;
      BadTTL: AugmentedError<ApiType>;
      BelowMinTransferSize: AugmentedError<ApiType>;
      DestinationAmmIdNotWhitelisted: AugmentedError<ApiType>;
      ExceedsMaxTransferSize: AugmentedError<ApiType>;
      InsufficientBudget: AugmentedError<ApiType>;
      NetworkDisabled: AugmentedError<ApiType>;
      NoClaimableTx: AugmentedError<ApiType>;
      NoOutgoingTx: AugmentedError<ApiType>;
      NoStaleTransactions: AugmentedError<ApiType>;
      Overflow: AugmentedError<ApiType>;
      RelayerNotSet: AugmentedError<ApiType>;
      RemoteAmmIdAlreadyExists: AugmentedError<ApiType>;
      RemoteAmmIdNotFound: AugmentedError<ApiType>;
      TxStillLocked: AugmentedError<ApiType>;
      UnsupportedAsset: AugmentedError<ApiType>;
      UnsupportedNetwork: AugmentedError<ApiType>;
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
      CannotRespectMinimumRequested: AugmentedError<ApiType>;
      InvalidAmount: AugmentedError<ApiType>;
      InvalidAsset: AugmentedError<ApiType>;
      InvalidFees: AugmentedError<ApiType>;
      InvalidPair: AugmentedError<ApiType>;
      InvalidSaleState: AugmentedError<ApiType>;
      MissingAmount: AugmentedError<ApiType>;
      MissingMinExpectedAmount: AugmentedError<ApiType>;
      MoreThanTwoAssetsNotYetSupported: AugmentedError<ApiType>;
      MustBeOwner: AugmentedError<ApiType>;
      NoLpTokenForLbp: AugmentedError<ApiType>;
      NotEnoughLiquidity: AugmentedError<ApiType>;
      NotEnoughLpToken: AugmentedError<ApiType>;
      NoXTokenForLbp: AugmentedError<ApiType>;
      PairMismatch: AugmentedError<ApiType>;
      PoolNotFound: AugmentedError<ApiType>;
      StakingPoolConfigError: AugmentedError<ApiType>;
      WeightsMustBeNonZero: AugmentedError<ApiType>;
      WeightsMustSumToOne: AugmentedError<ApiType>;
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
      TooLarge: AugmentedError<ApiType>;
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
    relayerXcm: {
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
    stakingRewards: {
      BackToTheFuture: AugmentedError<ApiType>;
      /**
       * Invalid end block number provided for creating a pool.
       **/
      EndBlockMustBeAfterStartBlock: AugmentedError<ApiType>;
      FnftNotFound: AugmentedError<ApiType>;
      /**
       * AssetId is invalid, asset IDs must be greater than 0
       **/
      InvalidAssetId: AugmentedError<ApiType>;
      /**
       * Reward's max limit reached.
       **/
      MaxRewardLimitReached: AugmentedError<ApiType>;
      /**
       * No duration presets configured.
       **/
      NoDurationPresetsConfigured: AugmentedError<ApiType>;
      /**
       * No duration presets were provided upon pool creation.
       **/
      NoDurationPresetsProvided: AugmentedError<ApiType>;
      /**
       * Not enough assets for a stake.
       **/
      NotEnoughAssets: AugmentedError<ApiType>;
      /**
       * only the owner of stake can unstake it
       **/
      OnlyStakeOwnerCanInteractWithStake: AugmentedError<ApiType>;
      /**
       * Error when creating reduction configs.
       **/
      ReductionConfigProblem: AugmentedError<ApiType>;
      /**
       * Reward asset not found in reward pool.
       **/
      RewardAssetNotFound: AugmentedError<ApiType>;
      /**
       * Error when creating reward configs.
       **/
      RewardConfigProblem: AugmentedError<ApiType>;
      /**
       * Reward pool already exists
       **/
      RewardsPoolAlreadyExists: AugmentedError<ApiType>;
      /**
       * Rewards pool has not started.
       **/
      RewardsPoolHasNotStarted: AugmentedError<ApiType>;
      /**
       * Rewards pool not found.
       **/
      RewardsPoolNotFound: AugmentedError<ApiType>;
      /**
       * The rewards pot for this pool is empty.
       **/
      RewardsPotEmpty: AugmentedError<ApiType>;
      /**
       * Slashed amount of minimum reward is less than existential deposit
       **/
      SlashedAmountTooLow: AugmentedError<ApiType>;
      /**
       * Slashed amount of minimum staking amount is less than existential deposit
       **/
      SlashedMinimumStakingAmountTooLow: AugmentedError<ApiType>;
      /**
       * Staked amount is less than the minimum staking amount for the pool.
       **/
      StakedAmountTooLow: AugmentedError<ApiType>;
      /**
       * Staked amount after split is less than the minimum staking amount for the pool.
       **/
      StakedAmountTooLowAfterSplit: AugmentedError<ApiType>;
      /**
       * No stake found for given id.
       **/
      StakeNotFound: AugmentedError<ApiType>;
      /**
       * Invalid start block number provided for creating a pool.
       **/
      StartBlockMustBeAfterCurrentBlock: AugmentedError<ApiType>;
      /**
       * Too many rewarded asset types per pool violating the storage allowed.
       **/
      TooManyRewardAssetTypes: AugmentedError<ApiType>;
      /**
       * Unimplemented reward pool type.
       **/
      UnimplementedRewardPoolConfiguration: AugmentedError<ApiType>;
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
    technicalCollective: {
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
    technicalMembership: {
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
    transfer: {
      /**
       * Error opening channel
       **/
      ChannelInitError: AugmentedError<ApiType>;
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
       * Invalid Ibc denom
       **/
      InvalidIbcDenom: AugmentedError<ApiType>;
      /**
       * Invalid params passed
       **/
      InvalidParams: AugmentedError<ApiType>;
      /**
       * Invalid timestamp
       **/
      InvalidTimestamp: AugmentedError<ApiType>;
      /**
       * The interchain token transfer was not successfully initiated
       **/
      TransferFailed: AugmentedError<ApiType>;
      /**
       * Error Decoding utf8 bytes
       **/
      Utf8Error: AugmentedError<ApiType>;
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
    vault: {
      /**
       * It is not possible to perform a privileged action using an ordinary account
       **/
      AccountIsNotManager: AugmentedError<ApiType>;
      /**
       * Vaults must allocate the proper ratio between reserved and strategies, so that the
       * ratio sums up to one.
       **/
      AllocationMustSumToOne: AugmentedError<ApiType>;
      /**
       * Deposit amounts not exceeding [`MinimumDeposit`](Config::MinimumDeposit) are declined
       * and result in `AmountMustGteMinimumDeposit`.
       **/
      AmountMustGteMinimumDeposit: AugmentedError<ApiType>;
      /**
       * Withdrawal amounts not exceeding [`MinimumWithdrawal`](Config::MinimumWithdrawal) are
       * declined and result in `AmountMustGteMinimumWithdrawal`.
       **/
      AmountMustGteMinimumWithdrawal: AugmentedError<ApiType>;
      /**
       * Failures in creating LP tokens during vault creation result in `CannotCreateAsset`.
       **/
      CannotCreateAsset: AugmentedError<ApiType>;
      /**
       * The vault has deposits halted, see [Capabilities](crate::capabilities::Capabilities).
       **/
      DepositsHalted: AugmentedError<ApiType>;
      /**
       * Creating vaults with invalid creation deposits results in
       * `InsufficientCreationDeposit`.
       **/
      InsufficientCreationDeposit: AugmentedError<ApiType>;
      /**
       * Vaults may have insufficient funds for withdrawals, as well as users wishing to deposit
       * an incorrect amount.
       **/
      InsufficientFunds: AugmentedError<ApiType>;
      /**
       * Requesting withdrawals for more LP tokens than available to the user result in
       * `InsufficientLpTokens`
       **/
      InsufficientLpTokens: AugmentedError<ApiType>;
      /**
       * Existentially funded vaults do not require extra funds.
       **/
      InvalidAddSurcharge: AugmentedError<ApiType>;
      InvalidDeletionClaim: AugmentedError<ApiType>;
      /**
       * Attempting to tombstone a vault which has rent remaining results in
       * `InvalidSurchargeClaim`.
       **/
      InvalidSurchargeClaim: AugmentedError<ApiType>;
      /**
       * Minting failures result in `MintFailed`. In general this should never occur.
       **/
      MintFailed: AugmentedError<ApiType>;
      /**
       * If the vault contains too many assets (close to the `Balance::MAX`), it is considered
       * full as arithmetic starts overflowing.
       **/
      NoFreeVaultAllocation: AugmentedError<ApiType>;
      /**
       * When trying to withdraw too much from the vault, `NotEnoughLiquidity` is returned.
       **/
      NotEnoughLiquidity: AugmentedError<ApiType>;
      /**
       * Not all vaults have an associated LP token. Attempting to perform LP token related
       * operations result in `NotVaultLpToken`.
       **/
      NotVaultLpToken: AugmentedError<ApiType>;
      OnlyManagerCanDoThisOperation: AugmentedError<ApiType>;
      /**
       * The vault could not be deleted, as it was not tombstoned for long enough.
       **/
      TombstoneDurationNotExceeded: AugmentedError<ApiType>;
      /**
       * Vaults may have up to [`MaxStrategies`](Config::MaxStrategies) strategies.
       **/
      TooManyStrategies: AugmentedError<ApiType>;
      /**
       * Failures to transfer funds from the vault to users or vice- versa result in
       * `TransferFromFailed`.
       **/
      TransferFromFailed: AugmentedError<ApiType>;
      /**
       * Querying/operating on invalid vault id's result in `VaultDoesNotExist`.
       **/
      VaultDoesNotExist: AugmentedError<ApiType>;
      /**
       * The vault could not be deleted, as it was not yet tombstoned.
       **/
      VaultNotTombstoned: AugmentedError<ApiType>;
      /**
       * The vault has withdrawals halted, see
       * [Capabilities](crate::capabilities::Capabilities).
       **/
      WithdrawalsHalted: AugmentedError<ApiType>;
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
