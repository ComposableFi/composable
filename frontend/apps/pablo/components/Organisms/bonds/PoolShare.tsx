import { getToken } from "@/defi/Tokens";
import { TokenId } from "@/defi/types";
import {
  alpha,
  Box,
  BoxProps,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";

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
  tokenId1: TokenId;
  tokenId2: TokenId;
  price: BigNumber;
  revertPrice: BigNumber;
  share: BigNumber;
} & BoxProps;

export const PoolShare: React.FC<PoolShareProps> = ({
  tokenId1,
  tokenId2,
  price,
  revertPrice,
  share,
  ...rest
}) => {
  const theme = useTheme();
  const token1 = getToken(tokenId1);
  const token2 = getToken(tokenId2);
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
          value={price.toFixed()}
          label={`${token2.symbol} per ${token1.symbol}`}
        />
        <ItemBox
          value={revertPrice.toFixed()}
          label={`${token1.symbol} per ${token2.symbol}`}
        />
        <ItemBox value={`${share.toFixed()}%`} label="Share of pool" />
      </Box>
    </Box>
  );
};
