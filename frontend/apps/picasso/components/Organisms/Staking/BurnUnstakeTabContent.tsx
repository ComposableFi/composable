import { BurnCheckboxList } from "@/components/Organisms/Staking/BurnCheckboxList";
import { BurnModal } from "@/components/Organisms/Staking/BurnModal";
import { Box } from "@mui/material";
import { FC, useState } from "react";
import { useStore } from "@/stores/root";
import { SplitModal } from "./SplitModal";

export const BurnUnstakeTabContent: FC = () => {
  const [selectedToken, setSelectedToken] = useState<[string, string]>([
    "",
    "",
  ]);
  const isBurnModalOpen = useStore(
    (state) => state.ui.stakingRewards.isBurnModalOpen
  );
  const setIsBurnModalOpen = useStore((state) => state.ui.setBurnModalState);
  const setSplitModalOpen = useStore((state) => state.ui.setSplitModalState);
  const isSplitModalOpen = useStore(
    (state) => state.ui.stakingRewards.isSplitModalOpen
  );

  return (
    <Box>
      <BurnCheckboxList
        openSplitModal={() => setSplitModalOpen(true)}
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
      <SplitModal
        open={isSplitModalOpen}
        selectedToken={selectedToken}
        onClose={() => setSplitModalOpen(false)}
      />
    </Box>
  );
};
