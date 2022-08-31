import { 
  Box, 
  BoxProps,
} from "@mui/material";
import { ContractDetails } from "./ContractDetails";
import { LaunchDescription } from "./LaunchDescription";
import { LaunchDetails } from "./LaunchDetails";
import { AuctionSettings } from "./AuctionSettings";
import { AuctionStatistics } from "./AuctionStatistics";
import { LiquidityBootstrappingPoolStats } from "@/store/pools/pools.types";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/defi/types";

export type AuctionDetailsProps = {
  auction: LiquidityBootstrappingPool,
  baseAsset?: MockedAsset,
  quoteAsset?: MockedAsset,
  stats: LiquidityBootstrappingPoolStats,
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
      <LaunchDescription auction={auction} mt={8} />
      <LaunchDetails auction={auction} mt={8} />
      <AuctionStatistics auction={auction} stats={stats} mt={8} baseAsset={baseAsset} quoteAsset={quoteAsset} />
      <AuctionSettings stats={stats} auction={auction} mt={8} baseAsset={baseAsset} quoteAsset={quoteAsset} />
    </Box>
  );
}