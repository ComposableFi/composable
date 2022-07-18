import { TokenValue } from "@/components/Molecules";
import { TOKENS } from "@/defi/Tokens";
import { useAppSelector } from "@/hooks/store";
import { alpha, Box, BoxProps, Button, Grid, Theme, useTheme } from "@mui/material";
import { BoxWrapper } from "../BoxWrapper";

const defaultPageSize = {
  sm: 12,
  md: 4,
};

const defaultTokenValueProps = (theme: Theme) => ({
  justifyContent: "space-between",
  borderRadius: 0.5,
  px: 3,
  py: 2.25,
  sx: {
    background: alpha(theme.palette.common.white, theme.custom.opacity.lighter),
  }
} as const);

export const ClaimableRewards: React.FC<BoxProps> = ({
  ...boxProps
}) => {

  const theme = useTheme();

  const {
    ksm,
    pica,
    pablo,
  } = useAppSelector((state) => state.polkadot.claimableRewards);

  return (
    <BoxWrapper
      title="Claimable rewards"
      {...boxProps}
    >
      <Grid container spacing={3}>
        <Grid item {...defaultPageSize}>
          <TokenValue
            token={TOKENS.ksm}
            value={ksm.toFormat()}
            {...defaultTokenValueProps(theme)}
          />
        </Grid>
        <Grid item {...defaultPageSize}>
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
        </Grid>
      </Grid>
      <Box mt={3}>
        <Button variant="outlined" fullWidth size="large">
          Claim all
        </Button>
      </Box>
    </BoxWrapper>
  );
};
