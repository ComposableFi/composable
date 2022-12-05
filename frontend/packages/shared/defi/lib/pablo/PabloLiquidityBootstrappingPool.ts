import { fromPerbill, humanizedBnToBn } from "shared";
import { PabloPoolFeeConfig } from "./PabloPoolFeeConfig";
import { ApiPromise } from "@polkadot/api";
import { PabloPoolPair } from "./PabloPoolPair";
import { BasePabloPool } from "./BasePabloPool";
import BigNumber from "bignumber.js";
import BN from "bn.js";

class LiquidityBootstrappingPoolSaleConfig {
    readonly start: BigNumber;
    readonly end: BigNumber;
    readonly initialWeight: number;
    readonly finalWeight: number;

    static fromJSON(saleConfig: {
        start: string | number;
        end: string | number;
        initialWeight: string;
        finalWeight: string;
    }): LiquidityBootstrappingPoolSaleConfig {
        try {
            return new LiquidityBootstrappingPoolSaleConfig(
                humanizedBnToBn(saleConfig.start),
                humanizedBnToBn(saleConfig.end),
                fromPerbill(saleConfig.initialWeight).toNumber(),
                fromPerbill(saleConfig.finalWeight).toNumber()
            );
        } catch (err: any) {
            console.error("[LiquidityBootstrappingPoolSaleConfig] ", err.message);
            throw new Error(err.message);
        }
    }

    constructor(
        start: BigNumber,
        end: BigNumber,
        initialWeight: number,
        finalWeight: number
    ) {
        this.start = start;
        this.end = end;
        this.initialWeight = initialWeight;
        this.finalWeight = finalWeight;
    }

    getSaleStartBlock(): BigNumber {
        return this.start;
    }

    getSaleEndBlock(): BigNumber {
        return this.end;
    }

    getInitialWeight(): number {
        return this.initialWeight;
    }

    getFinalWeight(): number {
        return this.finalWeight;
    }
}

export class PabloLiquidityBootstrappingPool extends BasePabloPool {
    protected readonly __owner: string;
    protected readonly __saleConfig: LiquidityBootstrappingPoolSaleConfig;

    static async fromPoolId(poolId: BN, api: ApiPromise): Promise<void> {
        try {
            const pool = await api.query.pablo.pools(poolId);
            const poolJSON = pool.toJSON();

            console.log(poolJSON);
        } catch (err: any) {
            console.error("[fromPoolIdApi] ", err.message);
            return Promise.reject(err.message);
        }
    }

    static fromJSON(
        poolIndex: BigNumber,
        api: ApiPromise,
        liquidityBootstrappingPoolJson: any
    ): PabloLiquidityBootstrappingPool {
        try {
            return new PabloLiquidityBootstrappingPool(
                poolIndex,
                liquidityBootstrappingPoolJson.owner,
                PabloPoolPair.fromJSON(liquidityBootstrappingPoolJson.pair),
                PabloPoolFeeConfig.fromJSON(liquidityBootstrappingPoolJson.feeConfig),
                LiquidityBootstrappingPoolSaleConfig.fromJSON(
                    liquidityBootstrappingPoolJson.sale
                ),
                api
            );
        } catch (err: any) {
            console.error("[LiquidityBootstrappingPool] ", err);
            throw new Error(err.message);
        }
    }

    constructor(
        poolId: BigNumber,
        owner: string,
        pair: PabloPoolPair,
        feeConfig: PabloPoolFeeConfig,
        saleConfig: LiquidityBootstrappingPoolSaleConfig,
        api: ApiPromise
    ) {
        super(api, poolId, pair, feeConfig);
        this.__owner = owner;
        this.__saleConfig = saleConfig;
    }

    getSaleConfig() {
        return this.__saleConfig;
    }

    async getDuration(averageBlockTime: BigNumber): Promise<number> {
        let nowBn = await this.__api.query.timestamp.now();
        let currentBlockBn = await this.__api.query.system.number();

        let now = new BigNumber(nowBn.toString());
        let currentBlock = new BigNumber(currentBlockBn.toString());
        const { end, start } = this.__saleConfig;

        if (currentBlock.lt(start) && currentBlock.lt(end)) {
            let startTs = start.times(averageBlockTime).plus(now);
            let endTs = end.times(averageBlockTime).plus(now);
            return endTs.minus(startTs).toNumber();
        } else if (currentBlock.gt(start) && currentBlock.lt(end)) {
            let startTs = now.minus(start.times(averageBlockTime));
            let endTs = end.times(averageBlockTime).plus(now);
            return endTs.minus(startTs).toNumber();
        } else {
            let startTs = now.minus(start.plus(end).times(averageBlockTime));
            let endTs = now.minus(end.times(averageBlockTime));
            return endTs.minus(startTs).toNumber();
        }
    }

    simulatePriceAt(
        blockNumber: BigNumber,
        baseAssetBalance: BigNumber,
        quoteAssetBalance: BigNumber
    ) {
        let { baseWeight, quoteWeight } = this.getWeightsAt(blockNumber);
        let baseNum = baseAssetBalance.div(baseWeight);
        let quoteNum = quoteAssetBalance.div(quoteWeight);
        return quoteNum.div(baseNum);
    }

    getWeightsAt(blockNumber: BigNumber): {
        baseWeight: BigNumber;
        quoteWeight: BigNumber;
    } {
        let baseWeight = new BigNumber(0);
        let quoteWeight = new BigNumber(0);
        let one = new BigNumber(1);
        const { start, end, finalWeight, initialWeight } = this.__saleConfig;
        let normalized_current_block = new BigNumber(blockNumber).minus(start);
        let pointInSale = normalized_current_block.div(
            new BigNumber(end).minus(end)
        );

        let weightRange = new BigNumber(initialWeight)
            .div(100)
            .minus(new BigNumber(finalWeight).div(100));

        baseWeight = new BigNumber(initialWeight)
            .div(100)
            .minus(pointInSale.times(weightRange));

        quoteWeight = one.minus(baseWeight);

        return {
            baseWeight,
            quoteWeight,
        };
    }

    getOwner(): string {
        return this.__owner;
    }


    async getSaleTiming(
        currentBlock: BigNumber,
        blockInterval: BigNumber
    ): Promise<{
        startTimestamp: number,
        endTimestamp: number
    }> {
        const startBlock = this.__saleConfig.getSaleStartBlock();
        const endBlock = this.__saleConfig.getSaleEndBlock();
        const nowBn = await this.__api.query.timestamp.now();
        const now = new BigNumber(nowBn.toString());

        let startTimestamp, endTimestamp, startOffset, endOffset;
        if (currentBlock.gt(startBlock)) {
            startOffset = currentBlock.minus(startBlock).times(blockInterval);
            startTimestamp = now.minus(startOffset).toNumber();
        } else {
            startOffset = startBlock.minus(currentBlock).times(blockInterval);
            startTimestamp = now.plus(startOffset).toNumber();
        }
        if (currentBlock.gt(endBlock)) {
            endOffset = currentBlock.minus(endBlock).times(blockInterval);
            endTimestamp = now.minus(endOffset).toNumber();
        } else {
            endOffset = endBlock.minus(currentBlock).times(blockInterval);
            endTimestamp = now.plus(endOffset).toNumber();
        }

        return {
            startTimestamp,
            endTimestamp
        }
    }
}
