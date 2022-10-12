import { Box, Button, useTheme } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { useEffect, useMemo, useState } from "react";
import { BoxProps } from "@mui/material";
import { SelectLockPeriod } from "./SelectLockPeriod";
import { StakingRewardPool } from "@/defi/types";
import { useAssetBalance } from "@/store/assets/hooks";
import { DEFAULT_NETWORK_ID, PBLO_ASSET_ID } from "@/defi/utils";
import { useAsset } from "@/defi/hooks";
import { useStake } from "@/defi/hooks/stakingRewards";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import BigNumber from "bignumber.js";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import {
  extractDurationPresets,
} from "@/defi/utils/stakingRewards/durationPresets";

export type Multiplier = {
  value?: number;
  expiry?: number;
};

export const StakeForm: React.FC<
  BoxProps & { stakingRewardPool?: StakingRewardPool }
> = ({ stakingRewardPool, ...boxProps }) => {
  const theme = useTheme();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [selectedMultiplier, setSelectedMultiplier] = useState<number>(0);
  const pabloAsset = useAsset(PBLO_ASSET_ID);
  const balance = useAssetBalance(DEFAULT_NETWORK_ID, PBLO_ASSET_ID);

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

  const validMultiplier =
  stakingRewardPool &&
  durationPresetSelected &&
  durationPresetSelected.periodInSeconds in
    stakingRewardPool.lock.durationPresets;

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
        LabelProps={{
          label: "Amount to lock",
          TypographyProps: { color: "text.secondary" },
          BalanceProps: {
            title: <AccountBalanceWalletIcon color="primary" />,
            balance: `${balance} ${pabloAsset?.symbol}`,
            BalanceTypographyProps: { color: "text.secondary" },
          },
        }}
        EndAdornmentAssetProps={{
          assets: pabloAsset
            ? [{ icon: pabloAsset.icon, label: pabloAsset.symbol }]
            : [],
        }}
      />

      <SelectLockPeriod
        mt={3}
        setMultiplier={setSelectedMultiplier}
        periodItems={multipliers}
        durationPresetSelected={durationPresetSelected}
        multiplier={selectedMultiplier}
      />

      <Box mt={3}>
        <Button
          onClick={handleStake}
          fullWidth
          variant="contained"
          disabled={!valid || !validMultiplier || isStaking}
        >
          Stake and mint
        </Button>
      </Box>

      <ConfirmingModal open={isStaking} />
    </Box>
  );
};
