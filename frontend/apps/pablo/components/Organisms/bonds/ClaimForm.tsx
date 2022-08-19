import { BigNumberInput, Label } from "@/components/Atoms";
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
import { useState } from "react";
import BigNumber from "bignumber.js";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import usePrincipalAssetSymbol from "@/defi/hooks/bonds/usePrincipalAssetSymbol";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";
import useBondOfferROI from "@/defi/hooks/bonds/useBondOfferROI";
import {
  calculateClaimableAt,
  DEFAULT_NETWORK_ID,
  DEFAULT_UI_FORMAT_DECIMALS,
} from "@/defi/utils";
import useBlockNumber from "@/defi/hooks/useBlockNumber";
import { useVestingClaim } from "@/defi/hooks";

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
  const { rewardAsset } = bond;

  const blockNumber = useBlockNumber(DEFAULT_NETWORK_ID);
  const vestingTime = useBondVestingTime(bond.selectedBondOffer);
  const roi = useBondOfferROI(bond.selectedBondOffer);

  const { pendingRewards, claimable } = calculateClaimableAt(
    bond.vestingSchedules[0],
    blockNumber
  );

  const handleClaim = useVestingClaim(
    bond.selectedBondOffer ? bond.selectedBondOffer.reward.asset : "",
    bond.vestingSchedules.length > 0
      ? bond.vestingSchedules[0].vestingScheduleId
      : new BigNumber(0)
  );

  const principalSymbol = usePrincipalAssetSymbol(bond.principalAsset);

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Typography variant="h6">Claim</Typography>
      <Box mt={6}>
        <BigNumberInput
          disabled={true}
          value={claimable}
          maxValue={claimable}
          EndAdornmentAssetProps={{
            assets: rewardAsset
              ? [{ icon: rewardAsset.icon, label: rewardAsset.symbol }]
              : [],
            separator: "/",
            LabelProps: { variant: "body1" },
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: claimable
              ? {
                  title: <AccountBalanceWalletIcon color="primary" />,
                  balance: `${claimable.toFixed(2)} ${rewardAsset?.symbol}`,
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
          disabled={claimable.lte(0)}
          onClick={handleClaim}
        >
          Claim
        </Button>
      </Box>
      <Box mt={6}>
        <Label
          {...defaultLabelProps(
            "Pending Rewards",
            `${pendingRewards.toFixed(2)} ${rewardAsset?.symbol}`
          )}
        />
        <Label
          {...defaultLabelProps(
            "Claimable Rewards",
            `${claimable.toFixed(2)} ${rewardAsset?.symbol}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Time until fully vested", `${0} days`)}
          mt={2}
        />
        <Label {...defaultLabelProps("Vested", `${vestingTime}`)} mt={2} />
        <Label
          {...defaultLabelProps(
            "ROI",
            `${roi.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}%`
          )}
          mt={2}
        />
      </Box>
    </Box>
  );
};
