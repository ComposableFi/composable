import { getAsset } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { 
  alpha, 
  Box, 
  BoxProps, 
  Typography, 
  useTheme, 
} from "@mui/material"
import BigNumber from "bignumber.js";

type PoolShareProps = {
  baseAsset: AssetId;
  quoteAsset: AssetId;
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
  const bAsset = getAsset(baseAsset);
  const qAsset = getAsset(quoteAsset);

  return (
    <Box mt={4} {...rest}>
      <Typography variant="inputLabel">Price and pool share</Typography>
      <Box 
        display="flex"
        gap={4}
        mt={1.5}
        sx={{
          [theme.breakpoints.down('sm')]: {
            flexDirection: 'column',
          }
        }}
      >
        
        <Box
          sx={{
            background: alpha(theme.palette.common.white, theme.custom.opacity.lighter),
            borderRadius: 0.666,
            padding: theme.spacing(1.875, 1),
            textAlign: "center",
            width: '100%',
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
            {bAsset?.symbol} per {qAsset?.symbol}
          </Typography>
        </Box>
      
        <Box
          sx={{
            background: alpha(theme.palette.common.white, theme.custom.opacity.lighter),
            borderRadius: 0.666,
            padding: theme.spacing(1.875, 1),
            textAlign: "center",
            width: '100%',
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
            {qAsset?.symbol} per {bAsset?.symbol}
          </Typography>
        </Box>
      
        <Box
          sx={{
            background: alpha(theme.palette.common.white, theme.custom.opacity.lighter),
            borderRadius: 0.666,
            padding: theme.spacing(1.875, 1),
            textAlign: "center",
            width: '100%',
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
  )
}