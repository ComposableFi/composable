import BigNumber from "bignumber.js";
import {
  alpha,
  Box,
  Button,
  Slider,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { formatNumber } from "shared";
import { AlertBox, BigNumberInput } from "@/components";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC, useEffect, useState } from "react";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { useSnackbar } from "notistack";
import {
  calculateStakingPeriodAPR,
  formatDurationOption,
  stake,
} from "@/defi/polkadot/pallets/StakingRewards";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";
import { useSigner } from "substrate-react";

export const StakeTabContent: FC = () => {
  const theme = useTheme();
  const [lockablePICA, setLockablePICA] = useState<BigNumber>(new BigNumber(0));
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const {
    hasRewardPools,
    picaRewardPool,
    balance,
    pica,
    executor,
    parachainApi,
    assetId,
  } = useStakingRewards();

  const options = hasRewardPools
    ? Object.entries(picaRewardPool.lock.durationPresets).reduce(
        (acc, [duration, _]) => [
          ...acc,
          {
            label: "",
            value: Number(duration),
          },
        ],
        [] as any
      )
    : [];

  const minDuration = hasRewardPools
    ? Object.entries(picaRewardPool.lock.durationPresets).reduce(
        (a, [b, _]) => (a !== 0 && a < Number(b) ? a : Number(b)),
        0
      )
    : 0;
  const maxDuration = hasRewardPools
    ? Object.entries(picaRewardPool.lock.durationPresets).reduce(
        (a, [b, _]) => (a > Number(b) ? a : Number(b)),
        0
      )
    : 0;

  const [lockPeriod, setLockPeriod] = useState<string>("");
  const account = useSelectedAccount();
  const inputValue = parseInt(lockPeriod) || minDuration;
  const signer = useSigner();

  useEffect(() => {
    if (inputValue.toString() !== lockPeriod) {
      setLockPeriod(inputValue.toString());
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [inputValue]);

  const setValidation = () => {};
  return (
    <Stack sx={{ marginTop: theme.spacing(9) }} gap={4}>
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
              {formatNumber(balance)}&nbsp;
              {pica.symbol}
            </Typography>
          </Box>
        </Box>
        <BigNumberInput
          isValid={setValidation}
          setter={setLockablePICA}
          maxValue={balance}
          value={lockablePICA}
          tokenId={pica.id}
          maxDecimals={18}
        />
      </Stack>
      {/*  Radiobutton groups*/}
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
      {options.length > 0 && inputValue && (
        <>
          <Box
            display="flex"
            justifyContent="space-between"
            alignItems="center"
          >
            <Typography variant="h6">
              {inputValue &&
                picaRewardPool.lock.durationPresets &&
                formatDurationOption(
                  inputValue.toString(),
                  picaRewardPool.lock.durationPresets[inputValue.toString()]
                )}
            </Typography>
            <Typography
              variant="subtitle1"
              color={theme.palette.featured.lemon}
            >
              ≈%
              {calculateStakingPeriodAPR(
                inputValue.toString(),
                picaRewardPool.lock.durationPresets
              )}
            </Typography>
          </Box>

          <Slider
            key={`slider-${inputValue}`}
            marks={options}
            name="duration-presets"
            aria-labelledby="lock-period-slider"
            step={null}
            value={inputValue}
            min={minDuration}
            max={maxDuration}
            onChange={(_, value) => setLockPeriod(value.toString())}
          />
        </>
      )}
      <TextWithTooltip tooltip="Unlock date">Unlock date</TextWithTooltip>
      <FutureDatePaper duration={lockPeriod} />
      <AlertBox status="warning" icon={<WarningAmberRounded color="warning" />}>
        <Typography variant="body2">Warning</Typography>
        <Typography variant="inputLabel" color="text.secondary">
          Your {pica.symbol} will be locked until the expiry date.
        </Typography>
      </AlertBox>
      <Button
        fullWidth
        onClick={() =>
          stake({
            executor,
            parachainApi,
            account,
            assetId,
            lockablePICA,
            lockPeriod,
            enqueueSnackbar,
            closeSnackbar,
            signer,
          })
        }
        variant="contained"
        color="primary"
        disabled={!lockablePICA.isGreaterThan(0) || !lockPeriod}
      >
        <Typography variant="button">Lock and mint</Typography>
      </Button>
    </Stack>
  );
};
