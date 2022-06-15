import { PalletsContext } from "@/defi/polkadot/context/PalletsContext";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { useDispatch } from "react-redux";
import { setApolloAssets } from "./apollo";
import { useContext, useEffect, useState } from "react";
import { toTokenUnitsBN } from "../../../utils/BN";
import { ApolloTableData } from "./apollo";

const ASSETS = {
  1: "pica",
  4: "ksm",
  149: "pablo",
  246: "usdc",
};

let binancePrice = async (apolloStats: any, value: string) => {
  return await apolloStats
    .queryBinancePrice(value.toUpperCase())
    .then((binPrice: any) => {
      return parseFloat(binPrice.price).toFixed(2);
    })
    .catch((err: any) => {
      console.log("BIN PRICE ERR", value, err);
      return undefined;
    });
};

let binancePriceChange = async (apolloStats: any, value: string) => {
  return await apolloStats
    .queryBinanace24hrChange(value.toUpperCase())
    .then((binChange: any) => {
      return parseFloat(binChange.priceChangePercent).toFixed(2);
    })
    .catch((err: any) => {
      console.log("BIN CHANGE ERROR", value, err);
      return undefined;
    });
};

let oraclePrice = async (apolloStats: any, key: string) => {
  return await apolloStats
    .queryOracleAssetPrice(parseInt(key))
    .then((res: { price: number; block: number } | null) => {
      if (res != null) {
        return toTokenUnitsBN(res.price, 12).toNumber().toFixed(2);
      }
      return undefined;
    })
    .catch((err: any) => {
      console.log("ORACLE ERROR", err);
      return undefined;
    });
};

const ApolloStatsUpdater = () => {
  const appDispatch = useDispatch();
  const picassoProvider = usePicassoProvider();
  const { apolloStats } = useContext(PalletsContext);

  useEffect(() => {
    const { parachainApi } = picassoProvider;
    if (apolloStats && parachainApi) {
      let tempPricesArray: Array<ApolloTableData> = [];

      Object.entries(ASSETS).forEach(async ([key, value]) => {
        let temp = tempPricesArray.push({
          symbol: value,
          binanceValue: await binancePrice(apolloStats, value),
          apolloValue: await oraclePrice(apolloStats, key),
          changeValue: await binancePriceChange(apolloStats, value),
        });

        // console.log("TEMP", temp, tempPricesArray);
        if (temp >= 4) {
          appDispatch(setApolloAssets(tempPricesArray));
        }
      });

      // console.log("STORE APOLLO ARRAY", tempPricesArray);
    }
  }, [picassoProvider.apiStatus]);

  return null;
};

export default ApolloStatsUpdater;
