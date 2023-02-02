import { Grid } from "@mui/material";
import { HighlightBox } from "@/components/Organisms/Staking/HighlightBox";
import { FC } from "react";

export const StakingHighlights: FC = () => {
  const totalPicaDeposited = "20,325,651";
  const maximumXPICAAPR = "265%";
  const averageLockTime = "0";

  return (
    <Grid container spacing={4} mt={9} maxWidth={1032}>
      <Grid item xs={6} sm={4}>
        <HighlightBox
          title={"Total $PICA deposited"}
          value={totalPicaDeposited}
        />
      </Grid>
      <Grid item xs={6} sm={4}>
        <HighlightBox
          title={"Maximum xPICA APR"}
          value={maximumXPICAAPR}
        />
      </Grid>
      <Grid item xs={6} sm={4}>
        <HighlightBox
          title={"Average lock time"}
          value={averageLockTime}
        />
      </Grid>
    </Grid>
  );
};
