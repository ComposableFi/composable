import { Box, useTheme, Button } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import { useState } from "react";
import { PoolDetailsProps } from "./index";
import { useLiquidityPoolDetails } from "@/store/hooks/useLiquidityPoolDetails";
import { fromChainUnits } from "@/defi/utils";
import BigNumber from "bignumber.js";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useStakingPositions } from "@/store/hooks/useStakingPositions";

export const PoolUnstakeForm: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const theme = useTheme();
  const { baseAsset, quoteAsset, lpBalance, pool } =
    useLiquidityPoolDetails(poolId);
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);

  const positions = useStakingPositions({ stakedAssetId: pool?.lpToken });
  const canUnstake = false;

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
                  ? fromChainUnits(positions[0].lockedPrincipalAsset)
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
