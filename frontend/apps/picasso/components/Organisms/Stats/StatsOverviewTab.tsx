import { Grid, useTheme } from "@mui/material";
import { FeaturedBox } from "@/components/Molecules";
import { DailyActiveUsersChart } from "@/components/Organisms/Stats/DailyActiveUsersChart";
import { useOverviewStats } from "@/apollo/hooks/useOverviewStats";
import { useCirculatingSupply } from "@/apollo/hooks/useCirculatingSupply";
import { TotalValueLockedChart } from "@/components/Organisms/Stats/TotalValueLockedChart";
import { FC, useEffect, useMemo } from "react";
import { formatNumber, humanBalance } from "shared";
import { subscribePools } from "@/stores/defi/polkadot/pablo/subscribePools";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import * as O from "fp-ts/Option";
import { pipe } from "fp-ts/function";
import { subscribePoolAmount } from "@/stores/defi/polkadot/pablo/subscribePoolAmount";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { subscribeCoingeckoPrices } from "@/stores/defi/coingecko";
import { useTotalValueLocked } from "@/components/Organisms/Stats/utils";

export const StatsOverviewTab: FC = () => {
  const circulatingSupply = useCirculatingSupply();
  const { data, loading } = useOverviewStats();
  const { parachainApi } = usePicassoProvider();
  const theme = useTheme();
  const price = usePicaPriceDiscovery();
  const totalValueLocked = useTotalValueLocked();

  useEffect(() => {
    const unsubPrices = subscribeCoingeckoPrices();
    const unsubPools = pipe(
      parachainApi,
      O.fromNullable,
      O.map((api) => subscribePools(api))
    );
    const unsubPoolAmount = pipe(
      parachainApi,
      O.fromNullable,
      O.map((api) => subscribePoolAmount(api))
    );

    return () => {
      pipe(
        O.bindTo("uPoolAmount")(unsubPoolAmount),
        O.bind("uPools", () => unsubPools),
        O.map(({ uPools, uPoolAmount }) => {
          uPools();
          uPoolAmount();
        })
      );
      unsubPrices();
    };
  }, [parachainApi]);
  const marketCap = useMemo(
    () => circulatingSupply.multipliedBy(price),
    [circulatingSupply, price]
  );

  return (
    <Grid container spacing={4}>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title:
                "The total value of PICA in USD deposited in Picasso's smart contracts.",
            }}
            textAbove="Total value locked"
            title={`$${totalValueLocked}`}
          />
        )}
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title: "The number of people possessing a Picasso account.",
            }}
            textAbove="Account holders"
            title={data?.overviewStats.accountHoldersCount.toString()}
          />
        )}
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title: "The total number of finalized transactions.",
            }}
            textAbove="Total transactions"
            title={formatNumber(data?.overviewStats.transactionsCount)}
          />
        )}
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite,
          }}
          TooltipProps={{
            title:
              "The total value of all minted PICA in USD. (total supply * current market price)",
          }}
          textAbove="Picasso market cap"
          title={`$${humanBalance(marketCap)}`}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite,
          }}
          TooltipProps={{
            title: "The number of coins publicly available in the market.",
          }}
          textAbove="Picasso total supply"
          title={circulatingSupply.toFormat(0)}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title:
                "The number of people interacting with Picasso in the last 24 hours.",
            }}
            textAbove="Active users"
            title={data?.overviewStats.activeUsersCount.toString()}
          />
        )}
      </Grid>
      <Grid item xs={12}>
        <TotalValueLockedChart />
      </Grid>
      <Grid item xs={12}>
        <DailyActiveUsersChart />
      </Grid>
    </Grid>
  );
};
