import {
  alpha,
  Box,
  BoxProps,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { FC } from "react";
import { PoolConfig } from "@/store/pools/types";
import { Asset } from "shared";
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
  simulated: BigNumber;
} & BoxProps;

export const PoolShare: FC<PoolShareProps> = ({
  pool,
  input,
  amounts,
  simulated,
  ...rest
}) => {
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const [assetOne, assetTwo] = input;
  const [amountOne, amountTwo] = amounts;
  const { spotPrice } = usePoolSpotPrice(pool, input);
  const twoPerOne = BigNumber(1).div(spotPrice).isFinite()
    ? BigNumber(1).div(spotPrice).toFixed(4)
    : "0.0000";
  const onePerTwo =
    spotPrice.isFinite() && !spotPrice.isNaN()
      ? spotPrice.toFormat(4)
      : "0.0000";

  const totalIssued = useStore((store) => store.pools.totalIssued);

  const shareOfPool = simulated
    .div(totalIssued[pool.poolId.toString()].plus(simulated))
    .multipliedBy(100);

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
        <ItemBox value={`${shareOfPool.toFixed(4)}%`} label="Share of pool" />
      </Box>
    </Box>
  );
};
