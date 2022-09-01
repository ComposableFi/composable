/**
 * Required for type augmentation
 * to work
 */
import "@composable/types/augment-api";
import "@composable/types/augment-types";

import dotenv from "dotenv";
dotenv.config();

import { cryptoWaitReady } from "@polkadot/util-crypto";
import * as definitions from "@composable/types/definitions";
import { getSudoWallet, getSubstrateWallets, createRPC, createTypes, buildApi } from "@composable/bootstrap_pallets/helpers";
import config from "@composable/bootstrap_pallets/constants/config.json";
import { bootstrapBondOffers } from "./bootstrap/bondedFinance";
import { bootstrapPools } from "./bootstrap/pablo";
import { bootstrapAssets } from "./bootstrap/assets";
import { bootstrapStakingRewardPools } from "./bootstrap/stakingRewards";
import { logger } from "@composable/bootstrap_pallets/utils";
// import { updateStakingRewardPoolRewardConfig } from "./lib/pallets/stakingRewards";

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
  
  if (config.bootstrapRewardPools) {
    await bootstrapStakingRewardPools(api, walletSudo);
  }

  if (config.mintAssetsToWallets) {
    await bootstrapAssets(api, walletSudo, config.mintAssets as [string, string, string][]);
  }

  // await updateStakingRewardPoolRewardConfig(
  //   api,
  //   walletSudo,
  //   "100000000001",
  //   {
  //     "5": {
  //       rewardRate: {
  //         period: "PerSecond",
  // 1 million in 5 months
  //         amount: api.createType("u128", "77160494000")
  //       }
  //     }
  //   }
  // )

  await api.disconnect();
  process.exit(0);
};

cryptoWaitReady().then(() => {
  main().catch(err => {
    logger.error(err.message);
    process.exit(0);
  });
});
