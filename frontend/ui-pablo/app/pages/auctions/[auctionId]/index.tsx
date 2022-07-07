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
import { useEffect, useState } from "react";
import { DEFAULT_NETWORK_ID, fetchSpotPrice } from "@/defi/utils";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { useAuctionsChart } from "@/store/hooks/useAuctionsChart";
import moment from "moment-timezone";
import useLiquidityBootstrappingPoolStore from "@/store/useStore";
import { useAsset } from "@/defi/hooks/assets/useAsset";
import { useRouter } from "next/router";

const Auction: NextPage = () => {
  const theme = useTheme();
  const router = useRouter();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const {
    pools: {
      setLiquidityBootstrappingPoolSpotPrice,
    },
    resetActiveLBP,
    auctions: { activeLBP, activeLBPStats }
  } = useLiquidityBootstrappingPoolStore();

  const baseAsset = useAsset(activeLBP.pair.base.toString())
  const quoteAsset = useAsset(activeLBP.pair.quote.toString())

  useEffect(() => {
    if (!selectedAccount) {
      router.push('/auctions');
    }
  }, [router, selectedAccount]);

  useEffect(() => {
    if (parachainApi && activeLBP.poolId !== -1) {
      const interval = setInterval(() => {
        const pair = {
          base: activeLBP.pair.base.toString(),
          quote: activeLBP.pair.quote.toString()
        }
        fetchSpotPrice(parachainApi, pair, activeLBP.poolId).then(
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
  }, [parachainApi, activeLBP.poolId, activeLBP.pair, setLiquidityBootstrappingPoolSpotPrice]);


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

  const {
    currentPriceSeries,
    predictedPriceSeries
  } = useAuctionsChart(activeLBP)

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
  }, [resetActiveLBP]);

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
              baseAsset={baseAsset}
              quoteAsset={quoteAsset}
              stats={activeLBPStats}
              auction={activeLBP}
              mt={6}
            />

            <Grid container mt={6}>
              <Grid item md={6} pr={1.75}>
                <AuctionPriceChart
                  baseAsset={baseAsset}
                  quoteAsset={quoteAsset}
                  priceSeries={currentPriceSeries}
                  predictedPriceSeries={predictedPriceSeries}
                  height="100%"
                  dateFormat={(timestamp: number | string) => {
                    return moment(timestamp).utc().format("MMM D, h:mm:ss A");
                  }}
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
              <AuctionDetails stats={activeLBPStats} auction={activeLBP} baseAsset={baseAsset} quoteAsset={quoteAsset} />
            </TabPanel>
            <TabPanel value={tab} index={1}>
              <AuctionHistoriesTable auction={activeLBP} baseAsset={baseAsset} quoteAsset={quoteAsset} />
            </TabPanel>
          </Box>
        </Box>
      </Container>
    </Default>
  );
};

export default Auction;
