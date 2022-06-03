import { getAssetById } from "@/defi/polkadot/Assets";
import { LiquidityBootstrappingPool, ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { percentageToNumber } from "@/utils/number";
import BigNumber from "bignumber.js";
import { AVERAGE_BLOCK_TIME, DEFAULT_NETWORK_ID, DUMMY_LAUNCH_DESCRIPTION } from "./constants";

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
    fee: poolItem.fee.replace("%", ""),
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
    fee: pool.fee.replace("%", ""),
    ownerFee: pool.ownerFee,
  };
};

export const decodeSsp = (pool: any, poolId: number): StableSwapPool => {
  return {
    poolId,
    owner: pool.owner,
    pair: {
      base: stringToBigNumber(pool.pair.base).toNumber(),
      quote: stringToBigNumber(pool.pair.base).toNumber(),
    },
    lpToken: stringToBigNumber(pool.lpToken).toString(),
    amplificationCoefficient: stringToBigNumber(
      pool.amplificationCoefficient
    ).toString(),
    fee: pool.fee.replace("%", ""),
    ownerFee: pool.ownerFee.replace("%", ""),
  };
};