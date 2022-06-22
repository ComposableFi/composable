import { getAssetById } from "@/defi/polkadot/Assets";
import { LiquidityBootstrappingPool, ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { percentageToNumber } from "@/utils/number";
import BigNumber from "bignumber.js";
import { AVERAGE_BLOCK_TIME, DAYS, DEFAULT_NETWORK_ID, DUMMY_LAUNCH_DESCRIPTION } from "../../constants";
import { stringToBigNumber } from "../../misc";

export const decodeLbp = (
    poolItem: any,
    poolIndex: number,
    currentBlock: BigNumber
  ): LiquidityBootstrappingPool => {
    const startBlock = stringToBigNumber(poolItem.sale.start as string);
    const endBlock = stringToBigNumber(poolItem.sale.end as string);
  
    const start = currentBlock.gt(startBlock)
      ? Date.now() - ((currentBlock.minus(startBlock).toNumber()) * AVERAGE_BLOCK_TIME)
      : Date.now() + ((startBlock.minus(currentBlock)).toNumber() * AVERAGE_BLOCK_TIME);
    const end = currentBlock.gt(endBlock)
      ? Date.now() - (currentBlock.minus(endBlock).toNumber()) * AVERAGE_BLOCK_TIME
      : Date.now() + (endBlock.minus(currentBlock).toNumber()) * AVERAGE_BLOCK_TIME;
    const duration = Math.round((end - start) / DAYS);
  
    const baseAssetId = Number(
      (poolItem.pair.base as string).replaceAll(",", "")
    );
    const quoteAssetId = Number(
      (poolItem.pair.quote as string).replaceAll(",", "")
    );
   
    return {
      id: poolIndex.toString(),
      poolId: poolIndex,
      networkId: DEFAULT_NETWORK_ID,
      owner: poolItem.owner,
      pair: {
        base: baseAssetId,
        quote: quoteAssetId,
      },
      sale: {
        startBlock: startBlock.toString(),
        endBlock: endBlock.toString(),
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