import { TokenValue } from "@/components/Molecules";
import { BoxWrapper } from "../BoxWrapper";
import {
  useClaimableRewards,
  useClaimStakingRewards,
} from "@/defi/hooks/stakingRewards";
import { DEFAULT_NETWORK_ID, PBLO_ASSET_ID } from "@/defi/utils";
import {
  alpha,
  Box,
  BoxProps,
  Button,
  Grid,
  Theme,
  useTheme,
} from "@mui/material";
import millify from "millify";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { ConfirmingModal } from "../swap/ConfirmingModal";

const defaultPageSize = {
  sm: 12,
  md: 4,
};

const defaultTokenValueProps = (theme: Theme) =>
  ({
    justifyContent: "space-between",
    borderRadius: 1,
    px: 3,
    py: 2.25,
    sx: {
      background: alpha(
        theme.palette.common.white,
        theme.custom.opacity.lighter
      ),
    },
  } as const);

export const ClaimableRewards: React.FC<BoxProps & {
  financialNftCollectionId?: string;
}> = ({ financialNftCollectionId, ...boxProps }) => {
  const theme = useTheme();
  const connectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const claimableRewards = useClaimableRewards({
    stakedAssetId: PBLO_ASSET_ID,
  });
  const { financialNftInstanceId, claimableAssets } = claimableRewards;
  const canClaim =
    claimableAssets.some((x) => x.getClaimable().gt(0)) &&
    claimableRewards.financialNftInstanceId !== "-";

  const onClaimStakingRewards = useClaimStakingRewards({
    principalAssetId: PBLO_ASSET_ID,
    financialNftInstanceId:
      financialNftInstanceId === "-" ? undefined : financialNftInstanceId,
    financialNftCollectionId,
  });

  const isClaiming = usePendingExtrinsic(
    "claim",
    "stakingRewards",
    connectedAccount?.address ?? "-"
  )

  return (
    <BoxWrapper title="Claimable rewards" {...boxProps}>
      <Grid container spacing={3}>
        {claimableRewards.claimableAssets.map((asset) => {
          return (
            <Grid key={asset.getSymbol()} item {...defaultPageSize}>
              <TokenValue
                token={asset}
                value={millify(asset.getClaimable().toNumber())}
                {...defaultTokenValueProps(theme)}
              />
            </Grid>
          );
        })}
      </Grid>
      <Box mt={3}>
        <Button
          onClick={onClaimStakingRewards}
          disabled={!canClaim}
          variant="outlined"
          fullWidth
          size="large"
        >
          Claim all
        </Button>
      </Box>
      <ConfirmingModal open={isClaiming} />
    </BoxWrapper>
  );
};
