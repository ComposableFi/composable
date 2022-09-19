import type { NextPage } from "next";
import Default from "@/components/Templates/Default";
import { Box, useTheme } from "@mui/material";
import { StakingDisconnected } from "@/components/Organisms/Staking/StakingDisconnected";
import { StakingConnected } from "@/components/Organisms/Staking/StakingConnected";
import { StakingPageHeading } from "@/components/Organisms/Staking/StakingPageHeading";
import { useDotSamaContext } from "substrate-react";

const Staking: NextPage = () => {
  const theme = useTheme();
  const { extensionStatus } = useDotSamaContext();
  const isDisconnected = extensionStatus !== "connected";
  const standardPageSize = {
    xs: 12
  };

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} mt={9}>
        <StakingPageHeading />
        {isDisconnected && (
          <StakingDisconnected gridSize={standardPageSize} theme={theme} />
        )}
        {!isDisconnected && <StakingConnected />}
      </Box>
    </Default>
  );
};

export default Staking;
