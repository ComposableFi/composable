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
        {!loading && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title:
                "The total value of PICA in USD deposited in Picasso's smart contracts.",
              disableHoverListener: true,
            }}
            textAbove="Total value locked"
            title={"123,456,789"}
          />
        )}
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        {!loading && (
          <FeaturedBox
            TextAboveProps={{
              color: theme.palette.common.darkWhite,
            }}
            TooltipProps={{
              title: "The number of people possessing a Picasso account.",
              disableHoverListener: true,
            }}
            textAbove="Account holders"
            title={"1234"}
          />
        )}
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite,
          }}
          TooltipProps={{
            title: "The total number of finalized transactions.",
            disableHoverListener: true,
          }}
          textAbove="Total transactions"
          title={"123,456,789"}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite,
          }}
          TooltipProps={{
            title:
              "The total value of all minted PICA in USD. (total supply * current market price)",
            disableHoverListener: true,
          }}
          textAbove="Picasso market cap"
          title={`$0.00`}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite,
          }}
          TooltipProps={{
            title: "The number of coins publicly available in the market.",
            disableHoverListener: true,
          }}
          textAbove="Picasso circulating supply"
          title={`${circulatingSupply.toFormat(0)} PICA`}
        />
      </Grid>
      <Grid item xs={12} sm={6} md={4}>
        <FeaturedBox
          TextAboveProps={{
            color: theme.palette.common.darkWhite,
          }}
          TooltipProps={{
            title:
              "The number of people interacting with Picasso in the last 24 hours.",
            disableHoverListener: true,
          }}
          textAbove="Active users"
          title={"123,456"}
        />
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
