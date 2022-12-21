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
import { PoolAmount } from "@/store/pools/types";
import { Asset } from "shared";

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
  poolShare: PoolAmount;
  assetOne: Asset;
  assetTwo: Asset;
  price: BigNumber;
  revertPrice: BigNumber;
  share: BigNumber;
} & BoxProps;

export const PoolShare: FC<PoolShareProps> = ({
  poolShare,
  assetOne,
  assetTwo,
  price,
  revertPrice,
  share,
  ...rest
}) => {
  // TODO:Implement pool share
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
          value={price.toFixed(2)}
          label={`${assetTwo.getSymbol()} per ${assetOne.getSymbol()}`}
        />
        <ItemBox
          value={revertPrice.toFixed(2)}
          label={`${assetOne.getSymbol()} per ${assetTwo.getSymbol()}`}
        />
        <ItemBox value={`${share.toFixed()}%`} label="Share of pool" />
      </Box>
    </Box>
  );
};
