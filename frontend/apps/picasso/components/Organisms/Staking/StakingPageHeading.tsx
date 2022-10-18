import { Link, PageTitle } from "@/components";
import { alpha, Box, Grid, Typography, useTheme } from "@mui/material";

export const StakingPageHeading = () => {
  const theme = useTheme();
  return (
    <Grid container>
      <Grid item xs={12}>
        <PageTitle
          title="Staking"
          subtitle="Lock PICA to mint xPICA, the yield bearing governance fNFT."
          textAlign="center"
        />
        <Box display="flex" alignItems="center" justifyContent="center" mt={3}>
          <Box
            sx={{
              padding: theme.spacing(2.25, 4),
              backgroundColor: alpha(theme.palette.common.white, 0.1),
              borderRadius: theme.spacing(1.5),
            }}
          >
            <Typography variant="body2">
              Staking will be available soon. For more information do check{" "}
              <Link href="https://docs.composable.finance">
                docs.composable.finance
              </Link>
            </Typography>
          </Box>
        </Box>
      </Grid>
    </Grid>
  );
};
