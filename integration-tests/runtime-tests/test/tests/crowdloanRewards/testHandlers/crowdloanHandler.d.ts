/// <reference types="@composable/types/interfaces/types-lookup" />
import { KeyringPair } from "@polkadot/keyring/types";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types/interfaces";
import { u128 } from "@polkadot/types-codec";
import { ApiPromise } from "@polkadot/api";
export declare const ethAccount: (seed: number) => import("web3-core").Account;
export declare class TxCrowdloanRewardsTests {
    /**
     * Providing the crowdloan pallet with funds
     *
     * Unfortunately we can't directly mint into the pallet therefore,
     * we mint into the Alice wallet and transfer funds from there.
     *
     * @param {ApiPromise} api Connected API Client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param amount
     */
    static beforeCrowdloanTestsProvideFunds(api: ApiPromise, sudoKey: KeyringPair, amount: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types/interfaces").AccountId32, u128]>>;
    /**
     * tx.crowdloanRewards.initialize
     *
     * @param {ApiPromise} api Connected API Client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     */
    static txCrowdloanRewardsInitializeTest(api: ApiPromise, sudoKey: KeyringPair): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    /**
     * tx.crowdloanRewards.populate
     *
     * @param {ApiPromise} api Connected API Client.
     * @param {Web3} web3 Web3 Object, to be received using `connectionHelper.getNewConnection()`
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param testContributorWallet KSM Wallet of contributor to populate with.
     */
    static txCrowdloanRewardsPopulateTest(api: ApiPromise, sudoKey: KeyringPair, testContributorWallet: KeyringPair): Promise<PalletCrowdloanRewardsModelsRemoteAccount>;
    /**
     * tx.crowdloanRewards.populate
     *
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {KeyringPair} contributors List of contributors to be transacted.
     */
    static txCrowdloanRewardsPopulateTestHandler(api: ApiPromise, sudoKey: KeyringPair, contributors: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    /**
     * tx.crowdloanRewards.associate RelayChain
     *
     * @param {KeyringPair} contributor The contributor relay chain wallet public key.
     * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
     */
    static txCrowdloanRewardsRelayAssociateTests(api: ApiPromise, contributor: KeyringPair, contributorRewardAccount: any): Promise<import("@polkadot/types/types").IEvent<[PalletCrowdloanRewardsModelsRemoteAccount, import("@polkadot/types/interfaces").AccountId32]>>;
    /**
     * tx.crowdloanRewards.associate ETH Chain
     *
     * @param {KeyringPair} contributor The contributor ETH chain wallet public key.
     * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
     */
    static txCrowdloanRewardsEthAssociateTest(api: ApiPromise, contributor: any, contributorRewardAccount: any): Promise<import("@polkadot/types/types").IEvent<[PalletCrowdloanRewardsModelsRemoteAccount, import("@polkadot/types/interfaces").AccountId32]>>;
    /**
     * tx.crowdloanRewards.claim
     *
     * @param { KeyringPair } wallet The reward account which tries to claim.
     */
    static txCrowdloanRewardsClaimTest(api: ApiPromise, wallet: KeyringPair): Promise<import("@polkadot/types/types").IEvent<[PalletCrowdloanRewardsModelsRemoteAccount, import("@polkadot/types/interfaces").AccountId32, u128]>>;
}
