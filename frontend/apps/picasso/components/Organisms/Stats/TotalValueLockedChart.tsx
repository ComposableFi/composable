import { FC, useMemo, useState } from "react";
import {
  formatNumber,
  fromChainIdUnit,
  getRange,
  PRESET_RANGE,
  PresetRange,
  SubstrateNetworkId,
  tail,
} from "shared";
import { useQuery } from "@apollo/client";
import { Box, Typography, useTheme } from "@mui/material";
import { Chart } from "@/components";
import {
  GET_TOTAL_VALUE_LOCKED,
  TotalValueLocked,
} from "@/apollo/queries/totalValueLocked";
import { ChartLoadingSkeleton } from "@/components/Organisms/Stats/ChartLoadingSkeleton";
import { changeCalculator } from "@/components/Organisms/Stats/utils";
import * as O from "fp-ts/Option";
import { pipe } from "fp-ts/function";
import { useStore } from "@/stores/root";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { useCoingecko } from "coingecko";
import BigNumber from "bignumber.js";

export const TotalValueLockedChart: FC = () => {
  const theme = useTheme();
  const [range, setRange] = useState<PresetRange>("24h");
  const tokens = useStore((store) => store.substrateTokens.tokens);
  const { data, loading, error } = useQuery<TotalValueLocked>(
    GET_TOTAL_VALUE_LOCKED,
    {
      variables: {
        range: getRange(range),
        source: "Pablo",
      },
      pollInterval: 60_000, // Every minute
    }
  );
  const picaPrice = usePicaPriceDiscovery();
  const prices = useCoingecko((store) => store.prices);
  const chartSeries: [number, number][] = useMemo(() => {
    const getTokenById = (id: string, network: SubstrateNetworkId) =>
      Object.values(tokens).find(
        (token) => token.chainId[network]?.toString() === id
      );
    return pipe(
      data,
      O.fromNullable,
      O.map((data) =>
        data.totalValueLocked.map((item) => {
          const date = Date.parse(item.date);
          const tvl = item.lockedValues
            .filter((lv) => lv.assetId === "1")
            .reduce((acc, cur) => {
              return pipe(
                getTokenById(cur.assetId, "picasso"),
                O.fromNullable,
                O.map((asset) =>
                  pipe(
                    asset,
                    (a) => fromChainIdUnit(cur.amount, a.decimals.picasso),
                    (amount) =>
                      amount.multipliedBy(
                        asset.id === "pica" ? picaPrice : prices[asset.id].usd
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
            }, new BigNumber(0));

          return [date, tvl.dp(0).toNumber()] as [number, number];
        })
      ),
      O.fold(
        () => [],
        (v) => v
      )
    );
  }, [data, picaPrice, prices, tokens]);
  const change = useMemo(() => {
    return changeCalculator(chartSeries, theme);
  }, [chartSeries, theme]);
  const changeTextPrimary = useMemo(() => {
    const first = tail(chartSeries);
    return formatNumber(first?.[1] ?? 0);
  }, [chartSeries]);

  if (loading) {
    return <ChartLoadingSkeleton />;
  }

  if (error) {
    return <>{"error:" + error}</>;
  }

  return (
    <Box sx={{ height: 337 }}>
      <Chart
        height="100%"
        title="Total value locked"
        changeTextColor={theme.palette.text.primary}
        ChangeTextTypographyProps={{
          variant: "h5",
        }}
        changeText={
          <>
            <Typography variant="h5">{changeTextPrimary}</Typography>
            <Typography color={change.color} variant="body1">
              {change.value}
            </Typography>
          </>
        }
        AreaChartProps={{
          data: chartSeries,
          height: 118,
          shorthandLabel: "Locked value",
          labelFormat: (n: number) => n.toFixed(),
          color: theme.palette.primary.main,
        }}
        onIntervalChange={setRange}
        intervals={PRESET_RANGE as unknown as string[]}
        currentInterval={range}
      />
    </Box>
  );
};
