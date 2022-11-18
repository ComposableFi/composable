import { TokenId, TOKENS } from "tokens";
import {
  oracleCurrencies,
  OracleCurrency,
  setOraclePrice,
} from "@/store/oracle/slice";
import useStore from "@/store/useStore";
import axios from "axios";
import BigNumber from "bignumber.js";

export const subscription = useStore.subscribe(
  (state) => state.substrateTokens,
  (state) => {
    const { hasFetchedTokens, tokens } = state;

    if (hasFetchedTokens) {
      let vs_currencies = oracleCurrencies;
      let app_supported: TokenId[] = [];

      for (const [id, asset] of Object.entries(tokens)) {
        if (asset.isSupportedOn("picasso")) {
          app_supported.push(id as TokenId);
        }
      }

      if (app_supported.length > 0) {
        const url = `https://api.coingecko.com/api/v3/simple/price?ids=${app_supported
          .filter((k) => !!TOKENS[k].coinGeckoId)
          .map((k) => {
            return TOKENS[k].coinGeckoId;
          })
          .join(",")}&vs_currencies=${vs_currencies.join(",")}`;

        const allTokenMetadata = Object.values(TOKENS);
        axios.get(url).then((response: any) => {
          const tokenIdPrice = Object.entries(response.data) as [
            string,
            Record<OracleCurrency, number>
          ][];

          for (const tokenAndPrice of tokenIdPrice) {
            const token = allTokenMetadata.find((x) => (x.coinGeckoId === tokenAndPrice[0]));

            if (token) {
              const baseCurrencies = Object.keys(tokenAndPrice[1]);

              for (const baseCurrency of baseCurrencies) {
                console.log('[CoinGecko Subscription] adding price of ', token.symbol, ' base currency ', baseCurrency);
                setOraclePrice(
                  token.symbol,
                  "coingecko",
                  baseCurrency as OracleCurrency,
                  new BigNumber(
                    tokenAndPrice[1][baseCurrency as OracleCurrency]
                  )
                );
              }
            }
          }
        });
      }
    }
  }
);
