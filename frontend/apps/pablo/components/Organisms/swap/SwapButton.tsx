import { useMemo } from "react";
import { Box, Button } from "@mui/material";
import { setUiState } from "@/store/ui/ui.slice";
import { useDotSamaContext, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useSwaps } from "@/defi/hooks";
import useStore from "@/store/useStore";

export const SwapButton = () => {
  const { extensionStatus } = useDotSamaContext();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { valid, assetOneAmount, assetTwoAmount } = useSwaps({
    selectedAccount,
  });
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
