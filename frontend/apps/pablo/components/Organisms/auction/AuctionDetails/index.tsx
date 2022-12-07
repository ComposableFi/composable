import { 
  Box, 
  BoxProps,
} from "@mui/material";
import { ContractDetails } from "./ContractDetails";
import { LaunchDescription } from "./LaunchDescription";
import { LaunchDetails } from "./LaunchDetails";
import { AuctionSettings } from "./AuctionSettings";
import { AuctionStatistics } from "./AuctionStatistics";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import { Asset, PabloLiquidityBootstrappingPool } from "shared";

export type AuctionDetailsProps = {
  auction: PabloLiquidityBootstrappingPool,
  baseAsset: Asset,
  quoteAsset: Asset,
  stats: LiquidityBootstrappingPoolStatistics,
} & BoxProps;

export const AuctionDetails: React.FC<AuctionDetailsProps> = ({
  auction,
  baseAsset,
  quoteAsset,
  stats,
  ...rest
}) => {

  return (
    <Box {...rest}>
      <ContractDetails auction={auction} baseAsset={baseAsset} />
      <LaunchDescription mt={8} />
      <LaunchDetails auction={auction} mt={8} />
      <AuctionStatistics stats={stats} mt={8} baseAsset={baseAsset} quoteAsset={quoteAsset} />
      <AuctionSettings stats={stats} auction={auction} mt={8} baseAsset={baseAsset} quoteAsset={quoteAsset} />
    </Box>
  );
}