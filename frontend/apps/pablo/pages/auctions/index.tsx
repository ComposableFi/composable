import type { NextPage } from "next";
import { Box, Container } from "@mui/material";
import Default from "@/components/Templates/Default";
import { useDotSamaContext } from "substrate-react";
import { ConnectWalletFeaturedBox, PageTitle } from "@/components";
import { AuctionTable } from "@/components/Organisms/auction/AuctionTable";

const Auctions: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  return (
    <Default>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle
            title="Token Launch Auctions"
            subtitle="Liquidity Bootstrapping Pools for Pablo"
          />
        </Box>
        {connected ? <AuctionTable /> : <ConnectWalletFeaturedBox />}
      </Container>
    </Default>
  );
};

export default Auctions;
