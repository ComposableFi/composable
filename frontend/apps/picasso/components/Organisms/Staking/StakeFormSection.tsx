import { Grid } from "@mui/material";
import { Tabs } from "@/components";
import { TAB_ITEMS } from "@/components/Organisms/Staking/constants";
import { StakeTabContent } from "@/components/Organisms/Staking/StakeTabContent";
import { BurnUnstakeTabContent } from "@/components/Organisms/Staking/BurnUnstakeTabContent";
import { ClaimableRewards } from "@/components/Organisms/Staking/ClaimableRewards";
import { useState } from "react";

const STAKE_TAB = {
  STAKE: 0,
  UNSTAKE: 1,
} as const;
type StakeKey = keyof typeof STAKE_TAB;
type StakeTab = typeof STAKE_TAB[StakeKey];

export const StakeFormSection = () => {
  const [stakeTab, setStakeTab] = useState<StakeTab>(STAKE_TAB.STAKE);

  return (
    <Grid container mt={9}>
      <Grid item xs={12}>
        <Tabs
          items={TAB_ITEMS}
          value={stakeTab}
          onChange={(_e, target: number) => setStakeTab(target as StakeTab)}
        />
        {stakeTab === STAKE_TAB.STAKE && <StakeTabContent />}
        {stakeTab === STAKE_TAB.UNSTAKE && <BurnUnstakeTabContent />}
      </Grid>
      <Grid item xs={12} marginTop={9}>
        <ClaimableRewards />
      </Grid>
    </Grid>
  );
};
