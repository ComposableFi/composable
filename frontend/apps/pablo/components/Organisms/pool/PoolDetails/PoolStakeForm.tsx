import { Box, useTheme, Button } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { PoolDetailsProps } from "./index";
import { useLiquidityPoolDetails } from "@/store/hooks/useLiquidityPoolDetails";
import { useStake } from "@/defi/hooks/stakingRewards";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";

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

  const durationPresets = useMemo(() => {
    if (stakingRewardPool) {
      return Object.keys(stakingRewardPool.lock.durationPresets);
    }
    return [];
  }, [stakingRewardPool]);

  const handleStake = useStake({
    amount,
    poolId: stakingRewardPool ? stakingRewardPool.assetId : undefined,
    durationPreset:
      durationPresets.length > 0
        ? new BigNumber(durationPresets[0])
        : undefined,
  });

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
              balance: `${poolDetails.lpBalance} ${baseAsset?.symbol}/${quoteAsset?.symbol}`,
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
      </Box>
      <Box mt={4}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          onClick={handleStake}
          disabled={!valid}
        >
          {`Stake ${baseAsset?.symbol}/${quoteAsset?.symbol}`}
        </Button>
      </Box>
    </Box>
  );
};
