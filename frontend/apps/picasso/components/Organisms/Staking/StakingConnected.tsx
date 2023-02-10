import { FC } from "react";
import { StakingPortfolio } from "./StakingPortfolio";
import { StakeFormSection } from "./StakeFormSection";
import { StakingHighlights } from "./StakingHighlights";

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
