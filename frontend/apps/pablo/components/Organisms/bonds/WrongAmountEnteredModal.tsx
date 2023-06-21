import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Box, Typography, useTheme, Button } from "@mui/material";
import { WarningAmberRounded } from "@mui/icons-material";
import { setUiState } from "@/store/ui/ui.slice";

export const WrongAmountEnteredModal: React.FC<ModalProps> = (props) => {
  const theme = useTheme();
  
  const handleCancel = () => {
    setUiState({isWrongAmountEnteredModalOpen: false })
  };

  const handleConfirm = () => {
    handleCancel();
    setUiState({ isOpenPreviewPurchaseModal: true })
  };


  return (
    <Modal {...props} onClose={handleCancel} dismissible>
      <Box
        textAlign="center"
        sx={{
          width: 560,
          [theme.breakpoints.down("sm")]: {
            width: "100%",
          },
          padding: theme.spacing(3),
        }}
      >
        <Box display="flex" justifyContent="center">
          <WarningAmberRounded sx={{ fontSize: 96 }} />
        </Box>
        <Typography variant="h5" mt={8}>
          Warning
        </Typography>
        <Typography variant="body1" mt={2} color="text.secondary">
          This bond is currently at a negative discount. Please consider waiting
          until bond returns to positive discount.
        </Typography>
        <Button
          onClick={handleCancel}
          sx={{ marginTop: 8 }}
          fullWidth
          variant="contained"
        >{`OK, I'll Wait`}</Button>
        <Button onClick={handleConfirm} sx={{ marginTop: 4 }} fullWidth>
          I want to burn money
        </Button>
      </Box>
    </Modal>
  );
};
