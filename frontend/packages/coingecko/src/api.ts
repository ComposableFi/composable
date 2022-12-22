import type { AxiosResponse } from "axios";
import axios from "axios";
import { pipe } from "fp-ts/function";
import { either, option } from "fp-ts";

function coinGeckoApiUrl(): string {
  return pipe(
    process.env.NODE_ENV,
    either.fromPredicate(
      (v) => v === "production",
      () => "https://api.coingecko.com/api/v3/simple/price"
    ),
    either.fold(
      (v) => v,
      () => "https://pro-api.coingecko.com/api/v3/simple/price"
    )
  );
}

function getCoingeckoKey(): option.Option<string> {
  return pipe(process.env.COINGECKO_KEY, option.fromNullable);
}

function coinGeckoHeaders(): option.Option<{
  headers: { x_cg_pro_api_key: string };
}> {
  return pipe(
    getCoingeckoKey(),
    option.map((key) => ({ headers: { x_cg_pro_api_key: key } }))
  );
}

export function coingeckoRequest(
  targetTokens: string[],
  fiatCurrencies: string[]
): Promise<AxiosResponse<any, any>> {
  return axios.get(
    `${coinGeckoApiUrl()}?ids=${targetTokens.join(
      ","
    )}&vs_currencies=${fiatCurrencies.join(",")}&include_24hr_change=true`,
    pipe(coinGeckoHeaders(), option.toUndefined)
  );
}
