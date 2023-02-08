import { alpha, Box, Paper, Stack, Typography, useTheme } from "@mui/material";
import { useMemo } from "react";
import { TokenAsset } from "@/components";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { useStore } from "@/stores/root";
import BigNumber from "bignumber.js";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { ClaimButton } from "./ClaimButton";
import { TokenWithUSD } from "@/components/Organisms/Staking/TokenWithUSD";

export const ClaimableRewards = () => {
  const theme = useTheme();
  const picaPrice = usePicaPriceDiscovery();
  const picaToken = useStore((store) => store.substrateTokens.tokens.pica);
  const allClaimable = useStore((store) => store.claimableRewards);
  const { stakingPortfolio } = useStakingRewards();
  const hasClaimable = useMemo(() => {
    return Object.values(stakingPortfolio).length > 0;
  }, [stakingPortfolio]);
  const claimableAmount = useMemo(() => {
    return Object.values(allClaimable).reduce((acc, currentInstance) => {
      return acc.plus(
        currentInstance.reduce((balances, currentAsset) => {
          if (currentAsset.assetId === "1") {
            return balances.plus(currentAsset.balance);
          }
          return balances;
        }, new BigNumber(0))
      );
    }, new BigNumber(0));
  }, [allClaimable]);
  const claimable = claimableAmount.toFormat(0);
  const usdValue = claimableAmount.multipliedBy(picaPrice).toFormat(2);

  if (!hasClaimable) {
    return null;
  }

  return (
    <Paper sx={{ padding: theme.spacing(6) }}>
      <Stack gap={6}>
        <Typography variant="h6">
          Claimable ${picaToken.symbol} Rewards
        </Typography>
        <Stack
          direction="row"
          alignItems="center"
          justifyContent="space-between"
          width="100%"
          sx={{
            p: 4,
            borderRadius: 1,
            border: `1px solid ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            )}`,
          }}
        >
          <Box>
            <TokenAsset tokenId={picaToken.id} label={picaToken.symbol} />
          </Box>
          <Box display="flex" alignItems="center" gap={1}>
            <TokenWithUSD
              symbol={picaToken.symbol}
              amount={claimable}
              price={usdValue}
            />
          </Box>
        </Stack>
        <ClaimButton />
      </Stack>
    </Paper>
  );
};
