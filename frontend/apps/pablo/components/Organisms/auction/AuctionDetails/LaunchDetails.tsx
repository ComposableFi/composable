import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography
} from "@mui/material";
import { AuctionStatusIndicator } from "../AuctionStatusIndicator";
import { PabloLiquidityBootstrappingPool } from "shared";
import { useAuctionTiming } from "@/defi/hooks/auctions/useAuctionTiming";
import moment from "moment-timezone";

export type LaunchDetailsProps = {
  auction: PabloLiquidityBootstrappingPool,
} & BoxProps;

export const LaunchDetails: React.FC<LaunchDetailsProps> = ({
  auction,
  ...rest
}) => {
  const { isActive, isEnded, startTimestamp, endTimestamp } = useAuctionTiming(auction);
  const getStatusLabel = () => {
    return isActive ? 'Active' : (isEnded ? 'Ended' : 'Starting soon');
  };

  return (
    <Box {...rest}>
      <Typography variant="h6">
        Launch Details
      </Typography>
      <Grid container mt={4}>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Status
          </Typography>
          <AuctionStatusIndicator 
            auction={auction} 
            label={getStatusLabel()} 
            LabelProps={{variant: "subtitle1"}}
            mt={1} 
          />
        </Grid>
        <Grid item xs={12} sm={12} md={5}>
          <Typography variant="body1" color="text.secondary">
            Start Date
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {moment(startTimestamp).utc().format("MMM DD, YYYY, h:mm A z")}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={4}>
          <Typography variant="body1" color="text.secondary">
            End Date
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {moment(endTimestamp).utc().format("MMM DD, YYYY, h:mm A z")}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
}