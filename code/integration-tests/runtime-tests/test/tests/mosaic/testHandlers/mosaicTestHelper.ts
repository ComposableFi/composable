import { KeyringPair } from "@polkadot/keyring/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import {
  CommonMosaicRemoteAssetId,
  PalletMosaicDecayBudgetPenaltyDecayer,
  PalletMosaicNetworkInfo
} from "@composable/types/interfaces";
import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { IEvent } from "@polkadot/types/types";

export class TxMosaicTests {
  /**
   * Makes a transaction to set the relayer.
   *
   * @param {ApiPromise} api Connected api client.
   * @param {KeyringPair} sudoKey Wallet with sudo rights.
   * @param {string} relayerWalletAddress Relayer wallet address to be set to.
   */
  public static async testSetRelayer(api: ApiPromise, sudoKey: KeyringPair, relayerWalletAddress: string) {
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.mosaic.setRelayer(relayerWalletAddress))
    );
  }

  /**
   * Makes a transaction to rotate the relayer.
   *
   * @param {ApiPromise} api Connected api client.
   * @param {KeyringPair} startRelayerWallet Initial relayer wallet.
   * @param {string} newRelayerWalletAddress Relayer wallet address to be set to.
   */
  public static async testRotateRelayer(
    api: ApiPromise,
    startRelayerWallet: KeyringPair,
    newRelayerWalletAddress: string
  ) {
    const paramTtl = api.createType("u32", 90);
    return await sendAndWaitForSuccess(
      api,
      startRelayerWallet,
      api.events.mosaic.RelayerRotated.is,
      api.tx.mosaic.rotateRelayer(newRelayerWalletAddress, paramTtl)
    );
  }

  /**
   * Makes a transaction to set the network.
   *
   * @param {ApiPromise} api Connected api client.
   * @param {KeyringPair} walletId
   * @param {number} networkId ID of the network to be set to.
   * @param {PalletMosaicNetworkInfo} networkInfo Object with information of the network.
   */
  public static async testSetNetwork(
    api: ApiPromise,
    walletId: KeyringPair,
    networkId: u128,
    networkInfo: PalletMosaicNetworkInfo
  ) {
    return await sendAndWaitForSuccess(
      api,
      walletId,
      api.events.mosaic.NetworksUpdated.is,
      api.tx.mosaic.setNetwork(networkId, networkInfo)
    );
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
  public static async testSetBudget(
    api: ApiPromise,
    sudoKey: KeyringPair,
    assetId: number,
    amount: number,
    decay: PalletMosaicDecayBudgetPenaltyDecayer
  ) {
    const pAssetId = api.createType("u128", assetId);
    const pAmount = api.createType("u128", amount);
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.mosaic.setBudget(pAssetId, pAmount, decay))
    );
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
  public static async testUpdateAssetMapping(
    api: ApiPromise,
    sudoKey: KeyringPair,
    assetId: number,
    networkId: u128,
    remoteAssetId: CommonMosaicRemoteAssetId
  ) {
    const pAssetId = api.createType("u128", assetId);
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.mosaic.updateAssetMapping(pAssetId, networkId, remoteAssetId))
    );
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
  public static async testTransferTo(
    api: ApiPromise,
    relayerWallet: KeyringPair,
    networkId: u128,
    assetId: number,
    ethAddress: string,
    transferAmount: number
  ) {
    const pAssetId = api.createType("u128", assetId);
    const contractAddress = api.createType("[u8;20]", ethAddress);
    const pAmount = api.createType("u128", transferAmount);
    const pMinimumAmountOut = api.createType("u128", transferAmount * 0.5);
    const pSwapToNative = api.createType("bool", false);
    const pSourceUserAccount = api.createType("AccountId32", relayerWallet.address);
    const pAmmSwapInfo = api.createType("Option<PalletMosaicAmmSwapInfo>", null);
    const pKeepAlive = api.createType("bool", false);
    return await sendAndWaitForSuccess(
      api,
      relayerWallet,
      api.events.mosaic.TransferOut.is,
      api.tx.mosaic.transferTo(
        networkId,
        pAssetId,
        contractAddress,
        pAmount,
        pMinimumAmountOut,
        pSwapToNative,
        pSourceUserAccount,
        pAmmSwapInfo,
        pKeepAlive
      )
    );
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
  public static async lockFunds(
    api: ApiPromise,
    wallet: KeyringPair,
    networkId: u128,
    remoteAsset: CommonMosaicRemoteAssetId,
    sentWalletAddress: string,
    transferAmount: number
  ) {
    const paramId = api.createType("H256", "0x");
    const pAmount = api.createType("u128", transferAmount);
    const pLockTime = api.createType("u32", 10);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.TransferInto.is,
      api.tx.mosaic.timelockedMint(networkId, remoteAsset, sentWalletAddress, pAmount, pLockTime, paramId)
    );
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
  public static async testAcceptTransfer(
    api: ApiPromise,
    wallet: KeyringPair,
    senderWallet: KeyringPair,
    networkId: u128,
    remoteAssetId: CommonMosaicRemoteAssetId,
    transferAmount: number
  ) {
    const amount = api.createType("u128", transferAmount);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.TransferAccepted.is,
      api.tx.mosaic.acceptTransfer(senderWallet.address, networkId, remoteAssetId, amount)
    );
  }

  /**
   * Making a transaction to claim a transaction.
   *
   * @param {ApiPromise} api Connected api client.
   * @param {KeyringPair} wallet Wallet trying to claim.
   * @param {KeyringPair} receiverWallet Wallet that is supposed to receive the funds.
   * @param {number} assetId ID of the asset.
   */
  public static async testClaimTransactions(
    api: ApiPromise,
    wallet: KeyringPair,
    receiverWallet: KeyringPair,
    assetId: number
  ) {
    const pAssetId = api.createType("u128", assetId);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.TransferClaimed.is,
      api.tx.mosaic.claimTo(pAssetId, receiverWallet.address)
    );
  }

  /**
   * Making a transaction claim stale funds.
   *
   * @param {ApiPromise} api Connected api client.
   * @param {KeyringPair} wallet Wallet trying to claim.
   * @param {number} assetId ID of the asset.
   */
  public static async testClaimStaleFunds(
    api: ApiPromise,
    wallet: KeyringPair,
    assetId: number
  ): Promise<IEvent<[to: AccountId32, by: AccountId32, assetId: u128, amount: u128]>> {
    const pAssetId = api.createType("u128", assetId);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.StaleTxClaimed.is,
      api.tx.mosaic.claimStaleTo(pAssetId, wallet.address)
    );
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
  public static async testRescindTimeLockedFunds(
    api: ApiPromise,
    wallet: KeyringPair,
    returnWallet: KeyringPair,
    remoteAssetId: CommonMosaicRemoteAssetId,
    transferAmount: number
  ) {
    const amount = api.createType("u128", transferAmount - 3000);
    const networkId = api.createType("u32", 1);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.TransferIntoRescined.is,
      api.tx.mosaic.rescindTimelockedMint(networkId, remoteAssetId, returnWallet.address, amount)
    );
  }
}
