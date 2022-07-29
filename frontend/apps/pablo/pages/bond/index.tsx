import type { NextPage } from "next";
import {
  Container,
  Box,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { Bonds, PageTitle } from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { useDotSamaContext } from "substrate-react";

const BondPage: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  return (
    <Default>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle title="Bond" subtitle="Something about earning PICA" />
        </Box>
        {connected ? <Bonds mb={25} /> : <ConnectWalletFeaturedBox />}
      </Container>
    </Default>
  );
};

export default BondPage;
