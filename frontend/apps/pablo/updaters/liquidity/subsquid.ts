import { makeClient } from "@/subsquid";

export const queryLiquidityByPoolId = (poolId: number, limit: number = 500) => makeClient().query(`
query MyQuery {
    pabloPools(orderBy: calculatedTimestamp_DESC, where:{poolId_eq:${poolId}}, limit: ${limit}) {
        poolId
        totalLiquidity
        calculatedTimestamp
    }
}  
`).toPromise();