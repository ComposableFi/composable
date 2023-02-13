import { Grid } from "@mui/material";
import { Tabs } from "@/components";
import { StakeTabContent } from "@/components/Organisms/Staking/StakeTabContent";
import { BurnUnstakeTabContent } from "@/components/Organisms/Staking/BurnUnstakeTabContent";
import { ClaimableRewards } from "@/components/Organisms/Staking/ClaimableRewards";
import { useStore } from "@/stores/root";

const STAKE_TAB = {
  STAKE: 0,
  UNSTAKE: 1,
} as const;
type StakeKey = keyof typeof STAKE_TAB;
type StakeTab = typeof STAKE_TAB[StakeKey];

export const StakeFormSection = () => {
  const stakeTab = useStore((store) => store.ui.stakingRewards.stakeTab);
  const setStakingTab = useStore((store) => store.ui.setStakingTab);
  const hasPortfolio = useStore((store) =>
    Boolean(store.stakingPortfolio.size)
  );
  const tabItems = [
    {
      label: "Stake and mint",
    },
    {
      label: "Burn and unstake",
      disabled: !hasPortfolio,
    },
  ];

  return (
    <Grid container mt={9}>
      <Grid item xs={12}>
        <Tabs
          items={tabItems}
          value={stakeTab}
          onChange={(_e, target: number) => setStakingTab(target as StakeTab)}
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
