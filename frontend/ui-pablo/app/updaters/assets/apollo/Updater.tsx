import {
  useParachainApi,
} from "substrate-react";
import useStore from "@/store/useStore";
import { APOLLO_UPDATE_BLOCKS, DEFAULT_NETWORK_ID } from "../../constants";
import { useEffect, useRef } from "react";
import BigNumber from "bignumber.js";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";
import _ from "lodash";
// import { fetchApolloPriceByAssetId } from "../utils";
// import _ from "lodash";

const Updater = () => {
  const { updateApolloPrice } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const onChainAssetIds = useOnChainAssetIds();

  const lastUpdatedBlocked = useRef<BigNumber>(new BigNumber(-1));

    useEffect(() => {
        if (parachainApi && onChainAssetIds.size) {
            let subscription: any = undefined;

            parachainApi.rpc.chain.subscribeNewHeads((header) => {
                let currentBlockNumber = new BigNumber(header.number.toString());

                if (currentBlockNumber.minus(lastUpdatedBlocked.current).gte(APOLLO_UPDATE_BLOCKS)) {
                    lastUpdatedBlocked.current = new BigNumber(currentBlockNumber)

                    Array.from(onChainAssetIds.values()).map(async asset => {
                        // const price = await fetchApolloPriceByAssetId(parachainApi, onChainId.toString())
                        updateApolloPrice(asset.toString(), _.random(0.85, 0.99).toString())
                    })
                    console.log('Apollo Update Block: ', lastUpdatedBlocked.current.toString())
                }
            }).then(_sub => {
                subscription = _sub;
            });

            return () => {
                if (subscription) {
                    subscription();
                }
            }
        }
    }, [parachainApi, onChainAssetIds])

  return null;
};

export default Updater;
