import {
  LiquidityBootstrappingPool,
  ConstantProductPool,
  StableSwapPool,
  PoolFeeConfig,
} from "@/defi/types";
import { humanizedBnToBn, humanizedPermillToBigNumber } from "shared";
import BigNumber from "bignumber.js";
import {
  AVERAGE_BLOCK_TIME,
  DAYS,
  DEFAULT_NETWORK_ID,
  DUMMY_LAUNCH_DESCRIPTION,
} from "@/defi/utils/constants";

export function decodeFeeConfig(poolItem: any): PoolFeeConfig {
  return {
    feeRate: humanizedPermillToBigNumber(poolItem.feeConfig.feeRate).toString(),
    ownerFeeRate: humanizedPermillToBigNumber(
      poolItem.feeConfig.ownerFeeRate
    ).toString(),
    protocolFeeRate: humanizedPermillToBigNumber(
      poolItem.feeConfig.protocolFeeRate
    ).toString(),
  };
}

export function decodePoolPair(poolItem: any): { base: number; quote: number } {
  const base =
    poolItem.pair && poolItem.pair.base
      ? Number(humanizedBnToBn(poolItem.pair.base))
      : -1;
  const quote =
    poolItem.pair && poolItem.pair.quote
      ? Number(humanizedBnToBn(poolItem.pair.quote))
      : -1;
  return { base, quote };
}

export const decodeLbp = (
  poolItem: any,
  poolIndex: number,
  currentBlock: BigNumber
): LiquidityBootstrappingPool => {
  const startBlock = humanizedBnToBn(poolItem.sale.start as string);
  const endBlock = humanizedBnToBn(poolItem.sale.end as string);

  const start = currentBlock.gt(startBlock)
    ? Date.now() -
      currentBlock.minus(startBlock).toNumber() * AVERAGE_BLOCK_TIME
    : Date.now() +
      startBlock.minus(currentBlock).toNumber() * AVERAGE_BLOCK_TIME;
  const end = currentBlock.gt(endBlock)
    ? Date.now() - currentBlock.minus(endBlock).toNumber() * AVERAGE_BLOCK_TIME
    : Date.now() + endBlock.minus(currentBlock).toNumber() * AVERAGE_BLOCK_TIME;
  const duration = Math.round((end - start) / DAYS);

  return {
    id: poolIndex.toString(),
    poolId: poolIndex,
    networkId: DEFAULT_NETWORK_ID,
    owner: poolItem.owner,
    pair: decodePoolPair(poolItem),
    sale: {
      startBlock: startBlock.toString(),
      endBlock: endBlock.toString(),
      start,
      end,
      duration,
      initialWeight: humanizedPermillToBigNumber(
        poolItem.sale.initialWeight
      ).toNumber(),
      finalWeight: humanizedPermillToBigNumber(
        poolItem.sale.finalWeight
      ).toNumber(),
    },
    feeConfig: decodeFeeConfig(poolItem),
    history: [],
    auctionDescription: DUMMY_LAUNCH_DESCRIPTION(),
  } as LiquidityBootstrappingPool;
};

export const decodeCpp = (
  poolItem: any,
  poolId: number
): ConstantProductPool => {
  return {
    poolId,
    owner: poolItem.owner,
    pair: decodePoolPair(poolItem),
    lpToken: humanizedBnToBn(poolItem.lpToken).toString(),
    feeConfig: decodeFeeConfig(poolItem),
    baseWeight: humanizedPermillToBigNumber(poolItem.baseWeight).toString(),
  };
};

export const decodeSsp = (poolItem: any, poolId: number): StableSwapPool => {
  return {
    poolId,
    owner: poolItem.owner,
    pair: decodePoolPair(poolItem),
    lpToken: humanizedBnToBn(poolItem.lpToken).toString(),
    amplificationCoefficient: humanizedBnToBn(
      poolItem.amplificationCoefficient
    ).toString(),
    feeConfig: decodeFeeConfig(poolItem),
  };
};
