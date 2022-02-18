import {sendAndWaitForSuccess, waitForBlocks} from "@composable/utils/polkadotjs";
import {expect} from "chai";
import {KeyringPair} from "@polkadot/keyring/types";


export async function handleAssetMintSetup(sudoKey, assets:number[], wallet:KeyringPair, mintingAmount) {
  for (const asset of assets) {
    const {data:[result]} = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.assets.mintInto(asset, wallet.publicKey, mintingAmount)
      )
    )
    expect(result.isOk).to.be.true;
  }
  await waitForBlocks();
}

export async function handleLendingVaultSetup(
  vaultAssetId,
  reserved,
  vaultManagerWallet,
  strategies,
  depositAmount
) {
  const vault = api.createType('ComposableTraitsVaultVaultConfig', {
    assetId: api.createType('u128', vaultAssetId),
    reserved: api.createType('Perquintill', reserved),
    manager: api.createType('AccountId32', vaultManagerWallet.publicKey),
    strategies: strategies//api.createType('(AccountId32, Perquintill)', strategies)
  });
  return await sendAndWaitForSuccess(
    api,
    vaultManagerWallet,
    api.events.vault.VaultCreated.is,
    api.tx.vault.create(vault, depositAmount)
  );
}
