import { queryPabloPoolById } from "@/updaters/pools/subsquid";
import { DAYS } from "../../constants";
import BigNumber from "bignumber.js";
import { fromChainUnits } from "../../units";
import { StableSwapPool, ConstantProductPool } from "@/defi/types";

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

        if (!response.data) throw new Error("Unable to Fetch Data");

        let { pabloPools } = response.data;
        if (!pabloPools) throw new Error("[fetchPoolStats] Unable to retreive data from query");

        pabloPools = pabloPools.map((poolState: any) => {
            return {
                totalLiquidity: fromChainUnits(poolState.totalLiquidity),
                totalVolume: fromChainUnits(poolState.totalVolume),
                totalFees: fromChainUnits(poolState.totalFees),
                calculatedTimestamp: Number(poolState.calculatedTimestamp),
                transactionCount: Number(poolState.transactionCount),
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