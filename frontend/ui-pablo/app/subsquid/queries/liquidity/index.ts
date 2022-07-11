import { subsquidClient } from "../../";

export const queryLiquidityByPoolId = (poolId: number, limit: number = 500) => subsquidClient.query(`
query MyQuery {
    pabloPools(orderBy: calculatedTimestamp_DESC, where:{poolId_eq:${poolId}}, limit: ${limit}) {
        poolId
        totalLiquidity
        calculatedTimestamp
    }
}  
`).toPromise();