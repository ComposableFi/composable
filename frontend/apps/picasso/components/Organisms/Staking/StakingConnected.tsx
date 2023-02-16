import { StakingPortfolio } from "./StakingPortfolio";
import { StakeFormSection } from "./StakeFormSection";
import { StakingHighlights } from "./StakingHighlights";

export const StakingConnected = () => {
  return (
    <>
      <StakingHighlights />
      <StakingPortfolio />
      <StakeFormSection />
    </>
  );
};
