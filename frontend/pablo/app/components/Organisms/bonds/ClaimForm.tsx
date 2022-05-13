import { BigNumberInput, Label } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { BondDetails } from "@/defi/types";
import { Box, Button, BoxProps, Typography, Theme, useTheme, alpha } from "@mui/material";
import AccountBalanceWalletIcon from '@mui/icons-material/AccountBalanceWallet';
import { useState } from "react";
import BigNumber from "bignumber.js";

const containerBoxProps = (theme: Theme) => ({
  p: 4,
  borderRadius: 1.5,
  sx: {
    background: theme.palette.gradient.secondary,
    border: `1px solid ${alpha(theme.palette.common.white, theme.custom.opacity.light)}`
  },
});

const defaultLabelProps = (label: string, balance: string) => ({
  label: label,
  BalanceProps: {
    balance: balance,
    BalanceTypographyProps: {
      variant: "body1",
      fontWeight: "600",
    }
  }
} as const);

export type ClaimFormProps = {
  bond: BondDetails,
} & BoxProps;

export const ClaimForm: React.FC<ClaimFormProps> = ({
  bond,
  ...boxProps
}) => {

  const theme = useTheme();
  const token1 = getToken(bond.tokenId1);
  const token2 = getToken(bond.tokenId2);
  const chaos = getToken("chaos");
  const pablo = getToken("pablo");

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);

  const claimable = !bond.claimable_amount.eq(0);

  const handleClaim = () => {
    //TODO: handle deposit here
  };

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Typography variant="h6">
        Claim
      </Typography>
      <Box mt={6}>
        <BigNumberInput
          disabled={!claimable}
          value={amount}
          setValue={setAmount}
          maxValue={bond.claimable_amount}
          setValid={setValid}
          EndAdornmentAssetProps={{
            assets: [
              { icon: chaos.icon, label: chaos.symbol },
            ],
            separator: "/",
            LabelProps: {variant: 'body1'},

          }}
          buttonLabel="Max"
          ButtonProps={{
            onClick: () => setAmount(bond.claimable_amount),
            sx: {
              padding: theme.spacing(1),
            }
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: (
              claimable ? {
                title: <AccountBalanceWalletIcon color="primary"/>,
                balance: `${bond.claimable_amount} ${token1.symbol}/${token2.symbol}`
              } : undefined
            ),
          }}
        />
      </Box>
      <Box mt={3}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          disabled={!claimable || !valid}
          onClick={handleClaim}
        >
          Claim
        </Button>
      </Box>
      <Box mt={6}>
        <Label
          {...defaultLabelProps("Pending Rewards", `${bond.pending_amount} LP`)}
        />
        <Label
          {...defaultLabelProps("Claimable Rewards", `${bond.claimable_amount} ${pablo.symbol}`)}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Time until fully vested", `${bond.remaining_term} days`)}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Vested", `${bond.vested_term} days`)}
          mt={2}
        />
        <Label
          {...defaultLabelProps("ROI", `${bond.roi}%`)}
          mt={2}
        />
      </Box>
    </Box>
  );
};
