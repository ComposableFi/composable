import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import {
  logger,
  toChainUnits,
  toConstantProductPoolInitConfig,
  toLiquidityBootstrappingPoolInitConfig,
  toPabloPoolPair,
  toStableSwapPoolInitConfig
} from "@bootstrap-pallets/utils";
import config from "@bootstrap-pallets/constants/config.json";
import BigNumber from "bignumber.js";
import {
  mintAssetsToWallets,
  addLiquidity,
  createConstantProductPool,
  createLiquidityBootstrappingPool,
  enableTwap,
  updateDexRoute
} from "@bootstrap-pallets/lib";

export async function bootstrapPools(api: ApiPromise, wallets: KeyringPair[], walletSudo: KeyringPair): Promise<void> {
  // Mint 10 Tokens For Gas
  await mintAssetsToWallets(api, wallets, walletSudo, ["1"], toChainUnits(50));

  let walletIndex = 0;
  for (const pool of config.pools as any[]) {
    let poolId;
    try {
      if (pool.sale) {
        const start = await api.query.system.number();
        const lbpConfig = toLiquidityBootstrappingPoolInitConfig(
          api,
          wallets[walletIndex],
          pool,
          new BigNumber(start.toString()),
          25
        );
        const lbpCreated = await createLiquidityBootstrappingPool(api, wallets[walletIndex], lbpConfig);
        logger.log('info', `LBP Created: ${lbpCreated.data[0].toString()}`);
        poolId = new BigNumber(lbpCreated.data[0].toString());
      } else if (pool.baseWeight) {
        const cppConfig = toConstantProductPoolInitConfig(api, wallets[walletIndex], pool);
        const cppCreated = await createConstantProductPool(api, wallets[walletIndex], cppConfig);
        logger.log('info', `CPP Created: ${cppCreated.data[0].toString()}`);
        poolId = new BigNumber(cppCreated.data[0].toString());
      } else if (pool.amplificationCoefficient) {
        const ssConfig = toStableSwapPoolInitConfig(api, wallets[walletIndex], pool);
        const ssCreated = await createConstantProductPool(api, wallets[walletIndex], ssConfig);
        logger.log('info', `Stable Swap Pool Created: ${ssCreated.data[0].toString()}`);
        poolId = new BigNumber(ssCreated.data[0].toString());
      }

      if (poolId) {
        if (pool.addLiquidity) {
          await mintAssetsToWallets(
            api,
            [wallets[walletIndex]],
            walletSudo,
            [pool.pair.base],
            new BigNumber(pool.liquidityAmount.base)
          );
          await mintAssetsToWallets(
            api,
            [wallets[walletIndex]],
            walletSudo,
            [pool.pair.quote],
            new BigNumber(pool.liquidityAmount.quote)
          );

          await addLiquidity(
            api,
            wallets[walletIndex],
            poolId,
            pool.liquidityAmount.base,
            pool.liquidityAmount.quote
          );
          logger.log('info', `Liquidity Added to Pool: ${poolId.toString()}`);
        }
        if (pool.addDexRoute) {
          let pair = toPabloPoolPair(api, pool.pair.base, pool.pair.quote);
          await updateDexRoute(api, walletSudo, pair, poolId.toNumber());
          logger.log('info', `Dex Route Added: ${pool.pair.base}-${pool.pair.quote}`);
        }
        if (pool.enableTwap) {
          await enableTwap(api, walletSudo, poolId.toNumber());
          logger.log('info', `Twap Enabled: ${poolId.toString()}`);
        }
      }

      walletIndex = (walletIndex + 1) % wallets.length;
    } catch (err) {
      console.error(err);
      return;
    }
  }
}
