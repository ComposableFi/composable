import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography, 
  useTheme, 
} from "@mui/material";
import { useAppSelector } from "@/hooks/store";
import { Label } from "@/components";
import { LiquidityBootstrappingPool, LiquidityBootstrappingPoolStats } from "@/store/pools/liquidityBootstrapping/liquidityBootstrapping.types";
import { getAssetById } from "@/defi/polkadot/Assets";

export type AuctionSettingsProps = {
  auction: LiquidityBootstrappingPool,
  stats: LiquidityBootstrappingPoolStats,
} & BoxProps;

export const AuctionSettings: React.FC<AuctionSettingsProps> = ({
  auction,
  stats,
  ...rest
}) => {

  const theme = useTheme();
  const baseAsset = getAssetById("picasso", auction.pair.base);
  const quoteAsset = getAssetById("picasso", auction.pair.quote);

  return (
    <Box {...rest}>
      <Typography variant="h6">
        Auction settings
      </Typography>
      <Grid container mt={4}>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Start weights
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${auction.sale.initialWeight}% ${baseAsset?.symbol} + ${100 - auction.sale.initialWeight}% ${quoteAsset?.symbol}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            End weights
          </Typography>
          <Typography variant="subtitle1" mt={1}>
          {`${auction.sale.finalWeight}% ${baseAsset?.symbol} + ${100 - auction.sale.finalWeight}% ${quoteAsset?.symbol}`}
          </Typography>
        </Grid>
      </Grid>
    
      {/* <Typography variant="body1" color="text.secondary" mt={4}>
        Enabled auction rights
      </Typography>
      <Grid container mt={1} columnGap={1.5} rowGap={2}>
        {
          rights.map((right) => (
            <Grid item key={right.name}>
              <Box 
                display="flex" 
                alignItems="center" 
                gap={1.75}
                sx={{
                  background: theme.palette.gradient.secondary,
                  height: 56,
                  paddingX: theme.spacing(3),
                  borderRadius: 99999,
                }}
              >
                <Label 
                  label={right.name}
                  TypographyProps={{variant: "subtitle1"}}
                  TooltipProps={{
                    title: right.description,
                  }}
                  mb={0}
                />  
              </Box>
            </Grid>
          ))
        }
      </Grid> */}

      <Grid container mt={4}>
        <Grid item xs={12} sm={12} md={3}>
          <Typography variant="body1" color="text.secondary">
            Total volume 
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {`${stats.totalVolume}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={9}>
          <Typography variant="body1" color="text.secondary">
            Trading fee (collected by {baseAsset?.symbol} project)
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {auction.fee} %
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
}