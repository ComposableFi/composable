import { Stack } from "@mui/material";
import { HighlightBox } from "@/components/Organisms/Staking/HighlightBox";
import { FC } from "react";
import { useStore } from "@/stores/root";

export const StakingHighlights: FC = () => {
  const totalPicaDeposited = useStore((state) => state.maximumPicaStaked);
  const maximumXPICAAPR = "~";
  const averageLockDuration = useStore((state) => state.averageStakingLockTime);

  return (
    <Stack direction="row" spacing={2} mt={9}>
      <HighlightBox flexGrow={1} title={"xPICA APR"} value={maximumXPICAAPR} />
      <HighlightBox
        title={"Total $PICA deposited"}
        value={totalPicaDeposited.toFormat()}
        flexGrow={1}
      />
      <HighlightBox
        flexGrow={1}
        title={"Average lock time"}
        value={averageLockDuration}
      />
    </Stack>
  );
};
