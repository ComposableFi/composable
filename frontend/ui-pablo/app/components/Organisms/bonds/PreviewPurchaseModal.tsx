import React, { useEffect, useState } from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label } from "@/components/Atoms";
import { Box, Typography, useTheme, Button } from "@mui/material";

import { useDispatch } from "react-redux";
import { closeConfirmingModal, setMessage } from "@/stores/ui/uiSlice";
import BigNumber from "bignumber.js";
import { ISupplySummary } from "../../../store/bonds/bonds.types";
import { useAsyncEffect } from "../../../hooks/useAsyncEffect";

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
  supplySummary: ISupplySummary;
  amount: BigNumber;
  rewardableTokens: string;
} & ModalProps;

export const PreviewPurchaseModal: React.FC<PreviewPurchaseModalProps> = ({
  supplySummary,
  amount,
  rewardableTokens,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const principalAsset = supplySummary.principalAsset;

  const [bondPrice, setBondPrice] = useState(0);
  const [marketPrice, setMarketPrice] = useState(0);
  const discountPercent =
    marketPrice === 0 ? 0 : ((marketPrice - bondPrice) / marketPrice) * 100;

  const handlePurchaseBond = () => {
    dispatch(
      setMessage({
        title: "Transaction successfull",
        text: "Bond",
        link: "/",
        severity: "success",
      })
    );
    dispatch(closeConfirmingModal());
  };

  const handleCancelBond = () => {
    dispatch(closeConfirmingModal());
  };

  useAsyncEffect(async () => {
    setBondPrice(await supplySummary.bondPriceInUSD());
    setMarketPrice(await supplySummary.marketPriceInUSD());
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
