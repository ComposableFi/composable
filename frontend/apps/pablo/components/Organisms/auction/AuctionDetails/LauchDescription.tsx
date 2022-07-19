import { LiquidityBootstrappingPool } from "@/defi/types";
import { 
  Box, 
  BoxProps, 
  Typography, 
} from "@mui/material";

export type LaunchDescritionProps = {
  auction: LiquidityBootstrappingPool,
} & BoxProps;

export const LaunchDescrition: React.FC<LaunchDescritionProps> = ({
  auction,
  ...rest
}) => {
  return (
    <Box {...rest}>
      <Typography variant="h6">
        Launch description
      </Typography>
      {
        auction.auctionDescription.map((paragraph, index) => (
          <Typography variant="subtitle1" color="text.secondary" mt={4} key={index}>
            {paragraph}
          </Typography>
        ))
      }
    </Box>
  );
}