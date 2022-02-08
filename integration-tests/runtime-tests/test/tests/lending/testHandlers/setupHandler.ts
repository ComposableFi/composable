import {sendAndWaitForSuccess, waitForBlocks} from "@composable/utils/polkadotjs";


export async function handleLendingAssetMintSetup(sudoKey, assetId, lendingWallet, mintingAmount) {
  const {data: [result1],} = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(assetId, lendingWallet.publicKey, mintingAmount)
    )
  );
  await waitForBlocks(); // Let's wait a block, cause we sometimes failed here otherwise.
  const {data: [result2],} = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(1, lendingWallet.publicKey, mintingAmount)
    )
  );
  return [result1, result2];
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
