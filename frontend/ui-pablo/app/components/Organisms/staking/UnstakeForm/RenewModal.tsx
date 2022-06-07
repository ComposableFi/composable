import React, { useState } from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import {
  Box,
  Typography,
  Button,
} from "@mui/material";

import { useDispatch } from "react-redux";
import { XPablo } from "@/defi/types";
import { Label } from "@/components/Atoms";
import { TokenValueItem } from "../TokenValueItem";
import { TOKENS } from "@/defi/Tokens";
import { setMessage } from "@/stores/ui/uiSlice";
import { SelectLockPeriod } from "../StakeForm/SelectLockPeriod";
import { Multiplier } from "../StakeForm";
import { isNumber } from "lodash";

export type RenewModalProps = {
  xPablo: XPablo,
} & ModalProps;

export const RenewModal: React.FC<RenewModalProps> = ({
  xPablo,
  onClose,
  ...modalProps
}) => {
  const dispatch = useDispatch();

  const [multiplier, setMultiplier] = useState<Multiplier>({});

  const validMultiplier = isNumber(multiplier.value);

  const handleUntake = () => {
    dispatch(setMessage(
      {
        title: "Transaction successfull",
        text: "Renew staking period confirmed",
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
          Renew staking period
        </Typography>

        <Box mt={7}>
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
        </Box>

        <SelectLockPeriod
          mt={7}
          multiplier={multiplier}
          setMultiplier={setMultiplier}
        />

        <Box mt={7}>
          <Button
            variant="contained"
            fullWidth
            size="large"
            onClick={handleUntake}
            disabled={validMultiplier}
          >
            Renew period
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};
