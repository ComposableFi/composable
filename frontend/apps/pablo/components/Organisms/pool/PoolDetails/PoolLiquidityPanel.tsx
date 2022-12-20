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
import { pipe } from "fp-ts/lib/function";
import { option } from "fp-ts";
import { BaseAsset } from "@/components";
import BigNumber from "bignumber.js";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  value: string;
} & GridProps;

const Item: React.FC<ItemProps> = ({ value, children, ...gridProps }) => {
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

function getPercentage(a: number, b: number) {
  const aValue = a || 0;
  const bValue = b || 100;

  const a_percent = (aValue / bValue) * 100;
  const b_percent = 100 - a_percent;

  return [a_percent, b_percent];
}

export const PoolLiquidityPanel: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const router = useRouter();
  const theme = useTheme();
  const getPoolById = useStore((store) => store.pools.getPoolById);
  const poolAmount = useStore((store) => store.pools.poolAmount);
  const pool = getPoolById(poolId);
  const lpTokens = useStore((store) => store.ownedLiquidity.tokens);
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const totalIssued = useStore((store) => store.pools.totalIssued);

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
  )
    return null;

  return pipe(
    pool,
    option.fold(
      () => null,
      (p) => {
        const [assetIn, assetOut] = p.config.assets;
        const amount = poolAmount[p.poolId.toString()];
        const amountIn = amount
          ? amount[assetIn.getPicassoAssetId() as string]
          : "0";
        const amountOut = amount
          ? amount[assetOut.getPicassoAssetId() as string]
          : "0";
        return (
          <BoxWrapper {...boxProps}>
            <Grid container>
              <Grid item {...twoColumnPageSize}>
                <Typography variant="h5">
                  {lpTokens[p.config.lpToken].balance.free.toFormat(4)}
                </Typography>
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
                    disabled={lpTokens[p.config.lpToken].balance.free.isZero()}
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
                      lpTokens[p.config.lpToken].balance.free
                        .div(totalIssued[p.poolId.toString()])
                        .multipliedBy(100)
                        .toNumber() || 0,
                      new BigNumber(100)
                        .minus(
                          lpTokens[p.config.lpToken].balance.free
                            .div(totalIssued[p.poolId.toString()])
                            .multipliedBy(100)
                        )
                        .toNumber() || 100,
                    ]}
                    colors={[
                      alpha(
                        theme.palette.common.white,
                        theme.custom.opacity.main
                      ),
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
                        lpTokens[p.config.lpToken].balance.free
                          .div(totalIssued[p.poolId.toString()])
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
      }
    )
  );
};
