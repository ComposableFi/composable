import type { NextPage } from "next";
import {
  Container,
  Box,
  Grid,
  useTheme,
  Typography,
  alpha,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import {
  AuctionStatusIndicator,
  Link,
  TabItem,
  Tabs,
  TabPanel,
} from "@/components";
import { AuctionInformation } from "@/components/Organisms/auction/AuctionInformation";
import { AuctionDetails } from "@/components/Organisms/auction/AuctionDetails";
import { BuyForm } from "@/components/Organisms/auction/BuyForm";
import { AuctionHistoriesTable } from "@/components/Organisms/auction/AuctionHistoriesTable";
import { AuctionPriceChart } from "@/components/Organisms/auction/AuctionPriceChart";
import moment from "moment-timezone";
import useLiquidityBootstrappingPoolStore from "@/store/useStore";
import { useEffect, useState } from "react";
import { getAssetById } from "@/defi/polkadot/Assets";
import { fetchSpotPrice } from "@/updaters/swaps/utils";
import { useParachainApi } from "substrate-react";

const Auction: NextPage = () => {
  const theme = useTheme();
  const {
    pools: {
      setLiquidityBootstrappingPoolSpotPrice,
    },
    resetActiveLBP,
    auctions: { activeLBP, activeLBPStats, activeChart },
  } = useLiquidityBootstrappingPoolStore();
  const { parachainApi } = useParachainApi("picasso");

  useEffect(() => {
    if (parachainApi && activeLBP.poolId !== -1) {
      const interval = setInterval(() => {
        console.log("SP Interval");
        fetchSpotPrice(parachainApi, activeLBP.pair, activeLBP.poolId).then(
          (spotPrice) => {
            setLiquidityBootstrappingPoolSpotPrice(
              activeLBP.poolId,
              spotPrice.toFixed(4)
            );
          }
        );
      }, 1000 * 60);

      return () => clearInterval(interval);
    }
  }, [parachainApi, activeLBP, setLiquidityBootstrappingPoolSpotPrice]);

  const baseAsset = getAssetById("picasso", activeLBP.pair.base);
  const quoteAsset = getAssetById("picasso", activeLBP.pair.quote);
  const [currentTimestamp] = useState<number>(Date.now());

  const isActive: boolean =
    activeLBP.sale.start <= currentTimestamp &&
    activeLBP.sale.end >= currentTimestamp;
  const isEnded: boolean = activeLBP.sale.end < currentTimestamp;

  const [tab, setTab] = useState(0);
  const tabItems: TabItem[] = [
    {
      label: "Auction Details",
    },
    {
      label: "Auction History",
    },
  ];

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/auctions">
      <Typography key="addliquidity" variant="body1">
        Auctions
      </Typography>
    </Link>,
    <Typography key="addliquidity" variant="body1" color="text.primary">
      Select Auction
    </Typography>,
  ];

  const getStatusLabel = () => {
    return isActive ? "Active" : isEnded ? "Ended" : "Starting soon";
  };

  useEffect(() => {
    return () => {
      resetActiveLBP();
    };
    // We can omit dependencies because it's a cleanup case and it only runs on destruction of pages by React.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <Default breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box
            sx={{
              background: theme.palette.gradient.secondary,
              borderRadius: 1,
              padding: theme.spacing(6),
              [theme.breakpoints.down("md")]: {
                padding: theme.spacing(2),
              },
            }}
            border={`1px solid ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            )}`}
          >
            <Box display="flex" alignItems="center">
              <Typography variant="h5" pr={4}>
                {baseAsset?.symbol} Token Launch Auction
              </Typography>
              <AuctionStatusIndicator
                auction={activeLBP}
                label={getStatusLabel()}
                LabelProps={{ variant: "subtitle1" }}
                padding={theme.spacing(1, 2, 1, 1.5)}
                borderRadius={1}
                sx={{
                  background: alpha(
                    theme.palette.primary.main,
                    theme.custom.opacity.light
                  ),
                  height: 48,
                }}
              />
            </Box>

            <AuctionInformation
              stats={activeLBPStats}
              auction={activeLBP}
              mt={6}
            />

            <Grid container mt={6}>
              <Grid item md={6} pr={1.75}>
                <AuctionPriceChart
                  baseAsset={baseAsset}
                  quoteAsset={quoteAsset}
                  data={activeChart.price}
                  height="100%"
                  dateFormat={(timestamp: number | string) => {
                    return moment(timestamp).utc().format("MMM D, h:mm:ss A");
                  }}
                  pastCount={1}
                  color={theme.palette.primary.main}
                />
              </Grid>
              <Grid item md={6} pl={1.75}>
                <BuyForm auction={activeLBP} />
              </Grid>
            </Grid>
          </Box>

          <Box mt={8}>
            <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
            <TabPanel value={tab} index={0}>
              <AuctionDetails stats={activeLBPStats} auction={activeLBP} />
            </TabPanel>
            <TabPanel value={tab} index={1}>
              <AuctionHistoriesTable auction={activeLBP} />
            </TabPanel>
          </Box>
        </Box>
      </Container>
    </Default>
  );
};

export default Auction;
