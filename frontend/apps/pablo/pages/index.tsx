import type { NextPage } from "next";
import { Container, Box, Grid } from "@mui/material";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components";
import { Statistics } from "@/components/Organisms/overview/Statistics";
import { useDotSamaContext } from "substrate-react";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { WalletBreakdownBox } from "@/components/Organisms/overview/WalletBreakdownBox";
import { LiquidityProvidersBox } from "@/components/Organisms/overview/LiquidityProvidersBox";
import { YourBondsBox } from "@/components/Organisms/overview/YourBondsBox";
import { XPablosBox } from "@/components/Organisms/XPablosBox";
import { TVLChart } from "@/components/Organisms/overview/TVLChart";
import { VolumeChart } from "@/components/Organisms/overview/VolumeChart";

const Home: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  return (
    <Default>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle title="Overview" />
          </Box>

          <Grid container>
            {!connected && <Grid item xs={12}>
              <ConnectWalletFeaturedBox
                mt={8}
                title="Connect wallet"
                textBelow="To see your portfolio, wallet needs to be connected."
                ButtonProps={{ label: "Connect Wallet", size: "small" }}
              />
            </Grid>}
            <Grid item xs={12} mt={8}>
              <Statistics />
            </Grid>
            <Grid container spacing={8}>
              <Grid item xs={12} md={6} mt={8}>
                <TVLChart />
              </Grid>
              <Grid item xs={12} md={6} mt={8}>
                <VolumeChart />
              </Grid>
            </Grid>
          </Grid>

          {connected && (
            <>
              <WalletBreakdownBox mt={8} key="wallet-breakdown" />
              <LiquidityProvidersBox mt={8} key="liquidity-provider-box" />
              <YourBondsBox mt={8} key="your-bond-box" />
              <XPablosBox financialNftCollectionId="-" mt={8} key="xpablos-box" />
            </>
          )}
        </Box>
      </Container>
    </Default>
  );
};

export default Home;
