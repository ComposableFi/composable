import React, { FC } from "react";
import { CircularProgress } from "@/components/Atoms";
import { Modal, ModalProps } from "@/components/Molecules";
import { alpha, Box, Typography, useTheme } from "@mui/material";
import { SupplyModalProps } from "./ConfirmSupplyModal";
import { setUiState } from "@/store/ui/ui.slice";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

export const ConfirmingSupplyModal: FC<SupplyModalProps & ModalProps> = ({
  pool,
  inputConfig,
  expectedLP,
  amountTwo,
  amountOne,
  ...rest
}) => {
  const theme = useTheme();
  const assetOne = pool.config.assets[0];
  const assetTwo = pool.config.assets[1];

  const handelClose = () => {
    setUiState({ isConfirmingSupplyModalOpen: false });
  };

  return (
    <Modal onClose={() => handelClose()} {...rest}>
      <Box
        textAlign="center"
        sx={{
          width: 550,
          [theme.breakpoints.down("sm")]: {
            width: "100%",
          },
          padding: theme.spacing(3),
        }}
      >
        <Box display="flex" justifyContent="center">
          <CircularProgress size={96} />
        </Box>
        <Typography variant="h5" mt={8}>
          Waiting for confirmation
        </Typography>
        <Typography variant="subtitle1" mt={2} color="text.secondary">
          Adding{" "}
          {`${amountOne.toFormat(assetOne.getDecimals(DEFAULT_NETWORK_ID))}`}{" "}
          {assetOne.getSymbol()} and{" "}
          {`${amountTwo.toFormat(assetTwo.getDecimals(DEFAULT_NETWORK_ID))}`}{" "}
          {assetTwo.getSymbol()}
        </Typography>
        <Typography
          variant="body1"
          mt={2}
          sx={{
            color: alpha(theme.palette.common.white, theme.custom.opacity.main),
          }}
        >
          Confirming this transaction in your wallet
        </Typography>
      </Box>
    </Modal>
  );
};
