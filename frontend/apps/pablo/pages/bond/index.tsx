import { Bonds, PageTitle } from "@/components";
import Default from "@/components/Templates/Default";
import { Container } from "@mui/material";
import type { NextPage } from "next";
import { UpcomingFeature } from "@/components/Atoms/UpcomingFeature";
import { UnavailableFeature } from "@/components/Molecules/UnavailableFeature";

const BondPage: NextPage = () => {
  return (
    <Default>
      <Container maxWidth="lg">
        <PageTitle title="Bond" subtitle="Bond PICA for CHAOS" />
        <UnavailableFeature pageTitle={"Bond"} />
        <UpcomingFeature>
          <Bonds />
        </UpcomingFeature>
      </Container>
    </Default>
  );
};

export default BondPage;
