import { FC } from "react";
import { Alert, Paper, Stack, Typography, useTheme } from "@mui/material";
import { StakingPortfolioLoadingState } from "@/components/Organisms/Staking/StakingPortfolioLoadingState";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";
import { StakingPortfolioTable } from "@/components/Organisms/Staking/StakingPortfolioTable";

export const StakingPortfolio: FC = () => {
  const theme = useTheme();
  const {
    stakingPortfolio,
    isPositionsLoading
  } = useStakingRewards();

  if (isPositionsLoading) {
    return <StakingPortfolioLoadingState />;
  }

  return (
    <Paper sx={{ padding: theme.spacing(6), marginTop: theme.spacing(9) }}>
      <Stack gap={6}>
        <Typography variant="h6">Portfolio</Typography>
        {Object.values(stakingPortfolio).length > 0 ? (
          <StakingPortfolioTable stakingPortfolio={stakingPortfolio} />
        ) : (
          <>
            <Alert color="info">
              No position found.
            </Alert>
          </>
        )}
      </Stack>
    </Paper>
  );
};
