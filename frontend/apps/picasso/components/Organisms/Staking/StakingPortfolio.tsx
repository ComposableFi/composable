import { FC } from "react";
import { useTheme } from "@mui/material";
import { StakingPortfolioLoadingState } from "@/components/Organisms/Staking/StakingPortfolioLoadingState";

export const StakingPortfolio: FC = () => {
  const theme = useTheme();
  // const {
  //   stakingPortfolio,
  //   isPositionsLoading
  // } = useStakingRewards();

  if (true) {
    return <StakingPortfolioLoadingState />;
  }

  // return (
  //   <Paper sx={{ padding: theme.spacing(6), marginTop: theme.spacing(9) }}>
  //     <Stack gap={6}>
  //       <Typography variant="h6">Portfolio</Typography>
  //       {Object.values(stakingPortfolio).length > 0 ? (
  //         <StakingPortfolioTable stakingPortfolio={stakingPortfolio} />
  //       ) : (
  //         <>
  //           <Alert color="info">
  //             No position found.
  //           </Alert>
  //         </>
  //       )}
  //     </Stack>
  //   </Paper>
  // );
};
