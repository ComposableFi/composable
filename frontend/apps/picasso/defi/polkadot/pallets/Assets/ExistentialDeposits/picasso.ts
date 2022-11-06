import { fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export async function fetchAccountExistentialDeposit(
  api: ApiPromise,
  account: string,
  nativeAssetId: BigNumber,
  nativeDecimals: number = 12
): Promise<{
  assetId: BigNumber;
  existentialDeposit: BigNumber;
}> {
  const defaultEd = {
    assetId: nativeAssetId,
    existentialDeposit: new BigNumber(0),
  };

  try {
    const result: any = await api.query.assetTxPayment.paymentAssets(
      api.createType("AccountId32", account)
    );
    if (result.isNone) {
      /**
       * If no payment asset is found
       * use PICA
       */
      const ed = await api.query.currencyFactory.assetEd(
        nativeAssetId.toString()
      );
      const existentialString = ed.toString();
      const existentialDeposit = fromChainIdUnit(
        new BigNumber(existentialString),
        nativeDecimals
      );

      return {
        assetId: defaultEd.assetId,
        existentialDeposit,
      };
    }
    /**
     * Found an asset already set
     * as payment asset
     */
    const [assetId, ed] = result.toJSON();
    return {
      assetId: new BigNumber(assetId),
      existentialDeposit: fromChainIdUnit(unwrapNumberOrHex(ed)),
    };
  } catch (err) {
    console.error("fetchAccountExistentialDeposit ", err);
  } finally {
    return defaultEd;
  }
}
