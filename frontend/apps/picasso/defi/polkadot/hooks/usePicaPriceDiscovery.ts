import { useMemo } from "react";
import BigNumber from "bignumber.js";
import { calculateOutGivenIn } from "shared";
import { useStore } from "@/stores/root";
import { useCoingecko } from "coingecko";
import { pipe } from "fp-ts/function";
import * as A from "fp-ts/ReadonlyArray";
import * as O from "fp-ts/Option";

export const usePicaPriceDiscovery = (): BigNumber => {
  const pools = useStore((store) => store.pools.config);
  const poolAmount = useStore((store) => store.pools.poolAmount);
  const tokens = useStore((store) => store.substrateTokens.tokens);
  const usdtPrice = useCoingecko((store) => store.prices.usdt);
  const spotPrice = useMemo(() => {
    const picaUsdtPool = pipe(
      pools,
      A.fromArray,
      A.findFirst((item) =>
        item.config.assets.some((asset) => asset.id === "pica")
      )
    );

    return pipe(
      picaUsdtPool,
      O.bindTo("pool"),
      O.bind("amount", ({ pool }) =>
        pipe(poolAmount[pool.poolId.toString()], O.fromNullable)
      ),
      O.bind("usdtChainId", () =>
        pipe(tokens.usdt.chainId.picasso?.toString(), O.fromNullable)
      ),
      O.bind("baseAmount", ({ usdtChainId, amount }) =>
        pipe(
          amount[usdtChainId],
          O.fromNullable,
          O.map((i) => new BigNumber(i))
        )
      ),
      O.bind("quoteAmount", ({ amount }) =>
        pipe(
          amount["1"],
          O.fromNullable,
          O.map((i) => new BigNumber(i))
        )
      ),

      O.map(({ baseAmount, quoteAmount }) => {
        const calculated = calculateOutGivenIn(
          baseAmount,
          quoteAmount,
          new BigNumber(1),
          new BigNumber(5),
          new BigNumber(5)
        );

        return calculated;
      }),
      O.fold(
        () => new BigNumber(0),
        (some) => some
      )
    );
  }, [poolAmount, pools, tokens.usdt.chainId.picasso]);

  return spotPrice.isZero()
    ? new BigNumber(0)
    : usdtPrice.usd.multipliedBy(spotPrice);
};
