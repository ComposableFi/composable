import React, { useState } from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import {
  Box,
  Typography,
  Button,
} from "@mui/material";
import { Label } from "@/components/Atoms";
import { TokenValueItem } from "../TokenValueItem";
import { TOKENS } from "tokens";
import { SelectLockPeriod } from "../StakeForm/SelectLockPeriod";
import { Multiplier } from "../StakeForm";
import { isNumber } from "lodash";
import { StakedFinancialNftPosition } from "@/defi/types";

export type RenewModalProps = {
  xPablo: StakedFinancialNftPosition,
} & ModalProps;

export const RenewModal: React.FC<RenewModalProps> = ({
  xPablo,
  onClose,
  ...modalProps
}) => {
  const [multiplier, _setMultiplier] = useState<Multiplier>({});

  const validMultiplier = isNumber(multiplier.value);

  const handleUnstake = () => {
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
            token={TOKENS.pblo}
            value={xPablo.lockedPrincipalAsset.toFormat()}
            ValueProps={{color: "text.secondary"}}
          />
        </Box>

        <SelectLockPeriod
          mt={7}
          multiplier={0}
          periodItems={[]}
        />

        <Box mt={7}>
          <Button
            variant="contained"
            fullWidth
            size="large"
            onClick={handleUnstake}
            disabled={validMultiplier}
          >
            Renew period
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};
