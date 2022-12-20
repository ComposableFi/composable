import { Box, Button, useTheme } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { FC, useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { PoolDetailsProps } from "./index";
import { useStake } from "@/defi/hooks/stakingRewards";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import { SelectLockPeriod } from "@/components";
import { extractDurationPresets } from "@/defi/utils/stakingRewards/durationPresets";

export const PoolStakeForm: FC<PoolDetailsProps> = ({ pool, ...boxProps }) => {
  const theme = useTheme();
  const baseAsset = pool.config.assets[0];
  const quoteAsset = pool.config.assets[1];
  const lpBalance = new BigNumber(0);
  const stakingRewardPool = useStakingRewardPool(
    pool.config.lpToken.toString() ?? "-"
  );
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [selectedMultiplier, setSelectedMultiplier] = useState<number>(0);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const multipliers = useMemo(() => {
    return extractDurationPresets(stakingRewardPool);
  }, [stakingRewardPool]);

  const durationPresetSelected = useMemo(() => {
    return multipliers.find((mul) => mul.value === selectedMultiplier);
  }, [multipliers, selectedMultiplier]);

  const handleStake = useStake({
    amount,
    poolId: pool.poolId ?? undefined,
    durationPreset: durationPresetSelected
      ? new BigNumber(durationPresetSelected.periodInSeconds)
      : undefined,
  });

  const isStaking = usePendingExtrinsic(
    "stake",
    "stakingRewards",
    selectedAccount ? selectedAccount.address : "-"
  );

  return (
    <Box {...boxProps}>
      <Box>
        <BigNumberInput
          maxValue={lpBalance}
          setValid={setValid}
          noBorder
          value={amount}
          setValue={setAmount}
          buttonLabel={"Max"}
          ButtonProps={{
            onClick: () => setAmount(lpBalance),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount to stake",
            TypographyProps: { color: "text.secondary" },
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${lpBalance}`,
              BalanceTypographyProps: { color: "text.secondary" },
            },
          }}
          EndAdornmentAssetProps={{
            assets: baseAsset && quoteAsset ? [] : [],
            separator: "/",
          }}
        />

        <SelectLockPeriod
          mt={3}
          durationPresetSelected={durationPresetSelected}
          setMultiplier={setSelectedMultiplier}
          periodItems={multipliers}
          principalAssetSymbol={""}
          multiplier={selectedMultiplier}
        />
      </Box>

      <Box mt={4}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          onClick={handleStake}
          disabled={!valid}
        >
          {`Stake`}
        </Button>
      </Box>

      <ConfirmingModal open={isStaking} />
    </Box>
  );
};
