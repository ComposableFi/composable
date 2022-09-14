import { Box, Button, useTheme } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { useMemo, useState } from "react";
import { BoxProps } from "@mui/material";
import { SelectLockPeriod } from "./SelectLockPeriod";
import { isNumber } from "lodash";
import { useAppDispatch } from "@/hooks/store";
import { setMessage } from "@/stores/ui/uiSlice";
import { StakingRewardPool } from "@/defi/types";
import { useAssetBalance } from "@/store/assets/hooks";
import { DEFAULT_NETWORK_ID, PBLO_ASSET_ID } from "@/defi/utils";
import { useAsset } from "@/defi/hooks";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import BigNumber from "bignumber.js";
import moment from "moment";
import { extractDurationPresets } from "@/defi/utils/stakingRewards/durationPresets";
import { useStake } from "@/defi/hooks/stakingRewards";

export type Multiplier = {
  value?: number;
  expiry?: number;
};

export const StakeForm: React.FC<
  BoxProps & { stakingRewardPool: StakingRewardPool | null }
> = ({ stakingRewardPool, ...boxProps }) => {
  const theme = useTheme();
  const dispatch = useAppDispatch();
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [durationPreset, setDurationPreset] = useState<string>("0");

  const pabloAsset = useAsset(PBLO_ASSET_ID);
  const balance = useAssetBalance(DEFAULT_NETWORK_ID, PBLO_ASSET_ID);

  const multipliers = useMemo(() => {
    return extractDurationPresets(stakingRewardPool)
  }, [stakingRewardPool]);


  const validMultiplier = isNumber(durationPreset);
  const handleStake = useStake({
    poolId: new BigNumber(PBLO_ASSET_ID),
    amount,
    durationPreset: new BigNumber(durationPreset)
  })

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
          assets: pabloAsset ? [
            { icon: pabloAsset.icon, label: pabloAsset.symbol },
          ] : [],
        }}
      />

      <SelectLockPeriod
        mt={3}
        durationPreset={durationPreset}
        periodItems={multipliers}
        setDurationPreset={setDurationPreset}
      />

      <Box mt={3}>
        <Button
          onClick={handleStake}
          fullWidth
          variant="contained"
          disabled={!valid || !validMultiplier}
        >
          Stake and mint
        </Button>
      </Box>
    </Box>
  );
};
