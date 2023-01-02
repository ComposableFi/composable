import { DEFAULT_NETWORK_ID, toChainUnits } from "@/defi/utils";
import { option } from "fp-ts";
import { AssetWithBalance } from "@/components/Organisms/pool/AddLiquidity/useSimulateAddLiquidity";

export function getAssetTree(
  leftAssetWithBalance: AssetWithBalance,
  rightAssetWithBalance: AssetWithBalance
) {
  const leftId = leftAssetWithBalance.asset.getPicassoAssetId()?.toString() ?? "";
  const rightId = rightAssetWithBalance.asset.getPicassoAssetId()?.toString() ?? "";

  const left = {
    [leftId]: toChainUnits(
      leftAssetWithBalance.balance,
      leftAssetWithBalance.asset.getDecimals(DEFAULT_NETWORK_ID)
    ).toString()
  };
  const right = {
    [rightId]: toChainUnits(
      rightAssetWithBalance.balance,
      rightAssetWithBalance.asset.getDecimals(DEFAULT_NETWORK_ID)
    ).toString()
  };

  const out = {
    ...left,
    ...right
  };

  return Object.keys(out).length === 2 ? option.some(out) : option.none;
}
