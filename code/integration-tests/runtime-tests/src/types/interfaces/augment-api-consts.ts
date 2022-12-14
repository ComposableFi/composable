// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/consts';

import type { FrameSupportPalletId, FrameSupportWeightsRuntimeDbWeight, PalletCosmwasmInstrumentCostRules, SpVersionRuntimeVersion, XcmV1MultiLocation } from '@composable/types/interfaces/crowdloanRewards';
import type { FrameSystemLimitsBlockLength, FrameSystemLimitsBlockWeights } from '@composable/types/interfaces/system';
import type { ApiTypes, AugmentedConst } from '@polkadot/api-base/types';
import type { Bytes, Option, Text, U8aFixed, bool, u128, u16, u32, u64, u8 } from '@polkadot/types-codec';
import type { Codec } from '@polkadot/types-codec/types';
import type { AccountId32, Perbill, Permill } from '@polkadot/types/interfaces/runtime';

export type __AugmentedConst<ApiType extends ApiTypes> = AugmentedConst<ApiType>;

declare module '@polkadot/api-base/types/consts' {
  interface AugmentedConsts<ApiType extends ApiTypes> {
    assets: {
      nativeAssetId: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    assetTxPayment: {
      /**
       * where to allow configuring default asset per user
       **/
      useUserConfiguration: bool & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    authorship: {
      /**
       * The number of blocks back we should accept uncles.
       * This means that we will deal with uncle-parents that are
       * `UncleGenerations + 1` before `now`.
       **/
      uncleGenerations: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    balances: {
      /**
       * The minimum amount required to keep an account open.
       **/
      existentialDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The maximum number of locks that should exist on an account.
       * Not strictly enforced, but used for weight estimation.
       **/
      maxLocks: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum number of named reserves that can exist on an account.
       **/
      maxReserves: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    bondedFinance: {
      /**
       * The minimum reward for an offer.
       * 
       * Must be > T::Vesting::MinVestedTransfer.
       **/
      minReward: u128 & AugmentedConst<ApiType>;
      /**
       * The pallet ID, required to create sub-accounts used by offers.
       **/
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * The stake a user has to put to create an offer.
       **/
      stake: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    callFilter: {
      maxStringSize: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    cosmwasm: {
      /**
       * Current chain ID. Provided to the contract via the [`Env`].
       **/
      chainId: Text & AugmentedConst<ApiType>;
      /**
       * Max wasm branch table size limit.
       **/
      codeBranchTableSizeLimit: u32 & AugmentedConst<ApiType>;
      /**
       * Max wasm globals limit.
       **/
      codeGlobalVariableLimit: u32 & AugmentedConst<ApiType>;
      /**
       * Max wasm functions parameters limit.
       **/
      codeParameterLimit: u32 & AugmentedConst<ApiType>;
      /**
       * Max wasm stack size limit.
       **/
      codeStackLimit: u32 & AugmentedConst<ApiType>;
      /**
       * Price of a byte when uploading new code.
       * The price is expressed in [`Self::NativeAsset`].
       * This amount is reserved from the owner and released when the code is destroyed.
       **/
      codeStorageByteDeposit: u32 & AugmentedConst<ApiType>;
      /**
       * Max wasm table size.
       **/
      codeTableSizeLimit: u32 & AugmentedConst<ApiType>;
      /**
       * Price of extracting a byte from the storage.
       **/
      contractStorageByteReadPrice: u32 & AugmentedConst<ApiType>;
      /**
       * Price of writing a byte in the storage.
       **/
      contractStorageByteWritePrice: u32 & AugmentedConst<ApiType>;
      /**
       * Max accepted code size.
       **/
      maxCodeSize: u32 & AugmentedConst<ApiType>;
      /**
       * Max contract label size.
       **/
      maxContractLabelSize: u32 & AugmentedConst<ApiType>;
      /**
       * Max contract trie id size.
       **/
      maxContractTrieIdSize: u32 & AugmentedConst<ApiType>;
      /**
       * Max number of frames a contract is able to push, a.k.a recursive calls.
       **/
      maxFrames: u32 & AugmentedConst<ApiType>;
      /**
       * Max assets in a [`FundsOf`] batch.
       **/
      maxFundsAssets: u32 & AugmentedConst<ApiType>;
      /**
       * Max instantiate salt.
       **/
      maxInstantiateSaltSize: u32 & AugmentedConst<ApiType>;
      /**
       * Max code size after gas instrumentation.
       **/
      maxInstrumentedCodeSize: u32 & AugmentedConst<ApiType>;
      /**
       * Max message size.
       **/
      maxMessageSize: u32 & AugmentedConst<ApiType>;
      /**
       * Pallet unique ID.
       **/
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      wasmCostRules: PalletCosmwasmInstrumentCostRules & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    crowdloanRewards: {
      /**
       * The AccountId of this pallet.
       **/
      accountId: AccountId32 & AugmentedConst<ApiType>;
      /**
       * The upfront liquidity unlocked at first claim.
       **/
      initialPayment: Perbill & AugmentedConst<ApiType>;
      /**
       * If claimed amounts should be locked by the pallet
       **/
      lockByDefault: bool & AugmentedConst<ApiType>;
      /**
       * The unique identifier for locks maintained by this pallet.
       **/
      lockId: U8aFixed & AugmentedConst<ApiType>;
      /**
       * The percentage of excess funds required to trigger the `OverFunded` event.
       **/
      overFundedThreshold: Perbill & AugmentedConst<ApiType>;
      /**
       * The unique identifier of this pallet.
       **/
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * The arbitrary prefix used for the proof.
       **/
      prefix: Bytes & AugmentedConst<ApiType>;
      /**
       * The time you have to wait to unlock another part of your reward.
       **/
      vestingStep: u64 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    democracy: {
      /**
       * Period in blocks where an external proposal may not be re-submitted after being vetoed.
       **/
      cooloffPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * Runtime unique identifier for locking currency.
       * May be equivalent to PalletId.
       **/
      democracyId: U8aFixed & AugmentedConst<ApiType>;
      /**
       * The period between a proposal being approved and enacted.
       * 
       * It should generally be a little more than the unstake period to ensure that
       * voting stakers have an opportunity to remove themselves from the system in the case
       * where they are on the losing side of a vote.
       **/
      enactmentPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * Minimum voting period allowed for a fast-track referendum.
       **/
      fastTrackVotingPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * Indicator for whether an emergency origin is even allowed to happen. Some chains may
       * want to set this permanently to `false`, others may want to condition it on things such
       * as an upgrade having happened recently.
       **/
      instantAllowed: bool & AugmentedConst<ApiType>;
      /**
       * How often (in blocks) new public referenda are launched.
       **/
      launchPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum number of public proposals that can exist at any time.
       **/
      maxProposals: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum number of votes for an account.
       * 
       * Also used to compute weight, an overly big value can
       * lead to extrinsic with very big weight: see `delegate` for instance.
       **/
      maxVotes: u32 & AugmentedConst<ApiType>;
      /**
       * The minimum amount to be used as a deposit for a public referendum proposal.
       **/
      minimumDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The amount of balance that must be deposited per byte of preimage stored.
       **/
      preimageByteDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The minimum period of vote locking.
       * 
       * It should be no shorter than enactment period to ensure that in the case of an approval,
       * those successful voters are locked into the consequences that their votes entail.
       **/
      voteLockingPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * How often (in blocks) to check for new votes.
       **/
      votingPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    dexRouter: {
      /**
       * The maximum hops in the route.
       **/
      maxHopsInRoute: u32 & AugmentedConst<ApiType>;
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    dutchAuction: {
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * ED taken to create position. Part of if returned when position is liquidated.
       **/
      positionExistentialDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    fnft: {
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    identity: {
      /**
       * The amount held on deposit for a registered identity
       **/
      basicDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The amount held on deposit per additional field for a registered identity.
       **/
      fieldDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O
       * required to access an identity, but can be pretty high.
       **/
      maxAdditionalFields: u32 & AugmentedConst<ApiType>;
      /**
       * Maxmimum number of registrars allowed in the system. Needed to bound the complexity
       * of, e.g., updating judgements.
       **/
      maxRegistrars: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum number of sub-accounts allowed per identified account.
       **/
      maxSubAccounts: u32 & AugmentedConst<ApiType>;
      /**
       * The amount held on deposit for a registered subaccount. This should account for the fact
       * that one storage item's value will increase by the size of an account ID, and there will
       * be another trie item whose value is the size of an account ID plus 32 bytes.
       **/
      subAccountDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    indices: {
      /**
       * The deposit needed for reserving an index.
       **/
      deposit: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    lending: {
      /**
       * Minimal price of borrow asset in Oracle price required to create.
       * Examples, 100 USDC.
       * Creators puts that amount and it is staked under Vault account.
       * So he does not owns it anymore.
       * So borrow is both stake and tool to create market.
       * 
       * # Why not pure borrow amount minimum?
       * 
       * Borrow may have very small price. Will imbalance some markets on creation.
       * 
       * # Why not native parachain token?
       * 
       * Possible option. But I doubt closing market as easy as transferring back rent.  So it is
       * not exactly platform rent only.
       * 
       * # Why borrow amount priced by Oracle?
       * 
       * We depend on Oracle to price in Lending. So we know price anyway.
       * We normalized price over all markets and protect from spam all possible pairs equally.
       * Locking borrow amount ensures manager can create market with borrow assets, and we force
       * him to really create it.
       * 
       * This solution forces to have amount before creating market.
       * Vault can take that amount if reconfigured so, but that may be changed during runtime
       * upgrades.
       **/
      oracleMarketCreationStake: u128 & AugmentedConst<ApiType>;
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    liquidations: {
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    mosaic: {
      /**
       * The minimum period for which we lock outgoing/incoming funds.
       **/
      minimumTimeLockPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * The minimum time to live before a relayer account rotation.
       **/
      minimumTTL: u32 & AugmentedConst<ApiType>;
      timelockPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    multisig: {
      /**
       * The base amount of currency needed to reserve for creating a multisig execution or to
       * store a dispatch call for later.
       * 
       * This is held for an additional storage item whose value size is
       * `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is
       * `32 + sizeof(AccountId)` bytes.
       **/
      depositBase: u128 & AugmentedConst<ApiType>;
      /**
       * The amount of currency needed per unit threshold when creating a multisig execution.
       * 
       * This is held for adding 32 bytes more into a pre-existing storage value.
       **/
      depositFactor: u128 & AugmentedConst<ApiType>;
      /**
       * The maximum amount of signatories allowed in the multisig.
       **/
      maxSignatories: u16 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    oracle: {
      maxHistory: u32 & AugmentedConst<ApiType>;
      maxPrePrices: u32 & AugmentedConst<ApiType>;
      msPerBlock: u64 & AugmentedConst<ApiType>;
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      twapWindow: u16 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    pablo: {
      msPerBlock: u32 & AugmentedConst<ApiType>;
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * AssetId of the PBLO asset
       **/
      pbloAssetId: u128 & AugmentedConst<ApiType>;
      /**
       * AssetId of the PICA asset
       **/
      picaAssetId: u128 & AugmentedConst<ApiType>;
      /**
       * The interval between TWAP computations.
       **/
      twapInterval: u64 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    proxy: {
      /**
       * The base amount of currency needed to reserve for creating an announcement.
       * 
       * This is held when a new storage item holding a `Balance` is created (typically 16
       * bytes).
       **/
      announcementDepositBase: u128 & AugmentedConst<ApiType>;
      /**
       * The amount of currency needed per announcement made.
       * 
       * This is held for adding an `AccountId`, `Hash` and `BlockNumber` (typically 68 bytes)
       * into a pre-existing storage value.
       **/
      announcementDepositFactor: u128 & AugmentedConst<ApiType>;
      /**
       * The maximum amount of time-delayed announcements that are allowed to be pending.
       **/
      maxPending: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum amount of proxies allowed for a single account.
       **/
      maxProxies: u32 & AugmentedConst<ApiType>;
      /**
       * The base amount of currency needed to reserve for creating a proxy.
       * 
       * This is held for an additional storage item whose value size is
       * `sizeof(Balance)` bytes and whose key size is `sizeof(AccountId)` bytes.
       **/
      proxyDepositBase: u128 & AugmentedConst<ApiType>;
      /**
       * The amount of currency needed per proxy added.
       * 
       * This is held for adding 32 bytes plus an instance of `ProxyType` more into a
       * pre-existing storage value. Thus, when configuring `ProxyDepositFactor` one should take
       * into account `32 + proxy_type.encode().len()` bytes of data.
       **/
      proxyDepositFactor: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    scheduler: {
      /**
       * The maximum weight that may be scheduled per block for any dispatchables of less
       * priority than `schedule::HARD_DEADLINE`.
       **/
      maximumWeight: u64 & AugmentedConst<ApiType>;
      /**
       * The maximum number of scheduled calls in the queue for a single block.
       * Not strictly enforced, but used for weight estimation.
       **/
      maxScheduledPerBlock: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    stakingRewards: {
      lockId: U8aFixed & AugmentedConst<ApiType>;
      /**
       * Maximum number of reward configurations per pool.
       **/
      maxRewardConfigsPerPool: u32 & AugmentedConst<ApiType>;
      /**
       * Maximum number of staking duration presets allowed.
       **/
      maxStakingDurationPresets: u32 & AugmentedConst<ApiType>;
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      pbloAssetId: u128 & AugmentedConst<ApiType>;
      pbloStakeFinancialNftCollectionId: u128 & AugmentedConst<ApiType>;
      picaAssetId: u128 & AugmentedConst<ApiType>;
      picaStakeFinancialNftCollectionId: u128 & AugmentedConst<ApiType>;
      /**
       * the size of batch to take each time trying to release rewards
       **/
      releaseRewardsPoolsBatchSize: u8 & AugmentedConst<ApiType>;
      treasuryAccount: AccountId32 & AugmentedConst<ApiType>;
      xPbloAssetId: u128 & AugmentedConst<ApiType>;
      xPicaAssetId: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    system: {
      /**
       * Maximum number of block number to block hash mappings to keep (oldest pruned first).
       **/
      blockHashCount: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum length of a block (in bytes).
       **/
      blockLength: FrameSystemLimitsBlockLength & AugmentedConst<ApiType>;
      /**
       * Block & extrinsics weights: base values and limits.
       **/
      blockWeights: FrameSystemLimitsBlockWeights & AugmentedConst<ApiType>;
      /**
       * The weight of runtime database operations the runtime can invoke.
       **/
      dbWeight: FrameSupportWeightsRuntimeDbWeight & AugmentedConst<ApiType>;
      /**
       * The designated SS85 prefix of this chain.
       * 
       * This replaces the "ss58Format" property declared in the chain spec. Reason is
       * that the runtime should know about the prefix in order to make use of it as
       * an identifier of the chain.
       **/
      ss58Prefix: u16 & AugmentedConst<ApiType>;
      /**
       * Get the chain's current version.
       **/
      version: SpVersionRuntimeVersion & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    timestamp: {
      /**
       * The minimum period between blocks. Beware that this is different to the *expected*
       * period that the block production apparatus provides. Your chosen consensus system will
       * generally work with this to determine a sensible block time. e.g. For Aura, it will be
       * double this period on default settings.
       **/
      minimumPeriod: u64 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    tokens: {
      maxLocks: u32 & AugmentedConst<ApiType>;
      /**
       * The maximum number of named reserves that can exist on an account.
       **/
      maxReserves: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    transactionPayment: {
      /**
       * A fee mulitplier for `Operational` extrinsics to compute "virtual tip" to boost their
       * `priority`
       * 
       * This value is multipled by the `final_fee` to obtain a "virtual tip" that is later
       * added to a tip component in regular `priority` calculations.
       * It means that a `Normal` transaction can front-run a similarly-sized `Operational`
       * extrinsic (with no tip), by including a tip value greater than the virtual tip.
       * 
       * ```rust,ignore
       * // For `Normal`
       * let priority = priority_calc(tip);
       * 
       * // For `Operational`
       * let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
       * let priority = priority_calc(tip + virtual_tip);
       * ```
       * 
       * Note that since we use `final_fee` the multiplier applies also to the regular `tip`
       * sent with the transaction. So, not only does the transaction get a priority bump based
       * on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`
       * transactions.
       **/
      operationalFeeMultiplier: u8 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    treasury: {
      /**
       * Percentage of spare funds (if any) that are burnt per spend period.
       **/
      burn: Permill & AugmentedConst<ApiType>;
      /**
       * The maximum number of approvals that can wait in the spending queue.
       * 
       * NOTE: This parameter is also used within the Bounties Pallet extension if enabled.
       **/
      maxApprovals: u32 & AugmentedConst<ApiType>;
      /**
       * The treasury's pallet id, used for deriving its sovereign account ID.
       **/
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * Fraction of a proposal's value that should be bonded in order to place the proposal.
       * An accepted proposal gets these back. A rejected proposal does not.
       **/
      proposalBond: Permill & AugmentedConst<ApiType>;
      /**
       * Maximum amount of funds that should be placed in a deposit for making a proposal.
       **/
      proposalBondMaximum: Option<u128> & AugmentedConst<ApiType>;
      /**
       * Minimum amount of funds that should be placed in a deposit for making a proposal.
       **/
      proposalBondMinimum: u128 & AugmentedConst<ApiType>;
      /**
       * Period between successive spends.
       **/
      spendPeriod: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    utility: {
      /**
       * The limit on the number of batched calls.
       **/
      batchedCallsLimit: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    vault: {
      /**
       * The minimum native asset needed to create a vault.
       **/
      creationDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The deposit needed for a vault to never be cleaned up. Should be significantly higher
       * than the rent.
       **/
      existentialDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The minimum amount needed to deposit in a vault and receive LP tokens.
       **/
      minimumDeposit: u128 & AugmentedConst<ApiType>;
      /**
       * The minimum amount of LP tokens to withdraw funds from a vault.
       **/
      minimumWithdrawal: u128 & AugmentedConst<ApiType>;
      /**
       * The id used as the `AccountId` of the vault. This should be unique across all pallets to
       * avoid name collisions with other pallets and vaults.
       **/
      palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
      /**
       * The rent being charged per block for vaults which have not committed the
       * `ExistentialDeposit`.
       **/
      rentPerBlock: u128 & AugmentedConst<ApiType>;
      /**
       * The duration that a vault may remain tombstoned before it can be deleted.
       **/
      tombstoneDuration: u32 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    vesting: {
      /**
       * The minimum amount transferred to call `vested_transfer`.
       **/
      minVestedTransfer: u128 & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
    xTokens: {
      /**
       * Base XCM weight.
       * 
       * The actually weight for an XCM message is `T::BaseXcmWeight +
       * T::Weigher::weight(&msg)`.
       **/
      baseXcmWeight: u64 & AugmentedConst<ApiType>;
      /**
       * Self chain location.
       **/
      selfLocation: XcmV1MultiLocation & AugmentedConst<ApiType>;
      /**
       * Generic const
       **/
      [key: string]: Codec;
    };
  } // AugmentedConsts
} // declare module
