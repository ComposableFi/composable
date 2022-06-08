import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { BondDetails, TokenId } from "@/defi/types";
import {
  Box,
  Typography,
  useTheme,
  Button,
} from "@mui/material";

import { useDispatch } from "react-redux";
import {
  closeConfirmingModal, setMessage,
} from "@/stores/ui/uiSlice";
import { useAppSelector } from "@/hooks/store";
import { setSelectedBond } from "@/stores/defi/bonds";
import BigNumber from "bignumber.js";

const defaultLabelProps = (label: string, balance: string) => ({
  label: label,
  TypographyProps: {variant: "body1"},
  BalanceProps: {
    balance: balance,
    BalanceTypographyProps: {
      variant: "body1",
    }
  }
} as const);

export type PreviewPurchaseModalProps = {
  bond: BondDetails,
  amount: BigNumber,
  setAmount: (v: BigNumber) => any
} & ModalProps;

export const PreviewPurchaseModal: React.FC<PreviewPurchaseModalProps> = ({
  bond,
  amount,
  setAmount,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const {
    tokenId1,
    tokenId2,
    balance,
    discount_price,
    market_price,
  } = useAppSelector((state) => state.bonds.selectedBond);

  const token1 = getToken(tokenId1 as TokenId);
  const token2 = getToken(tokenId2 as TokenId);
  const bondPrice = amount.multipliedBy(discount_price);
  const marketPrice = amount.multipliedBy(market_price);
  const discountPercent = market_price.minus(discount_price).dividedBy(market_price).multipliedBy(100);

  const handlePurchaseBond = () => {
    dispatch(setSelectedBond({
      balance: balance.minus(amount),
      pending_amount: amount,
    }));

    dispatch(setMessage(
      {
        title: "Transaction successfull",
        text: "Bond",
        link: "/",
        severity: "success",
      }
    ));

    setAmount(new BigNumber(0));

    dispatch(closeConfirmingModal());
  };

  const handleCancelBond = () => {
    dispatch(closeConfirmingModal());
  };

  return (
    <Modal
      onClose={handleCancelBond}
      {...modalProps}
    >
      <Box
        sx={{
          width: 480,
          margin: "auto",
          [theme.breakpoints.down('sm')]: {
            width: '100%',
            p: 2,
          },
        }}
      >
        <Typography variant="h5" textAlign="center">
          Purchase bond
        </Typography>
        <Typography variant="subtitle1" textAlign="center" color="text.secondary" mt={2}>
          Are you sure you want to bond for a negative discount? You will lose money if you do this...
        </Typography>

        <Box mt={8}>
          <Label {...defaultLabelProps("Bonding", `${amount} ${token1.symbol}-${token2.symbol}`)} />
          <Label mt={2} {...defaultLabelProps("You will get", `0 PAB`)} />
          <Label mt={2} {...defaultLabelProps("Bond Price", `$${bondPrice}`)} />
          <Label mt={2} {...defaultLabelProps("Market Price", `$${marketPrice}`)} />
          <Label mt={2} {...defaultLabelProps("Discount", `${discountPercent.toFormat(2)}%`)} />
        </Box>

        <Box mt={8}>
          <Button
            variant="contained"
            fullWidth
            size="large"
            onClick={handlePurchaseBond}
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
            Cancel bond
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};

