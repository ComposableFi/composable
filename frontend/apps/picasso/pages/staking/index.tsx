import { StakingConnected } from "@/components/Organisms/Staking/StakingConnected";
import { StakingDisconnected } from "@/components/Organisms/Staking/StakingDisconnected";
import { useTheme } from "@mui/material";
import type { NextPage } from "next";
import { useDotSamaContext } from "substrate-react";
import { StakingLayout } from "@/components/Templates/StakingLayout";

const Staking: NextPage = () => {
  const theme = useTheme();
  const { extensionStatus } = useDotSamaContext();
  const isDisconnected = extensionStatus !== "connected";
  const standardPageSize = {
    xs: 12,
  };

  return (
    <StakingLayout>
      {isDisconnected ? (
        <StakingDisconnected gridSize={standardPageSize} theme={theme} />
      ) : (
        <StakingConnected />
      )}
    </StakingLayout>
  );
};

export default Staking;
