import { ApiPromise } from "@polkadot/api";
import { SubmittableExtrinsic } from "@polkadot/api/types";
import { toChainUnits } from "@/defi/utils";

export function addLiquidityToPoolViaPablo(
  api: ApiPromise,
  poolId: string,
  assetTree: any,
  minLP: number = 0
): SubmittableExtrinsic<"promise"> {
  return api.tx.pablo.addLiquidity(
    poolId,
    api.createType("BTreeMap<u128, u128>", assetTree),
    api.createType("u128", toChainUnits(minLP).toString()),
    true
  );
}
