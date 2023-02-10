import { Stack } from "@mui/material";
import { HighlightBox } from "@/components/Organisms/Staking/HighlightBox";
import { FC } from "react";
import { useStore } from "@/stores/root";

export const StakingHighlights: FC = () => {
  const totalPicaDeposited = useStore((state) => state.maximumPicaStaked);
  const maximumXPICAAPR = "265%";
  const averageLockTime = "0";

  return (
    <Stack direction="row" spacing={2} mt={9}>
      <HighlightBox
        flexGrow={1}
        title={"Maximum xPICA APR"}
        value={maximumXPICAAPR}
      />
      <HighlightBox
        title={"Total $PICA deposited"}
        value={totalPicaDeposited.toFormat()}
        flexGrow={1}
      />
      <HighlightBox
        flexGrow={1}
        title={"Average lock time"}
        value={averageLockTime}
      />
    </Stack>
  );
};
