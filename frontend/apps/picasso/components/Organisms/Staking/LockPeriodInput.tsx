import BigNumber from "bignumber.js";
import { alpha, Box, Slider, Typography, useTheme } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import {
  calculateStakingPeriodAPR,
  formatDurationOption,
} from "@/defi/polkadot/pallets/StakingRewards";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";

type LockPeriodInputProps = {
  options: any;
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
  return (
    <>
      <Box display="flex" justifyContent="space-between" alignItems="center">
        <TextWithTooltip tooltip={"Select lock period"}>
          Select lock period (multiplier)
        </TextWithTooltip>
        <Box display="flex" justifyContent="flex-end" alignItems="center">
          <Typography
            variant="body2"
            color={alpha(theme.palette.common.white, 0.6)}
          >
            APR
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
              {picaRewardPool.lock.durationPresets &&
                formatDurationOption(
                  duration,
                  pipe(
                    picaRewardPool.lock.durationPresets[duration.toString()],
                    O.fromNullable,
                    O.fold(onNone, onSome)
                  )
                )}
            </Typography>
            <Typography
              variant="subtitle1"
              color={theme.palette.featured.lemon}
            >
              ≈%
              {calculateStakingPeriodAPR(
                duration,
                picaRewardPool.lock.durationPresets
              )}
            </Typography>
          </Box>

          {hasRewardPools ? (
            <Slider
              marks={options}
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
