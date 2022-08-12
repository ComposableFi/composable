import { mintAssetsToWallets } from "@composable/bootstrap_pallets/lib";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { createRewardPool } from "@composable/bootstrap_pallets/lib/pallets/stakingRewards";
import { toStakingRewardPoolConfig } from "@composable/bootstrap_pallets/utils/stakingRewards";
import BigNumber from "bignumber.js";
import config from "@composable/bootstrap_pallets/constants/config.json";

export async function bootstrapStakingRewardPools(api: ApiPromise, walletSudo: KeyringPair): Promise<void> {
    // minting gas
    // await mintAssetsToWallets(api, [walletSudo], walletSudo, ["1"], toChainUnits("10"));

    for (const pool of config.stakingRewardPools) {
        const currentBlock = await api.query.system.number();
        await mintAssetsToWallets(api, [walletSudo], walletSudo, [pool.rewardConfigs.assetId], new BigNumber(pool.rewardConfigs.maxRewards));
        const stakingRewardPoolConfig = toStakingRewardPoolConfig(api, currentBlock.toString(), pool);
        await createRewardPool(api, walletSudo, stakingRewardPoolConfig);
    }
}
