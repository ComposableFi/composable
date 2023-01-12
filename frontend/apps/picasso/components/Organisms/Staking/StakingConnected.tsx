import { FC } from "react";
import { StakingHighlights } from "@/components/Organisms/Staking/StakingHighlights";
import { StakingPortfolio } from "@/components/Organisms/Staking/StakingPortfolio";
import { StakeFormSection } from "@/components/Organisms/Staking/StakeFormSection";

type StakingConnectedProps = {};

export const StakingConnected: FC<StakingConnectedProps> = () => {
  return (
    <>
      <StakingHighlights />
      <StakingPortfolio />
      <StakeFormSection />
    </>
  );
};
