import { BaseAsset, PairAsset } from "@/components/Atoms";
import { useLiquidityPoolDetails } from "@/defi/hooks/useLiquidityPoolDetails";
import { useUserProvidedLiquidityByPool } from "@/defi/hooks/useUserProvidedLiquidityByPool";
import {
  alpha,
  Box,
  Button,
  Divider,
  Grid,
  GridProps,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { PoolDetailsProps } from "./index";
import { BoxWrapper } from "../../BoxWrapper";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { useAssets } from "@/defi/hooks";
import { Asset } from "shared";
import { useClaimStakingRewards } from "@/defi/hooks/stakingRewards/useClaimStakingRewards";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  value: string;
  intro?: string;
} & GridProps;

const Item: React.FC<ItemProps> = ({
  value,
  intro,
  children,
  ...gridProps
}) => {
  return (
    <Grid container {...gridProps}>
      <Grid item {...twoColumnPageSize}>
        {children}
      </Grid>
      <Grid item {...twoColumnPageSize} textAlign="right">
        {intro && (
          <Typography
            variant="subtitle1"
            color="text.secondary"
            component="span"
            mr={2}
          >
            {intro}
          </Typography>
        )}
        <Typography variant="subtitle1" fontWeight={600} component="span">
          {value}
        </Typography>
      </Grid>
    </Grid>
  );
};

export const PoolRewardsPanel: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const theme = useTheme();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const poolDetails = useLiquidityPoolDetails(poolId);
  const { pool } = poolDetails;
  const lpToken = pool?.getLiquidityProviderToken();
  const lpAssetId = lpToken?.getPicassoAssetId() as string ?? "-";
  const stakingRewardsPool = useStakingRewardPool(lpAssetId);
  const rewardAssets = useAssets(stakingRewardsPool ? Object.keys(stakingRewardsPool.rewards) : []);

  // WIP - awaiting Andres' subsquid changes
  const lpStaked = new BigNumber(0);
  const handleClaimRewards = useClaimStakingRewards({})

  const isPendingClaimStakingRewards = usePendingExtrinsic(
    "claim",
    "stakingRewards",
    selectedAccount ? selectedAccount.address : "-"
  )

  return (
    <BoxWrapper {...boxProps}>
      <Item
        value={`$${lpStaked}`}
        intro={`${lpStaked} ${lpToken?.getSymbol()}`}
      >
        <Typography variant="h6">Your deposits</Typography>
      </Item>
      <Item
        value={lpStaked.toFormat()}
        mt={4.375}
      >
        {
          lpToken && <PairAsset
            assets={lpToken.getUnderlyingAssetJSON()}
            separator="/"
          />
        }
      </Item>
      {/* <Item
        value={userProvidedLiquidity.tokenAmounts.quoteAmount.toFormat()}
        mt={2}
      >
        {poolDetails.quoteAsset && (
          <BaseAsset
            icon={poolDetails.quoteAsset.icon}
            label={poolDetails.quoteAsset.symbol}
          />
        )}
      </Item> */}

      <Box mt={4}>
        <Divider
          sx={{
            borderColor: alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            ),
          }}
        />
      </Box>

      <Item mt={4} mb={4} value={`$${0}`}>
        <Typography variant="h6">Your rewards</Typography>
      </Item>
      {rewardAssets.map((asset: Asset) => (
        <Item value={new BigNumber(0).toString()} mt={2} key={asset.getName()}>
          <BaseAsset
            icon={asset.getIconUrl()}
            label={asset.getSymbol()}
          />
        </Item>
      ))}

      <Box mt={4}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          onClick={handleClaimRewards}
        >
          Claim rewards
        </Button>
      </Box>

      <ConfirmingModal open={isPendingClaimStakingRewards} />
    </BoxWrapper>
  );
};
