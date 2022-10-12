import { alpha, Box, Typography, useTheme, Slider } from "@mui/material";
import { Alert, Label } from "@/components/Atoms";
import { BoxProps } from "@mui/material";
import { DurationPresetMark } from "@/defi/utils/stakingRewards/durationPresets";
import {
  calculatePresetExpiry,
} from "@/defi/utils/stakingRewards/durationPresets";
import { DATE_FORMAT } from "@/defi/utils";
import { useMemo } from "react";
import { calculateDurationPresetAPR } from "@/defi/utils/stakingRewards";
import BigNumber from "bignumber.js";

export type SelectLockPeriodProps = {
  periodItems: DurationPresetMark[];
  durationPresetSelected?: DurationPresetMark;
  setMultiplier?: (multiplier: number) => void;
  multiplier: number;
  principalAssetSymbol?: string
} & BoxProps;

export const SelectLockPeriod: React.FC<SelectLockPeriodProps> = ({
  periodItems,
  durationPresetSelected,
  setMultiplier,
  multiplier,
  principalAssetSymbol,
  ...boxProps
}) => {
  const theme = useTheme();

  const durationPresetExpiry = useMemo(() => {
    if (!durationPresetSelected) return null;

    return calculatePresetExpiry(+durationPresetSelected.periodInSeconds);
  }, [durationPresetSelected]);


  const apr = calculateDurationPresetAPR(
    durationPresetSelected ? new BigNumber(durationPresetSelected.periodInSeconds) : undefined,
    new BigNumber(multiplier)
  )

  return (
    <Box {...boxProps}>

      <Box display="flex" justifyContent="space-between" alignItems="center">
        <Label
          label="Select lock period (multiplier)"
          TypographyProps={{ color: "text.secondary" }}
          TooltipProps={{
            title: "Select lock period (multiplier)",
          }}
        />
        {durationPresetSelected && <Box display="flex" justifyContent="flex-end" alignItems="center">
          <Typography variant="body2" color={alpha(theme.palette.common.white, 0.6)}>
            APR
          </Typography>
        </Box>}
      </Box>
      {durationPresetSelected && <Box display="flex" justifyContent="space-between" alignItems="center">
        <Typography variant="h6">
          {durationPresetSelected?.periodInString}
        </Typography>
        <Typography variant="subtitle1" color={theme.palette.success.main}>
          {apr.toFixed(2)}%
        </Typography>
      </Box>}

      <Slider
        value={multiplier}
        step={null}
        max={periodItems.reduce((agg, curr) => {
          return curr.value > agg ? curr.value : agg
        }, Number.MIN_SAFE_INTEGER)}
        min={periodItems.reduce((agg, curr) => {
          return curr.value < agg ? curr.value : agg
        }, Number.MAX_SAFE_INTEGER)}
        marks={periodItems}
        onChange={(_evt, value) => {
          setMultiplier?.(value as number)
        }}
      />

      <Label
        mt={3}
        label="Unlock date"
        TypographyProps={{ color: "text.secondary" }}
        TooltipProps={{
          title: "Unlock date",
        }}
      />

      <Box
        py={2.25}
        borderRadius="50%"
        textAlign="center"
        sx={{
          background: theme.palette.gradient.secondary,
        }}
      >
        <Typography
          variant="body1"
          color={durationPresetExpiry ? "text.primary" : "text.secondary"}
        >
          {durationPresetExpiry !== null ? durationPresetExpiry.format(DATE_FORMAT) : "Select lock time"}
        </Typography>
      </Box>

      {durationPresetExpiry && (
        <Box mt={1.5}>
          <Alert
            severity="warning"
            alertTitle="Warning"
            alertText={`Your ${principalAssetSymbol} will be locked until the expiry date.`}
            AlertTextProps={{ color: "text.secondary" }}
          />
        </Box>
      )}
    </Box>
  );
};
