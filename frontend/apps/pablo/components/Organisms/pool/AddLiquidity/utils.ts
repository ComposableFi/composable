import { toChainUnits } from "@/defi/utils";
import { option } from "fp-ts";
import { AssetWithBalance } from "@/components/Organisms/pool/AddLiquidity/useSimulateAddLiquidity";

export function getAssetTree(
  leftAssetWithBalance: AssetWithBalance,
  rightAssetWithBalance: AssetWithBalance
) {
  const left = {
    [leftAssetWithBalance.assetIdOnChain]: toChainUnits(
      leftAssetWithBalance.balance
    ).toString(),
  };
  const right = {
    [rightAssetWithBalance.assetIdOnChain]: toChainUnits(
      rightAssetWithBalance.balance
    ).toString(),
  };

  const out = {
    ...left,
    ...right,
  };

  return Object.keys(out).length === 2 ? option.some(out) : option.none;
}
