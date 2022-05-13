import { getToken } from "@/defi/Tokens";
import { TokenId } from "@/defi/types";
import { 
  alpha, 
  Box, 
  BoxProps, 
  Typography, 
  useTheme, 
} from "@mui/material"

type PoolShareProps = {
  tokenId1: TokenId;
  tokenId2: TokenId;
  price: number;
  revertPrice: number;
  share: number;
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
            {price}
          </Typography>
          <Typography 
            variant="body2" 
            color="text.secondary"
            whiteSpace="nowrap"
          >
            {token2.symbol} per {token1.symbol}
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
            {revertPrice}
          </Typography>
          <Typography 
            variant="body2" 
            color="text.secondary"
            whiteSpace="nowrap"
          >
            {token1.symbol} per {token2.symbol}
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
            {share}%
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