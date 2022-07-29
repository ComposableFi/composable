import { Box, BoxProps, Typography, useTheme, Grid } from "@mui/material";
import AccessTimeRoundedIcon from "@mui/icons-material/AccessTimeRounded";
import { getFullHumanizedDateDiff } from "shared";
import {
  LiquidityBootstrappingPoolStats,
} from "@/store/pools/pools.types";
import { nFormatter } from "shared";
import BigNumber from "bignumber.js";
import { useMemo } from "react";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import { useAuctionSpotPrice } from "@/defi/hooks/auctions";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/defi/types";
import useBlockNumber from "@/defi/hooks/useBlockNumber";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

export type AuctionInformationProps = {
  auction: LiquidityBootstrappingPool;
  baseAsset?: MockedAsset;
  quoteAsset?: MockedAsset;
  stats: LiquidityBootstrappingPoolStats;
} & BoxProps;

export const AuctionInformation: React.FC<AuctionInformationProps> = ({
  auction,
  baseAsset,
  quoteAsset,
  stats,
  ...rest
}) => {
  const theme = useTheme();

  const currentBlock = useBlockNumber(DEFAULT_NETWORK_ID);
  const isActive: boolean =
    currentBlock.gte(auction.sale.startBlock) &&
    currentBlock.lte(auction.sale.endBlock);
  const isEnded: boolean = currentBlock.gt(auction.sale.endBlock);

  const standardPageSize = {
    xs: 12,
    sm: 6,
    md: 3,
  };

  const getTimeLabel = () => {
    return isActive ? "Ends in" : isEnded ? "Ended" : "Starts in";
  };

  const getTime = () => {
    if (isActive) {
      return getFullHumanizedDateDiff(Date.now(), auction.sale.end);
    } else if (isEnded) {
      return "-";
    } else {
      return "Not started";
    }
  };

  const spotPrice = useAuctionSpotPrice(auction.poolId);
  const quoteAssetPrice = useUSDPriceByAssetId(auction.pair.quote.toString());

  let tokenRaised = useMemo(() => {
    return new BigNumber(stats.currentBalances.quote).minus(
      stats.startBalances.quote
    );
  }, [stats.currentBalances.quote, stats.startBalances.quote]);

  let tokenSold = useMemo(() => {
    return new BigNumber(stats.startBalances.base).minus(
      stats.currentBalances.base
    );
  }, [stats.startBalances.base, stats.currentBalances.base]);

  let totalBase = useMemo(() => {
    return new BigNumber(stats.startBalances.base);
  }, [stats.startBalances.base]);

  let soldPercentage = useMemo(() => {
    const { base } = stats.startBalances;
    if (base === "0") return "0";

    return tokenSold.div(new BigNumber(base)).times(100).toFixed(2);
  }, [stats.startBalances, tokenSold]);

  return (
    <Box {...rest}>
      <Grid container rowGap={6}>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Duration
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">{auction.sale.duration} days</Typography>
            <AccessTimeRoundedIcon />
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            {getTimeLabel()}
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">{getTime()}</Typography>
            <AccessTimeRoundedIcon />
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Total Volume
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              {new BigNumber(stats.totalVolume)
                .times(quoteAssetPrice)
                .toFixed(2)}
            </Typography>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Liquidity
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              $
              {new BigNumber(stats.liquidity).times(quoteAssetPrice).toFixed(2)}
            </Typography>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Price
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              ${new BigNumber(quoteAssetPrice).times(spotPrice).toFixed(2)}
            </Typography>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Tokens sold
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">{soldPercentage}%</Typography>
          </Box>
          <Typography variant="body1" color="text.secondary" fontWeight="bold">
            {nFormatter(tokenSold.toNumber())} of{" "}
            {nFormatter(totalBase.toNumber())}
          </Typography>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Funds raised
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">{tokenRaised.toFixed(4)}</Typography>
          </Box>
          <Typography variant="body1" color="text.secondary" fontWeight="bold">
            {quoteAsset?.symbol}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
};
