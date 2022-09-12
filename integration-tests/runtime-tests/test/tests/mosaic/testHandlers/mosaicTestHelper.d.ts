/// <reference types="@composable/types/interfaces/types-lookup" />
import { KeyringPair } from "@polkadot/keyring/types";
import { CommonMosaicRemoteAssetId, PalletMosaicDecayBudgetPenaltyDecayer, PalletMosaicNetworkInfo } from "@composable/types/interfaces";
import { ApiPromise } from "@polkadot/api";
export declare class TxMosaicTests {
    /**
     * Makes a transaction to set the relayer.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {string} relayerWalletAddress Relayer wallet address to be set to.
     */
    static testSetRelayer(api: ApiPromise, sudoKey: KeyringPair, relayerWalletAddress: string): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    /**
     * Makes a transaction to rotate the relayer.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} startRelayerWallet Initial relayer wallet.
     * @param {string} newRelayerWalletAddress Relayer wallet address to be set to.
     */
    static testRotateRelayer(api: ApiPromise, startRelayerWallet: KeyringPair, newRelayerWalletAddress: string): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").u32, import("@polkadot/types/interfaces").AccountId32]>>;
    /**
     * Makes a transaction to set the network.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} walletId
     * @param {number} networkId ID of the network to be set to.
     * @param {PalletMosaicNetworkInfo} networkInfo Object with information of the network.
     */
    static testSetNetwork(api: ApiPromise, walletId: KeyringPair, networkId: number, networkInfo: PalletMosaicNetworkInfo): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").u32, PalletMosaicNetworkInfo]>>;
    /**
     * Makes a transaction to set the budget.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {number} assetId ID of the asset to set budget of.
     * @param {number} amount Budget amount
     * @param {PalletMosaicDecayBudgetPenaltyDecayer} decay
     */
    static testSetBudget(api: ApiPromise, sudoKey: KeyringPair, assetId: number, amount: number, decay: PalletMosaicDecayBudgetPenaltyDecayer): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    /**
     * Makes a transaction to update the asset mapping.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {number} assetId ID of the asset to set budget of.
     * @param {number} networkId ID of the network to be set to.
     * @param {remoteAssetId} ID of the asset on the remote blockchain.
     */
    static testUpdateAssetMaping(api: ApiPromise, sudoKey: KeyringPair, assetId: number, networkId: number, remoteAssetId: CommonMosaicRemoteAssetId): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    /**
     * Makes a transaction to a remote blockchain.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} relayerWallet Wallet of the relayer.
     * @param {number} networkId ID of the network to be set to.
     * @param {number} assetId ID of the asset to set budget of.
     * @param {string} ethAddress ETH wallet receiving the funds.
     * @param {transferAmount} transferAmount Amount to be transferred.
     */
    static testTransferTo(api: ApiPromise, relayerWallet: KeyringPair, networkId: number, assetId: number, ethAddress: string, transferAmount: number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").H256, import("@composable/types/interfaces").ComposableSupportEthereumAddress, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").u32, CommonMosaicRemoteAssetId, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").bool, import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").Option<import("@composable/types/interfaces").PalletMosaicAmmSwapInfo>, import("@polkadot/types-codec").u128]>>;
    /**
     * Locks the funds on our chain as preparation to send them to a remote one.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet trying to move funds.
     * @param {number} networkId ID of the network receiving funds.
     * @param {CommonMosaicRemoteAssetId} remoteAsset ID of the asset on the remote chain.
     * @param {string} sentWalletAddress
     * @param {number} transferAmount Amount to be transferred.
     */
    static lockFunds(api: ApiPromise, wallet: KeyringPair, networkId: number, remoteAsset: CommonMosaicRemoteAssetId, sentWalletAddress: string, transferAmount: number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").H256, import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u32, CommonMosaicRemoteAssetId, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").u128]>>;
    /**
     * Makes a transaction to accept an incoming transfer.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet trying to receive funds.
     * @param {KeyringPair} senderWallet Wallet receiving funds.
     * @param {number} networkId ID of the network sending funds.
     * @param {CommonMosaicRemoteAssetId} remoteAssetId ID of the asset on the remote chain.
     * @param {number} transferAmount Amount to be transferred.
     */
    static testAcceptTransfer(api: ApiPromise, wallet: KeyringPair, senderWallet: KeyringPair, networkId: number, remoteAssetId: CommonMosaicRemoteAssetId, transferAmount: number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").u32, CommonMosaicRemoteAssetId, import("@polkadot/types-codec").u128]>>;
    /**
     * Making a transaction to claim a transaction.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet trying to claim.
     * @param {KeyringPair} receiverWallet Wallet that is supposed to receive the funds.
     * @param {number} assetId ID of the asset.
     */
    static testClaimTransactions(api: ApiPromise, wallet: KeyringPair, receiverWallet: KeyringPair, assetId: number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").u128]>>;
    /**
     * Making a transaction claim stale funds.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet trying to claim.
     * @param {number} assetId ID of the asset.
     */
    static testClaimStaleFunds(api: ApiPromise, wallet: KeyringPair, assetId: number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").u128]>>;
    /**
     * Making a transaction to rescind time locked funds
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet making the transaction.
     * @param {KeyringPair} returnWallet Wallet receiving the returned funds.
     * @param {CommonMosaicRemoteAssetId} remoteAssetId ID of the asset on the remote chain.
     * @param {number} transferAmount Amount to be transferred.
     */
    static testRescindTimeLockedFunds(api: ApiPromise, wallet: KeyringPair, returnWallet: KeyringPair, remoteAssetId: CommonMosaicRemoteAssetId, transferAmount: number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/interfaces").AccountId32, import("@polkadot/types-codec").u128, import("@polkadot/types-codec").u128]>>;
}
