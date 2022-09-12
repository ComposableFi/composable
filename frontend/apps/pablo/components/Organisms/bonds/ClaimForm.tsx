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
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { useVestingClaim } from "@/defi/hooks";
import {
  useBondedOfferVestingState,
  useBondOfferROI,
} from "@/store/bond/bond.slice";
import moment from "moment";

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
  const vestingTime = useBondVestingTime(bond.selectedBondOffer);
  const { claimable, milliSecondsSinceVestingStart, pendingRewards } =
    useBondedOfferVestingState(
      bond.selectedBondOffer ? bond.selectedBondOffer.offerId.toString() : "-"
    );
  const roi = useBondOfferROI(
    bond.selectedBondOffer ? bond.selectedBondOffer.offerId.toString() : "-"
  );

  const handleClaim = useVestingClaim(
    bond.selectedBondOffer ? bond.selectedBondOffer.reward.asset : "",
    bond.vestingSchedules.length > 0
      ? bond.vestingSchedules[0].vestingScheduleId
      : new BigNumber(-1)
  );

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
          {...defaultLabelProps(
            "Time vested",
            `${moment
              .duration(milliSecondsSinceVestingStart.toNumber(), "milliseconds")
              .humanize()}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Vested",
            `${claimable.toFixed(2)} ${rewardAsset?.symbol}`
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
