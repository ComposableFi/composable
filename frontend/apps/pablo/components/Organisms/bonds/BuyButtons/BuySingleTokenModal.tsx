import React, { useEffect, useState } from "react";
import {
  DropdownCombinedBigNumberInput,
  Modal,
  ModalProps,
} from "@/components/Molecules";
import {
  alpha,
  Box,
  Button,
  IconButton,
  Theme,
  Tooltip,
  Typography,
  useTheme,
} from "@mui/material";
import { TokenId } from "@/defi/types";
import BigNumber from "bignumber.js";
import { FormTitle } from "../../FormTitle";
import { useDispatch } from "react-redux";
import { openTransactionSettingsModal } from "@/stores/ui/uiSlice";
import { useMobile } from "@/hooks/responsive";
import { getToken, getTokenOptions } from "@/defi/Tokens";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { InfoOutlined, SwapVertRounded } from "@mui/icons-material";
import { BigNumberInput } from "@/components/Atoms";
import { TransactionSettings } from "../../TransactionSettings";

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

const defaultCombinedSelectProps = (isMobile: boolean) => ({
  dropdownModal: true,
  dropdownForceWidth: 320,
  forceHiddenLabel: isMobile ? true : false,
  renderShortLabel: true,
  borderLeft: false,
  minWidth: isMobile ? undefined : 150,
  searchable: true,
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

const defaultCombinedInputProps = (valid: boolean) => ({
  isAnchorable: true,
  noBorder: true,
  InputProps: {
    disabled: !valid,
  },
  referenceText: valid ? "50%" : undefined,
  buttonLabel: valid ? "Max" : undefined,
});

export type BuySingleTokenModalProps = {
  tokenId: TokenId;
} & ModalProps;

export const BuySingleTokenModal: React.FC<BuySingleTokenModalProps> = ({
  tokenId,
  open,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();
  const isMobile = useMobile();

  const [balance1] = useState<BigNumber>(new BigNumber(500.35523));
  const [balance2] = useState<BigNumber>(new BigNumber(4560.9153));
  const [price] = useState<BigNumber>(new BigNumber(100));
  const [valid1, setValid1] = useState<boolean>(false);
  const [valid2, setValid2] = useState<boolean>(false);
  const [tokenId1, setTokenId1] = useState<TokenId | "none">("none");
  const [amount1, setAmount1] = useState<BigNumber>(new BigNumber(0));
  const [amount2, setAmount2] = useState<BigNumber>(new BigNumber(0));
  const validToken1 = tokenId1 !== "none";
  const token2 = getToken(tokenId);

  useEffect(() => {
    setAmount1(new BigNumber(0));
    setAmount2(new BigNumber(0));
  }, [tokenId]);

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  const onSwapHandler = () => {
    // TODO: swap hander
  };

  return (
    <Modal open={open} {...modalProps}>
      <Box {...containerProps(theme)}>
        <FormTitle
          title="Swap"
          onSettingHandler={onSettingHandler}
          TitleProps={{
            textAlign: "left",
            width: "100%",
          }}
        />

        <Box mt={4}>
          <DropdownCombinedBigNumberInput
            maxValue={balance1}
            setValid={setValid1}
            value={amount1}
            setValue={setAmount1}
            {...defaultCombinedInputProps(validToken1)}
            ReferenceTextProps={{
              onClick: () => setAmount1(balance1.multipliedBy(0.5)),
              sx: {
                cursor: "pointer",
                "&:hover": {
                  color: theme.palette.primary.main,
                },
              },
            }}
            ButtonProps={{
              onClick: () => setAmount1(balance1),
            }}
            CombinedSelectProps={{
              value: tokenId1,
              setValue: setTokenId1,
              options: getTokenOptions("Select"),
              ...defaultCombinedSelectProps(isMobile),
            }}
            LabelProps={labelProps("From", balance1, validToken1)}
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
            <SwapVertRounded />
          </IconButton>
        </Box>

        <Box mt={4}>
          <BigNumberInput
            value={amount2}
            setValue={setAmount2}
            maxValue={balance2}
            setValid={setValid2}
            EndAdornmentAssetProps={{
              assets: [{ icon: token2.icon, label: "ETH" }],
              LabelProps: { variant: "body1" },
            }}
            LabelProps={labelProps("To", balance2)}
          />
        </Box>

        <Box
          mt={4}
          display="flex"
          justifyContent="center"
          alignItems="center"
          gap={2}
          height={26}
        >
          {validToken1 && (
            <>
              <Typography variant="body2">
                1 {token2.symbol} = {price.toFixed()}{" "}
                {getToken(tokenId1).symbol}
              </Typography>
              <Tooltip
                title={`1 ${token2.symbol} = ${price.toFixed()} ${
                  getToken(tokenId1).symbol
                }`}
                placement="top"
              >
                <InfoOutlined sx={{ color: theme.palette.primary.main }} />
              </Tooltip>
            </>
          )}
        </Box>

        <Box mt={4}>
          <Button
            onClick={onSwapHandler}
            variant="contained"
            fullWidth
            disabled={!valid1 || !valid2}
          >
            Swap
          </Button>
        </Box>

        <TransactionSettings />
      </Box>
    </Modal>
  );
};
