import { Box, BoxProps, Button, useTheme } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { FC, useMemo, useState } from "react";
import { SelectLockPeriod } from "@/components";
import { StakingRewardPool } from "@/defi/types";
import { useAsset } from "@/defi/hooks";
import { DEFAULT_NETWORK_ID, PBLO_ASSET_ID } from "@/defi/utils";
import { useStake } from "@/defi/hooks/stakingRewards";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import { extractDurationPresets } from "@/defi/utils/stakingRewards/durationPresets";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import BigNumber from "bignumber.js";

export type Multiplier = {
  value?: number;
  expiry?: number;
};

export const StakeForm: FC<
  BoxProps & { stakingRewardPool?: StakingRewardPool }
> = ({ stakingRewardPool, ...boxProps }) => {
  const theme = useTheme();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [_valid, setValid] = useState<boolean>(false);
  const [selectedMultiplier, setSelectedMultiplier] = useState<number>(0);
  const pabloAsset = useAsset(PBLO_ASSET_ID);
  const balance = new BigNumber(0);

  const multipliers = useMemo(() => {
    return extractDurationPresets(stakingRewardPool);
  }, [stakingRewardPool]);

  const durationPresetSelected = useMemo(() => {
    return multipliers.find((mul) => mul.value === selectedMultiplier);
  }, [multipliers, selectedMultiplier]);

  const handleStake = useStake({
    poolId: new BigNumber(PBLO_ASSET_ID),
    amount,
    durationPreset: new BigNumber(
      durationPresetSelected ? durationPresetSelected.periodInSeconds : "-"
    ),
  });

  const isStaking = usePendingExtrinsic(
    "stake",
    "stakingRewards",
    selectedAccount ? selectedAccount.address : "-"
  );

  return (
    <Box {...boxProps}>
      <BigNumberInput
        maxValue={balance}
        setValid={setValid}
        noBorder
        value={amount}
        setValue={setAmount}
        buttonLabel={"Max"}
        ButtonProps={{
          onClick: () => setAmount(balance),
          sx: {
            padding: theme.spacing(1),
          },
        }}
        disabled
        LabelProps={{
          label: "Amount to lock",
          TypographyProps: { color: "text.secondary" },
          BalanceProps: {
            title: <AccountBalanceWalletIcon color="primary" />,
            balance: `${balance} ${pabloAsset?.getSymbol()}`,
            BalanceTypographyProps: { color: "text.secondary" },
          },
        }}
        EndAdornmentAssetProps={{
          assets: pabloAsset ? [] : [],
        }}
      />

      <SelectLockPeriod
        mt={3}
        setMultiplier={setSelectedMultiplier}
        periodItems={multipliers}
        durationPresetSelected={durationPresetSelected}
        multiplier={selectedMultiplier}
        disabled
      />

      <Box mt={3}>
        <Button onClick={handleStake} fullWidth variant="contained" disabled>
          Stake and mint
        </Button>
      </Box>

      <ConfirmingModal open={isStaking} />
    </Box>
  );
};
