import React, { useEffect, useState } from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label } from "@/components/Atoms";
import { Box, Typography, useTheme, Button } from "@mui/material";

import { useDispatch } from "react-redux";
import { closeConfirmingModal, setMessage } from "@/stores/ui/uiSlice";
import BigNumber from "bignumber.js";
import {
  BondOffer,
  IDepositSummary,
  ISupplySummary,
} from "../../../store/bonds/bonds.types";
import { useAsyncEffect } from "../../../hooks/useAsyncEffect";
import { usePurchaseBond } from "../../../store/hooks/bond/usePurchaseBond";
import { useCancelOffer } from "../../../store/hooks/bond/useCancelOffer";

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
  principalAsset: BondOffer["asset"];
  bondPriceInUSD: ISupplySummary["bondPriceInUSD"];
  marketPriceInUSD: ISupplySummary["marketPriceInUSD"];
  amount: BigNumber;
  nbOfBonds: IDepositSummary["nbOfBonds"];
  rewardableTokens: string;
  setAmount: (v: BigNumber) => any;
} & ModalProps;

export const PreviewPurchaseModal: React.FC<PreviewPurchaseModalProps> = ({
  offerId,
  principalAsset,
  bondPriceInUSD,
  marketPriceInUSD,
  amount,
  nbOfBonds,
  rewardableTokens,
  setAmount,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const bond = usePurchaseBond();
  const cancel = useCancelOffer();

  const [bondPrice, setBondPrice] = useState(0);
  const [marketPrice, setMarketPrice] = useState(0);
  const discountPercent =
    marketPrice === 0 ? 0 : ((marketPrice - bondPrice) / marketPrice) * 100;

  const handlePurchaseBond = async () => {
    const result = await bond(offerId, nbOfBonds(amount.toNumber()));

    if (result) {
      dispatch(
        setMessage({
          title: "Transaction successful",
          text: "Purchase bond",
          link: "/",
          severity: "success",
        })
      );
    } else {
      dispatch(
        setMessage({
          title: "Transaction error",
          text: "Purchase bond",
          link: "/",
          severity: "error",
        })
      );
    }
    dispatch(closeConfirmingModal());
    setAmount(new BigNumber(0));
  };

  const handleCancelBond = async () => {
    const result = await cancel(offerId);
    if (result) {
      dispatch(
        setMessage({
          title: "Transaction successful",
          text: "Cancel offer",
          link: "/",
          severity: "success",
        })
      );
    } else {
      dispatch(
        setMessage({
          title: "Transaction error",
          text: "Cancel offer",
          link: "/",
          severity: "error",
        })
      );
    }
    dispatch(closeConfirmingModal());
  };

  useAsyncEffect(async () => {
    setBondPrice(await bondPriceInUSD());
    setMarketPrice(await marketPriceInUSD());
  }, []);

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
              `${amount} ${
                "base" in principalAsset
                  ? `${principalAsset.base.symbol}-${principalAsset.quote.symbol}`
                  : principalAsset.symbol
              }`
            )}
          />
          <Label
            mt={2}
            {...defaultLabelProps("You will get", `${rewardableTokens} PAB`)}
          />
          <Label mt={2} {...defaultLabelProps("Bond Price", `$${bondPrice}`)} />
          <Label
            mt={2}
            {...defaultLabelProps("Market Price", `$${marketPrice}`)}
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
