import { Bonds, PageTitle } from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import Default from "@/components/Templates/Default";
import { Container } from "@mui/material";
import type { NextPage } from "next";
import { useDotSamaContext } from "substrate-react";

const BondPage: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const connected = extensionStatus === "connected";

  return (
    <Default>
      <Container maxWidth="lg">
        <PageTitle title="Bond" subtitle="Something about earning PICA" />
        {connected ? <Bonds mb={25} /> : <ConnectWalletFeaturedBox />}
      </Container>
    </Default>
  );
};

export default BondPage;
