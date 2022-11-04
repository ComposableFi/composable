import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useCallback, useEffect } from "react";
import { PicassoAssetsRPCMetadata } from "@/store/tokens/types";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";

const Updater = () => {
  const { substrateTokens } = useStore();
  const { setTokens } = substrateTokens;
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  const fetchPicassoAssets = useCallback(() => {
    if (!parachainApi) return;

    parachainApi.rpc.assets.listAssets().then((assetList) => {
      setTokens({
        picasso: {
          list: assetList.map((asset) => {
            return {
              id: new BigNumber(asset.id.toString()),
              name: asset.name.toUtf8(),
              decimals: 12,
              foreignId: null,
            };
          }) as PicassoAssetsRPCMetadata,
          api: parachainApi,
        },
      });
    });
  }, [setTokens, parachainApi]);

  useEffect(() => {
    fetchPicassoAssets();
  }, [fetchPicassoAssets]);

  return null;
};

export default Updater;
