import { Grid, useTheme } from "@mui/material";
import { FeaturedBox } from "@/components/Molecules";
import { DailyActiveUsersChart } from "@/components/Organisms/Stats/DailyActiveUsersChart";
import { useOverviewStats } from "@/apollo/hooks/useOverviewStats";
import { useCirculatingSupply } from "@/apollo/hooks/useCirculatingSupply";
import { useMarketCap } from "@/apollo/hooks/useMarketCap";
import { TotalValueLockedChart } from "@/components/Organisms/Stats/TotalValueLockedChart";
import { FC } from "react";
import { formatNumber } from "shared";

export const StatsOverviewTab: FC = () => {
  const circulatingSupply = useCirculatingSupply();
  const marketCap = useMarketCap();
  const { data, loading } = useOverviewStats();
  const theme = useTheme();

  return (
    <Grid container spacing={4}>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title: "tooltip",
            }}
            textAbove="Total value locked"
            title={data?.overviewStats.totalValueLocked.toString()}
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
              title: "tooltip",
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
              title: "tooltip",
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
            title: "tooltip",
          }}
          textAbove="Picasso market cap"
          title={`$${marketCap.toFormat(2)}`}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite,
          }}
          TooltipProps={{
            title: "tooltip",
          }}
          textAbove="Picasso circulating supply"
          title={`${circulatingSupply.toFormat(0)} PICA`}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title: "tooltip",
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
