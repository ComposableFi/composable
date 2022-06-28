import type { NextPage } from "next";
import {
  Container,
  Box,
  useTheme,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { Bonds, PageTitle } from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { useDotSamaContext } from "substrate-react";
import { DEFI_CONFIG } from "@/defi/config";
import { useState } from "react";

const standardPageSize = {
  xs: 12,
};

const twoColumnPageSize = {
  xs: 12,
  md: 6,
};

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
