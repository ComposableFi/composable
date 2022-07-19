import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import {
  Box,
  Typography,
  useTheme,
  Button,
  Grid,
} from "@mui/material";

import { useDispatch } from "react-redux";
import { XPablo } from "@/defi/types";
import { Label } from "@/components/Atoms";
import { TokenValueItem } from "../TokenValueItem";
import { TOKENS } from "@/defi/Tokens";
import { setMessage } from "@/stores/ui/uiSlice";

export type UnstakeModalProps = {
  xPablo: XPablo,
} & ModalProps;

export const UnstakeModal: React.FC<UnstakeModalProps> = ({
  xPablo,
  onClose,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const handleUntake = () => {
    dispatch(setMessage(
      {
        title: "Transaction successfull",
        text: "Burn and unstake confirmed",
        link: "/",
        severity: "success",
      }
    ));
    onClose?.({}, "backdropClick");
  };

  return (
    <Modal
      maxWidth="lg"
      onClose={onClose}
      {...modalProps}
    >
      <Box width={{md: 968}} margin="auto">
        <Typography variant="h5" textAlign="center" mt={8}>
          Burn and unstake you position
        </Typography>

        <Box mt={7}>
          <Grid container spacing={3}>
            <Grid item sm={12} md={6}>
              <Label
                label="Withdrawable PBLO"
                TypographyProps={{color: "text.secondary"}}
                TooltipProps={{
                  title: "Withdrawable PBLO",
                }}
              />
              <TokenValueItem
                token={TOKENS.pablo}
                value={xPablo.withdrawableAmount.toFormat()}
              />
            </Grid>

            <Grid item sm={12} md={6}>
              <Label
                label="Initial PBLO deposit"
                TypographyProps={{color: "text.secondary"}}
                TooltipProps={{
                  title: "Initial PBLO deposit",
                }}
              />
              <TokenValueItem
                token={TOKENS.pablo}
                value={xPablo.amount.toFormat()}
                ValueProps={{color: "text.secondary"}}
              />
            </Grid>
          </Grid>
        </Box>

        <Box mt={7}>
          <Button
            variant="contained"
            fullWidth
            size="large"
            onClick={handleUntake}
          >
            Burn and unstake
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};
