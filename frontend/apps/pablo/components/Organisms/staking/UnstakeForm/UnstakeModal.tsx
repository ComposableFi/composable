import React, { useMemo } from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Box, Typography, useTheme, Button, Grid } from "@mui/material";
import { Label } from "@/components/Atoms";
import { TokenValueItem } from "../TokenValueItem";
import { TOKENS } from "tokens";
import { StakedFinancialNftPosition, StakingRewardPool } from "@/defi/types";
import BigNumber from "bignumber.js";

export type UnstakeModalProps = {
  stakingRewardPool?: StakingRewardPool;
  xPablo: StakedFinancialNftPosition;
  onDismiss: () => void,
  onUnstake: () => void,
} & ModalProps;

export const UnstakeModal: React.FC<UnstakeModalProps> = ({
  xPablo,
  onDismiss,
  onUnstake,
  stakingRewardPool,
  ...modalProps
}) => {
  const theme = useTheme();

  const amount = useMemo(() => {
    if (!stakingRewardPool) return new BigNumber(0);

    return xPablo.isExpired
      ? xPablo.lockedPrincipalAsset
      : xPablo.lockedPrincipalAsset.minus(
          xPablo.lockedPrincipalAsset.times(
            stakingRewardPool.lock.unlockPenalty.div(100)
          )
        );
  }, [xPablo, stakingRewardPool]);

  return (
    <Modal maxWidth="lg" onClose={onDismiss} {...modalProps}>
      <Box width={{ md: 968 }} margin="auto">
        <Typography variant="h5" textAlign="center" mt={8}>
          Burn and unstake you position
        </Typography>

        <Box mt={7}>
          <Grid container spacing={3}>
            <Grid item sm={12} md={6}>
              <Label
                label="Withdrawable PBLO"
                TypographyProps={{ color: "text.secondary" }}
                TooltipProps={{
                  title: "Withdrawable PBLO",
                }}
              />
              <TokenValueItem
                token={TOKENS.pblo}
                value={amount.toFormat()}
              />
            </Grid>

            <Grid item sm={12} md={6}>
              <Label
                label="Initial PBLO deposit"
                TypographyProps={{ color: "text.secondary" }}
                TooltipProps={{
                  title: "Initial PBLO deposit",
                }}
              />
              <TokenValueItem
                token={TOKENS.pblo}
                value={xPablo.lockedPrincipalAsset.toFormat()}
                ValueProps={{ color: "text.secondary" }}
              />
            </Grid>
          </Grid>
        </Box>

        <Box mt={7}>
          <Button
            variant="contained"
            fullWidth
            size="large"
            onClick={onUnstake}
          >
            Burn and unstake
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};
