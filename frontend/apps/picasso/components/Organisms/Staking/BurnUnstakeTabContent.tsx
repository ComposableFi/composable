import { BurnCheckboxList } from "@/components/Organisms/Staking/BurnCheckboxList";
import { BurnModal } from "@/components/Organisms/Staking/BurnModal";
import { Box } from "@mui/material";
import { FC, useState } from "react";

export const BurnUnstakeTabContent: FC = () => {
  const [selectedToken, setSelectedToken] = useState<[string, string]>([
    "",
    "",
  ]);
  const [isBurnModalOpen, setIsBurnModalOpen] = useState<boolean>(false);

  return (
    <Box>
      <BurnCheckboxList
        openBurnModal={() => setIsBurnModalOpen(true)}
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
    </Box>
  );
};
