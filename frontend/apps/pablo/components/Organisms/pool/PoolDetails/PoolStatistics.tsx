import {
  Box,
  useTheme,
  Typography,
  BoxProps,
  Grid,
  alpha,
} from "@mui/material";
import { useLiquidityPoolDetails } from "@/defi/hooks/useLiquidityPoolDetails";
import { PoolDetailsProps } from "./index";
import { BaseAsset } from "@/components/Atoms";
import { calculatePoolTotalValueLocked } from "@/defi/utils";
import { useStakingRewardsPoolApy } from "@/defi/hooks/stakingRewards/useStakingRewardsPoolApy";
import { useMemo } from "react";
import BigNumber from "bignumber.js";
import millify from "millify";
import { useAssetIdOraclePrice } from "@/defi/hooks";
import { useLiquidity } from "@/defi/hooks/useLiquidity";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  label: string;
  value?: string;
} & BoxProps;

const Item: React.FC<ItemProps> = ({ label, value, children, ...boxProps }) => {
  const theme = useTheme();
  return (
    <Box
      py={3.5}
      borderRadius={1}
      textAlign="center"
      border={`1px solid ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`}
      sx={{
        background: theme.palette.gradient.secondary,
      }}
      {...boxProps}
    >
      <Typography variant="body1" color="text.secondary">
        {label}
      </Typography>
      {value && (
        <Typography variant="h6" mt={0.5}>
          {value}
        </Typography>
      )}
      {children && children}
    </Box>
  );
};

export const PoolStatistics: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const { pool, poolStats } = useLiquidityPoolDetails(poolId);

  const pair = pool?.getPair() ?? null;
  const base = pair?.getBaseAsset().toString() ?? "-";
  const quote = pair?.getQuoteAsset().toString() ?? "-";

  const { baseAmount, quoteAmount } = useLiquidity(pool);

  const baseAssetPriceUSD = useAssetIdOraclePrice(
    base
  );
  const quoteAssetPriceUSD = useAssetIdOraclePrice(
    quote
  );

  const stakePoolApy = useStakingRewardsPoolApy(pool?.getLiquidityProviderToken().getPicassoAssetId() as string ?? "-");
  const rewardAPYs = useMemo(() => {
    return Object.keys(stakePoolApy).reduce((v, i) => {
      return v.plus(stakePoolApy[i])
    }, new BigNumber(0))
  }, [stakePoolApy]);

  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item {...twoColumnPageSize}>
          <Item
            label="Pool value"
            value={`$${millify(
              calculatePoolTotalValueLocked(
                baseAmount,
                quoteAmount,
                baseAssetPriceUSD,
                quoteAssetPriceUSD
              ).toNumber()
            )}`}
          />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Rewards left" py={2}>
            {poolStats.dailyRewards.map((asset) => (
              <BaseAsset
                key={asset.assetId}
                icon={asset.icon}
                label={asset.rewardAmountLeft}
                justifyContent="center"
                mt={0.5}
              />
            ))}
          </Item>
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Volume (24H)" value={`$${poolStats._24HrVolumeValue}`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Fees (24H)" value={`$${poolStats._24HrFeeValue}`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="APY" value={`${rewardAPYs.toFixed(2)}%`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item
            label="Transactions (24H)"
            value={`${poolStats._24HrTransactionCount}`}
          />
        </Grid>
      </Grid>
    </Box>
  );
};
