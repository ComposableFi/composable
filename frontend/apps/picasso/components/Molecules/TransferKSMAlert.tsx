import { Alert, Typography } from "@mui/material";

export const TransferKSMAlert = () => (
  <Alert variant="filled" color="error">
    <Typography variant="body2">
      To make this transfer, you need at least 0.01 KSM in your wallet.
    </Typography>
  </Alert>
);
