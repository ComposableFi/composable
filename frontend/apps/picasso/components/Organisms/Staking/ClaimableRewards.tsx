import { alpha, Box, Paper, Stack, Typography, useTheme } from "@mui/material";
import { TokenAsset } from "@/components";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { useStore } from "@/stores/root";
import { ClaimButton } from "./ClaimButton";
import { TokenWithUSD } from "@/components/Organisms/Staking/TokenWithUSD";
import { getClaimableAmount } from "@/stores/defi/polkadot/stakingRewards/accessor";

export const ClaimableRewards = () => {
  const theme = useTheme();
  const picaPrice = usePicaPriceDiscovery();
  const picaToken = useStore((store) => store.substrateTokens.tokens.pica);
  const claimable = useStore(getClaimableAmount);
  const usdValue = claimable.multipliedBy(picaPrice).toFormat(2);

  if (claimable.eq(0)) {
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
              amount={claimable.toFormat(4)}
              price={usdValue}
            />
          </Box>
        </Stack>
        <ClaimButton />
      </Stack>
    </Paper>
  );
};
