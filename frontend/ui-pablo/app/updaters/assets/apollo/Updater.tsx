import {
  useParachainApi,
} from "substrate-react";
import useStore from "@/store/useStore";
import { APOLLO_UPDATE_BLOCKS, DEFAULT_NETWORK_ID } from "../../constants";
import { useEffect, useRef } from "react";
import BigNumber from "bignumber.js";
import { fetchApolloPriceByAssetId } from "../utils";
import { AssetId } from "@/defi/polkadot/types";
import { getAssetOnChainId } from "@/defi/polkadot/Assets";
import _ from "lodash";

const Updater = () => {
  const { assets, updateApolloPrice } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const lastUpdatedBlocked = useRef<BigNumber>(new BigNumber(-1));

    useEffect(() => {
        if (parachainApi) {
            let subscription: any = undefined;

            parachainApi.rpc.chain.subscribeNewHeads((header) => {
                let currentBlockNumber = new BigNumber(header.number.toString());

                if (currentBlockNumber.minus(lastUpdatedBlocked.current).gte(APOLLO_UPDATE_BLOCKS)) {
                    lastUpdatedBlocked.current = new BigNumber(currentBlockNumber)

                    Object.keys(assets).map(async asset => {
                        const onChainId = getAssetOnChainId(DEFAULT_NETWORK_ID, asset as AssetId)
                        if (onChainId) {
                            // const price = await fetchApolloPriceByAssetId(parachainApi, onChainId.toString())
                            updateApolloPrice(onChainId.toString(), _.random(0.85, 0.99).toString())
                        }
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
    }, [parachainApi])

  return null;
};

export default Updater;
