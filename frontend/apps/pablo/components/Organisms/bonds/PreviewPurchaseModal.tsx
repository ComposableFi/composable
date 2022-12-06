import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label } from "@/components/Atoms";
import { Box, Typography, useTheme, Button } from "@mui/material";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { setUiState } from "@/store/ui/ui.slice";
import { useAssetIdOraclePrice } from "@/defi/hooks";
import BigNumber from "bignumber.js";

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
  rewardableTokens: BigNumber;
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

  const { roi } = bond;
  const handleCancelBond = async () => {
    setUiState({ isOpenPreviewPurchaseModal: false })
  };

  // WIP
  const principalPriceUSD = new BigNumber(0);
  const bondMarketPrice = principalPriceUSD.times(bond.principalAssetPerBond);
  const rewardPriceUSD = useAssetIdOraclePrice(bond.selectedBondOffer ?  bond.selectedBondOffer.getRewardAssetId() as string : "none");
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
              "Amount of Bonds",
              `${amount}`
            )}
          />
          <Label
            mt={2}
            {...defaultLabelProps("You will get", `${rewardableTokens.times(amount)} ${bond.rewardAsset?.getSymbol()}`)}
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
