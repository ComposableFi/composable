import React, { useEffect, useMemo, useState } from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label } from "@/components/Atoms";
import { Box, Typography, useTheme, Button } from "@mui/material";

import { useDispatch } from "react-redux";
import { closeConfirmingModal, setMessage } from "@/stores/ui/uiSlice";
import BigNumber from "bignumber.js";
import {
  IDepositSummary,
  ISupplySummary,
} from "../../../store/bonds/bonds.types";
import { useAsyncEffect } from "../../../hooks/useAsyncEffect";
import { usePurchaseBond } from "../../../store/hooks/bond/usePurchaseBond";
import { useCancelOffer } from "../../../store/hooks/bond/useCancelOffer";
import { BondOffer } from "@/defi/types";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { MockedAsset } from "@/store/assets/assets.types";

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
  offerId: number;
  selectedBondOffer: SelectedBondOffer,
  amount: BigNumber;
  nbOfBonds: IDepositSummary["nbOfBonds"];
  rewardableTokens: string;
  setAmount: (v: BigNumber) => any;
} & ModalProps;

export const PreviewPurchaseModal: React.FC<PreviewPurchaseModalProps> = ({
  offerId,
  selectedBondOffer,
  amount,
  nbOfBonds,
  rewardableTokens,
  setAmount,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const { principalAsset } = selectedBondOffer;
  const bond = usePurchaseBond();
  const cancel = useCancelOffer();

  const discountPercent = 0
    // marketPrice === 0 ? 0 : ((marketPrice - bondPrice) / marketPrice) * 100;

  const handlePurchaseBond = async () => {
    await bond(offerId, nbOfBonds(amount.toNumber()));
    dispatch(closeConfirmingModal());
    setAmount(new BigNumber(0));
  };

  const handleCancelBond = async () => {
    await cancel(offerId);
    dispatch(closeConfirmingModal());
  };

  let principalSymbol = useMemo(() => {
    return principalAsset &&
      (principalAsset as any).baseAsset &&
      (principalAsset as any).quoteAsset
      ? (principalAsset as any).baseAsset.symbol +
          "/" +
          (principalAsset as any).quoteAsset
      : (principalAsset as MockedAsset).symbol
      ? (principalAsset as MockedAsset).symbol
      : "";
  }, [principalAsset]);

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
            {...defaultLabelProps("You will get", `${rewardableTokens} PAB`)}
          />
          <Label mt={2} {...defaultLabelProps("Bond Price", `$${0}`)} />
          <Label
            mt={2}
            {...defaultLabelProps("Market Price", `$${0}`)}
          />
          <Label
            mt={2}
            {...defaultLabelProps("Discount", `${discountPercent}%`)}
          />
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
