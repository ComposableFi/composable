import { getAssetById } from "@/defi/polkadot/Assets";
import {
  LiquidityBootstrappingPool,
  ConstantProductPool,
  StableSwapPool,
} from "@/store/pools/pools.types";
import { percentageToNumber } from "@/utils/number";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import {
  AVERAGE_BLOCK_TIME,
  DEFAULT_NETWORK_ID,
  DUMMY_LAUNCH_DESCRIPTION,
} from "../constants";

export const stringToBigNumber = (value: string): BigNumber =>
  new BigNumber(value.replaceAll(",", ""));

export const decodeLbp = (
  poolItem: any,
  poolIndex: number,
  currentBlock: BigNumber
): LiquidityBootstrappingPool => {
  const startBlock = stringToBigNumber(poolItem.sale.start as string);
  const endBlock = stringToBigNumber(poolItem.sale.end as string);

  const start = currentBlock.gt(startBlock)
    ? Date.now() - startBlock.toNumber() * AVERAGE_BLOCK_TIME
    : Date.now() + startBlock.toNumber() * AVERAGE_BLOCK_TIME;
  const end = currentBlock.gt(endBlock)
    ? Date.now() - endBlock.toNumber() * AVERAGE_BLOCK_TIME
    : Date.now() + endBlock.toNumber() * AVERAGE_BLOCK_TIME;
  const duration = Math.round((end - start) / (1000 * 60 * 24 * 24));

  const baseAssetId = Number(
    (poolItem.pair.base as string).replaceAll(",", "")
  );
  const quoteAssetId = Number(
    (poolItem.pair.quote as string).replaceAll(",", "")
  );

  const baseAsset = getAssetById("picasso", baseAssetId);
  const quoteAsset = getAssetById("picasso", quoteAssetId);
  let poolId = `${baseAsset?.symbol.toLowerCase()}-${quoteAsset?.symbol.toLowerCase()}-${poolIndex}`;
  const icon = baseAsset ? baseAsset.icon : quoteAsset ? quoteAsset.icon : "-";

  return {
    id: poolId,
    poolId: poolIndex,
    networkId: DEFAULT_NETWORK_ID,
    icon,
    owner: poolItem.owner,
    pair: {
      base: baseAssetId,
      quote: quoteAssetId,
    },
    sale: {
      start,
      end,
      duration,
      initialWeight: percentageToNumber(poolItem.sale.initialWeight),
      finalWeight: percentageToNumber(poolItem.sale.finalWeight),
    },
    spotPrice: "0",
    feeConfig: {
      feeRate: poolItem.feeConfig.feeRate.replace("%", ""),
      ownerFeeRate: poolItem.feeConfig.ownerFeeRate.replace("%", ""),
      protocolFeeRate: poolItem.feeConfig.protocolFeeRate.replace("%", ""),
    },
    history: [],
    auctionDescription: DUMMY_LAUNCH_DESCRIPTION(),
  } as LiquidityBootstrappingPool;
};

export const decodeCpp = (pool: any, poolId: number): ConstantProductPool => {
  return {
    poolId,
    owner: pool.owner,
    pair: {
      base: stringToBigNumber(pool.pair.base).toNumber(),
      quote: stringToBigNumber(pool.pair.quote).toNumber(),
    },
    lpToken: stringToBigNumber(pool.lpToken).toString(),
    feeConfig: {
      feeRate: pool.feeConfig.feeRate.replace("%", ""),
      ownerFeeRate: pool.feeConfig.ownerFeeRate.replace("%", ""),
      protocolFeeRate: pool.feeConfig.protocolFeeRate.replace("%", ""),
    },
    baseWeight: pool.baseWeight.replace("%", ""),
  };
};

export const decodeSsp = (pool: any, poolId: number): StableSwapPool => {
  return {
    poolId,
    owner: pool.owner,
    pair: {
      base: stringToBigNumber(pool.pair.base).toNumber(),
      quote: stringToBigNumber(pool.pair.quote).toNumber(),
    },
    lpToken: stringToBigNumber(pool.lpToken).toString(),
    amplificationCoefficient: stringToBigNumber(
      pool.amplificationCoefficient
    ).toString(),
    feeConfig: {
      feeRate: pool.feeConfig.feeRate.replace("%", ""),
      ownerFeeRate: pool.feeConfig.ownerFeeRate.replace("%", ""),
      protocolFeeRate: pool.feeConfig.protocolFeeRate.replace("%", ""),
    },
  };
};

export async function fetchPool(
  parachainApi: ApiPromise,
  poolId: number
): Promise<
  StableSwapPool | ConstantProductPool | LiquidityBootstrappingPool | null
> {
  try {
    const pool = await parachainApi.query.pablo.pools(poolId);
    const decodedPool: any = pool.toHuman();

    if (!decodedPool) throw new Error("Pool with ID not found");

    if ("LiquidityBootstrapping" in decodedPool) {
      const currentBlock = await parachainApi.query.system.number();
      const currentBlockBN = new BigNumber(currentBlock.toString());

      return decodeLbp(
        decodedPool.LiquidityBootstrapping,
        poolId,
        currentBlockBN
      );
    }

    if ("ConstantProduct" in decodedPool) {
      return decodeCpp(decodedPool.ConstantProduct, poolId);
    }
    if ("StableSwap" in decodedPool) {
      return decodeSsp(decodedPool.ConstantProduct, poolId);
    }

    return null;
  } catch (err) {
    return null;
  }
}
