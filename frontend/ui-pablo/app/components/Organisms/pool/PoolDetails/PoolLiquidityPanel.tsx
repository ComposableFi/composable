import {
  Box,
  useTheme,
  Typography,
  BoxProps,
  Grid,
  alpha,
  Button,
  GridProps,
} from "@mui/material";
import { useAppSelector } from "@/hooks/store";
import { BaseAsset } from "@/components/Atoms";
import { TOKENS } from "@/defi/Tokens";
import { BoxWrapper } from "../../BoxWrapper";
import { useRouter } from "next/router";
import { DonutChart } from "@/components/Atoms/DonutChart";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  value: string,
} & GridProps;

const Item: React.FC<ItemProps> = ({
  value,
  children,
  ...gridProps
}) => {
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

export const PoolLiquidityPanel: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const router = useRouter();
  const theme = useTheme();
  const {
    tokenId1,
    tokenId2,
    pooledAmount1,
    pooledAmount2,
    share,
    amount,
  } = useAppSelector(
    (state) => state.pool.currentLiquidity
  );

  const donutChartData = useAppSelector(
    (state) => state.pool.selectedPoolLiquidityChartData
  );

  const validToken1 = tokenId1 !== "none";
  const validToken2 = tokenId2 !== "none";

  const handleAddLiquidity = () => {
    router.push("/pool/add-liquidity");
  };

  const handleRemoveLiquidity = () => {
    router.push("/pool/remove-liquidity");
  };

  return (
    <BoxWrapper {...boxProps}>
      <Grid container>
        <Grid item {...twoColumnPageSize}>
          <Typography variant="h5">
            {`$${amount.toFormat()}`}
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Liquidity
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
              data={donutChartData.series}
              colors={[
                alpha(theme.palette.common.white, theme.custom.opacity.main),
                theme.palette.primary.main,
              ]}
              labels={donutChartData.labels}
              height={"249px"}
            />
          </Grid>
          <Grid item {...twoColumnPageSize}>
            <Box mt={8}>
              {validToken1 && (
                <Item value={pooledAmount1.toFormat()}>
                  <BaseAsset
                    label={`Pooled ${TOKENS[tokenId1].symbol}`}
                    icon={TOKENS[tokenId1].icon}
                  />
                </Item>
              )}
              {validToken2 && (
                <Item value={pooledAmount2.toFormat()} mt={4}>
                  <BaseAsset
                    label={`Pooled ${TOKENS[tokenId2].symbol}`}
                    icon={TOKENS[tokenId2].icon}
                  />
                </Item>
              )}
              <Item value={`${share.toFormat()}%`} mt={4}>
                <Typography variant="body1">
                  Pool share
                </Typography>
              </Item>
            </Box>
          </Grid>
        </Grid>
      </Box>
    </BoxWrapper>
  );
};

