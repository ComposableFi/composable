import { ApiPromise } from "@polkadot/api";
import { SubmittableExtrinsic } from "@polkadot/api/types";

export function addLiquidityToPoolViaPablo(
  api: ApiPromise,
  poolId: number,
  baseAmount: string,
  quoteAmount: string,
  minMintAmount: number = 0
): SubmittableExtrinsic<"promise"> {
  const baseAmountParam = api.createType("u128", baseAmount);
  const quoteAmountParam = api.createType("u128", quoteAmount);
  const keepAliveParam = api.createType("bool", true);

  return api.tx.pablo.addLiquidity(
    poolId,
    baseAmountParam,
    quoteAmountParam,
    minMintAmount,
    keepAliveParam
  );
}
