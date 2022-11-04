import { APOLLO_UPDATE_BLOCKS, DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useParachainApi } from "substrate-react";
import { useCallback, useEffect, useMemo, useRef } from "react";
import useBlockNumber from "@/defi/hooks/useBlockNumber";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { Apollo } from "shared";

const Updater = () => {
  const { substrateTokens } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  // const apollo = useMemo(() => {
  //   if (!parachainApi || assets.length === 0) return null;

  //   const apollo = new Apollo(parachainApi)
  //   apollo.getPrice(assets).then(setPrices);
  //   return apollo;
  // }, [parachainApi, assets, setPrices]);

  // const lastUpdatedBlocked = useRef<BigNumber>(new BigNumber(-1));

  // const updateAssetPrices = useCallback(async () => {
  //   if (!apollo || assets.length == 0) return;

  //   apollo.getPrice(assets).then(setPrices);
  // }, [apollo, assets, setPrices]);

  // const currentBlockNumber = useBlockNumber(DEFAULT_NETWORK_ID);

  // useEffect(() => {
  //   if (currentBlockNumber.minus(lastUpdatedBlocked.current).gte(APOLLO_UPDATE_BLOCKS)) {
  //     lastUpdatedBlocked.current = new BigNumber(currentBlockNumber);
  //     updateAssetPrices();
  //   }
  // }, [currentBlockNumber, updateAssetPrices])

  return null;
};

export default Updater;