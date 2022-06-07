import {
  Box,
  useTheme,
  BoxProps,
  Button,
} from "@mui/material";
import { useAppSelector } from "@/hooks/store";
import { BigNumberInput } from "@/components/Atoms";
import { TOKENS } from "@/defi/Tokens";
import { useState } from "react";
import BigNumber from "bignumber.js";
import { getTokenIdsFromSelectedPool } from "@/stores/defi/pool";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";

export const PoolUnstakeForm: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const theme = useTheme();

  const [balance] = useState<BigNumber>(new BigNumber(200.0));
  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const {
    tokenId1,
    tokenId2,
  } = useAppSelector(
    getTokenIdsFromSelectedPool
  );

  const handleUnstake = () => {
    // TODO: handle stake here
  }

  return (
    <Box {...boxProps}>
      <Box>
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
            label: "Amount to unstake",
            TypographyProps: {color: "text.secondary"},
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${balance} ${TOKENS[tokenId1].symbol}/${TOKENS[tokenId2].symbol}`,
              BalanceTypographyProps: {color: "text.secondary"},
            },
          }}
          EndAdornmentAssetProps={{
            assets: [
              {icon: TOKENS[tokenId1].icon, label: TOKENS[tokenId1].symbol},
              {icon: TOKENS[tokenId2].icon, label: TOKENS[tokenId2].symbol},
            ],
            separator: "/",
          }}
        />
      </Box>
      <Box mt={4}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          onClick={handleUnstake}
          disabled={!valid}
        >
          {`Unstake ${TOKENS[tokenId1].symbol}/${TOKENS[tokenId2].symbol}`}
        </Button>
      </Box>
    </Box>
  );
};

