import { FC, useEffect } from "react";
import Default from "@/components/Templates/Default";
import { usePicassoProvider } from "substrate-react";
import { subscribeRewardPools } from "@/stores/defi/polkadot/stakingRewards/subscribeRewardPools";
import { useStore } from "@/stores/root";
import { Box, Grid, Skeleton } from "@mui/material";
import { StakingPageHeading } from "@/components/Organisms/Staking/StakingPageHeading";
import { useSubscribeStakingPositions } from "@/defi/polkadot/hooks/stakingRewards/useSubscribeStakingPositions";
import { subscribePortfolio } from "@/stores/defi/polkadot/stakingRewards/subscribePortfolio";
import { subscribeClaimableRewards } from "@/stores/defi/polkadot/stakingRewards/subscribeClaimableRewards";
import { useSubscribeStats } from "@/defi/polkadot/hooks/stakingRewards/useSubscribeStats";
import { subscribeXPicaAPR } from "@/stores/defi/polkadot/stakingRewards/subscribeXPicaAPR";
import { useStakingRewardsNotifications } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewardsNotifications";
import { useStakingRewardsFee } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewardsFee";

const sxProps = {
  mt: 20,
};
export const StakingLayout: FC = ({ children }) => {
  useSubscribeStakingPositions();
  useSubscribeStats();
  useStakingRewardsNotifications();
  useStakingRewardsFee();
  const { parachainApi } = usePicassoProvider();
  const isLoaded = useStore((store) => store.isRewardPoolLoaded);

  useEffect(() => subscribeRewardPools(parachainApi), [parachainApi]);
  useEffect(() => subscribePortfolio(parachainApi), [parachainApi]);
  useEffect(() => subscribeXPicaAPR(), []);
  useEffect(() => {
    const unsub = subscribeClaimableRewards(parachainApi);

    return () => {
      unsub?.then((f) => f?.());
    };
  }, [parachainApi]);

  if (!isLoaded || !parachainApi) {
    return (
      <Default>
        <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} mt={9}>
          <StakingPageHeading />
          <Box
            display="flex"
            justifyContent="center"
            alignItems="center"
            width="100%"
          >
            <Grid container spacing={2} maxWidth="lg">
              <Grid item xs={12} sm={4}>
                <Skeleton
                  variant="rounded"
                  width="100%"
                  height="100%"
                  sx={sxProps}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <Skeleton
                  variant="rounded"
                  width="100%"
                  height="100%"
                  sx={sxProps}
                />
              </Grid>
              <Grid item xs={12} sm={4}>
                <Skeleton
                  variant="rounded"
                  width="100%"
                  height="100%"
                  sx={sxProps}
                />
              </Grid>
              <Grid item xs={12}>
                <Skeleton
                  variant="rounded"
                  width="100%"
                  height="600px"
                  sx={sxProps}
                />
              </Grid>
            </Grid>
          </Box>
        </Box>
      </Default>
    );
  }

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} mt={9}>
        <StakingPageHeading />
        {children}
      </Box>
    </Default>
  );
};
