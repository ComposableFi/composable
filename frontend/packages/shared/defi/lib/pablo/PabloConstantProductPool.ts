import { fromPerbill, humanizedBnToBn } from "shared";
import { ApiPromise } from "@polkadot/api";
import { BasePabloPool } from "./BasePabloPool";
import { PabloPoolFeeConfig } from "./PabloPoolFeeConfig";
import { PabloPoolPair } from "./PabloPoolPair";
import BigNumber from "bignumber.js";

export class PabloConstantProductPool extends BasePabloPool {
    protected __owner: string;
    protected __lpToken: BigNumber;
    protected __baseWeight: BigNumber;

    static fromJSON(
        poolIndex: BigNumber,
        api: ApiPromise,
        constantProductPoolJSON: any
    ): PabloConstantProductPool {
        try {
            const lpToken = humanizedBnToBn(constantProductPoolJSON.lpToken);
            const baseWeight = fromPerbill(constantProductPoolJSON.baseWeight);
            return new PabloConstantProductPool(
                api,
                poolIndex,
                PabloPoolPair.fromJSON(constantProductPoolJSON.pair),
                PabloPoolFeeConfig.fromJSON(constantProductPoolJSON.feeConfig),
                constantProductPoolJSON.owner,
                lpToken,
                baseWeight
            );
        } catch (err: any) {
            console.error("[LiquidityBootstrappingPool] ", err);
            throw new Error(err.message);
        }
    }

    constructor(
        api: ApiPromise,
        poolId: BigNumber,
        pair: PabloPoolPair,
        feeConfig: PabloPoolFeeConfig,
        owner: string,
        lpToken: BigNumber,
        baseWeight: BigNumber
    ) {
        super(api, poolId, pair, feeConfig);
        this.__owner = owner;
        this.__lpToken = lpToken;
        this.__baseWeight = baseWeight;
    }
}