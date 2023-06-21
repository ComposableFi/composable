import { mintAssetsToWallets } from "@composable/bootstrap_pallets/lib";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import {
  addRewardsToPot,
  createRewardPool,
  updateStakingRewardPoolRewardConfig
} from "@composable/bootstrap_pallets/lib/pallets/stakingRewards";
import { toStakingRewardPoolConfig } from "@composable/bootstrap_pallets/utils/stakingRewards";
import BigNumber from "bignumber.js";
import config from "@composable/bootstrap_pallets/constants/config.json";
import { logger } from "../..";

export async function bootstrapStakingRewardPools(api: ApiPromise, walletSudo: KeyringPair): Promise<void> {
  // minting gas
  // await mintAssetsToWallets(api, [walletSudo], walletSudo, ["1"], toChainUnits("10"));
  try {
    for (const pool of config.stakingRewardPools) {
      const currentBlock = await api.query.system.number();

      for (const rewardConfig of Object.values(pool.rewardConfigs)) {
        await mintAssetsToWallets(
          api,
          [walletSudo],
          walletSudo,
          [rewardConfig.assetId],
          new BigNumber(rewardConfig.maxRewards)
        );
      }

      const stakingRewardPoolConfig = toStakingRewardPoolConfig(api, currentBlock.toString(), walletSudo, pool);
      const rewardPoolCreated = await createRewardPool(api, walletSudo, stakingRewardPoolConfig);
      logger.info("Staking Reward Pool Created: ", rewardPoolCreated.toString());

      for (const rewardConfig of Object.values(pool.rewardConfigs)) {
        const rewardPotIncreased = await addRewardsToPot(api, walletSudo, {
          poolId: api.createType("u128", pool.assetId),
          rewardAssetId: api.createType("u128", rewardConfig.assetId),
          rewardAssetAmount: api.createType("u128", rewardConfig.maxRewards),
        }, false);

        logger.info("Reward Pot Increased: ", rewardPotIncreased.toString());

        const rewardRateUpdated = await updateStakingRewardPoolRewardConfig(api, walletSudo, pool.assetId, {
          [rewardConfig.assetId]: {
            rewardRate: {
              period: rewardConfig.rewardRate.period as "PerSecond",
              amount: api.createType("u128", rewardConfig.rewardRate.amount)
            }
          }
        });

        logger.info("Reward Config Updated: ", rewardRateUpdated.toString());
      }
    }
  } catch (err: any) {
    logger.error(err.message);
  }
}
