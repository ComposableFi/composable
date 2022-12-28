import type { NextPage } from "next";
import { Box, Container, Grid, Typography, useTheme } from "@mui/material";
import { PageTitle } from "@/components";
import { useDotSamaContext } from "substrate-react";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { WalletBreakdownBox } from "@/components/Organisms/overview/WalletBreakdownBox";
import { LiquidityProvidersBox } from "@/components/Organisms/overview/LiquidityProvidersBox";
import { PoolLayout } from "@/components/Templates/pools/PoolLayout";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

const Home: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";
  const theme = useTheme();

  return (
    <PoolLayout>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle
              title="Overview"
              subtitle="Visualize general stats, your portfolio and your open positions."
            />
          </Box>

          <Grid container>
            {!connected && (
              <Grid item xs={12}>
                <ConnectWalletFeaturedBox
                  mt={8}
                  title="Connect wallet"
                  textBelow="To see your portfolio, connect your wallet."
                  ButtonProps={{ label: "Connect Wallet", size: "small" }}
                />
              </Grid>
            )}
            <Grid item xs={12} mt={8}></Grid>
            <Grid container spacing={8}>
              <Grid item xs={12} md={6} mt={8}>
                <HighlightBox>
                  <Typography variant="h5" textAlign="left" mb={4}>
                    Total value locked
                  </Typography>
                  <Box
                    sx={{
                      height: theme.spacing(20),
                      minHeight: theme.spacing(20),
                      width: "100%",
                      display: "flex",
                      alignItems: "center",
                      flexDirection: "column",
                    }}
                    gap={2}
                  >
                    <Box
                      sx={{
                        height: theme.spacing(20),
                        minHeight: theme.spacing(20),
                        display: "flex",
                        alignItems: "center",
                        flexDirection: "column",
                      }}
                      gap={2}
                    >
                      <Typography variant="body2" textAlign="left">
                        Chart will be available once enough data is gathered...
                      </Typography>
                    </Box>
                  </Box>
                </HighlightBox>
              </Grid>
              <Grid item xs={12} md={6} mt={8}>
                <HighlightBox>
                  <Typography variant="h5" textAlign="left" mb={4}>
                    Volume
                  </Typography>
                  <Box
                    sx={{
                      height: theme.spacing(20),
                      minHeight: theme.spacing(20),
                      display: "flex",
                      alignItems: "center",
                      flexDirection: "column",
                    }}
                    gap={2}
                  >
                    <Typography variant="body2" textAlign="left">
                      Chart will be available once enough data is gathered...
                    </Typography>
                  </Box>
                </HighlightBox>
              </Grid>
            </Grid>
          </Grid>

          {connected && (
            <>
              <WalletBreakdownBox mt={8} key="wallet-breakdown" />
              <LiquidityProvidersBox mt={8} key="liquidity-provider-box" />
            </>
          )}
        </Box>
      </Container>
    </PoolLayout>
  );
};

export default Home;
