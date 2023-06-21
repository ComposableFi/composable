import { FC, useMemo } from "react";
import { Box, Button } from "@mui/material";
import { setUiState } from "@/store/ui/ui.slice";
import { useDotSamaContext } from "substrate-react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export const SwapButton: FC<{
  valid: boolean;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
}> = ({ valid, assetOneAmount, assetTwoAmount }) => {
  const { extensionStatus } = useDotSamaContext();
  const selectedPool = useStore((store) => store.swaps.selectedPool);

  const buttonText = useMemo(() => {
    if (extensionStatus !== "connected") {
      return "Connect wallet";
    }
    if (!selectedPool) {
      return "Pool not found";
    }
    return "Swap";
  }, [extensionStatus, selectedPool]);

  const shouldDisableButton =
    extensionStatus !== "connected" ||
    !valid ||
    !selectedPool ||
    assetOneAmount.isZero() ||
    assetOneAmount.isNaN() ||
    assetTwoAmount.isZero() ||
    assetTwoAmount.isNaN();

  const handleButtonClick = () => {
    if (extensionStatus !== "connected") {
      setUiState({ isPolkadotModalOpen: true });
    } else {
      setUiState({ isSwapPreviewModalOpen: true });
    }
  };

  return (
    <Box mt={4}>
      <Button
        onClick={handleButtonClick}
        variant="contained"
        fullWidth
        disabled={shouldDisableButton}
      >
        {buttonText}
      </Button>
    </Box>
  );
};
