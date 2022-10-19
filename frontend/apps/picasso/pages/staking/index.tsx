import { UpcomingFeature } from "@/components/Molecules/UpcomingFeature";
import { MockStaking } from "@/components/Organisms/Staking/MockStaking";
import { StakingPageHeading } from "@/components/Organisms/Staking/StakingPageHeading";
import Default from "@/components/Templates/Default";
import { Box } from "@mui/material";
import type { NextPage } from "next";

const Staking: NextPage = () => {
  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} mt={9}>
        <StakingPageHeading />
        <UpcomingFeature>
          <MockStaking />
        </UpcomingFeature>
      </Box>
    </Default>
  );
};

export default Staking;
