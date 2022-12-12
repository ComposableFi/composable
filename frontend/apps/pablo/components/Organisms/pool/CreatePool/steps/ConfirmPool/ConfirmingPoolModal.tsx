import React from "react";
import { CircularProgress } from "@/components/Atoms";
import { ModalProps, Modal } from "@/components/Molecules";
import {
  alpha,
  Box,
  Typography,
  useTheme,
} from "@mui/material";
import { useUiSlice } from "@/store/ui/ui.slice";

type ConfirmingPoolModalProps = {
  poolName: string | null,
  isFunding?: boolean,
} & Omit<ModalProps, "open">;

export const ConfirmingPoolModal: React.FC<ConfirmingPoolModalProps> = ({
  poolName,
  isFunding = false,
  ...modalProps
}) => {

  const theme = useTheme();
  const { isConfirmingModalOpen } = useUiSlice();
  const isModalOpen = isConfirmingModalOpen;

  return (
    <Modal
      open={isModalOpen}
      maxWidth="sm"
      {...modalProps}
    >

      <Box textAlign="center">
        <Box display="flex" justifyContent="center">
          <CircularProgress size={96} />
        </Box>
        <Typography variant="h5" mt={8}>
          Waiting for confirmation
        </Typography>
        <Typography variant="subtitle1" mt={2} color="text.secondary">
          {isFunding ? "Funding" : "Creating"} pool {poolName}
        </Typography>
        <Typography
          variant="body1"
          mt={2}
          sx={{
            color: alpha(theme.palette.common.white, theme.custom.opacity.main),
          }}
        >
          Confirm this transaction in your wallet
        </Typography>
      </Box>
    </Modal>
  );
};

