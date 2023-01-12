import {
  fromChainIdUnit,
  head,
  humanBalance,
  SubstrateNetworkId,
  tail,
} from "shared";
import { Theme } from "@mui/material";
import { useOverviewStats } from "@/apollo/hooks/useOverviewStats";
import { useCoingecko } from "coingecko";
import { useStore } from "@/stores/root";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { useMemo } from "react";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";
import BigNumber from "bignumber.js";

export function changeCalculator(
  chartSeries: [number, number][],
  theme: Theme
) {
  const first = head(chartSeries);
  const last = tail(chartSeries);

  if (first && last) {
    const firstValue = first[1];
    const lastValue = last[1];
    const percentageDifference = ((firstValue - lastValue) / firstValue) * 100;
    return {
      value:
        humanBalance(
          Math.abs(
            Number.isNaN(percentageDifference) ? 0 : percentageDifference
          ).toFixed(2)
        ) + "%",
      color:
        firstValue > lastValue
          ? theme.palette.error.main
          : theme.palette.success.main,
    };
  }

  return {
    value: "0",
    color: theme.palette.text.primary,
  };
}

export const useTotalValueLocked = () => {
  const { data } = useOverviewStats();
  const prices = useCoingecko((store) => store.prices);
  const tokens = useStore((store) => store.substrateTokens.tokens);
  const price = usePicaPriceDiscovery();
  return useMemo(() => {
    const tvl = data?.overviewStats.totalValueLocked;
    if (!tvl) {
      return "0";
    }

    return tvl
      .filter((item) => item.assetId === "1")
      .reduce((acc, cur) => {
        const getTokenById = (id: string, network: SubstrateNetworkId) =>
          Object.values(tokens).find(
            (token) => token.chainId[network]?.toString() === id
          );

        return pipe(
          getTokenById(cur.assetId, "picasso"),
          O.fromNullable,
          O.map((asset) =>
            pipe(
              asset,
              (a) => fromChainIdUnit(cur.amount, a.decimals.picasso),
              (amount) =>
                amount.multipliedBy(
                  asset.id === "pica" ? price : prices[asset.id].usd
                )
            )
          ),
          O.fold(
            () => acc,
            (price) => {
              return acc.plus(price);
            }
          )
        );
      }, new BigNumber(0))
      .toFormat(0);
  }, [data?.overviewStats.totalValueLocked, price, prices, tokens]);
};
