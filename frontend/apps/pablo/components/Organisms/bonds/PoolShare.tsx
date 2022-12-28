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
import { getPriceAndRatio, getStats, GetStatsReturn } from "@/defi/utils";
import useStore from "@/store/useStore";
import { usePoolSpotPrice } from "@/defi/hooks/pools/usePoolSpotPrice";

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

const ItemBox: FC<ItemBoxProps> = ({ value, label }) => {
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

  const [assetOne, assetTwo] = input;
  const [amountOne, amountTwo] = amounts;
  const { spotPrice } = usePoolSpotPrice(pool, input);

  if (!stats) return <CircularProgress />;

  const { shareOfPool } = getPriceAndRatio(
    stats,
    assetOne,
    amountOne,
    amountTwo,
    assetTwo
  );

  const onePerTwo = BigNumber(1).div(spotPrice).isFinite() ?
    BigNumber(1).div(spotPrice).toFixed(4) :
    "0.0000";
  const twoPerOne = spotPrice.isFinite() && !spotPrice.isNaN() ?
    spotPrice.toFormat(4) : "0.0000";
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
          value={onePerTwo}
          label={`${assetOne.getSymbol()} per ${assetTwo.getSymbol()}`}
        />
        <ItemBox
          value={twoPerOne}
          label={`${assetTwo.getSymbol()} per ${assetOne.getSymbol()}`}
        />
        <ItemBox value={`${shareOfPool.toFixed()}%`} label="Share of pool" />
      </Box>
    </Box>
  );
};
