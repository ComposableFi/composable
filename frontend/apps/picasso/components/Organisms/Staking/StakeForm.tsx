import BigNumber from "bignumber.js";
import { Button, Stack, Typography, useTheme } from "@mui/material";
import { StakeInputLabel } from "@/components/Organisms/Staking/StakeInputLabel";
import { AlertBox, BigNumberInput } from "@/components";
import { LockPeriodInput } from "@/components/Organisms/Staking/LockPeriodInput";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { WarningAmberRounded } from "@mui/icons-material";

type StakeFormProps = {
  amount: any;
  pica: any;
  valid: () => void;
  setter: any;
  value: any;
  options: any;
  picaRewardPool: any;
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
  onClick: () => void;
  formValid: any;
};

export function StakeForm({
  amount,
  pica,
  valid,
  setter,
  value,
  options,
  picaRewardPool,
  duration,
  onNone,
  onSome,
  hasRewardPools,
  min,
  max,
  onChange,
  onClick,
  formValid,
}: StakeFormProps) {
  const theme = useTheme();
  return (
    <Stack sx={{ marginTop: theme.spacing(9) }} gap={4}>
      <Stack gap={1.5}>
        <StakeInputLabel amount={amount} pica={pica} />
        <BigNumberInput
          isValid={valid}
          setter={setter}
          maxValue={amount}
          value={value}
          tokenId={pica.id}
          maxDecimals={pica.decimals.picasso ?? undefined}
        />
      </Stack>
      {/*  Radiobutton groups*/}
      <LockPeriodInput
        options={options}
        picaRewardPool={picaRewardPool}
        duration={duration}
        onNone={onNone}
        onSome={onSome}
        hasRewardPools={hasRewardPools}
        min={min}
        max={max}
        onChange={onChange}
      />
      <TextWithTooltip tooltip="Unlock date">Unlock date</TextWithTooltip>
      <FutureDatePaper duration={duration} />
      <AlertBox status="warning" icon={<WarningAmberRounded color="warning" />}>
        <Typography variant="body2">Warning</Typography>
        <Typography variant="inputLabel" color="text.secondary">
          Your {pica.symbol} will be locked until the expiry date.
        </Typography>
      </AlertBox>
      <Button
        fullWidth
        onClick={onClick}
        variant="contained"
        color="primary"
        disabled={!formValid}
      >
        <Typography variant="button">Lock and mint</Typography>
      </Button>
    </Stack>
  );
}
