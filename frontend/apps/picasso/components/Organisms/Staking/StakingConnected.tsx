import { Grid } from "@mui/material";
import { Tabs } from "@/components";
import { FC, useState } from "react";
import { StakingHighlights } from "@/components/Organisms/Staking/StakingHighlights";
import { ClaimableRewards } from "@/components/Organisms/Staking/ClaimableRewards";
import { StakingPortfolio } from "@/components/Organisms/Staking/StakingPortfolio";
import { StakeTabContent } from "@/components/Organisms/Staking/StakeTabContent";
import { TAB_ITEMS } from "@/components/Organisms/Staking/constants";
import { BurnUnstakeTabContent } from "@/components/Organisms/Staking/BurnUnstakeTabContent";

type StakingConnectedProps = {};

export const StakingConnected: FC<StakingConnectedProps> = () => {
  const [stakeTab, setStakeTab] = useState<0 | 1>(0);

  return (
    <>
      <StakingHighlights />
      <StakingPortfolio />
      <Grid container mt={9}>
        <Grid item xs={12}>
          <Tabs
            items={TAB_ITEMS}
            value={stakeTab}
            onChange={(_e, target) => setStakeTab(target)}
          />
          {stakeTab === 0 && <StakeTabContent />}
          {stakeTab === 1 && <BurnUnstakeTabContent />}
        </Grid>
        <Grid item xs={12} marginTop={9}>
          <ClaimableRewards onClaimButtonClick={() => {
          } /* TODO: */} />
        </Grid>
      </Grid>
    </>
  );
};
