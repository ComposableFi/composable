import {
  alpha,
  Box,
  Button,
  Grid,
  GridProps,
  Typography,
  useTheme,
} from "@mui/material";
import { BoxWrapper } from "../../BoxWrapper";
import { DonutChart } from "@/components/Atoms/DonutChart";
import { PoolDetailsProps } from "./index";
import { useRouter } from "next/router";
import useStore from "@/store/useStore";
import { BaseAsset } from "@/components";
import BigNumber from "bignumber.js";
import { usePoolRatio } from "@/defi/hooks/pools/usePoolRatio";
import { FC } from "react";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  value: string;
} & GridProps;

const Item: FC<ItemProps> = ({ value, children, ...gridProps }) => {
  return (
    <Grid container {...gridProps}>
      <Grid item {...twoColumnPageSize}>
        {children}
      </Grid>
      <Grid item {...twoColumnPageSize} textAlign="right">
        <Typography variant="subtitle1" fontWeight={600}>
          {value}
        </Typography>
      </Grid>
    </Grid>
  );
};

const DonutChartLabels = ["My Position", "Total Value Locked"];

export const PoolLiquidityPanel: FC<PoolDetailsProps> = ({
  pool,
  ...boxProps
}) => {
  const router = useRouter();
  const theme = useTheme();
  const poolAmount = useStore((store) => store.pools.poolAmount);
  const lpTokens = useStore((store) => store.ownedLiquidity.tokens);
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const totalIssued = useStore((store) => store.pools.totalIssued);
  const { userTVL, lpRatio } = usePoolRatio(pool);
  const poolId = pool.poolId.toString();

  const handleAddLiquidity = () => {
    router.push(`/pool/add-liquidity/${poolId}`);
  };
  const handleRemoveLiquidity = () => {
    router.push(`/pool/remove-liquidity/${poolId}`);
  };

  if (
    !isPoolsLoaded ||
    Object.keys(lpTokens).length === 0 ||
    Object.keys(totalIssued).length === 0
  ) {
    return null;
  }

  const [assetIn, assetOut] = pool.config.assets;
  const amount = poolAmount[pool.poolId.toString()];
  const amountIn = new BigNumber(
    amount ? amount[assetIn.getPicassoAssetId() as string] : "0"
  )
    .multipliedBy(lpRatio)
    .div(100)
    .toFormat(4);
  const amountOut = new BigNumber(
    amount ? amount[assetOut.getPicassoAssetId() as string] : "0"
  )
    .multipliedBy(lpRatio)
    .div(100)
    .toFormat(4);

  return (
    <BoxWrapper {...boxProps}>
      <Grid container>
        <Grid item {...twoColumnPageSize}>
          <Typography variant="h5">${userTVL.toFormat(2)}</Typography>
          <Typography variant="body1" color="text.secondary">
            Liquidity Provided
          </Typography>
        </Grid>
        <Grid container item {...twoColumnPageSize} spacing={3}>
          <Grid item {...twoColumnPageSize}>
            <Button
              variant="contained"
              size="large"
              fullWidth
              onClick={handleAddLiquidity}
            >
              Add liquidity
            </Button>
          </Grid>
          <Grid item {...twoColumnPageSize}>
            <Button
              disabled={lpTokens[pool.config.lpToken].balance.free.isZero()}
              variant="outlined"
              size="large"
              fullWidth
              onClick={handleRemoveLiquidity}
            >
              Remove liquidity
            </Button>
          </Grid>
        </Grid>
      </Grid>

      <Box mt={4}>
        <Grid container spacing={4}>
          <Grid item {...twoColumnPageSize}>
            <DonutChart
              data={[
                lpTokens[pool.config.lpToken].balance.free
                  .div(totalIssued[pool.poolId.toString()])
                  .multipliedBy(100)
                  .toNumber() || 0,
                new BigNumber(100)
                  .minus(
                    lpTokens[pool.config.lpToken].balance.free
                      .div(totalIssued[pool.poolId.toString()])
                      .multipliedBy(100)
                  )
                  .toNumber() || 100,
              ]}
              colors={[
                alpha(theme.palette.common.white, theme.custom.opacity.main),
                theme.palette.primary.main,
              ]}
              labels={DonutChartLabels}
              height={"249px"}
            />
          </Grid>
          <Grid item {...twoColumnPageSize}>
            <Box mt={8}>
              {" "}
              <Item value={amountIn}>
                <BaseAsset
                  label={`Pooled ${assetIn.getSymbol()}`}
                  icon={assetIn.getIconUrl()}
                />
              </Item>
              <Item value={amountOut} mt={4}>
                <BaseAsset
                  label={`Pooled ${assetOut.getSymbol()}`}
                  icon={assetOut.getIconUrl()}
                />
              </Item>
              <Item
                value={`${(
                  lpTokens[pool.config.lpToken].balance.free
                    .div(totalIssued[pool.poolId.toString()])
                    .multipliedBy(100)
                    .toNumber() || 0
                ).toFixed(2)}%`}
                mt={4}
              >
                <Typography variant="body1">Pool share</Typography>
              </Item>
            </Box>
          </Grid>
        </Grid>
      </Box>
    </BoxWrapper>
  );
};
