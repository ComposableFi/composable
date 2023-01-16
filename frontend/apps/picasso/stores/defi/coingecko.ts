import { TokenId, TOKENS } from "tokens";
import { useStore } from "@/stores/root";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { pipe } from "fp-ts/function";
import { option, readonlyArray, taskEither } from "fp-ts";
import { coingeckoRequest, useCoingecko } from "coingecko";
import { tryCatch } from "fp-ts/TaskEither";
import BigNumber from "bignumber.js";

function tryFetchCoingeckoPrices(tokens: Record<TokenId, TokenMetadata>) {
  return tryCatch(
    () =>
      coingeckoRequest(
        pipe(
          Object.entries<TokenMetadata>(tokens),
          readonlyArray.fromArray,
          readonlyArray.filter(([_, meta]) => meta.chainId.picasso !== null),
          readonlyArray.filterMap(([tokenId, _]) =>
            pipe(TOKENS[tokenId as TokenId].coinGeckoId, option.fromNullable)
          ),
          readonlyArray.toArray
        ),
        ["usd"]
      ),
    () => option.none
  );
}

export const subscribeCoingeckoPrices: () => () => void = () =>
  useStore.subscribe(
    (state) => ({
      isLoaded: state.substrateTokens.isLoaded,
      tokens: state.substrateTokens.tokens,
    }),
    ({ isLoaded, tokens }) => {
      if (isLoaded) {
        pipe(
          tryFetchCoingeckoPrices(tokens),
          taskEither.map((response) => {
            pipe(
              response.data as {
                [key in string]: { usd: number; usd_24h_change: number };
              },
              Object.entries,
              readonlyArray.fromArray,
              readonlyArray.map(([coingeckoId, { usd, usd_24h_change }]) =>
                pipe(
                  Object.values(TOKENS).find(
                    (t) => t.coinGeckoId === coingeckoId
                  ),
                  option.fromNullable,
                  option.map((meta) =>
                    useCoingecko
                      .getState()
                      .setPrice(
                        meta.id,
                        new BigNumber(usd),
                        "usd",
                        usd_24h_change
                      )
                  )
                )
              )
            );
          })
        )();
      }
    },
    {
      fireImmediately: true,
      equalityFn: (a, b) => a.isLoaded === b.isLoaded && b.isLoaded,
    }
  );
