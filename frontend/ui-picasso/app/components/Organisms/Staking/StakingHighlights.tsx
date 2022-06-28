import { Grid } from "@mui/material";
import { HighlightBox } from "@/components/Organisms/Staking/HighlightBox";
import { FC } from "react";
import { useStore } from "@/stores/root";

export const StakingHighlights: FC = () => {
  const {
    totalPicaLocked,
    totalChaosAPY,
    totalChaosMinted,
    averageLockMultiplier,
    averageLockTime,
  } = useStore(({ staking }) => staking.highlights);

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
          tooltip={"Total CHAOS APY"}
          title={"Total CHAOS APY"}
          value={totalChaosAPY}
        />
      </Grid>
      <Grid item xs={6} sm={4}>
        <HighlightBox
          tooltip={"Total CHAOS minted"}
          title={"Total CHAOS minted"}
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
