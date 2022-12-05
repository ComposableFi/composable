import { Box, BoxProps, Grid, Typography, useTheme } from "@mui/material";
import { BaseAsset, Link } from "@/components";
import { useCallback } from "react";
import { Asset, PabloLiquidityBootstrappingPool } from "shared";
import ContentCopyOutlinedIcon from "@mui/icons-material/ContentCopyOutlined";
import OpenInNewRoundedIcon from "@mui/icons-material/OpenInNewRounded";

export type ContractDetailsProps = {
  auction: PabloLiquidityBootstrappingPool;
  baseAsset: Asset;
} & BoxProps;

export const ContractDetails: React.FC<ContractDetailsProps> = ({
  auction,
  baseAsset,
  ...rest
}) => {
  const theme = useTheme();

  const getTokenLink = () => {
    return ``;
  };

  const owner = auction.getOwner();
  const copyTokenAddress = useCallback(() => {
    navigator.clipboard.writeText(owner);
  }, [owner]);

  return (
    <Box {...rest}>
      <Grid container>
        <Grid item xs={12} sm={12} md={6}>

            <BaseAsset
              icon={baseAsset.getIconUrl()}
              label={`${baseAsset.getName()} Token`}
              LabelProps={{ variant: "h6" }}
            />

          <Typography variant="body1" color="text.secondary" mt={4}>
            Token owner address
          </Typography>
          <Box display="flex" alignItems="center" gap={2} mt={1}>
            <Typography variant="subtitle1">
              {owner.substring(0, 6) +
                "..." +
                owner.substring(
                  owner.length - 4,
                  owner.length
                )}
            </Typography>
            <ContentCopyOutlinedIcon
              onClick={copyTokenAddress}
              color="primary"
              sx={{ transform: "scaleY(-1)", cursor: "pointer" }}
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
                {`${baseAsset.getSymbol()} on polkascan`}
              </Typography>
              <Link href={getTokenLink()} target="_blank">
                <OpenInNewRoundedIcon />
              </Link>
            </Box>
            <Box display="flex" alignItems="center" gap={1.5} mt={2}>
              <Typography variant="subtitle1">
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
};
