import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography, 
  useTheme, 
} from "@mui/material";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";

export type AuctionSettingsProps = {
  auction: LiquidityBootstrappingPool,
  stats: LiquidityBootstrappingPoolStatistics,
  baseAsset: MockedAsset | undefined,
  quoteAsset: MockedAsset | undefined,
} & BoxProps;

export const AuctionSettings: React.FC<AuctionSettingsProps> = ({
  auction,
  baseAsset,
  quoteAsset,
  stats,
  ...rest
}) => {

  const theme = useTheme();

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
            {`${stats.totalVolume.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}`}
          </Typography>
        </Grid>
        <Grid item xs={12} sm={12} md={9}>
          <Typography variant="body1" color="text.secondary">
            Trading fee (collected by {baseAsset?.symbol} project)
          </Typography>
          <Typography variant="subtitle1" mt={1}>
            {auction.feeConfig.feeRate} %
          </Typography>
        </Grid>
      </Grid>
    </Box>
  );
}