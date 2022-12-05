import { Box, BoxProps, Typography, useTheme, Grid } from "@mui/material";
import { getFullHumanizedDateDiff, PabloLiquidityBootstrappingPool } from "shared";
import { nFormatter } from "shared";
import { useMemo } from "react";
import { useAuctionSpotPrice } from "@/defi/hooks/auctions";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import { useAssetIdOraclePrice } from "@/defi/hooks";
import { useAuctionTiming } from "@/defi/hooks/auctions/useAuctionTiming";
import { Asset } from "shared";
import BigNumber from "bignumber.js";
import AccessTimeRoundedIcon from "@mui/icons-material/AccessTimeRounded";

export type AuctionInformationProps = {
  auction: PabloLiquidityBootstrappingPool;
  baseAsset: Asset;
  quoteAsset: Asset;
  stats: LiquidityBootstrappingPoolStatistics;
} & BoxProps;

export const AuctionInformation: React.FC<AuctionInformationProps> = ({
  auction,
  baseAsset,
  quoteAsset,
  stats,
  ...rest
}) => {
  const theme = useTheme();
  const { isActive, isEnded, duration, endTimestamp } = useAuctionTiming(auction);
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
      return getFullHumanizedDateDiff(Date.now(), endTimestamp);
    } else if (isEnded) {
      return "-";
    } else {
      return "Not started";
    }
  };

  const spotPrice = useAuctionSpotPrice(auction ? (auction.getPoolId(true) as BigNumber).toNumber() : -1);
  const quoteAssetPrice = useAssetIdOraclePrice(auction ? auction.getPair().getQuoteAsset().toString() : "-");

  let {
    tokenRaised,
    soldPercentage,
    liquidityInUSD,
    totalVolume,
    nFormattedBaseTokenSold,
    nFormattedStartBaseToken,
  } = useMemo(() => {
    let tokenRaised = new BigNumber(stats.liquidity.quoteAmount)
      .minus(stats.startLiquidity.quoteAmount)
      .toFixed(DEFAULT_UI_FORMAT_DECIMALS);

    let tokenSold = new BigNumber(stats.startLiquidity.baseAmount)
      .minus(stats.liquidity.baseAmount)
      .toFixed(DEFAULT_UI_FORMAT_DECIMALS);

    let startBase = new BigNumber(stats.startLiquidity.baseAmount);
    let soldPercentageBn = !startBase.eq(0)
      ? new BigNumber(tokenSold).div(new BigNumber(startBase)).times(100)
      : new BigNumber(0);

    let soldPercentage = soldPercentageBn.toFixed(DEFAULT_UI_FORMAT_DECIMALS);

    let liquidityInUSD = stats.totalLiquidity
      .times(quoteAssetPrice)
      .toFixed(DEFAULT_UI_FORMAT_DECIMALS);

    let totalVolume = stats.totalVolume
      .times(quoteAssetPrice)
      .toFixed(DEFAULT_UI_FORMAT_DECIMALS);

    let nFormattedBaseTokenSold = nFormatter(
      new BigNumber(tokenSold).toNumber()
    );
    let nFormattedStartBaseToken = nFormatter(
      new BigNumber(stats.startLiquidity.baseAmount).toNumber()
    );

    return {
      nFormattedBaseTokenSold,
      nFormattedStartBaseToken,
      totalVolume,
      tokenRaised,
      tokenSold,
      soldPercentage,
      liquidityInUSD,
    };
  }, [
    stats.liquidity.quoteAmount,
    stats.liquidity.baseAmount,
    stats.startLiquidity.quoteAmount,
    stats.startLiquidity.baseAmount,
    stats.totalLiquidity,
    stats.totalVolume,
    quoteAssetPrice,
  ]);

  const baseAssetAuctionPoolPrice = useMemo(() => {
    return spotPrice.times(quoteAssetPrice).toFixed(DEFAULT_UI_FORMAT_DECIMALS);
  }, [spotPrice, quoteAssetPrice]);

  return (
    <Box {...rest}>
      <Grid container rowGap={6}>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Duration
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">{duration} days</Typography>
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
            <Typography variant="h6">{totalVolume}</Typography>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Liquidity
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">${liquidityInUSD}</Typography>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Price
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">${baseAssetAuctionPoolPrice}</Typography>
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
            {nFormattedBaseTokenSold} of {nFormattedStartBaseToken}
          </Typography>
        </Grid>
        <Grid item {...standardPageSize}>
          <Typography variant="body1" color="text.secondary">
            Funds raised
          </Typography>
          <Box display="flex" alignItems="center" gap={1.75}>
            <Typography variant="h6">{tokenRaised}</Typography>
          </Box>
          <Typography variant="body1" color="text.secondary" fontWeight="bold">
            {quoteAsset?.getSymbol()}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
};
