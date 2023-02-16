import {
  Button,
  CircularProgress,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { StakeInputLabel } from "@/components/Organisms/Staking/StakeInputLabel";
import { AlertBox, BigNumberInput } from "@/components";
import { LockPeriodInput } from "@/components/Organisms/Staking/LockPeriodInput";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { InfoOutlined } from "@mui/icons-material";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { formatDate } from "shared";
import React, { useMemo } from "react";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import config from "@/constants/config";

type StakeFormProps = {
  amount: any;
  pica: any;
  setter: any;
  value: any;
  errorMsg: string;
  options: any;
  picaRewardPool: any;
  duration: any;
  hasRewardPools: boolean;
  min: number;
  max: number;
  onChange: (
    event: Event,
    value: number | number[],
    activeThumb: number
  ) => any;
  onClick: () => void;
  formValid: any;
};

export function StakeForm({
  amount,
  pica,
  setter,
  value,
  errorMsg,
  options,
  picaRewardPool,
  duration,
  hasRewardPools,
  min,
  max,
  onChange,
  onClick,
  formValid,
}: StakeFormProps) {
  const theme = useTheme();
  const shouldShowWarning = duration !== "0";
  const account = useSelectedAccount(config.defaultNetworkId);
  const isPendingStake = usePendingExtrinsic(
    "stake",
    "stakingRewards",
    account?.address ?? "-"
  );

  return (
    <Stack sx={{ marginTop: theme.spacing(9) }} gap={4}>
      <Stack gap={1.5}>
        <StakeInputLabel amount={amount} pica={pica} />
        <BigNumberInput
          setter={setter}
          maxValue={amount}
          value={value}
          tokenId={pica.id}
          InputProps={{
            sx: {
              "& .MuiOutlinedInput-input": {
                textAlign: "center",
              },
            },
          }}
          maxDecimals={pica.decimals.picasso ?? undefined}
          disabled={isPendingStake}
        />
        {errorMsg.length > 0 && (
          <Typography variant="caption" color="error">
            {errorMsg}
          </Typography>
        )}
      </Stack>
      {/*  Radiobutton groups*/}
      <LockPeriodInput
        options={options}
        picaRewardPool={picaRewardPool}
        duration={duration}
        hasRewardPools={hasRewardPools}
        min={min}
        max={max}
        onChange={onChange}
        disabled={isPendingStake}
      />
      <Typography variant="body2">Unlock date</Typography>
      <FutureDatePaper duration={duration} />
      {shouldShowWarning && (
        <PICALockedWarning duration={duration} token={pica} />
      )}
      <StakeButton
        disabled={
          !formValid ||
          isPendingStake ||
          picaRewardPool.minimumStakingAmount.gt(value)
        }
        isPendingStake={isPendingStake}
        onClick={onClick}
      />
    </Stack>
  );
}

const PICALockedWarning = ({
  token,
  duration,
}: {
  token: TokenMetadata;
  duration: string;
}) => {
  const { date, days } = useMemo(() => {
    const now = new Date();
    const date = new Date(now.getTime() + Number(duration) * 1000);
    const formatted = formatDate(date);
    const days = (date.getTime() - new Date().getTime()) / 86400_000;
    return {
      days,
      date: formatted,
    };
  }, [duration]);

  return (
    <AlertBox status="info" icon={<InfoOutlined color="info" />}>
      <Typography variant="body2">You are locking ${token.symbol}</Typography>
      <Typography variant="inputLabel" color="text.secondary">
        Your ${token.symbol} will be locked for {days} days until {date}
      </Typography>
    </AlertBox>
  );
};

const StakeButton = ({
  disabled,
  onClick,
  isPendingStake,
}: {
  disabled: boolean;
  isPendingStake: boolean;
  onClick: () => void;
}) => {
  return (
    <Button
      fullWidth
      onClick={onClick}
      variant="contained"
      color="primary"
      disabled={disabled}
    >
      {isPendingStake ? (
        <CircularProgress variant="indeterminate" size={24} />
      ) : (
        <Typography variant="button">Stake and mint</Typography>
      )}
    </Button>
  );
};
