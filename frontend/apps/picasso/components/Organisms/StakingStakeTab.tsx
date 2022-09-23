import { Box, Button, CircularProgress, Typography, useTheme } from "@mui/material";
import React from "react";
import { Input, Modal } from "@/components";
import { TOKEN_IDS } from "tokens";

export type StakingStakeTabProps = {
  flow: "stake" | "unstake";
};

export const StakingStakeTab: React.FC<StakingStakeTabProps> = ({ flow }) => {
  const theme = useTheme();
  const [amount, setAmount] = React.useState<number>();
  const [approval, setApproval] = React.useState(false);
  const [isLoading, setIsLoading] = React.useState(false);

  const stake = () => {
  };

  const unstake = () => {
  };

  const onButtonClick = () => {
    if (approval) {
      flow === "stake" ? stake() : unstake();
    } else {
      setIsLoading(true);
      setApproval(true);
    }
  };

  const handleOnClick = () => {
  };

  return (
    <>
      <Box mb={4}>
        <Input
          value={amount}
          placeholder="0.0"
          tokenId={TOKEN_IDS[11]}
          buttonLabel="Max"
          onClick={handleOnClick}
          LabelProps={{
            mainLabelProps: { label: "Amount" },
            balanceLabelProps: {
              label: flow === "stake" ? "Balance:" : "Staked Balance:",
              balanceText: "200 xPICA"
            }
          }}
        />
      </Box>
      <Box>
        <Button
          onClick={onButtonClick}
          variant="contained"
          color="primary"
          fullWidth
          disabled={!amount}
        >
          <Typography variant="button">
            {flow === "stake" ? (!approval ? "Approve" : "Stake") : "Unstake"}
          </Typography>
        </Button>
      </Box>
      <Modal
        onClose={() => setIsLoading(false)}
        open={isLoading}
        maxWidth="md"
        dismissible
      >
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            flexDirection: "column",
            gap: theme.spacing(1)
          }}
        >
          <CircularProgress size={76} sx={{ mb: theme.spacing(8) }} />
          <Typography variant="h5">Confirming transaction</Typography>
          <Typography variant="body1" color="text.secondary">
            Confirm this transaction in your wallet.
          </Typography>
        </Box>
      </Modal>
    </>
  );
};
