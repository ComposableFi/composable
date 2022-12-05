import { Grid } from "@mui/material";
import { HighlightBox } from "@/components/Organisms/Staking/HighlightBox";
import { FC } from "react";

export const StakingHighlights: FC = () => {
  const totalPicaLocked = "0";
  const totalChaosAPY = "0";
  const totalChaosMinted = "0";
  const averageLockMultiplier = "0";
  const averageLockTime = "0";

  return (
    <Grid container spacing={4} mt={9}>
      <Grid item xs={6} sm={4}>
        <HighlightBox
          tooltip={"Total pica locked"}
          title={"Total PICA locked"}
          value={totalPicaLocked}
        />
      </Grid>
      <Grid item xs={6} sm={4}>
        <HighlightBox
          tooltip={"Total xPICA APY"}
          title={"Total xPICA APY"}
          value={totalChaosAPY}
        />
      </Grid>
      <Grid item xs={6} sm={4}>
        <HighlightBox
          tooltip={"Total xPICA minted"}
          title={"Total xPICA minted"}
          value={totalChaosMinted}
        />
      </Grid>
      <Grid item xs={6}>
        <HighlightBox
          tooltip={"Average lock multiplier"}
          title={"Average lock multiplier"}
          value={averageLockMultiplier}
        />
      </Grid>
      <Grid item xs={6}>
        <HighlightBox
          tooltip={"Average lock time"}
          title={"Average lock time"}
          value={averageLockTime}
        />
      </Grid>
    </Grid>
  );
};
