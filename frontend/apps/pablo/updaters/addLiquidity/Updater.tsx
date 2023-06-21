import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { isValidAssetPair } from "@/defi/utils";
import { useAddLiquiditySlice, setPool, setSelection } from "@/store/addLiquidity/addLiquidity.slice";
import { useAllLpTokenRewardingPools } from "@/defi/hooks/useAllLpTokenRewardingPools";
import useStore from "@/store/useStore";

/**
 * Updates zustand store with all 
 * pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const { substrateTokens } = useStore();
  const pools = useAllLpTokenRewardingPools();
  const { ui, pool, findPoolManually } = useAddLiquiditySlice();
  const { parachainApi } = useParachainApi("picasso");

  useEffect(() => {
    if (
      substrateTokens.hasFetchedTokens && 
      parachainApi &&
      isValidAssetPair(ui.assetOne, ui.assetTwo) &&
      findPoolManually
      ) {

      const pool = pools.find((i) => {
        const pair = i.getAssets().intoAssets(Object.values(substrateTokens.tokens));
        const base = pair[0].getPicassoAssetId() as string;
        const quote = pair[1].getPicassoAssetId() as string;
        return (
          (base === ui.assetOne &&
            quote === ui.assetTwo) ||
          (quote === ui.assetTwo &&
            base === ui.assetOne)
        );
      });

      if (pool) {
        setPool(pool)
      } else {
        setPool(undefined)
      }
    }
  }, [pools, parachainApi, ui.assetOne, ui.assetTwo, findPoolManually, substrateTokens.hasFetchedTokens, substrateTokens.tokens]);

  useEffect(() => {
    if (parachainApi && !findPoolManually && pool) {
      const pair = pool.getAssets().intoAssets(Object.values(substrateTokens.tokens));
      const base = pair[0].getPicassoAssetId() as string;
      const quote = pair[1].getPicassoAssetId() as string;

      setSelection({
        assetOne: base,
        assetTwo: quote
      })
    }
  }, [parachainApi, findPoolManually, pool, substrateTokens.tokens]);

  return null;
};

export default Updater;
