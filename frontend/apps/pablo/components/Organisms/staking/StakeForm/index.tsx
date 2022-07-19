import { Box, Button, useTheme } from "@mui/material";
import { BigNumberInput } from "@/components/Atoms";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useState } from "react";
import BigNumber from "bignumber.js";
import { TOKENS } from "@/defi/Tokens";
import { BoxProps } from "@mui/material";
import { NETWORKS } from "@/defi/Networks";
import { SelectLockPeriod } from "./SelectLockPeriod";
import { isNumber } from "lodash";
import { useAppDispatch } from "@/hooks/store";
import { setMessage } from "@/stores/ui/uiSlice";

export type Multiplier = {
  value?: number,
  expiry?: number,
};

export const StakeForm: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const theme = useTheme();
  const dispatch = useAppDispatch();
  const [balance] = useState<BigNumber>(new BigNumber(200.0));
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [multiplier, setMultiplier] = useState<Multiplier>({});

  const validMultiplier = isNumber(multiplier.value);

  const handleStake = () => {
    //TODO: handling stake action
    dispatch(setMessage(
      {
        title: "Transaction successfull",
        text: "Stake and mint confirmed",
        link: "/",
        severity: "success",
      }
    ));
  };

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
          TypographyProps: {color: "text.secondary"},
          BalanceProps: {
            title: <AccountBalanceWalletIcon color="primary" />,
            balance: `${balance} ${TOKENS.pablo.symbol}`,
            BalanceTypographyProps: {color: "text.secondary"},
          },
        }}
        EndAdornmentAssetProps={{
          assets: [
            {icon: TOKENS.pablo.icon, label: NETWORKS[1].defaultTokenSymbol}
          ],
        }}
      />

      <SelectLockPeriod
        mt={3}
        multiplier={multiplier}
        setMultiplier={setMultiplier}
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
