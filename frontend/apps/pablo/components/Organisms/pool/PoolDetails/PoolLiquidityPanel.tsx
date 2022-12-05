import {
  Box,
  useTheme,
  Typography,
  Grid,
  alpha,
  Button,
  GridProps,
} from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import { BoxWrapper } from "../../BoxWrapper";
import { useRouter } from "next/router";
import { DonutChart } from "@/components/Atoms/DonutChart";
import { useLiquidityPoolDetails } from "@/defi/hooks/useLiquidityPoolDetails";
import { PoolDetailsProps } from "./index";
import { useRemoveLiquidityState } from "@/store/removeLiquidity/hooks";
import {
  setManualPoolSearch,
  setPool,
} from "@/store/addLiquidity/addLiquidity.slice";
import { useUserProvidedLiquidityByPool } from "@/defi/hooks/useUserProvidedLiquidityByPool";
import { calculatePoolTotalValueLocked } from "@/defi/utils";
import { useLiquidity } from "@/defi/hooks/useLiquidity";
import { useAssetIdOraclePrice, useLpTokenPrice, useLpTokenUserBalance } from "@/defi/hooks";

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

export const PoolLiquidityPanel: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const router = useRouter();
  const theme = useTheme();
  const { setRemoveLiquidity } = useRemoveLiquidityState();

  const poolDetails = useLiquidityPoolDetails(poolId);
  const { pool } = poolDetails;
  const { baseAmount, quoteAmount } = useLiquidity(pool);
  const lpTokenBalance = useLpTokenUserBalance(pool);
  const lpTokenPrice = useLpTokenPrice(pool?.getLiquidityProviderToken());
  const providedLiquidityAmount = useUserProvidedLiquidityByPool(poolId)

  const pair = pool?.getPair() ?? null;
  const base = pair?.getBaseAsset().toString() ?? "-";
  const quote = pair?.getQuoteAsset().toString() ?? "-";

  const baseAssetPriceUSD = useAssetIdOraclePrice(
    base
  );
  const quoteAssetPriceUSD = useAssetIdOraclePrice(
    quote
  );

  const handleAddLiquidity = () => {
    if (poolDetails.baseAsset && poolDetails.quoteAsset && poolDetails.pool) {
      setManualPoolSearch(false);
      setPool(poolDetails.pool);
      router.push("/pool/add-liquidity");
    }
  };

  const handleRemoveLiquidity = () => {
    if (poolDetails.baseAsset && poolDetails.quoteAsset) {
      setRemoveLiquidity({
        poolId,
      });
      router.push("/pool/remove-liquidity");
    }
  };

  const totalValueLocked = calculatePoolTotalValueLocked(
    quoteAmount,
    baseAmount,
    baseAssetPriceUSD,
    quoteAssetPriceUSD
  );

  const valueProvided = totalValueLocked.minus(
    lpTokenPrice.times(lpTokenBalance)
  );

  return (
    <BoxWrapper {...boxProps}>
      <Grid container>
        <Grid item {...twoColumnPageSize}>
          <Typography variant="h5">{`$${valueProvided.toFormat(
            2
          )}`}</Typography>
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
              disabled={lpTokenBalance.eq(0)}
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
              data={[valueProvided.toNumber(), totalValueLocked.minus(valueProvided).toNumber()]}
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
              {poolDetails.baseAsset && (
                <Item
                  value={providedLiquidityAmount.tokenAmounts.baseAmount.toFormat(2)}
                >
                  <BaseAsset
                    label={`Pooled ${poolDetails.baseAsset.getSymbol()}`}
                    icon={poolDetails.baseAsset.getIconUrl()}
                  />
                </Item>
              )}
              {poolDetails.quoteAsset && (
                <Item
                  value={providedLiquidityAmount.tokenAmounts.quoteAmount.toFormat(2)}
                  mt={4}
                >
                  <BaseAsset
                    label={`Pooled ${poolDetails.quoteAsset.getSymbol()}`}
                    icon={poolDetails.quoteAsset.getIconUrl()}
                  />
                </Item>
              )}
              <Item
                value={`${
                  totalValueLocked.eq(0)
                    ? "0"
                    : valueProvided
                        .div(totalValueLocked)
                        .times(100)
                        .toFixed(2)
                }%`}
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
