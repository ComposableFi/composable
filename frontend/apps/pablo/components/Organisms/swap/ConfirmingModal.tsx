import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import {
  Box,
  Typography,
  useTheme,
} from "@mui/material";
import { CircularProgress } from "@/components/Atoms";

export const ConfirmingModal: React.FC<ModalProps> = ({
  ...modalProps
}) => {
  const theme = useTheme();

  return (
    <Modal
      {...modalProps}
    >
      <Box
        textAlign="center"
        sx={{
          width: 560,
          [theme.breakpoints.down('sm')]: {
            width: '100%',
          },
          padding: theme.spacing(3)
        }}
      >
        <Box display="flex" justifyContent="center">
          <CircularProgress size={96} />
        </Box>
        <Typography variant="h5" mt={8}>
          Confirming transaction
        </Typography>
        <Typography
          variant="body1"
          mt={2}
          color="text.secondary"
        >
          Confirming this transaction in your wallet.
        </Typography>
      </Box>
    </Modal>
  );
};

