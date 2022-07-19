import { BigNumberInput, Label } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { BondDetails } from "@/defi/types";
import {
  Box,
  Button,
  BoxProps,
  Typography,
  Theme,
  useTheme,
  alpha,
} from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { MockedAsset } from "@/store/assets/assets.types";
import { usePrincipalAssetSymbol } from "@/defi/hooks/bonds/usePrincipalAssetSymbol";

const containerBoxProps = (theme: Theme) => ({
  p: 4,
  borderRadius: 1.5,
  sx: {
    background: theme.palette.gradient.secondary,
    border: `1px solid ${alpha(
      theme.palette.common.white,
      theme.custom.opacity.light
    )}`,
  },
});

const defaultLabelProps = (label: string, balance: string) =>
  ({
    label: label,
    BalanceProps: {
      balance: balance,
      BalanceTypographyProps: {
        variant: "body1",
        fontWeight: "600",
      },
    },
  } as const);

export type ClaimFormProps = {
  bond: SelectedBondOffer;
} & BoxProps;

export const ClaimForm: React.FC<ClaimFormProps> = ({ bond, ...boxProps }) => {
  const theme = useTheme();
  const { principalAsset, rewardAsset } = bond;

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);

  const claimable = false;

  const handleClaim = () => {
    //TODO: handle deposit here
  };

  const principalSymbol = usePrincipalAssetSymbol(bond.principalAsset);

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Typography variant="h6">Claim</Typography>
      <Box mt={6}>
        <BigNumberInput
          disabled={!claimable}
          value={amount}
          setValue={setAmount}
          maxValue={new BigNumber(0)}
          setValid={setValid}
          EndAdornmentAssetProps={{
            assets: rewardAsset
              ? [{ icon: rewardAsset.icon, label: rewardAsset.symbol }]
              : [],
            separator: "/",
            LabelProps: { variant: "body1" },
          }}
          buttonLabel="Max"
          ButtonProps={{
            onClick: () => setAmount(new BigNumber(0)),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: claimable
              ? {
                  title: <AccountBalanceWalletIcon color="primary" />,
                  balance: `${0} ${principalSymbol}`,
                }
              : undefined,
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
          {...defaultLabelProps("Pending Rewards", `${0} LP`)}
        />
        <Label
          {...defaultLabelProps(
            "Claimable Rewards",
            `${0} ${rewardAsset?.symbol}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Time until fully vested",
            `${0} days`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Vested", `${bond.vestingPeriod} days`)}
          mt={2}
        />
        <Label {...defaultLabelProps("ROI", `${0}%`)} mt={2} />
      </Box>
    </Box>
  );
};
