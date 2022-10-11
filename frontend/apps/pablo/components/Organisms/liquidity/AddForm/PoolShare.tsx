import { MockedAsset } from "@/store/assets/assets.types";
import { alpha, Box, BoxProps, Typography, useTheme } from "@mui/material";
import BigNumber from "bignumber.js";

type PoolShareProps = {
  baseAsset: MockedAsset | undefined;
  quoteAsset: MockedAsset | undefined;
  price: BigNumber;
  revertPrice: BigNumber;
  share: number;
} & BoxProps;

export const PoolShare: React.FC<PoolShareProps> = ({
  baseAsset,
  quoteAsset,
  price,
  revertPrice,
  share,
  ...rest
}) => {
  const theme = useTheme();

  return (
    <Box mt={4} {...rest}>
      <Typography variant="inputLabel">Price and pool share</Typography>
      <Box
        display="flex"
        gap={4}
        mt={1.5}
        sx={{
          [theme.breakpoints.down("sm")]: {
            flexDirection: "column",
          },
        }}
      >
        <Box
          sx={{
            background: alpha(
              theme.palette.common.white,
              theme.custom.opacity.lighter
            ),
            borderRadius: 1,
            padding: theme.spacing(1.875, 1),
            textAlign: "center",
            width: "100%",
          }}
        >
          <Typography variant="body1" mb={0.5}>
            {price.toFixed(4)}
          </Typography>
          <Typography
            variant="body2"
            color="text.secondary"
            whiteSpace="nowrap"
          >
            {baseAsset?.symbol} per {quoteAsset?.symbol}
          </Typography>
        </Box>

        <Box
          sx={{
            background: alpha(
              theme.palette.common.white,
              theme.custom.opacity.lighter
            ),
            borderRadius: 1,
            padding: theme.spacing(1.875, 1),
            textAlign: "center",
            width: "100%",
          }}
        >
          <Typography variant="body1" mb={0.5}>
            {revertPrice.toFixed(4)}
          </Typography>
          <Typography
            variant="body2"
            color="text.secondary"
            whiteSpace="nowrap"
          >
            {quoteAsset?.symbol} per {baseAsset?.symbol}
          </Typography>
        </Box>

        <Box
          sx={{
            background: alpha(
              theme.palette.common.white,
              theme.custom.opacity.lighter
            ),
            borderRadius: 1,
            padding: theme.spacing(1.875, 1),
            textAlign: "center",
            width: "100%",
          }}
        >
          <Typography variant="body1" mb={0.5}>
            {share.toFixed(4)}%
          </Typography>
          <Typography
            variant="body2"
            color="text.secondary"
            whiteSpace="nowrap"
          >
            Share of pool
          </Typography>
        </Box>
      </Box>
    </Box>
  );
};
