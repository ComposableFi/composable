import {sendAndWaitForSuccess, waitForBlocks} from "@composable/utils/polkadotjs";


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
