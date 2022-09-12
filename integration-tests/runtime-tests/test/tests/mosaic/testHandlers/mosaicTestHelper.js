"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TxMosaicTests = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
class TxMosaicTests {
    /**
     * Makes a transaction to set the relayer.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {string} relayerWalletAddress Relayer wallet address to be set to.
     */
    static async testSetRelayer(api, sudoKey, relayerWalletAddress) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.mosaic.setRelayer(relayerWalletAddress)));
    }
    /**
     * Makes a transaction to rotate the relayer.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} startRelayerWallet Initial relayer wallet.
     * @param {string} newRelayerWalletAddress Relayer wallet address to be set to.
     */
    static async testRotateRelayer(api, startRelayerWallet, newRelayerWalletAddress) {
        const paramTtl = api.createType("u32", 90);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, startRelayerWallet, api.events.mosaic.RelayerRotated.is, api.tx.mosaic.rotateRelayer(newRelayerWalletAddress, paramTtl));
    }
    /**
     * Makes a transaction to set the network.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} walletId
     * @param {number} networkId ID of the network to be set to.
     * @param {PalletMosaicNetworkInfo} networkInfo Object with information of the network.
     */
    static async testSetNetwork(api, walletId, networkId, networkInfo) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId, api.events.mosaic.NetworksUpdated.is, api.tx.mosaic.setNetwork(networkId, networkInfo));
    }
    /**
     * Makes a transaction to set the budget.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {number} assetId ID of the asset to set budget of.
     * @param {number} amount Budget amount
     * @param {PalletMosaicDecayBudgetPenaltyDecayer} decay
     */
    static async testSetBudget(api, sudoKey, assetId, amount, decay) {
        const pAssetId = api.createType("u128", assetId);
        const pAmount = api.createType("u128", amount);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.mosaic.setBudget(pAssetId, pAmount, decay)));
    }
    /**
     * Makes a transaction to update the asset mapping.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {number} assetId ID of the asset to set budget of.
     * @param {number} networkId ID of the network to be set to.
     * @param {remoteAssetId} ID of the asset on the remote blockchain.
     */
    static async testUpdateAssetMaping(api, sudoKey, assetId, networkId, remoteAssetId) {
        const pAssetId = api.createType("u128", assetId);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.mosaic.updateAssetMapping(pAssetId, networkId, remoteAssetId)));
    }
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
    static async testTransferTo(api, relayerWallet, networkId, assetId, ethAddress, transferAmount) {
        const pAssetId = api.createType("u128", assetId);
        const contractAddress = api.createType("[u8;20]", ethAddress);
        const pAmount = api.createType("u128", transferAmount);
        const pMinimumAmountOut = api.createType("u128", transferAmount * 0.5);
        const pSwapToNative = api.createType("bool", false);
        const pSourceUserAccount = api.createType("AccountId32", relayerWallet.address);
        const pAmmSwapInfo = api.createType("Option<PalletMosaicAmmSwapInfo>", null);
        const pKeepAlive = api.createType("bool", false);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, relayerWallet, api.events.mosaic.TransferOut.is, api.tx.mosaic.transferTo(networkId, pAssetId, contractAddress, pAmount, pMinimumAmountOut, pSwapToNative, pSourceUserAccount, pAmmSwapInfo, pKeepAlive));
    }
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
    static async lockFunds(api, wallet, networkId, remoteAsset, sentWalletAddress, transferAmount) {
        const paramId = api.createType("H256", "0x");
        const pAmount = api.createType("u128", transferAmount);
        const pLockTime = api.createType("u32", 10);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.mosaic.TransferInto.is, api.tx.mosaic.timelockedMint(networkId, remoteAsset, sentWalletAddress, pAmount, pLockTime, paramId));
    }
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
    static async testAcceptTransfer(api, wallet, senderWallet, networkId, remoteAssetId, transferAmount) {
        const amount = api.createType("u128", transferAmount);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.mosaic.TransferAccepted.is, api.tx.mosaic.acceptTransfer(senderWallet.address, networkId, remoteAssetId, amount));
    }
    /**
     * Making a transaction to claim a transaction.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet trying to claim.
     * @param {KeyringPair} receiverWallet Wallet that is supposed to receive the funds.
     * @param {number} assetId ID of the asset.
     */
    static async testClaimTransactions(api, wallet, receiverWallet, assetId) {
        const pAssetId = api.createType("u128", assetId);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.mosaic.TransferClaimed.is, api.tx.mosaic.claimTo(pAssetId, receiverWallet.address));
    }
    /**
     * Making a transaction claim stale funds.
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet trying to claim.
     * @param {number} assetId ID of the asset.
     */
    static async testClaimStaleFunds(api, wallet, assetId) {
        const pAssetId = api.createType("u128", assetId);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.mosaic.StaleTxClaimed.is, api.tx.mosaic.claimStaleTo(pAssetId, wallet.address));
    }
    /**
     * Making a transaction to rescind time locked funds
     *
     * @param {ApiPromise} api Connected api client.
     * @param {KeyringPair} wallet Wallet making the transaction.
     * @param {KeyringPair} returnWallet Wallet receiving the returned funds.
     * @param {CommonMosaicRemoteAssetId} remoteAssetId ID of the asset on the remote chain.
     * @param {number} transferAmount Amount to be transferred.
     */
    static async testRescindTimeLockedFunds(api, wallet, returnWallet, remoteAssetId, transferAmount) {
        const amount = api.createType("u128", transferAmount - 3000);
        const networkId = api.createType("u32", 1);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.mosaic.TransferIntoRescined.is, api.tx.mosaic.rescindTimelockedMint(networkId, remoteAssetId, returnWallet.address, amount));
    }
}
exports.TxMosaicTests = TxMosaicTests;
//# sourceMappingURL=mosaicTestHelper.js.map