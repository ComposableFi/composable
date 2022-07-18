import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography
} from "@mui/material";
import { AuctionStatusIndicator } from "../AuctionStatusIndicator";
import moment from "moment-timezone";
import { LiquidityBootstrappingPool } from "@/defi/types";

export type LaunchDetailsProps = {
  auction: LiquidityBootstrappingPool,
} & BoxProps;

export const LaunchDetails: React.FC<LaunchDetailsProps> = ({
  auction,
  ...rest
}) => {

  const currentTimestamp = Date.now();
  const isActive: boolean = auction.sale.start <= currentTimestamp 
                    && auction.sale.end >= currentTimestamp;
  const isEnded: boolean = auction.sale.end < currentTimestamp;

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
            {moment(auction.sale.start).utc().format("MMM DD, YYYY, h:mm A z")}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={4}>
          <Typography variant="body1" color="text.secondary">
            End Date
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {moment(auction.sale.end).utc().format("MMM DD, YYYY, h:mm A z")}
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
}