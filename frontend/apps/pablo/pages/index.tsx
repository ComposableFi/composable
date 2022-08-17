import type { NextPage } from "next";
import { Container, Box, Grid } from "@mui/material";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components";
import { useDispatch } from "react-redux";
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
  const dispatch = useDispatch();
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  return (
    <Default>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle title="Overview" />
          </Box>
          <Box mt={8}>
            <Statistics />
          </Box>
          {!connected && (
            <>
              <Grid container gap={2}>
                <Grid xs={12}>
                  <ConnectWalletFeaturedBox
                    mt={8}
                    p={4}
                    title="Connect wallet"
                    textBelow="To see your portfolio, wallet needs to be connected."
                    ButtonProps={{ label: "Connect Wallet", size: "small" }}
                  />
                </Grid>
                <Grid item xs={12} md={6}>
                  <TVLChart />
                </Grid>
                <Grid item xs={12} md={6}>
                  <VolumeChart />
                </Grid>
              </Grid>
            </>
          )}

          {connected && (
            <>
              <WalletBreakdownBox mt={8} />
              <LiquidityProvidersBox mt={8} />
              <YourBondsBox mt={8} />
              <XPablosBox mt={8} />
            </>
          )}
        </Box>
      </Container>
    </Default>
  );
};

export default Home;
