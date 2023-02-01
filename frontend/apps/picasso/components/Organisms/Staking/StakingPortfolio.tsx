import { FC } from "react";
import { Paper, Stack, Typography, useTheme } from "@mui/material";
import { StakingPortfolioLoadingState } from "@/components/Organisms/Staking/StakingPortfolioLoadingState";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { StakingPortfolioTable } from "@/components/Organisms/Staking/StakingPortfolioTable";

export const StakingPortfolio: FC = () => {
  const theme = useTheme();
  const { stakingPortfolio, isPositionsLoading } = useStakingRewards();
  const hasPortfolio = Object.values(stakingPortfolio).length > 0;

  if (isPositionsLoading) {
    return <StakingPortfolioLoadingState />;
  }

  if (!hasPortfolio) return null;

  return (
    <Paper sx={{ padding: theme.spacing(6), marginTop: theme.spacing(9) }}>
      <Stack gap={6}>
        <Typography variant="h6">Portfolio</Typography>
        <StakingPortfolioTable stakingPortfolio={stakingPortfolio} />
      </Stack>
    </Paper>
  );
};
