import type { NextPage } from "next";
import {
  Container,
  Box,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components/Molecules";
import { TabItem } from "@/components/Atoms";
import { ConnectWalletFeaturedBox } from "@/components/Organisms";
import { Staking } from "@/components/Organisms";

import { useDotSamaContext } from "substrate-react";


const StakingPage: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected"
  return (
    <Default>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
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
