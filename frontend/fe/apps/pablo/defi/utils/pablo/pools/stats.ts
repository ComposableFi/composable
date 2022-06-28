import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { queryPabloPoolById } from "@/updaters/pools/subsquid";
import { DAYS, DEFAULT_NETWORK_ID } from "../../constants";
import BigNumber from "bignumber.js";

export interface PabloPoolQueryResponse {
    totalLiquidity: BigNumber;
    totalVolume: BigNumber;
    totalFees: BigNumber;
    calculatedTimestamp: number;
    transactionCount: number;
    quoteAssetId: number;
    poolId: number;
}

export async function fetchPoolStats(pool: ConstantProductPool | StableSwapPool): Promise<PabloPoolQueryResponse[]> {
    try {
        const response = await queryPabloPoolById(pool.poolId);
        const quoteAsset = getAssetByOnChainId(DEFAULT_NETWORK_ID, pool.pair.quote);

        if (!response.data) throw new Error("Unable to Fetch Data");

        let { pabloPools } = response.data;
        if (!pabloPools) throw new Error("[fetchPoolStats] Unable to retreive data from query");

        const decimals = new BigNumber(10).pow(quoteAsset.decimals);
        pabloPools = pabloPools.map((poolState: any) => {
            return {
                totalLiquidity: new BigNumber(poolState.totalLiquidity).div(decimals),
                totalVolume: new BigNumber(poolState.totalVolume).div(decimals),
                totalFees: new BigNumber(poolState.totalFees).div(decimals),
                transactionCount: Number(poolState.transactionCount),
                calculatedTimestamp: Number(poolState.calculatedTimestamp),
                quoteAssetId: Number(poolState.quoteAssetId),
                poolId: Number(poolState.poolId),
            };
        });

        return pabloPools;
    } catch (err: any) {
        console.error(err.message);
        return [];
    }
};


export function calculatePoolStats(data: PabloPoolQueryResponse[]):
    | {
        _24HrFee: string;
        _24HrVolume: string;
        totalVolume: string;
        _24HrTransactionCount: number;
        poolId: number;
    }
    | undefined {
    if (!data.length) return undefined;

    let yesterday = data[0].calculatedTimestamp - 1 * DAYS;
    const yesterdayState = data.find((i) => i.calculatedTimestamp < yesterday);

    let totalVolume = data[0].totalVolume;
    let _24HourVolume = data[0].totalVolume;
    let _24HourFee = data[0].totalFees;
    let _24HourTxCount = data[0].transactionCount;

    if (yesterdayState) {
        _24HourTxCount = _24HourTxCount - yesterdayState.transactionCount;

        _24HourVolume = data[0].totalVolume.minus(
            yesterdayState.totalVolume
        );

        _24HourFee = data[0].totalFees.minus(
            yesterdayState.totalFees
        );
    }

    return {
        _24HrFee: _24HourFee.toString(),
        _24HrVolume: _24HourVolume.toString(),
        totalVolume: totalVolume.toString(),
        _24HrTransactionCount: _24HourTxCount,
        poolId: data[0].poolId,
    };
}