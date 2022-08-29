import { APOLLO_UPDATE_BLOCKS, DEFAULT_NETWORK_ID, fetchApolloPriceByAssetIds } from "@/defi/utils";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";
import { useParachainApi } from "substrate-react";
import { useCallback, useEffect, useRef } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { fetchApolloPriceByAssetId } from "@/defi/utils";
import _ from "lodash";
import useBlockNumber from "@/defi/hooks/useBlockNumber";

const Updater = () => {
  const { updateApolloPrice } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const onChainAssetIds = useOnChainAssetIds();

  const lastUpdatedBlocked = useRef<BigNumber>(new BigNumber(-1));

  const updateAssetPrices = useCallback(async () => {
    if (parachainApi) {
      Array.from(onChainAssetIds).map(onChainAssetId => {
        fetchApolloPriceByAssetId(parachainApi, onChainAssetId).then(price => {
          if (onChainAssetId === "201") {
            updateApolloPrice(onChainAssetId, "1.5");
          } else {
            updateApolloPrice(onChainAssetId, "1");
          }
        })
      })

      // fetchApolloPriceByAssetIds(parachainApi, Array.from(onChainAssetIds)).then(prices => {

      // })
    }
  }, [onChainAssetIds, parachainApi, updateApolloPrice])

  const currentBlockNumber = useBlockNumber(DEFAULT_NETWORK_ID);

  useEffect(() => {
    if (currentBlockNumber.minus(lastUpdatedBlocked.current).gte(APOLLO_UPDATE_BLOCKS)) {
      lastUpdatedBlocked.current = new BigNumber(currentBlockNumber);
      updateAssetPrices();
    }
  }, [currentBlockNumber, updateAssetPrices])

  return null;
};

export default Updater;