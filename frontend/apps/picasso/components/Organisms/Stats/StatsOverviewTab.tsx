import { Grid, useTheme } from "@mui/material";
import { FeaturedBox } from "@/components/Molecules";
import { DailyActiveUsersChart } from "@/components/Organisms/Stats/DailyActiveUsersChart";
import { useOverviewStats } from "@/apollo/hooks/useOverviewStats";
import { useCirculatingSupply } from "@/apollo/hooks/useCirculatingSupply";
import { useMarketCap } from "@/apollo/hooks/useMarketCap";

export const StatsOverviewTab: React.FC<{}> = ({}) => {
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
              color: theme.palette.common.darkWhite
            }}
            textAbove="Active users"
            title={data?.overviewStats.activeUsersCount.toString()}
          />
        )}
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite
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
              color: theme.palette.common.darkWhite
            }}
            textAbove="Total transactions"
            title={data?.overviewStats.totalValueLocked.toString()}
          />
        )}
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite
          }}
          textAbove="Picasso market cap"
          title={`$${marketCap.toFormat(2)}`}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite
          }}
          textAbove="Picasso circulating supply"
          title={`${circulatingSupply.toFormat(0)} PICA`}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && data?.overviewStats && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite
            }}
            textAbove="Account holders"
            title={data?.overviewStats.accountHoldersCount.toString()}
          />
        )}
      </Grid>
      <Grid item xs={12}>
        <DailyActiveUsersChart />
      </Grid>
    </Grid>
  );
};
