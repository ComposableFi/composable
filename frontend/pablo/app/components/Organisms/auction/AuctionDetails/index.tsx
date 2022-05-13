import { 
  Box, 
  BoxProps, 
  useTheme, 
} from "@mui/material";
import { ContractDetails } from "./ContractDetails";
import { LaunchDescrition } from "./LauchDescription";
import { LaunchDetails } from "./LaunchDetails";
import { AuctionSettings } from "./AuctionSettings";
import { AuctionStatistics } from "./AuctionStatistics";
import { LiquidityBootstrappingPool, LiquidityBootstrappingPoolStats } from "@/store/pools/liquidityBootstrapping/liquidityBootstrapping.types";

export type AuctionDetailsProps = {
  auction: LiquidityBootstrappingPool,
  stats: LiquidityBootstrappingPoolStats,
} & BoxProps;

export const AuctionDetails: React.FC<AuctionDetailsProps> = ({
  auction,
  stats,
  ...rest
}) => {
  const currentTimestamp = Date.now();

  return (
    <Box {...rest}>
      <ContractDetails auction={auction} />
      <LaunchDescrition auction={auction} mt={8} />
      <LaunchDetails auction={auction} mt={8} />
      <AuctionStatistics auction={auction} stats={stats} mt={8} />
      <AuctionSettings stats={stats} auction={auction} mt={8} />
    </Box>
  );
}