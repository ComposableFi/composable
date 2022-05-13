import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography, 
  useTheme, 
} from "@mui/material";
import { LiquidityBootstrappingPool, LiquidityBootstrappingPoolStats } from "@/store/pools/liquidityBootstrapping/liquidityBootstrapping.types";
import { getAssetById } from "@/defi/polkadot/Assets";
import BigNumber from "bignumber.js";

export type AuctionStatisticsProps = {
  auction: LiquidityBootstrappingPool,
  stats: LiquidityBootstrappingPoolStats,
} & BoxProps;

export const AuctionStatistics: React.FC<AuctionStatisticsProps> = ({
  auction,
  stats,
  ...rest
}) => {
  const {
    currentBalances,
    startBalances,
    totalSold,
    totalRaised,
  } = stats;
  const baseAsset = getAssetById("picasso", auction.pair.base);
  const quoteAsset = getAssetById("picasso", auction.pair.quote);

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