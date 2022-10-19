import { PageTitle } from "@/components";
import { UnavailableFeature } from "@/components/Molecules/UnavailableFeature";
import { Grid } from "@mui/material";

export const StakingPageHeading = () => {
  return (
    <Grid container>
      <Grid item xs={12}>
        <PageTitle
          title="Staking"
          subtitle="Lock PICA to mint xPICA, the yield bearing governance fNFT."
          textAlign="center"
        />
        <UnavailableFeature pageTitle={"Staking"} />
      </Grid>
    </Grid>
  );
};
