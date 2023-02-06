import {
  alpha,
  Slider,
  Stack,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import { calculateStakingPeriodAPR } from "@/defi/polkadot/pallets/StakingRewards";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";

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
  return (
    <>
      <Stack
        direction="row"
        justifyContent="space-between"
        alignItems="center"
        mb={2}
      >
        <Typography
          variant="body2"
          color={alpha(theme.palette.common.white, 0.6)}
          {...LabelProps}
        >
          {label || "Select lock period"}
        </Typography>
        <Typography
          variant="body2"
          color={alpha(theme.palette.common.white, 0.6)}
        >
          ~APR
        </Typography>
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
            <Typography
              variant="subtitle1"
              color={theme.palette.featured.lemon}
            >
              %
              {calculateStakingPeriodAPR(
                duration,
                picaRewardPool.lock.durationPresets
              )}
            </Typography>
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
    </>
  );
}
