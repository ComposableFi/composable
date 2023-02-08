import { FC } from "react";
import { alpha, Stack, Typography, useTheme } from "@mui/material";
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
    <Stack
      gap={6}
      mt={9}
      p={6}
      sx={{
        borderRadius: 1,
        backgroundColor: alpha(theme.palette.common.white, 0.02),
      }}
    >
      <Typography variant="h6">My fNFTs</Typography>
      <StakingPortfolioTable stakingPortfolio={stakingPortfolio} />
    </Stack>
  );
};
