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
import { AuctionStatusIndicator, Link } from "@/components";
import { AuctionInformation } from "@/components/Organisms/auction/AuctionInformation";
import { BuyForm } from "@/components/Organisms/auction/BuyForm";
import { AuctionPriceChart } from "@/components/Organisms/auction/AuctionPriceChart";
import { useEffect } from "react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { useAssets } from "@/defi/hooks/assets/useAsset";
import { useRouter } from "next/router";
import AuctionDetailTabs from "@/components/Organisms/auction/AuctionDetailTabs";
import moment from "moment-timezone";
import {
  useAuctionsSlice,
} from "@/store/auctions/auctions.slice";
import { useAuctionsChart } from "@/defi/hooks";

const Auction: NextPage = () => {
  const theme = useTheme();
  const router = useRouter();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  const { activePool, activePoolStats } = useAuctionsSlice();
  const [baseAsset, quoteAsset] = useAssets(
    activePool.poolId !== -1
      ? [activePool.pair.base.toString(), activePool.pair.quote.toString()]
      : []
  );

  useEffect(() => {
    if (!selectedAccount) {
      router.push("/auctions");
    }
  }, [router, selectedAccount]);

  const chartSeries = useAuctionsChart(
    parachainApi,
    activePool
  );

  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/auctions">
      <Typography key="add-liquidity" variant="body1">
        Auctions
      </Typography>
    </Link>,
    <Typography key="add-liquidity" variant="body1" color="text.primary">
      Select Auction
    </Typography>,
  ];

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
                auction={activePool}
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
              stats={activePoolStats}
              auction={activePool}
              mt={6}
            />

            <Grid container mt={6}>
              <Grid item md={6} pr={1.75}>
                <AuctionPriceChart
                  baseAsset={baseAsset}
                  chartSeries={chartSeries}
                  height="100%"
                  dateFormat={(timestamp: number | string) => {
                    return moment(timestamp).utc().format("MMM D, h:mm:ss A");
                  }}
                  color={theme.palette.primary.main}
                />
              </Grid>
              <Grid item md={6} pl={1.75}>
                <BuyForm auction={activePool} />
              </Grid>
            </Grid>
          </Box>

          <AuctionDetailTabs />
        </Box>
      </Container>
    </Default>
  );
};

export default Auction;
