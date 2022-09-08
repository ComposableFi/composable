import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { isValidAssetPair } from "@/defi/utils";
import { useAddLiquiditySlice, setPool, setSelection } from "@/store/addLiquidity/addLiquidity.slice";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";

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
        return (
          (i.pair.base.toString() === ui.assetOne &&
            i.pair.quote.toString() === ui.assetTwo) ||
          (i.pair.base.toString() === ui.assetTwo &&
            i.pair.quote.toString() === ui.assetOne)
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
    if (parachainApi && !findPoolManually && pool && pool.poolId !== -1) {
      setSelection({
        assetOne: pool.pair.base.toString(),
        assetTwo: pool.pair.quote.toString()
      })
    }
  }, [parachainApi, findPoolManually, pool]);

  return null;
};

export default Updater;
