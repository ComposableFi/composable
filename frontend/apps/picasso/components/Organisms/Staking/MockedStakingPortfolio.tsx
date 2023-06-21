import { StakingPortfolioTable } from "@/components/Organisms/Staking/StakingPortfolioTable";
import { Paper, Stack, Typography, useTheme } from "@mui/material";

export const MockedStakingPortfolio = () => {
  const theme = useTheme();

  return (
    <Paper sx={{ padding: theme.spacing(6), marginTop: theme.spacing(9) }}>
      <Stack gap={6}>
        <Typography variant="h6">Portfolio</Typography>
        <StakingPortfolioTable stakingPortfolio={[]} />
      </Stack>
    </Paper>
  );
};
