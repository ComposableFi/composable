import { useStore } from "@/stores/root";
import { WebsocketClient } from "binance";
import BigNumber from "bignumber.js";
import { useEffect } from "react";
import {
  callbackGate,
  fromChainIdUnit,
  isPalletSupported,
  unwrapNumberOrHex,
} from "shared";
import { ComposableTraitsOraclePrice } from "defi-interfaces";
import { usePicassoProvider } from "substrate-react";

export function binanceMapPairToSourceSymbol(pair: string) {
  const out = {
    KSMUSDT: "KSM",
    USDCUSDT: "USDC",
  }[pair];

  return out ?? "";
}

export const useApolloStats = () => {
  const { parachainApi } = usePicassoProvider();
  const binanceAssets = useStore((state) => state.statsApollo.binanceAssets);
  const oracleAssets = useStore((state) => state.statsApollo.oracleAssets);
  const tokens = useStore((state) => state.substrateTokens.tokens);
  const setBinanceAssets = useStore(
    (state) => state.statsApollo.setBinanceAssets
  );
  const setOracleAssets = useStore(
    (state) => state.statsApollo.setOracleAssets
  );

  // move out from here
  const setupBinancePricePull = () => {
    const wsClient = new WebsocketClient({
      beautify: true,
      pingInterval: 60_000_000,
      disableHeartbeat: true,
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
    return setupBinancePricePull();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Pulls prices from oracle for the allowed_list
  useEffect(() => {
    const unsubscribes: Array<Promise<any>> = [];
    Object.keys(oracleAssets).forEach((symbol) => {
      const unsubPromise: Promise<() => void> = callbackGate(
        (api, picaId) => {
          return isPalletSupported(api)("Oracle")
            ? api.query.oracle.prices(
                picaId.toString(),
                (prices: ComposableTraitsOraclePrice) => {
                  setOracleAssets(
                    symbol,
                    null,
                    fromChainIdUnit(
                      unwrapNumberOrHex((prices as any).price.toString())
                    )
                  );
                }
              )
            : () => {};
        },
        parachainApi,
        tokens.pica.chainId.picasso
      );
      unsubscribes.push(unsubPromise);
    });

    return () => {
      Promise.all(unsubscribes).then((unsubs) =>
        unsubs.forEach((unsub) => unsub())
      );
    };
  }, [
    oracleAssets,
    parachainApi,
    setOracleAssets,
    tokens.pica.chainId.picasso,
  ]);

  return {
    binanceAssets,
    oracleAssets,
  };
};
