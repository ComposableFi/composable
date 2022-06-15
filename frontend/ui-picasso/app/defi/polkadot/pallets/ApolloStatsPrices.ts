import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { AnyAction, Dispatch } from "@reduxjs/toolkit";

const ASSETS = {
  1: "PICA",
  4: "KSM",
  149: "PABLO",
  246: "USDC",
};

const BINANCE_BASE_URL = "https://api.binance.com/api/v3";

export class ApolloStatsPrices {
  api: ApiPromise;
  dispatch: Dispatch<AnyAction>;

  constructor(api: ApiPromise, dispatch: Dispatch<AnyAction>) {
    this.api = api;
    this.dispatch = dispatch;
  }

  public async queryBinancePrice(tokenSymbol: string) {
    const BINANCE_PRICE_URL = `${BINANCE_BASE_URL}/ticker/price?symbol=${tokenSymbol}USDT`;

    const binancePrice = await fetch(BINANCE_PRICE_URL)
      .then((res) => {
        // console.log("TICKER PRICE", res);

        if (res.status !== 200) {
          return undefined;
        }
        return res.json();
      })
      .catch((err) => {
        console.log("TICKER PRICE ERROR", err);
        return undefined;
      });

    return binancePrice;
  }

  public async queryBinanace24hrChange(tokenSymbol: string) {
    const BINANCE_TICKER_URL = `${BINANCE_BASE_URL}/ticker/24hr?symbol=${tokenSymbol}USDT`;

    const binanceChange = await fetch(BINANCE_TICKER_URL)
      .then((res) => {
        // console.log("TICKER CHANGE", res);
        if (res.status !== 200) {
          return undefined;
        }
        return res.json();
      })
      .catch((err) => {
        console.log("TICKER CHANGE ERROR", err);
        return undefined;
      });

    return binanceChange;
  }

  public async queryOracleAssetPrice(tokenId: number) {
    // let assetCount = (await this.api.query.oracle.assetsCount()).toJSON();
    let response = (await this.api.query.oracle.prices(tokenId)).toJSON();

    return response;
  }
}
