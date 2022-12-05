import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Box, Typography, useTheme, Button } from "@mui/material";
import { WarningAmberRounded } from "@mui/icons-material";
import useStore from "@/store/useStore";
import { setUiState } from "@/store/ui/ui.slice";

export const UnverifiedPoolWarningModal: React.FC<ModalProps> = ({
  ...modalProps
}) => {
  const theme = useTheme();
  const {
    createPool: { currentStep, setSelectable }
  } = useStore();

  const handleClose = () => {
    setUiState({ isConfirmingModalOpen: false });
  };

  const handleContinue = () => {
    handleClose();
    setSelectable({ currentStep: currentStep + 1 });
  };

  return (
    <Modal onClose={handleClose} {...modalProps}>
      <Box width={{ sm: 480 }} margin="auto">
        <Box textAlign="center">
          <WarningAmberRounded sx={{ fontSize: 96 }} />
        </Box>
        <Typography variant="h5" textAlign="center" mt={8}>
          Warning
        </Typography>
        <Typography
          variant="subtitle1"
          textAlign="center"
          color="text.secondary"
          mt={2}
        >
          This pool is unverified and therefore if there is no enough liquidity
          added to the pool, LP holders would lose their money. Do you wish to
          proceed?
        </Typography>

        <Box mt={8}>
          <Button
            variant="contained"
            fullWidth
            size="large"
            onClick={handleContinue}
          >
            Yes, continue
          </Button>
        </Box>

        <Box mt={4}>
          <Button variant="text" fullWidth size="large" onClick={handleClose}>
            No, take me back
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};
