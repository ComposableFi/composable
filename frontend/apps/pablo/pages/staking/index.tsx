import type { NextPage } from "next";
import { Box, Container } from "@mui/material";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components/Molecules";
import { ConnectWalletFeaturedBox, Staking } from "@/components/Organisms";

import { useDotSamaContext } from "substrate-react";

const StakingPage: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";
  return (
    <Default>
      <Container maxWidth="lg">
        <Box
          sx={{
            mb: 4,
          }}
        >
          <PageTitle
            title="Stake"
            subtitle="Lock PBLO to mint CHAOS, the yield bearing governance fNFT."
          />
        </Box>
        {connected ? <Staking mb={25} /> : <ConnectWalletFeaturedBox />}
      </Container>
    </Default>
  );
};

export default StakingPage;
