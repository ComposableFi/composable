import { alpha, Box, Typography, useTheme, Slider } from "@mui/material";
import { Alert, Label } from "@/components/Atoms";
import { BoxProps } from "@mui/material";
import { DurationPresetMark } from "@/defi/utils/stakingRewards/durationPresets";
import {
  calculatePresetExpiry,
} from "@/defi/utils/stakingRewards/durationPresets";
import { DATE_FORMAT } from "@/defi/utils";
import { useMemo } from "react";

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
        <Box display="flex" justifyContent="flex-end" alignItems="center">
          <Typography variant="body2" color={alpha(theme.palette.common.white, 0.6)}>
            APR
          </Typography>
        </Box>
      </Box>
      <Box display="flex" justifyContent="space-between" alignItems="center">
        <Typography variant="h6">
          {durationPresetSelected?.label}
        </Typography>
        <Typography variant="subtitle1" color={theme.palette.success.main}>
          6.00
        </Typography>
      </Box>

      {/* <Grid container spacing={3}>
        {periodItems.map((item) => (
          <Grid item sm={12} md={6} lg={3} key={item.multiplier.toString()}>
            <Button
              variant="outlined"
              size="large"
              fullWidth
              sx={{
                borderWidth: selected(item.value) ? 1 : 0,
                background: selected(item.value)
                  ? alpha(
                      theme.palette.primary.main,
                      theme.custom.opacity.light
                    )
                  : alpha(
                      theme.palette.common.white,
                      theme.custom.opacity.lighter
                    ),
              }}
              onClick={() => setDurationPreset?.(item.value)}
            >
              <Typography
                variant="body1"
                color={selected(item.value) ? "text.primary" : "text.secondary"}
              >
                {item.label}
              </Typography>
            </Button>
          </Grid>
        ))}
      </Grid> */}

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
        borderRadius={9999}
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
