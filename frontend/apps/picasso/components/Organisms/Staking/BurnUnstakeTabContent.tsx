import { BurnCheckboxList } from "@/components/Organisms/Staking/BurnCheckboxList";
import { BurnModal } from "@/components/Organisms/Staking/BurnModal";
import { RenewModal } from "@/components/Organisms/Staking/RenewModal";
import { Box } from "@mui/material";
import { FC, useState } from "react";

export const BurnUnstakeTabContent: FC = () => {
  const [selectedToken, setSelectedToken] = useState<[string, string]>([
    "",
    ""
  ]);
  const [isBurnModalOpen, setIsBurnModalOpen] = useState<boolean>(false);
  const [isRenewModalOpen, setIsRenewModalOpen] = useState<boolean>(false);

  return (
    <Box>
      <BurnCheckboxList
        openBurnModal={() => setIsBurnModalOpen(true)}
        openRenewModal={() => setIsRenewModalOpen(true)}
        onSelectUnstakeToken={(collection, instance) =>
          setSelectedToken((prev) => {
            if (prev[0] === collection && prev[1] === instance) {
              return ["", ""];
            }
            return [collection, instance];
          })
        }
        unstakeTokenId={selectedToken}
      />
      <BurnModal
        open={isBurnModalOpen}
        selectedToken={selectedToken}
        onClose={() => setIsBurnModalOpen(false)}
      />
      <RenewModal
        open={isRenewModalOpen}
        selectedToken={selectedToken}
        onClose={() => setIsRenewModalOpen(false)}
      />
    </Box>
  );
};
