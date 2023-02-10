import {
  alpha,
  Box,
  Slider,
  Stack,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import { calculateStakingPeriodAPR } from "@/defi/polkadot/pallets/StakingRewards";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";
import { useMemo } from "react";

type LockPeriodInputProps = {
  options: {
    label: string;
    value: number;
  }[];
  picaRewardPool: RewardPool;
  duration: any;
  hasRewardPools: boolean;
  min: number;
  max: number;
  onChange: (
    event: Event,
    value: number | number[],
    activeThumb: number
  ) => any;
  label?: string;
  LabelProps?: TypographyProps;
};

export function LockPeriodInput({
  options,
  picaRewardPool,
  duration,
  hasRewardPools,
  min,
  max,
  onChange,
  label,
  LabelProps,
}: LockPeriodInputProps) {
  const theme = useTheme();
  const selectedDuration = options.find(
    (option) => option.value === Number(duration)
  );
  const multiplier = useMemo(() => {
    if (!selectedDuration) return 1;
    if (selectedDuration.value === 0) return 1;
    const durationConfig = Object.entries(
      picaRewardPool.lock.durationPresets
    ).find(([duration, _]) => Number(duration) === selectedDuration.value);
    if (!durationConfig) return 1;
    return durationConfig[1].div(100).toNumber();
  }, [picaRewardPool.lock.durationPresets, selectedDuration]);
  return (
    <Box>
      <Stack direction="row" justifyContent="space-between" alignItems="center">
        <Typography
          variant="body2"
          color={alpha(theme.palette.common.white, 0.6)}
          {...LabelProps}
        >
          {label || "Select lock period"}
        </Typography>
        <Stack direction="row" gap={1}>
          <Typography
            variant="body2"
            color={alpha(theme.palette.common.white, 0.6)}
            sx={{ width: "93px", ml: "-1ch" }}
          >
            ~APR
          </Typography>
          <Typography
            variant="body2"
            color={alpha(theme.palette.common.white, 0.6)}
            sx={{ width: "93px" }}
          >
            Multiplier
          </Typography>
        </Stack>
      </Stack>
      {options.length > 0 && (
        <>
          <Stack
            direction="row"
            justifyContent="space-between"
            alignItems="center"
          >
            <Typography variant="h6">
              {selectedDuration?.label ?? "Select a lock period"}
            </Typography>
            <Stack direction="row" gap={1}>
              <Typography
                variant="subtitle1"
                color={theme.palette.featured.lemon}
                sx={{ width: "93px" }}
                textAlign="start"
              >
                %
                {calculateStakingPeriodAPR(
                  duration,
                  picaRewardPool.lock.durationPresets
                )}
              </Typography>
              <Typography
                variant="subtitle1"
                color={theme.palette.featured.lemon}
                sx={{ width: "93px" }}
                textAlign="start"
              >
                {multiplier}X
              </Typography>
            </Stack>
          </Stack>

          {hasRewardPools && (
            <Slider
              marks={structuredClone(options).map((option) => {
                option.label = "";
                return option;
              })}
              name="duration-presets"
              aria-labelledby="lock-period-slider"
              step={null}
              value={Number(duration)}
              min={min}
              max={max}
              onChange={onChange}
            />
          )}
        </>
      )}
    </Box>
  );
}
