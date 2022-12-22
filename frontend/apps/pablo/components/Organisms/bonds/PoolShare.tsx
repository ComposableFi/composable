import {
  alpha,
  Box,
  BoxProps,
  CircularProgress,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { FC, useEffect, useState } from "react";
import { PoolConfig } from "@/store/pools/types";
import { Asset } from "shared";
import { getStats, GetStatsReturn } from "@/defi/utils";
import useStore from "@/store/useStore";

const itemBoxPropsSX = (theme: Theme) =>
  ({
    background: alpha(theme.palette.common.white, theme.custom.opacity.lighter),
    borderRadius: 1,
    padding: theme.spacing(1.875, 1),
    textAlign: "center",
    width: "100%",
  } as const);

const valueTypographyProps = {
  variant: "body1",
  mb: 0.5,
  fontWeight: 600,
} as const;

const labelTypographyProps = {
  variant: "body2",
  color: "text.secondary",
  whiteSpace: "nowrap",
} as const;

type ItemBoxProps = {
  value: string;
  label: string;
};

const ItemBox: React.FC<ItemBoxProps> = ({ value, label }) => {
  const theme = useTheme();
  return (
    <Box sx={itemBoxPropsSX(theme)}>
      <Typography {...valueTypographyProps}>{value}</Typography>
      <Typography {...labelTypographyProps}>{label}</Typography>
    </Box>
  );
};

export type PoolShareProps = {
  pool: PoolConfig;
  input: [Asset, Asset];
  amounts: [BigNumber, BigNumber];
} & BoxProps;

export const PoolShare: FC<PoolShareProps> = ({
  pool,
  input,
  amounts,
  ...rest
}) => {
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const [stats, setStats] = useState<GetStatsReturn>(null);
  useEffect(() => {
    if (isPoolsLoaded && pool) {
      getStats(pool).then((result) => {
        setStats(result);
      });
    }
  }, [isPoolsLoaded, pool]);

  const [assetLeft, assetRight] = input;
  const [amountLeft, amountRight] = amounts;
  if (!stats) return <CircularProgress />;

  const spotPriceOfATOB = stats[
    assetLeft.getPicassoAssetId().toString()
  ].spotPrice.isZero()
    ? amountLeft.div(amountRight).isNaN()
      ? new BigNumber(0)
      : amountLeft.div(amountRight)
    : stats[assetLeft.getPicassoAssetId().toString()].spotPrice;
  const spotPriceOfBToA = stats[
    assetRight.getPicassoAssetId().toString()
  ].spotPrice.isZero()
    ? amountRight.div(amountLeft).isNaN()
      ? new BigNumber(0)
      : amountRight.div(amountLeft)
    : stats[assetRight.getPicassoAssetId().toString()].spotPrice;
  const totalLiquidityA =
    stats[assetLeft.getPicassoAssetId().toString()].total.liquidity;
  const totalLiquidityB =
    stats[assetRight.getPicassoAssetId().toString()].total.liquidity;
  const ratioA = totalLiquidityA.isZero()
    ? 100
    : amountLeft.div(totalLiquidityA).multipliedBy(100).toNumber();
  const ratioB = totalLiquidityB.isZero()
    ? 100
    : amountRight.div(totalLiquidityB).multipliedBy(100).toNumber();

  return (
    <Box mt={4} {...rest}>
      <Typography variant="inputLabel">Price and pool share</Typography>
      <Box
        display="flex"
        gap={4}
        mt={1.5}
        flexDirection={{ sm: "column", md: "row" }}
      >
        <ItemBox
          value={spotPriceOfATOB.toFixed(2)}
          label={`${assetLeft.getSymbol()} per ${assetRight.getSymbol()}`}
        />
        <ItemBox
          value={spotPriceOfBToA.toFixed(2)}
          label={`${assetRight.getSymbol()} per ${assetLeft.getSymbol()}`}
        />
        <ItemBox
          value={`${((ratioA + ratioB) / 2).toFixed()}%`}
          label="Share of pool"
        />
      </Box>
    </Box>
  );
};
