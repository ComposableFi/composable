import { Box, useTheme, Button } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { useMemo, useState } from "react";
import { PoolDetailsProps } from "./index";
import { useLiquidityPoolDetails } from "@/store/hooks/useLiquidityPoolDetails";
import { fromChainUnits } from "@/defi/utils";
import { useStakedPositions } from "@/store/stakingRewards/stakingRewards.slice";
import BigNumber from "bignumber.js";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import moment from "moment";

export const PoolUnstakeForm: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const theme = useTheme();
  const { baseAsset, quoteAsset, lpBalance, pool } =
    useLiquidityPoolDetails(poolId);
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);

  const positions = useStakedPositions(pool?.lpToken ?? "-");
  const canUnstake = useMemo(() => {
    if (positions.length <= 0) return false;

    return Date.now() > new BigNumber(positions[0].endTimestamp).toNumber()
  }, [positions]);

  const handleUnStake = () => {
    // TODO: handle stake here
  };

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
            label: "Amount to Unstake",
            TypographyProps: { color: "text.secondary" },
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${
                positions.length > 0
                  ? fromChainUnits(positions[0].amount)
                  : new BigNumber(0)
              } ${baseAsset?.symbol}/${quoteAsset?.symbol}`,
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
          onClick={handleUnStake}
          disabled={!valid || !canUnstake}
        >
          {`Unstake ${baseAsset?.symbol}/${quoteAsset?.symbol}`}
        </Button>
      </Box>
    </Box>
  );
};
