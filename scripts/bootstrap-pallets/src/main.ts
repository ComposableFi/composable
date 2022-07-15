require("dotenv").config();
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { getSudoWallet, getSubstrateWallets } from "@bootstrap-pallets/helpers/wallets";
import * as definitions from "@composable/definitions";
import { createRPC, createTypes, buildApi } from "@bootstrap-pallets/helpers";
import config from "@bootstrap-pallets/constants/config.json";
import { bootstrapBondOffers } from "./bootstrap/bondedFinance";
import { bootstrapPools } from "./bootstrap/pablo";
import { bootstrapAssets } from "./bootstrap/assets";

const main = async () => {
  const rpcUrl = process.env.RPC_URL || "ws://127.0.0.1:9988";
  const chainName = process.env.CHAIN_NAME || "dali-local";
  
  const walletSudo = getSudoWallet(chainName);
  const dotWallets = getSubstrateWallets();

  const rpc = createRPC(definitions);
  const types = createTypes(definitions);
  const api = await buildApi(rpcUrl, types, rpc);

  // [WIP] bootstrapCrowdloanRewards
  // if (config.bootstrapCrowdloanRewards) {
  // }

  if (config.bootstrapBonds) {
    await bootstrapBondOffers(api, dotWallets[0], walletSudo);
  }

  if (config.bootstrapPools) {
    await bootstrapPools(api, dotWallets, walletSudo);
  }

  if (config.mintAssetsToWallets) {
    await bootstrapAssets(api, walletSudo, config.mintAssets as [string, string, string][]);
  }

  await api.disconnect();
  process.exit(0);
};

cryptoWaitReady().then(() => {
  main().catch(err => {
    console.error(err.message);
    process.exit(0);
  });
});
