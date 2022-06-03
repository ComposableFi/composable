import type { NextPage } from "next";
import {
  Container,
  Box,
  Typography,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { ConnectWalletFeaturedBox, Link, PageTitle } from "@/components";
import { PoolDetails } from "@/components/Organisms/pool/PoolDetails";
import { useDotSamaContext } from "substrate-react";

const PoolDetailsPage: NextPage = () => {

  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/pool">
      Pool
    </Link>,
    <Typography key="create-pool" color="text.primary">
      Select
    </Typography>,
  ];

  return (
    <Default breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle
            title="PICA/KSM Pool"
            subtitle="Earn tokens while adding liquidity."
          />
        </Box>
        {connected ? <PoolDetails mb={25} /> : <ConnectWalletFeaturedBox />}
      </Container>
    </Default>
  );
};

export default PoolDetailsPage;
