import { createOffer } from "@composable/bootstrap_pallets/lib/pallets/bondedFinance/extrinsics";
import { BondOffer } from "@composable/bootstrap_pallets/types";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import config from "@composable/bootstrap_pallets/constants/config.json";
import { logger, toBondOffer, toChainUnits } from "@composable/bootstrap_pallets/utils";
import BigNumber from "bignumber.js";
import { mintAssetsToWallets } from "@composable/bootstrap_pallets/lib";
import { u8aToHex } from "@polkadot/util";

export async function bootstrapBondOffers(api: ApiPromise, wallet: KeyringPair, walletSudo: KeyringPair): Promise<void> {
  await mintAssetsToWallets(api, [wallet], walletSudo, ["1"], toChainUnits(50));

  const beneficiary = wallet.publicKey;
  for (const offer of config.bondOffers) {
    const rewardAssetId = offer.reward.asset;
    const rewardAssetAmount = offer.reward.amount;

    logger.log("info", `Minting  ${rewardAssetAmount} ${rewardAssetId} for ${u8aToHex(wallet.publicKey)}`);
    await mintAssetsToWallets(api, [wallet], walletSudo, [rewardAssetId], new BigNumber(rewardAssetAmount));
    
    logger.log("info", 'Creating Bond Offer');
    const bondOffer: BondOffer & { beneficiary: Uint8Array } = { ...toBondOffer(api, offer), beneficiary };
    const created = await createOffer(api, wallet, bondOffer);
    
    logger.log("info", "Bond Offer Created: " + created.data.toString());
  }
}
