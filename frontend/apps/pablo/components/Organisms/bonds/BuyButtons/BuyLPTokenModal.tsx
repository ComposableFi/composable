import React, { useState } from "react";
import { Modal, ModalProps } from "@/components/Molecules";
import { alpha, Box, Button, IconButton, Theme, useTheme } from "@mui/material";
import { BondDetails } from "@/defi/types";
import { FormTitle } from "../../FormTitle";
import { getToken } from "tokens";
import { BigNumberInput } from "@/components/Atoms";
import { TransactionSettings } from "../../TransactionSettings";
import { PoolShare } from "../PoolShare";
import BigNumber from "bignumber.js";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { setUiState } from "@/store/ui/ui.slice";
import { Asset } from "shared";

const containerProps = (theme: Theme) => ({
  p: 4,
  borderRadius: 1,
  sx: {
    background: theme.palette.gradient.secondary,
    boxShadow: `-1px -1px ${alpha(
      theme.palette.common.white,
      theme.custom.opacity.light
    )}`,
  },
});

const labelProps = (
  label: string,
  balance: BigNumber,
  showBalance: boolean = true
) => ({
  label: label,
  BalanceProps: showBalance
    ? {
        title: <AccountBalanceWalletIcon color="primary" />,
        balance: balance.toString(),
      }
    : undefined,
});

export type BuyLPTokenModalProps = {
  bond: BondDetails;
} & ModalProps;

export const BuyLPTokenModal: React.FC<BuyLPTokenModalProps> = ({
  bond,
  open,
  onClose,
  ...modalProps
}) => {
  const theme = useTheme();

  const [balance1] = useState<BigNumber>(new BigNumber(500.35523));
  const [balance2] = useState<BigNumber>(new BigNumber(4560.9153));
  const [valid1, setValid1] = useState<boolean>(false);
  const [valid2, setValid2] = useState<boolean>(false);
  const [amount1, setAmount1] = useState<BigNumber>(new BigNumber(0));
  const [amount2, setAmount2] = useState<BigNumber>(new BigNumber(0));
  const token1 = getToken(bond.tokenId1);
  const token2 = getToken(bond.tokenId2);
  const [price] = useState<BigNumber>(new BigNumber(0.1));
  const [revertPrice] = useState<BigNumber>(new BigNumber(10));
  const [share] = useState<BigNumber>(new BigNumber(3.3));

  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true });
  };

  const onBackHandler = () => {
    onClose?.({}, "backdropClick");
  };

  const onApproveHandler = () => {
    // TODO: approve handler
  };

  return (
    <Modal open={open} onClose={onClose} {...modalProps}>
      <Box {...containerProps(theme)}>
        <FormTitle
          title={`Create ${token1.symbol}-${token2.symbol} LP`}
          onSettingHandler={onSettingHandler}
          onBackHandler={onBackHandler}
        />

        <Box mt={4}>
          <BigNumberInput
            value={amount1}
            setValue={setAmount1}
            maxValue={balance1}
            setValid={setValid1}
            EndAdornmentAssetProps={{
              assets: [],
              LabelProps: { variant: "body1" },
            }}
            LabelProps={labelProps(`${token1.symbol} Token`, balance1)}
            buttonLabel="Max"
            ButtonProps={{
              onClick: () => setAmount1(balance1),
            }}
          />
        </Box>

        <Box mt={4} textAlign="center">
          <IconButton
            sx={{
              width: 56,
              height: 56,
              border: `2px solid ${theme.palette.primary.main}`,
            }}
          >
            +
          </IconButton>
        </Box>

        <Box mt={4}>
          <BigNumberInput
            value={amount2}
            setValue={setAmount2}
            maxValue={balance2}
            setValid={setValid2}
            EndAdornmentAssetProps={{
              assets: [],
              LabelProps: { variant: "body1" },
            }}
            LabelProps={labelProps(`${token2.symbol} Token`, balance2)}
            buttonLabel="Max"
            ButtonProps={{
              onClick: () => setAmount2(balance2),
            }}
          />
        </Box>

        <PoolShare
          mt={4}
          assetOne={new Asset("", "", "", "ksm", undefined)}
          assetTwo={new Asset("", "", "", "pica", undefined)}
          poolShare={{}}
          price={price}
          revertPrice={revertPrice}
          share={share}
        />

        <Box mt={4}>
          <Button
            onClick={onApproveHandler}
            variant="contained"
            fullWidth
            disabled={!valid1 || !valid2}
          >
            Approve
          </Button>
        </Box>

        <TransactionSettings />
      </Box>
    </Modal>
  );
};
