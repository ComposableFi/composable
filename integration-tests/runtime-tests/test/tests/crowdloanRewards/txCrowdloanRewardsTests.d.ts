/// <reference types="@composable/types/interfaces/types-lookup" />
import { PalletCrowdloanRewardsModelsRemoteAccount } from '@composable/types/interfaces';
import { KeyringPair } from "@polkadot/keyring/types";
export declare class TxCrowdloanRewardsTests {
    /**
     * Task order list:
     *  * Populate the list of contributors
     *  * Initialize the crowdloan
     *  * Associate a picassso account
     */
    /**
     * tx.crowdloanRewards.initialize
     */
    static txCrowdloanRewardsInitializeTest(sudoKey: KeyringPair): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    /**
     * tx.crowdloanRewards.populate
     */
    static txCrowdloanRewardsPopulateTest(sudoKey: KeyringPair): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    /**
     * tx.crowdloanRewards.associate RelayChain
     * @param { KeyringPair } wallet
     * @param contributor
     * @param contributorRewardAccount
     */
    static txCrowdloanRewardsRelayAssociateTests(wallet: KeyringPair, contributor: any, contributorRewardAccount: any): Promise<import("@polkadot/types/types").IEvent<[PalletCrowdloanRewardsModelsRemoteAccount, import("@polkadot/types/interfaces").AccountId32]>>;
    /**
     * tx.crowdloanRewards.associate ETH Chain
     * @param { KeyringPair } wallet
     * @param contributor
     * @param contributorRewardAccount
     */
    static txCrowdloanRewardsEthAssociateTests(wallet: KeyringPair, contributor: any, contributorRewardAccount: any): Promise<import("@polkadot/types/types").IEvent<[PalletCrowdloanRewardsModelsRemoteAccount, import("@polkadot/types/interfaces").AccountId32]>>;
}
