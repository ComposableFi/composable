import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { isValidAssetPair } from "@/defi/utils";
import { useAddLiquiditySlice, setPool, setSelection } from "@/store/addLiquidity/addLiquidity.slice";
import { useAllLpTokenRewardingPools } from "@/defi/hooks/useAllLpTokenRewardingPools";

/**
 * Updates zustand store with all 
 * pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const pools = useAllLpTokenRewardingPools();
  const { ui, pool, findPoolManually } = useAddLiquiditySlice();
  const { parachainApi } = useParachainApi("picasso");

  useEffect(() => {
    if (
      parachainApi &&
      isValidAssetPair(ui.assetOne, ui.assetTwo) &&
      findPoolManually
      ) {

      const pool = pools.find((i) => {
        const pair = i.getPair();
        const base = pair.getBaseAsset().toString();
        const quote = pair.getQuoteAsset().toString();
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
  }, [
    pools,
    parachainApi,
    ui.assetOne,
    ui.assetTwo,
    findPoolManually,
  ]);

  useEffect(() => {
    if (parachainApi && !findPoolManually && pool) {
      const pair = pool.getPair();
      const base = pair.getBaseAsset().toString();
      const quote = pair.getQuoteAsset().toString();

      setSelection({
        assetOne: base,
        assetTwo: quote
      })
    }
  }, [parachainApi, findPoolManually, pool]);

  return null;
};

export default Updater;
