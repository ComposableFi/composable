import { Box, BoxProps, Typography, useTheme, Grid } from "@mui/material";
import { getFullHumanizedDateDiff } from "shared";
import { nFormatter } from "shared";
import { useMemo } from "react";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import { useAuctionSpotPrice } from "@/defi/hooks/auctions";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import BigNumber from "bignumber.js";
import AccessTimeRoundedIcon from "@mui/icons-material/AccessTimeRounded";
import useBlockNumber from "@/defi/hooks/useBlockNumber";

export type AuctionInformationProps = {
  auction: LiquidityBootstrappingPool;
  baseAsset?: MockedAsset;
  quoteAsset?: MockedAsset;
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
            {quoteAsset?.symbol}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
};
