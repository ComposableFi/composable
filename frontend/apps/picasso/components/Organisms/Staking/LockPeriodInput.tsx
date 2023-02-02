import BigNumber from "bignumber.js";
import { alpha, Box, Slider, Typography, useTheme } from "@mui/material";
import { calculateStakingPeriodAPR } from "@/defi/polkadot/pallets/StakingRewards";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";

type LockPeriodInputProps = {
  options: {
    label: string;
    value: number;
  }[];
  picaRewardPool: RewardPool;
  duration: any;
  onNone: () => BigNumber;
  onSome: (x: BigNumber) => BigNumber;
  hasRewardPools: boolean;
  min: number;
  max: number;
  onChange: (
    event: Event,
    value: number | number[],
    activeThumb: number
  ) => any;
};

export function LockPeriodInput({
  options,
  picaRewardPool,
  duration,
  onNone,
  onSome,
  hasRewardPools,
  min,
  max,
  onChange,
}: LockPeriodInputProps) {
  const theme = useTheme();
  const selectedDuration = options.find(
    (option) => option.value === Number(duration)
  );
  return (
    <>
      <Box display="flex" justifyContent="space-between" alignItems="center">
        <Typography
          variant="body2"
          color={alpha(theme.palette.common.white, 0.6)}
        >
          Select lock period
        </Typography>
        <Box display="flex" justifyContent="flex-end" alignItems="center">
          <Typography
            variant="body2"
            color={alpha(theme.palette.common.white, 0.6)}
          >
            ~APR
          </Typography>
        </Box>
      </Box>
      {options.length > 0 && (
        <>
          <Box
            display="flex"
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
          </Box>

          {hasRewardPools ? (
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
          ) : null}
        </>
      )}
    </>
  );
}
