import type { NextPage } from "next";
import { Container, Box } from "@mui/material";
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

const Home: NextPage = () => {
  const dispatch = useDispatch();
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  return (
    <Default>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle
              title="Overview"
              subtitle="Stake PABLO for sPABLO"
            />
          </Box>
          <Box mt={8}>
            <Statistics />
          </Box>
          {!connected && (
            <ConnectWalletFeaturedBox
              mt={8}
              p={4}
              title="Connect wallet"
              textBelow="To see your portfolio, wallet needs to be connected."
              ButtonProps={{label: "Connect Wallet", size: "small"}}
            />
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
