import { Box, BoxProps, Typography, useTheme, Grid } from "@mui/material";
import AccessTimeRoundedIcon from "@mui/icons-material/AccessTimeRounded";
import { getFullHumanizedDateDiff } from "@/utils/date";
import { LiquidityBootstrappingPool, LiquidityBootstrappingPoolStats } from "@/store/pools/pools.types";
import { nFormatter } from "@/utils/number";
import BigNumber from "bignumber.js";
import { getAssetById } from "@/defi/polkadot/Assets";
import { useMemo } from "react";
import { useAuctionSpotPrice } from "@/store/pools/hooks";

export type AuctionInformationProps = {
  auction: LiquidityBootstrappingPool,
  stats: LiquidityBootstrappingPoolStats,
} & BoxProps;

export const AuctionInformation: React.FC<AuctionInformationProps> = ({
  auction,
  stats,
  ...rest
}) => {
  const theme = useTheme();
  const currentTimestamp = Date.now();
  const isActive: boolean = auction.sale.start <= currentTimestamp 
                    && auction.sale.end >= currentTimestamp;
  const isEnded: boolean = auction.sale.end < currentTimestamp;

  const standardPageSize = {
    xs: 12,
    sm: 6,
    md: 3,
  };

  const getTimeLabel = () => {
    return isActive ? "Ends in" : isEnded ? "Ended" : "Starts in";
  };

  const getTime = () => {
    return (
      isActive
        ? getFullHumanizedDateDiff(Date.now(), auction.sale.end)
        : (
            isEnded
              ? "-"
              : "Not started"
        )
    );
  };

  const spotPrice = useAuctionSpotPrice(auction.poolId);
  let tokenRaised = useMemo(() => {
    return new BigNumber(stats.currentBalances.quote).minus(stats.startBalances.quote);
  }, [stats.currentBalances.quote, stats.startBalances.quote]);
  
  let tokenSold = useMemo(() => {
    return new BigNumber(stats.startBalances.base).minus(stats.currentBalances.base);
  }, [stats.startBalances.base, stats.currentBalances.base])

  let totalBase = useMemo(() => {
    return new BigNumber(stats.startBalances.base);
  }, [stats.startBalances.base]);

  let soldPercentage = useMemo(() => {
    const { base } = stats.startBalances;
    if (base === "0") return "0";

    return tokenSold.div(new BigNumber(base)).times(100).toFixed(2)
  }, [stats.startBalances.base, tokenSold]);

  return (
    <Box {...rest}>
      <Grid container rowGap={6}>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Duration
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              {auction.sale.duration} days
            </Typography>
            <AccessTimeRoundedIcon />
          </Box> 
        </Grid>
        <Grid item {...standardPageSize} >
          <Typography variant="body1" color="text.secondary">
            { getTimeLabel() }
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              { getTime() }
            </Typography>
            <AccessTimeRoundedIcon />
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Total Volume
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">{stats.totalVolume}</Typography>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Liquidity
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              ${ stats.liquidity } 
            </Typography> 
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Price
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              ${ spotPrice }
            </Typography>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Tokens sold
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              { soldPercentage }%
            </Typography>
          </Box>
          <Typography variant="body1" color="text.secondary" fontWeight="bold">
            {nFormatter(
              tokenSold.toNumber()
            )} of {nFormatter(totalBase.toNumber())}
          </Typography>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Funds raised
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">
              { tokenRaised.toFixed(4) }
            </Typography>
          </Box>
          <Typography variant="body1" color="text.secondary" fontWeight="bold">
            {getAssetById("picasso", auction.pair.quote)?.symbol}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
};
