import { Grid } from "@mui/material";
import { PageTitle } from "@/components";

export const StakingPageHeading = () => (
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
