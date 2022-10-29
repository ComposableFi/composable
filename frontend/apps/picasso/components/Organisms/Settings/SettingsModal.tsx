import { Modal } from "@/components";
import {
  alpha,
  Card,
  CircularProgress,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import React from "react";

export const SettingsModal = ({
  state,
  targetFeeItem,
  onClose,
}: {
  state: boolean;
  targetFeeItem: string;
  onClose: () => void;
}) => {
  const theme = useTheme();

  return (
    <Modal maxWidth="md" open={state} onClose={onClose} dismissible>
      <Card>
        <Stack direction="column" gap={4} alignItems="center">
          <CircularProgress size={64} />
          <Typography variant="h5" mt={8}>
            Changing gas token
          </Typography>
          <Typography
            variant="subtitle1"
            textAlign="center"
            color={alpha(theme.palette.common.white, 0.6)}
          >
            {`You're switching to ${targetFeeItem.toUpperCase()} for your gas token.
            This process typically takes about a minute to complete. Please
            confirm in your wallet.`}
          </Typography>
        </Stack>
      </Card>
    </Modal>
  );
};
