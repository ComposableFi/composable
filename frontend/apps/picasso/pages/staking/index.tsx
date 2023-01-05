import { StakingConnected } from "@/components/Organisms/Staking/StakingConnected";
import { StakingDisconnected } from "@/components/Organisms/Staking/StakingDisconnected";
import { StakingPageHeading } from "@/components/Organisms/Staking/StakingPageHeading";
import Default from "@/components/Templates/Default";
import { Box, useTheme } from "@mui/material";
import type { NextPage } from "next";
import { useDotSamaContext } from "substrate-react";

const Staking: NextPage = () => {
  const theme = useTheme();
  const { extensionStatus } = useDotSamaContext();
  const isDisconnected = extensionStatus !== "connected";
  const standardPageSize = {
    xs: 12,
  };

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} mt={9}>
        <StakingPageHeading />
        {isDisconnected && (
          <StakingDisconnected gridSize={standardPageSize} theme={theme} />
        )}
        {!isDisconnected && (
          <Box>
            <StakingConnected />
          </Box>
        )}
      </Box>
    </Default>
  );
};

export default Staking;
