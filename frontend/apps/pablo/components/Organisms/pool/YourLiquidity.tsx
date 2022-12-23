import { alpha, Box, Grid, Typography, useTheme } from "@mui/material";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { Link } from "@/components";
import useStore from "@/store/useStore";
import { YourLiquidityTable } from "@/components/Organisms/pool/YourLiquidityTable";

export const YourLiquidity = () => {
  const theme = useTheme();
  const pools = useStore((store) => store.pools.config);

  return (
    <Grid>
      <Grid item xs={12}>
        <HighlightBox>
          <Box
            display="flex"
            mb={3}
            justifyContent="space-between"
            alignItems="center"
          >
            <Typography variant="h6">Your Liquidity</Typography>
          </Box>
          <YourLiquidityTable pools={pools} />
        </HighlightBox>
        <Box mt={4} display="none" gap={1} justifyContent="center">
          <Typography
            textAlign="center"
            variant="body2"
            color={alpha(
              theme.palette.common.white,
              theme.custom.opacity.darker
            )}
          >
            {`Don't see a pool you joined?`}
          </Typography>
          <Link href="/pool/import" key="import">
            <Typography
              textAlign="center"
              variant="body2"
              color="primary"
              sx={{ cursor: "pointer" }}
            >
              Import it.
            </Typography>
          </Link>
        </Box>
      </Grid>
    </Grid>
  );
};
