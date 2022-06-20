import { 
  Box, 
  BoxProps, 
  Grid, 
  Typography, 
  useTheme,
} from "@mui/material";
import { BaseAsset, Link } from "@/components";
import ContentCopyOutlinedIcon from '@mui/icons-material/ContentCopyOutlined';
import OpenInNewRoundedIcon from "@mui/icons-material/OpenInNewRounded";
import { getAssetById } from "@/defi/polkadot/Assets";
import { useCallback } from "react";
import { LiquidityBootstrappingPool } from "@/store/pools/pools.types";

export type ContractDetailsProps = {
  auction: LiquidityBootstrappingPool,
} & BoxProps;

export const ContractDetails: React.FC<ContractDetailsProps> = ({
  auction,
  ...rest
}) => {

  const theme = useTheme();
  const asset = getAssetById("picasso", auction.pair.base);

  const getTokenLink = () => {
    return ``;
  }

  const copyTokenAddress = useCallback(() => {
    navigator.clipboard.writeText("");
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [auction.poolId]);

  return (
    <Box {...rest}>
      <Grid container>
        <Grid item xs={12} sm={12} md={6}>
          <BaseAsset 
            icon={asset?.icon} 
            label={`${asset?.name} Token`} 
            LabelProps={{variant: "h6"}}
          />
          <Typography variant="body1" color="text.secondary" mt={4}>
            Token owner address
          </Typography>
          <Box display="flex" alignItems="center" gap={2} mt={1}>
            <Typography variant="subtitle1">
              {auction.owner.substring(0, 6) + '...' + auction.owner.substring(auction.owner.length - 4, auction.owner.length)}
            </Typography>
            <ContentCopyOutlinedIcon
              onClick={copyTokenAddress}
              color="primary"
              sx={{transform: 'scaleY(-1)', cursor: 'pointer'}}
            />
          </Box>

        </Grid>
        <Grid item xs={12} sm={12} md={6} pl={5.75}>
          <Box
            borderRadius={1}
            sx={{
              p: 4,
              background: theme.palette.gradient.secondary,
            }}
          >
            <Typography variant="body1" color="text.secondary">
              Token contract address
            </Typography>
            <Box display="flex" alignItems="center" gap={1.5} mt={2}>
              <Typography variant="subtitle1">
                {`${asset?.symbol} on polkascan`}
              </Typography>
              <Link href={getTokenLink()} target="_blank">
                <OpenInNewRoundedIcon />
              </Link>
            </Box>
            <Box display="flex" alignItems="center" gap={1.5} mt={2}>
              <Typography variant="subtitle1" >
                {`Auction Owner on polkascan`}
              </Typography>
              <Link href={""} target="_blank">
                <OpenInNewRoundedIcon />
              </Link>
            </Box>
            <Box display="flex" alignItems="center" gap={1.5} mt={2}>
              <Typography variant="subtitle1">
                {`Token Launch Auction Documentation`}
              </Typography>
              <Link href={""} target="_blank">
                <OpenInNewRoundedIcon />
              </Link>
            </Box>
          </Box>
        </Grid>
      </Grid>
    </Box>
  );
}