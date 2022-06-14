import { APOLLO_UPDATE_BLOCKS, DEFAULT_NETWORK_ID } from "../../constants";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";
import { useParachainApi } from "substrate-react";
import { useCallback, useEffect, useRef, useState } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import _ from "lodash";
import { fetchApolloPriceByAssetId } from "@/utils/defi";

const Updater = () => {
  const { updateApolloPrice } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const onChainAssetIds = useOnChainAssetIds();

  const lastUpdatedBlocked = useRef<BigNumber>(new BigNumber(-1));
  const [currentBlockNumber, setCurrentBlockNumber] = useState<BigNumber>(new BigNumber(-1))

  const updateAssetPrices = useCallback(async () => {
    if (parachainApi) {
      Array.from(onChainAssetIds).map(onChainAssetId => {
        fetchApolloPriceByAssetId(parachainApi, onChainAssetId).then(price => {
          updateApolloPrice(onChainAssetId.toString(), price);
        })
      })
    }
  }, [onChainAssetIds, updateApolloPrice, parachainApi])

  useEffect(() => {
    if (parachainApi) {
      let subscription: any = undefined;

      parachainApi.rpc.chain
        .subscribeNewHeads((header) => {
          setCurrentBlockNumber(new BigNumber(header.number.toString()))
        })
        .then((_sub) => {
          console.log("Unsub apollo prices");
          subscription = _sub;
        });

      return () => {
        if (subscription) {
          subscription();
        }
      };
    }
  }, [parachainApi]);

  useEffect(() => {
    if (currentBlockNumber.minus(lastUpdatedBlocked.current).gte(APOLLO_UPDATE_BLOCKS)) {
      lastUpdatedBlocked.current = new BigNumber(currentBlockNumber);
      updateAssetPrices();
    }
  }, [currentBlockNumber, updateAssetPrices])

  return null;
};

export default Updater;
