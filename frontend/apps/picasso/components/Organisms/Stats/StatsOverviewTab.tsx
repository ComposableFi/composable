import { Grid, useTheme } from "@mui/material";
import { FeaturedBox } from "@/components/Molecules";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { u128 } from "@polkadot/types-codec";
import { ComposableTraitsOraclePrice } from "defi-interfaces";
import { Assets } from "@/defi/polkadot/Assets";
import { DailyActiveUsersChart } from "@/components/Organisms/Stats/DailyActiveUsersChart";
import { OVERVIEW_STATS, OverviewStats } from "@/apollo/queries/overviewStats";
import { useQuery } from "@apollo/client";

const useCirculatingSupply = () => {
  const { parachainApi } = usePicassoProvider();
  const [circulatingSupply, setCirculatingSupply] = useState<BigNumber>(
    new BigNumber(0)
  );

  useEffect(() => {
    callbackGate((api) => {
      api.query.balances.totalIssuance((totalIssuance: u128) =>
        setCirculatingSupply(
          fromChainIdUnit(unwrapNumberOrHex(totalIssuance.toHex()))
        )
      );
    }, parachainApi);
  }, [parachainApi]);

  return circulatingSupply;
};

const useMarketCap = () => {
  const circulatingSupply = useCirculatingSupply();
  const [picaPrice, setPicaPrice] = useState<BigNumber>(new BigNumber(0));
  const { parachainApi } = usePicassoProvider();
  useEffect(() => {
    callbackGate((api) => {
      api.query.oracle.prices(
        Assets.pica.supportedNetwork.picasso,
        (result: ComposableTraitsOraclePrice) => {
          if (!result.isEmpty) {
            const { price, block } = result.toJSON() as any;
            setPicaPrice(fromChainIdUnit(unwrapNumberOrHex(price)));
          }
        }
      );
    }, parachainApi);
  }, [parachainApi]);

  return circulatingSupply.multipliedBy(picaPrice);
};

const useOverviewStats = () => {
  const { data, error, loading } = useQuery<OverviewStats>(OVERVIEW_STATS);

  return { data, error, loading };
};

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
