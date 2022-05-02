import { KeyringPair } from "@polkadot/keyring/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import {
  CommonMosaicRemoteAssetId,
  PalletMosaicDecayBudgetPenaltyDecayer,
  PalletMosaicNetworkInfo
} from "@composable/types/interfaces";

export class TxMosaicTests {

  public static async testSetRelayer(sudoKey: KeyringPair, relayerWalletAddress: string) {
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.mosaic.setRelayer(relayerWalletAddress))
    );
  }

  public static async testRotateRelayer(startRelayerWallet: KeyringPair, newRelayerWalletAddress: string) {
    const paramTtl = api.createType("u32", 90);
    return await sendAndWaitForSuccess(
      api,
      startRelayerWallet,
      api.events.mosaic.RelayerRotated.is,
      api.tx.mosaic.rotateRelayer(newRelayerWalletAddress, paramTtl)
    );
  }

  public static async testSetNetwork(walletId: KeyringPair, networkId: number, networkInfo: PalletMosaicNetworkInfo) {
    return await sendAndWaitForSuccess(
      api,
      walletId,
      api.events.mosaic.NetworksUpdated.is,
      api.tx.mosaic.setNetwork(networkId, networkInfo)
    );
  }

  public static async testSetBudget(sudoKey: KeyringPair, assetId: number, amount: number, decay: PalletMosaicDecayBudgetPenaltyDecayer) {
    const pAssetId = api.createType("u128", assetId);
    const pAmount = api.createType("u128", amount);
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.mosaic.setBudget(pAssetId, pAmount, decay))
    );
  }

  public static async testUpdateAssetMaping(sudoKey: KeyringPair, assetId: number, networkId: number, remoteAssetId: CommonMosaicRemoteAssetId) {
    const pAssetId = api.createType("u128", assetId);
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.mosaic.updateAssetMapping(pAssetId, networkId, remoteAssetId))
    );
  }

  public static async testTransferTo(relayerWallet: KeyringPair, networkId: number, assetId: number, ethAddress: string, transferAmount: number) {
    const pAssetId = api.createType("u128", assetId);
    const contractAddress = api.createType("[u8;20]", ethAddress);
    const pAmount = api.createType("u128", transferAmount);
    const pKeepAlive = api.createType("bool", false);
    return await sendAndWaitForSuccess(
      api,
      relayerWallet,
      api.events.mosaic.TransferOut.is,
      api.tx.mosaic.transferTo(networkId, pAssetId, contractAddress, pAmount, pKeepAlive)
    );
  }

  public static async lockFunds(wallet: KeyringPair, networkId: number, remoteAsset: CommonMosaicRemoteAssetId, sentWalletAddress: string, transferAmount: number) {
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

  public static async testAcceptTransfer(wallet: KeyringPair, senderWallet: KeyringPair, networkId: number, remoteAssetId: CommonMosaicRemoteAssetId, transferAmount: number) {
    const amount = api.createType("u128", transferAmount);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.TransferAccepted.is,
      api.tx.mosaic.acceptTransfer(senderWallet.address,
        networkId,
        remoteAssetId,
        amount)
    );
  }

  public static async testClaimTransactions(wallet: KeyringPair, receiverWallet: KeyringPair, assetId: number) {
    const pAssetId = api.createType("u128", assetId);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.TransferClaimed.is,
      api.tx.mosaic.claimTo(pAssetId, receiverWallet.address)
    );
  }

  public static async testClaimStaleFunds(wallet: KeyringPair, assetId: number) {
    const pAssetId = api.createType("u128", assetId);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.mosaic.StaleTxClaimed.is,
      api.tx.mosaic.claimStaleTo(pAssetId, wallet.address)
    );
  }

  public static async testRescindTimeLockedFunds(wallet: KeyringPair, returnWallet: KeyringPair, remoteAssetId: CommonMosaicRemoteAssetId, transferAmount: number) {
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
