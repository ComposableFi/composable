import type { NextPage } from "next";
import { Box, Container } from "@mui/material";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components/Molecules";
import { Staking } from "@/components/Organisms";
import { UnavailableFeature } from "@/components/Molecules/UnavailableFeature";
import { UpcomingFeature } from "@/components/Atoms/UpcomingFeature";

const StakingPage: NextPage = () => {
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
            subtitle="Stake and lock tokens for additional yield"
          />
          <UnavailableFeature pageTitle={"Stake"} />
        </Box>
        <UpcomingFeature>
          <Staking />
        </UpcomingFeature>
      </Container>
    </Default>
  );
};

export default StakingPage;
