import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { getAsset, getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { isValidAssetPair } from "../utils";
import { AssetId } from "@/defi/polkadot/types";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useAddLiquiditySlice, setPool, setSelection } from "@/store/addLiquidity/addLiquidity.slice";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";

/**
 * Updates zustand store with all pools from pablo pallet
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
      const onChainBaseAssetId = getAsset(ui.assetOne as AssetId)
        .supportedNetwork.picasso as number;
      const onChainQuoteAssetId = getAsset(ui.assetTwo as AssetId)
        .supportedNetwork.picasso as number;

      const pool = pools.find((i) => {
        return (
          (i.pair.base === onChainBaseAssetId &&
            i.pair.quote === onChainQuoteAssetId) ||
          (i.pair.base === onChainQuoteAssetId &&
            i.pair.quote === onChainBaseAssetId)
        );
      });

      if (pool) {
        setPool(pool)
      } else {
        setPool(undefined)
      }
    }
  }, [
    parachainApi,
    ui.assetOne,
    ui.assetTwo,
    findPoolManually,
  ]);

  useEffect(() => {
    if (parachainApi && !findPoolManually && pool && pool.poolId !== -1) {
      const onChainBaseAssetId = getAssetByOnChainId(
        DEFAULT_NETWORK_ID,
        pool.pair.base
      );
      const onChainQuoteAssetId = getAssetByOnChainId(
        DEFAULT_NETWORK_ID,
        pool.pair.quote
      );

      setSelection({
        assetOne: onChainBaseAssetId.assetId,
        assetTwo: onChainQuoteAssetId.assetId
      })
    }
  }, [parachainApi, findPoolManually, pool]);

  return null;
};

export default Updater;
