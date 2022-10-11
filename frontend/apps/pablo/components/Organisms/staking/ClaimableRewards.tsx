import { TokenValue } from "@/components/Molecules";
import { BoxWrapper } from "../BoxWrapper";
import { useClaimableRewards } from "@/defi/hooks/stakingRewards";
import { DEFAULT_UI_FORMAT_DECIMALS, PBLO_ASSET_ID } from "@/defi/utils";
import {
  alpha,
  Box,
  BoxProps,
  Button,
  Grid,
  Theme,
  useTheme,
} from "@mui/material";

const defaultPageSize = {
  sm: 12,
  md: 4,
};

const defaultTokenValueProps = (theme: Theme) =>
  ({
    justifyContent: "space-between",
    borderRadius: 1,
    px: 3,
    py: 2.25,
    sx: {
      background: alpha(
        theme.palette.common.white,
        theme.custom.opacity.lighter
      ),
    },
  } as const);

export const ClaimableRewards: React.FC<BoxProps> = ({ ...boxProps }) => {
  const theme = useTheme();

  const claimableAssets = useClaimableRewards({ stakedAssetId: PBLO_ASSET_ID });

  return (
    <BoxWrapper title="Claimable rewards" {...boxProps}>
      <Grid container spacing={3}>
        {claimableAssets.map((asset) => {
          return (
            <Grid key={asset.symbol} item {...defaultPageSize}>
              <TokenValue
                token={asset}
                value={asset.claimable.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}
                {...defaultTokenValueProps(theme)}
              />
            </Grid>
          );
        })}
        {/* <Grid item {...defaultPageSize}>
          <TokenValue
            token={TOKENS.pica}
            value={pica.toFormat()}
            {...defaultTokenValueProps(theme)}
          />
        </Grid>
        <Grid item {...defaultPageSize}>
          <TokenValue
            token={TOKENS.pablo}
            value={pablo.toFormat()}
            {...defaultTokenValueProps(theme)}
          />
        </Grid> */}
      </Grid>
      <Box mt={3}>
        <Button variant="outlined" fullWidth size="large">
          Claim all
        </Button>
      </Box>
    </BoxWrapper>
  );
};
