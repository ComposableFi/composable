import { getAssetById } from "@/defi/polkadot/Assets";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { ParachainId } from "substrate-react/dist/dotsama/types";

export const fetchBalanceByAssetId = async (
    api: ApiPromise,
    networkId: ParachainId,
    accountId: string,
    assetId: string
  ): Promise<string> => {
    try {
      let assetDecimals = 12;
      let asset = getAssetById(networkId, Number(assetId));
      if (!asset) {
        // throw new Error("asset unavailable");
        console.error('Asset unavailable, using default decimals: 12');
        assetDecimals = 12;
      }
      const decimals = new BigNumber(10).pow(assetDecimals);
      const balance = await (api.rpc as any).assets.balanceOf(
        api.createType("CurrencyId", assetId),
        api.createType("AccountId32", accountId)
      );
      return new BigNumber(balance).div(decimals).toFixed(4);
    } catch (err: any) {
      return "0";
    }
  };