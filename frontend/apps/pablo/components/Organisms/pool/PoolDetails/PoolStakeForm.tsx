import { Box, useTheme, Button } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { PoolDetailsProps } from "./index";
import { useLiquidityPoolDetails } from "@/store/hooks/useLiquidityPoolDetails";
import { useStake } from "@/defi/hooks/stakingRewards";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import { SelectLockPeriod } from "@/components";
import { extractDurationPresets } from "@/defi/utils/stakingRewards/durationPresets";

export const PoolStakeForm: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const theme = useTheme();
  const poolDetails = useLiquidityPoolDetails(poolId);
  const { baseAsset, quoteAsset, pool } = poolDetails;
  const stakingRewardPool = useStakingRewardPool(pool ? pool.lpToken : "-");

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [selectedMultiplier, setSelectedMultiplier] = useState<number>(0);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const principalAssetSymbol = useMemo(() => {
    if (!baseAsset || !quoteAsset) return undefined;

    return `${baseAsset.symbol}/${quoteAsset.symbol}`;
  }, [baseAsset, quoteAsset]);

  const multipliers = useMemo(() => {
    return extractDurationPresets(stakingRewardPool);
  }, [stakingRewardPool]);

  const durationPresetSelected = useMemo(() => {
    return multipliers.find((mul) => mul.value === selectedMultiplier);
  }, [multipliers, selectedMultiplier]);

  const handleStake = useStake({
    amount,
    poolId: stakingRewardPool ? stakingRewardPool.assetId : undefined,
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
          maxValue={poolDetails.lpBalance}
          setValid={setValid}
          noBorder
          value={amount}
          setValue={setAmount}
          buttonLabel={"Max"}
          ButtonProps={{
            onClick: () => setAmount(poolDetails.lpBalance),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount to stake",
            TypographyProps: { color: "text.secondary" },
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${poolDetails.lpBalance} ${principalAssetSymbol}`,
              BalanceTypographyProps: { color: "text.secondary" },
            },
          }}
          EndAdornmentAssetProps={{
            assets:
              baseAsset && quoteAsset
                ? [
                    {
                      icon: baseAsset.icon,
                      label: baseAsset.symbol,
                    },
                    {
                      icon: quoteAsset.icon,
                      label: quoteAsset.symbol,
                    },
                  ]
                : [],
            separator: "/",
          }}
        />

        <SelectLockPeriod
          mt={3}
          durationPresetSelected={durationPresetSelected}
          setMultiplier={setSelectedMultiplier}
          periodItems={multipliers}
          multiplier={selectedMultiplier}
          principalAssetSymbol={principalAssetSymbol}
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
          {`Stake ${principalAssetSymbol}`}
        </Button>
      </Box>

      <ConfirmingModal open={isStaking} />
    </Box>
  );
};
