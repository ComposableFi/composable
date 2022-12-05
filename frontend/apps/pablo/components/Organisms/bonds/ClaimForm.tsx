import { BigNumberInput, Label } from "@/components/Atoms";
import {
  alpha,
  Box,
  BoxProps,
  Button,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import BigNumber from "bignumber.js";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { useVestingClaim } from "@/defi/hooks";
import {
  useBondedOfferVestingState,
  useBondOfferROI,
} from "@/store/bond/bond.slice";
import moment from "moment";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { ConfirmingModal } from "../swap/ConfirmingModal";

const containerBoxProps = (theme: Theme) => ({
  p: 4,
  borderRadius: 1,
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

  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { claimable, milliSecondsSinceVestingStart, pendingRewards } =
    useBondedOfferVestingState(
      bond.selectedBondOffer ? bond.selectedBondOffer.getBondOfferId() as string : "-"
    );
  const roi = useBondOfferROI(
    bond.selectedBondOffer ? bond.selectedBondOffer.getBondOfferId() as string : "-"
  );

  const handleClaim = useVestingClaim(
    bond.selectedBondOffer ? bond.selectedBondOffer.getRewardAssetId() as string : "",
    bond.vestingSchedules.length > 0
      ? bond.vestingSchedules[0].vestingScheduleId
      : new BigNumber(-1)
  );

  const isClaiming = usePendingExtrinsic(
    "claim",
    "vesting",
    selectedAccount?.address ?? "-"
  )

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Typography variant="h6">Claim</Typography>
      <ConfirmingModal open={isClaiming} />
      <Box mt={6}>
        <BigNumberInput
          disabled={true}
          value={claimable}
          maxValue={claimable}
          EndAdornmentAssetProps={{
            assets: rewardAsset
              ? [{ icon: rewardAsset.getIconUrl(), label: rewardAsset.getSymbol() }]
              : [],
            separator: "/",
            LabelProps: { variant: "body1" },
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: claimable
              ? {
                  title: <AccountBalanceWalletIcon color="primary" />,
                  balance: `${claimable.toFixed(2)} ${rewardAsset?.getSymbol()}`,
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
            `${pendingRewards.toFixed(2)} ${rewardAsset?.getSymbol()}`
          )}
        />
        <Label
          {...defaultLabelProps(
            "Claimable Rewards",
            `${claimable.toFixed(2)} ${rewardAsset?.getSymbol()}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Time vested",
            `${moment
              .duration(
                milliSecondsSinceVestingStart.toNumber(),
                "milliseconds"
              )
              .humanize()}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Vested",
            `${claimable.toFixed(2)} ${rewardAsset?.getSymbol()}`
          )}
          mt={2}
        />
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
