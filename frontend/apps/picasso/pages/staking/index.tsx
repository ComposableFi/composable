import { StakingConnected } from "@/components/Organisms/Staking/StakingConnected";
import { StakingDisconnected } from "@/components/Organisms/Staking/StakingDisconnected";
import type { NextPage } from "next";
import { useDotSamaContext } from "substrate-react";
import { StakingLayout } from "@/components/Templates/StakingLayout";

const standardPageSize = {
  xs: 12,
};
const Staking: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const isDisconnected = extensionStatus !== "connected";

  return (
    <StakingLayout>
      {isDisconnected ? (
        <StakingDisconnected gridSize={standardPageSize} />
      ) : (
        <StakingConnected />
      )}
    </StakingLayout>
  );
};

export default Staking;
