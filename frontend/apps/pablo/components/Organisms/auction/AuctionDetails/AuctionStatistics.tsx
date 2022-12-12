import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography
} from "@mui/material";
import { Asset } from "shared";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";

export type AuctionStatisticsProps = {
  baseAsset: Asset,
  quoteAsset: Asset,
  stats: LiquidityBootstrappingPoolStatistics,
} & BoxProps;

export const AuctionStatistics: React.FC<AuctionStatisticsProps> = ({
  baseAsset,
  quoteAsset,
  stats,
  ...rest
}) => {
  const {
    startLiquidity,
    liquidity,
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
            {`${startLiquidity.baseAmount.toFixed(DEFAULT_UI_FORMAT_DECIMALS)} ${baseAsset.getSymbol()}`}
          </Typography>
          <Typography variant="subtitle1">
            {`${startLiquidity.quoteAmount.toFixed(DEFAULT_UI_FORMAT_DECIMALS)} ${quoteAsset.getSymbol()}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Current balances
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${liquidity.baseAmount.toFixed(DEFAULT_UI_FORMAT_DECIMALS)} ${baseAsset.getSymbol()}`}
          </Typography>
          <Typography variant="subtitle1">
            {`${liquidity.quoteAmount.toFixed(DEFAULT_UI_FORMAT_DECIMALS)} ${quoteAsset.getSymbol()}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Total sold
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${startLiquidity.baseAmount.minus(liquidity.baseAmount).toFixed(4)} ${baseAsset.getSymbol()}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Total raised
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${liquidity.quoteAmount.minus(startLiquidity.quoteAmount).toFixed(4)} ${quoteAsset.getSymbol()}`}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
}