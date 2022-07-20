import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label } from "@/components/Atoms";
import { Box, Typography, useTheme, Button } from "@mui/material";
import { useDispatch } from "react-redux";
import { closeConfirmingModal } from "@/stores/ui/uiSlice";
import BigNumber from "bignumber.js";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import { usePrincipalAssetSymbol } from "@/defi/hooks/bonds/usePrincipalAssetSymbol";

const defaultLabelProps = (label: string, balance: string) =>
  ({
    label: label,
    TypographyProps: { variant: "body1" },
    BalanceProps: {
      balance: balance,
      BalanceTypographyProps: {
        variant: "body1",
      },
    },
  } as const);

export type PreviewPurchaseModalProps = {
  bond: SelectedBondOffer,
  amount: BigNumber;
  rewardableTokens: string;
  onPurchaseBond: () => Promise<any>;
  setAmount: (v: BigNumber) => any;
} & ModalProps;

export const PreviewPurchaseModal: React.FC<PreviewPurchaseModalProps> = ({
  bond,
  amount,
  rewardableTokens,
  onPurchaseBond,
  setAmount,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const { principalAsset, roi } = bond;
  const handleCancelBond = async () => {
    dispatch(closeConfirmingModal());
  };

  let principalSymbol = usePrincipalAssetSymbol(bond.principalAsset);
  const principalPriceUSD = useUSDPriceByAssetId(bond.selectedBondOffer ?  bond.selectedBondOffer.asset : "none")
  const bondMarketPrice = principalPriceUSD.times(bond.principalAssetPerBond);
  const rewardPriceUSD = useUSDPriceByAssetId(bond.selectedBondOffer ?  bond.selectedBondOffer.reward.asset : "none");
  const totalRewardsPrice = rewardPriceUSD.times(bond.rewardAssetPerBond);

  return (
    <Modal onClose={handleCancelBond} {...modalProps}>
      <Box
        sx={{
          width: 480,
          margin: "auto",
          [theme.breakpoints.down("sm")]: {
            width: "100%",
            p: 2,
          },
        }}
      >
        <Typography variant="h5" textAlign="center">
          Purchase bond
        </Typography>
        <Typography
          variant="subtitle1"
          textAlign="center"
          color="text.secondary"
          mt={2}
        >
          Are you sure you want to bond for a negative discount? You will lose
          money if you do this...
        </Typography>

        <Box mt={8}>
          <Label
            {...defaultLabelProps(
              "Bonding",
              `${amount} ${[principalSymbol]}`
            )}
          />
          <Label
            mt={2}
            {...defaultLabelProps("You will get", `${rewardableTokens} ${bond.rewardAsset?.symbol}`)}
          />
          <Label mt={2} {...defaultLabelProps("Bond Price", `$${bondMarketPrice.toFixed(2)}`)} />
          <Label
            mt={2}
            {...defaultLabelProps("Market Price", `$${totalRewardsPrice.toFixed(2)}`)}
          />
          <Label
            mt={2}
            {...defaultLabelProps("Discount", `${roi.toFormat(2)}%`)}
          />
        </Box>

        <Box mt={8}>
          <Button
            variant="contained"
            fullWidth
            size="large"
            onClick={onPurchaseBond}
          >
            Purchase bond
          </Button>
        </Box>

        <Box mt={4}>
          <Button
            variant="text"
            fullWidth
            size="large"
            onClick={handleCancelBond}
          >
            Cancel
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};
