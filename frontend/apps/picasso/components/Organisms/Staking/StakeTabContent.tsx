import BigNumber from "bignumber.js";
import { Box, Button, Stack, Typography, useTheme } from "@mui/material";
import { formatNumber } from "shared";
import { AlertBox, BigNumberInput } from "@/components";
import { RadioButtonGroup } from "@/components/Molecules/RadioButtonGroup";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC, useState } from "react";
import { DURATION_OPTION_ITEMS } from "@/components/Organisms/Staking/constants";
import { DurationOption } from "@/stores/defi/staking";

export const StakeTabContent: FC = () => {
  const theme = useTheme();
  const [lockablePICA, setLockablePICA] = useState<BigNumber>(new BigNumber(0));
  const [lockPeriod, setLockPeriod] = useState<DurationOption | undefined>(
    undefined
  );
  const match = (someValue?: DurationOption) => someValue === lockPeriod;
  const setValidation = () => {};
  return (
    <Stack sx={{ marginTop: theme.spacing(9) }} gap={4}>
      {/* BigNumberInput and descriptors */}
      <Stack gap={1.5}>
        <Box
          display="flex"
          width="100%"
          justifyContent="space-between"
          alignItems="center"
        >
          <Typography variant="inputLabel">Amount to lock</Typography>
          <Box display="flex" gap={1}>
            <Typography variant="inputLabel" color="text.secondary">
              Balance:
            </Typography>
            <Typography variant="inputLabel">
              {formatNumber(lockablePICA)} PICA
            </Typography>
          </Box>
        </Box>
        <BigNumberInput
          isValid={setValidation}
          setter={setLockablePICA}
          maxValue={new BigNumber(0)}
          value={lockablePICA}
          tokenId="pica"
          maxDecimals={18}
        />
      </Stack>
      {/*  Radiobutton groups*/}
      <RadioButtonGroup<DurationOption>
        label="Lock period (multiplier)"
        tooltip="Lock period (multiplier)"
        options={DURATION_OPTION_ITEMS}
        value={lockPeriod}
        isMatch={match}
        onChange={v => setLockPeriod(v)}
        sx={{
          marginTop: theme.spacing(4)
        }}
      />
      {/* Unlock date */}
      <TextWithTooltip tooltip="Unlock date">Unlock date</TextWithTooltip>
      <FutureDatePaper duration={lockPeriod} />
      <AlertBox status="warning" icon={<WarningAmberRounded color="warning" />}>
        <Typography variant="body2">Warning</Typography>
        <Typography variant="inputLabel" color="text.secondary">
          Your PICA will be locked until the expiry date.
        </Typography>
      </AlertBox>
      <Button
        fullWidth
        onClick={() => {} /* TODO: add action */}
        variant="contained"
        color="primary"
        disabled={!lockablePICA.isGreaterThan(0) || !lockPeriod}
      >
        <Typography variant="button">Lock and mint</Typography>
      </Button>
    </Stack>
  );
};
