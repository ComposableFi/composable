import { useEffect } from "react";
import { Assets, getAssetById } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import {
  useExtrinsics,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { ParachainId } from "substrate-react/dist/dotsama/types";

const processedTransactions: string[] = [];

export const retrieveAssetBalance = async (
  api: ApiPromise,
  networkId: ParachainId,
  accountId: string,
  assetId: string
): Promise<string> => {
  try {
    const asset = getAssetById(networkId, Number(assetId));
    if (!asset) throw new Error("asset unavailable");
    const decimals = new BigNumber(10).pow(asset.decimals);
    const balance = await (api.rpc as any).assets.balanceOf(
      api.createType("CurrencyId", assetId),
      api.createType("AccountId32", accountId)
    );
    return new BigNumber(balance).div(decimals).toFixed(4);
  } catch (err: any) {
    return "0";
  }
};

const Updater = () => {
  const { updateAssetBalance } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  const selectedAccount = useSelectedAccount("picasso");
  const extrinsicCalls = useExtrinsics();

  useEffect(() => {
    if (parachainApi && selectedAccount) {
      Object.keys(Assets).forEach((asset) => {
        let assetID: string | number | null =
          Assets[asset as AssetId].supportedNetwork["picasso"];
        if (assetID) {
          assetID = assetID.toString();
          retrieveAssetBalance(
            parachainApi,
            "picasso",
            selectedAccount.address,
            assetID
          ).then((balance) => {
            updateAssetBalance(asset as AssetId, "picasso", balance);
          });
        }
      });
    }
  }, [parachainApi, selectedAccount]);

  useEffect(() => {
    if (
      parachainApi &&
      selectedAccount &&
      Object.values(extrinsicCalls).length > 0
    ) {
      const txs = Object.values(extrinsicCalls);

      let shouldUpdate: string | null = null;
      txs.forEach((tx) => {
        if (
          tx.sender === selectedAccount.address &&
          tx.status === "isFinalized" &&
          (tx.section === "dexRouter" || tx.section === "pablo") &&
          !processedTransactions.includes(tx.hash)
        ) {
          shouldUpdate = tx.hash;
        }
      });

      if (shouldUpdate !== null) {
        const allPromises = Object.keys(Assets).map((asset) => {
          return new Promise((res, rej) => {
            let assetID: string | number | null =
              Assets[asset as AssetId].supportedNetwork["picasso"];
            if (assetID) {
              assetID = assetID.toString();
              retrieveAssetBalance(
                parachainApi,
                "picasso",
                selectedAccount.address,
                assetID
              ).then((balance) => {
                updateAssetBalance(asset as AssetId, "picasso", balance);
                res(asset);
              });
            } else {
              res(asset);
            }
          });
        });

        Promise.all(allPromises).then((updatedBalancesAssetList) => {
          processedTransactions.push(shouldUpdate as string);
        });
      }
    }
  }, [extrinsicCalls, parachainApi, selectedAccount]);

  return null;
};

export default Updater;
