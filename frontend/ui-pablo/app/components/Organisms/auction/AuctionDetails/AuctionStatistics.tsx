import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography, 
  useTheme, 
} from "@mui/material";
import BigNumber from "bignumber.js";
import { LiquidityBootstrappingPoolStats } from "@/store/pools/pools.types";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/defi/types";

export type AuctionStatisticsProps = {
  auction: LiquidityBootstrappingPool,
  baseAsset: MockedAsset | undefined,
  quoteAsset: MockedAsset | undefined,
  stats: LiquidityBootstrappingPoolStats,
} & BoxProps;

export const AuctionStatistics: React.FC<AuctionStatisticsProps> = ({
  auction,
  baseAsset,
  quoteAsset,
  stats,
  ...rest
}) => {
  const {
    currentBalances,
    startBalances,
  } = stats;

  return (
    <Box {...rest}>
      <Typography variant="h6">
        Auction Statistics
      </Typography>
      <Grid container mt={4}>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Start balances
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${startBalances.base} ${baseAsset?.symbol}`}
          </Typography>
          <Typography variant="subtitle1">
            {`${startBalances.quote} ${quoteAsset?.symbol}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Current balances
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${currentBalances.base} ${baseAsset?.symbol}`}
          </Typography>
          <Typography variant="subtitle1">
            {`${currentBalances.quote} ${quoteAsset?.symbol}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Total sold
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${new BigNumber(startBalances.base).minus(currentBalances.base).toFixed(4)} ${baseAsset?.symbol}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Total raised
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${new BigNumber(currentBalances.quote).minus(startBalances.quote).toFixed(4)} ${quoteAsset?.symbol}`}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
}