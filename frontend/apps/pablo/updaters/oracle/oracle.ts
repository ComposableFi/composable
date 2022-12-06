import { TokenId, TOKENS } from "tokens";
import {
  oracleCurrencies,
  OracleCurrency,
  setOraclePrice,
} from "@/store/oracle/slice";
import useStore from "@/store/useStore";
import axios, { AxiosResponse } from "axios";
import BigNumber from "bignumber.js";

function coinGeckoApiUrl(): string {
  switch (process.env.NODE_ENV) {
    case "production":
      return "https://pro-api.coingecko.com/api/v3/simple/price";
    default:
      return "https://api.coingecko.com/api/v3/simple/price";
  }
}

function coinGeckoHeaders():
  | { headers: { x_cg_pro_api_key: string } }
  | undefined {
  if (process.env.COINGECKO_KEY) {
    return { headers: { x_cg_pro_api_key: process.env.COINGECKO_KEY! } };
  }
}

function coingeckoRequest(
  targetTokens: string[],
  fiatCurrencies: string[]
): Promise<AxiosResponse<any, any>> {
  return axios.get(
    `${coinGeckoApiUrl()}?ids=${targetTokens.join(
      ","
    )}&vs_currencies=${fiatCurrencies.join(",")}`,
    coinGeckoHeaders()
  );
}

export const subscription = useStore.subscribe(
  (state) => ({
    hasFetchedTokens: state.substrateTokens.hasFetchedTokens,
    tokens: state.substrateTokens.tokens,
  }),
  ({ tokens }) => {
    let vs_currencies = [...oracleCurrencies];
    let app_supported: TokenId[] = [];

    for (const [id, asset] of Object.entries(tokens)) {
      if (asset.isSupportedOn("picasso")) {
        app_supported.push(id as TokenId);
      }
    }

    if (app_supported.length > 0) {
      const tokensToFetch = app_supported
        .filter((k) => !!TOKENS[k].coinGeckoId)
        .map((k) => {
          return TOKENS[k].coinGeckoId;
        });

      const allTokenMetadata = Object.values(TOKENS);
      coingeckoRequest(tokensToFetch as string[], vs_currencies)
        .then((response: any) => {
          const tokenIdPrice = Object.entries(response.data) as [
            string,
            Record<OracleCurrency, number>
          ][];

          for (const tokenAndPrice of tokenIdPrice) {
            const token = allTokenMetadata.find(
              (x) => x.coinGeckoId === tokenAndPrice[0]
            );

            if (token) {
              const baseCurrencies = Object.keys(tokenAndPrice[1]);

              for (const baseCurrency of baseCurrencies) {
                console.log(
                  "[CoinGecko Subscription] adding price of ",
                  token.symbol,
                  " base currency ",
                  baseCurrency
                );
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
        })
        .catch((err) => {
          console.log("[Coingecko] Oracle Subscription error", err.message);
        });
    }
  },
  { equalityFn: (curr, prev) => {
    return curr.hasFetchedTokens
  } }
);
