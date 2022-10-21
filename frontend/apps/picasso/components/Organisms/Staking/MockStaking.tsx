import { Tabs } from "@/components";
import { TAB_ITEMS } from "@/components/Organisms/Staking/constants";
import { MockedStakingTab } from "@/components/Organisms/Staking/MockedStakingTab";
import { StakingHighlights } from "@/components/Organisms/Staking/StakingHighlights";
import { Grid } from "@mui/material";
import { useState } from "react";

export const MockStaking = () => {
  const [stakeTab, setStakeTab] = useState<0 | 1>(0);

  return (
    <>
      <StakingHighlights />
      <Grid container mt={9}>
        <Grid item xs={12}>
          <Tabs items={TAB_ITEMS} value={stakeTab} onChange={(_e) => {}} />
          {stakeTab === 0 && <MockedStakingTab />}
        </Grid>
      </Grid>
    </>
  );
};
