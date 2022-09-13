import { Modal, TokenAsset } from "@/components";
import { FC, useState } from "react";
import { Box, Button, Paper, Stack, Typography, useTheme } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { RadioButtonGroup } from "@/components/Molecules/RadioButtonGroup";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { formatNumber } from "shared";
import BigNumber from "bignumber.js";
import { DurationOption } from "@/defi/polkadot/pallets/StakingRewards";

export const RenewModal: FC<{ open: boolean; onClose: () => void }> = ({
  open,
  onClose,
}) => {
  const [extendPeriod, setExtendPeriod] = useState<DurationOption | undefined>(
    undefined
  );
  const match = (someValue?: DurationOption) => someValue === extendPeriod;
  const theme = useTheme();
  const initialPicaDeposit = new BigNumber(0);

  const handleRenew = () => {
    onClose();
  };

  return (
    <Modal open={open} dismissible onClose={onClose} maxWidth="md">
      <Stack gap={4}>
        <Typography variant="h5" textAlign="center" marginBottom={4}>
          Renew staking period
        </Typography>
        <Stack gap={1.5}>
          <TextWithTooltip
            TypographyProps={{
              variant: "inputLabel",
            }}
            tooltip="Initial PICA deposit "
          >
            Initial PICA deposit
          </TextWithTooltip>
          <Paper
            sx={{
              position: "relative",
            }}
          >
            <Box
              sx={{
                position: "absolute",
                left: "1rem",
                top: "50%",
                transform: "translateY(-50%)",
              }}
            >
              <TokenAsset tokenId={"pica"} iconOnly />
            </Box>
            <Typography
              textAlign="center"
              variant="body2"
              color="text.secondary"
            >
              {formatNumber(initialPicaDeposit)}
            </Typography>
          </Paper>
        </Stack>
        <RadioButtonGroup<DurationOption>
          label="Lock period (multiplier)"
          tooltip="Lock period (multiplier)"
          options={[]}
          value={extendPeriod}
          isMatch={match}
          onChange={(value) => setExtendPeriod(value)}
          sx={{
            marginTop: theme.spacing(4),
          }}
        />
        <Stack gap={1.5} marginTop={4}>
          <TextWithTooltip tooltip="Unlock date">Unlock date</TextWithTooltip>
          <FutureDatePaper duration={""} />
        </Stack>
        <Button
          disabled={!extendPeriod}
          variant="contained"
          color="primary"
          fullWidth
          onClick={handleRenew}
        >
          Renew period
        </Button>
      </Stack>
    </Modal>
  );
};
