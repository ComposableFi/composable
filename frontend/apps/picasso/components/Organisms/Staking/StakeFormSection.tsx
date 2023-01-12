import { Grid } from "@mui/material";
import { Tabs } from "@/components";
import { TAB_ITEMS } from "@/components/Organisms/Staking/constants";
import { StakeTabContent } from "@/components/Organisms/Staking/StakeTabContent";
import { BurnUnstakeTabContent } from "@/components/Organisms/Staking/BurnUnstakeTabContent";
import { ClaimableRewards } from "@/components/Organisms/Staking/ClaimableRewards";
import { useState } from "react";
import { useExecutor, usePicassoProvider } from "substrate-react";

const STAKE_TAB = {
  STAKE: 0,
  UNSTAKE: 1,
} as const;
type StakeKey = keyof typeof STAKE_TAB;
type StakeTab = typeof STAKE_TAB[StakeKey];

export const StakeFormSection = () => {
  const [stakeTab, setStakeTab] = useState<StakeTab>(STAKE_TAB.STAKE);
  const executor = useExecutor();
  const { parachainApi } = usePicassoProvider();
  return (
    <Grid container mt={9}>
      <Grid item xs={12}>
        <Tabs
          items={TAB_ITEMS}
          value={stakeTab}
          onChange={(_e, target: number) => setStakeTab(target as StakeTab)}
        />
        {stakeTab === STAKE_TAB.STAKE && executor && parachainApi ? (
          <StakeTabContent executor={executor} parachainApi={parachainApi} />
        ) : null}
        {stakeTab === STAKE_TAB.UNSTAKE ? <BurnUnstakeTabContent /> : null}
      </Grid>
      <Grid item xs={12} marginTop={9}>
        <ClaimableRewards onClaimButtonClick={() => {} /* TODO: */} />
      </Grid>
    </Grid>
  );
};
