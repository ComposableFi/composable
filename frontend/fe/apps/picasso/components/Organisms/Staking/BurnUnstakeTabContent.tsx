import { BurnCheckboxList } from "@/components/Organisms/Staking/BurnCheckboxList";
import { BurnModal } from "@/components/Organisms/Staking/BurnModal";
import { RenewModal } from "@/components/Organisms/Staking/RenewModal";
import { Box } from "@mui/material";
import { FC, useState } from "react";

export const BurnUnstakeTabContent: FC = () => {
  const [unstakeToken, setUnstakeToken] =
    useState<string | undefined>(undefined);
  const [isBurnModalOpen, setIsBurnModalOpen] = useState<boolean>(false);
  const [isRenewModalOpen, setIsRenewModalOpen] = useState<boolean>(false);

  return (
    <Box>
      <BurnCheckboxList
        openBurnModal={() => setIsBurnModalOpen(true)}
        openRenewModal={() => setIsRenewModalOpen(true)}
        onSelectUnstakeToken={(v) => setUnstakeToken(v)}
        unstakeTokenId={unstakeToken}
      />
      <BurnModal
        open={isBurnModalOpen}
        onClose={() => setIsBurnModalOpen(false)}
      />
      <RenewModal
        open={isRenewModalOpen}
        onClose={() => setIsRenewModalOpen(false)}
      />
    </Box>
  );
};
