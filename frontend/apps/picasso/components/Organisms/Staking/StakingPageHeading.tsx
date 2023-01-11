import { PageTitle } from "@/components";
import { Grid, useTheme } from "@mui/material";

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
      </Grid>
    </Grid>
  );
};
