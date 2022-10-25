import { usePicassoProvider } from "@/defi/polkadot/hooks/index";
import { useStore } from "@/stores/root";
import { WebsocketClient } from "binance";
import BigNumber from "bignumber.js";
import { useEffect } from "react";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { Assets } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { BN } from "@polkadot/util";
import { ComposableTraitsOraclePrice } from "defi-interfaces";

export function binanceMapPairToSourceSymbol(pair: string) {
  const out = {
    KSMUSDT: "KSM",
    USDCUSDT: "USDC"
  }[pair];

  return out ?? "";
}

export const useApolloStats = () => {
  const { parachainApi } = usePicassoProvider();
  const binanceAssets = useStore((state) => state.statsApollo.binanceAssets);
  const oracleAssets = useStore((state) => state.statsApollo.oracleAssets);
  const setBinanceAssets = useStore(
    (state) => state.statsApollo.setBinanceAssets
  );
  const setOracleAssets = useStore(
    (state) => state.statsApollo.setOracleAssets
  );

  const setupBinancePricePull = () => {
    const wsClient = new WebsocketClient({
      beautify: true,
      pingInterval: 60_000_000,
      disableHeartbeat: true
    });
    let wsKey: string;

    // notification when a connection is opened
    wsClient.on("open", (data) => {
      wsKey = data.wsKey;
    });

    // receive formatted events with beautified keys. Any "known" floats stored in strings as parsed as floats.
    wsClient.on("formattedMessage", (data) => {
      if (
        "eventType" in data &&
        data.eventType &&
        data?.eventType === "24hrMiniTicker"
      ) {
        const { symbol, open, close } = data;

        setBinanceAssets(
          binanceMapPairToSourceSymbol(symbol),
          open ? new BigNumber(open) : null,
          close ? new BigNumber(close) : null
        );
      }
    });

    // Recommended: receive error events (e.g. first reconnection failed)
    wsClient.on("error", (data) => {
      console.log("ws saw error ", data?.wsKey);
    });

    wsClient.subscribeSymbolMini24hrTicker("KSMUSDT", "spot");
    wsClient.subscribeSymbolMini24hrTicker("USDCUSDT", "spot");

    return () => {
      wsClient.close(wsKey, false);
    };
  };

  // Pulls price from the binance websocket and updates the store
  useEffect(() => {
    const unsub = setupBinancePricePull();

    return unsub;
  }, []);

  // Pulls prices from oracle for the allowed_list
  useEffect(() => {
    const unsubscribes: Array<Promise<any>> = [];
    Object.keys(oracleAssets).forEach((symbol) => {
      const unsubPromise: Promise<() => void> = callbackGate((api) => {
        const asset = Assets[symbol.toLowerCase() as AssetId];
        return api.query.oracle.prices(
          asset.supportedNetwork.picasso,
          (response: ComposableTraitsOraclePrice) => {
            const { price }: { price: BN } = response.toJSON() as any;
            setOracleAssets(
              symbol,
              null,
              fromChainIdUnit(unwrapNumberOrHex(price.toString()))
            );
          }
        );
      }, parachainApi);
      unsubscribes.push(unsubPromise);
    });

    return () => {
      Promise.all(unsubscribes).then((unsubs) =>
        unsubs.forEach((unsub) => unsub())
      );
    };
  }, []);
  return {
    binanceAssets,
    oracleAssets
  };
};
