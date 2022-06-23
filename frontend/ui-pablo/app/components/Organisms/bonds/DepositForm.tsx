import { BigNumberInput, Label } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import {
  Box,
  Button,
  BoxProps,
  Typography,
  Theme,
  useTheme,
  alpha,
} from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import { useAppSelector } from "@/hooks/store";
import {
  openConfirmingModal,
  openWrongAmountEnteredModal,
} from "@/stores/ui/uiSlice";
import { PreviewPurchaseModal } from "./PreviewPurchaseModal";
import { useDispatch } from "react-redux";
import { WrongAmountEnteredModal } from "./WrongAmountEnteredModal";
import {
  IDepositSummary,
  ISupplySummary,
} from "../../../store/bonds/bonds.types";
import { useAsyncEffect } from "../../../hooks/useAsyncEffect";
import { MockedAsset } from "@/store/assets/assets.types";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";

const containerBoxProps = (theme: Theme) =>
  ({
    p: 4,
    borderRadius: 1.5,
    sx: {
      background: theme.palette.gradient.secondary,
      border: `1px solid ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`,
    },
  } as const);

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

export type DepositFormProps = {
  bond: SelectedBondOffer;
  offerId: string;
  depositSummary: IDepositSummary;
} & BoxProps;

export const DepositForm: React.FC<DepositFormProps> = ({
  bond,
  offerId,
  depositSummary,
  ...boxProps
}) => {
  const dispatch = useDispatch();
  const isOpenPreviewPurchaseModal = useAppSelector(
    (state) => state.ui.isConfirmingModalOpen
  );
  const isWrongAmountEnteredModalOpen = useAppSelector(
    (state) => state.ui.isWrongAmountEnteredModalOpen
  );
  const theme = useTheme();

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [approved, setApproved] = useState<boolean>(false);
  const [balance, setBalance] = useState("0");
  const [purchasableTokens, setPurchasableTokens] = useState("0");

  const { principalAsset, rewardAsset } = bond;
  const soldout = balance === "0";
  const isWrongAmount = false;

  const handleDeposit = () => {
    dispatch(
      isWrongAmount ? openWrongAmountEnteredModal() : openConfirmingModal()
    );
  };

  const handleButtonClick = () => {
    approved ? handleDeposit() : setApproved(true);
  };

  const buttonText = "Bond"
  // const buttonText = soldout
  //   ? "Sold out"
  //   : approved
  //   ? "Deposit"
  //   : `Approve bonding ${
  //       "baseAsset" in principalAsset && "quoteAsset" in principalAsset
  //         ? `${principalAsset.baseAsset.symbol}-${principalAsset.quote.symbol}`
  //         : principalAsset.symbol
  //     }`;
  const disabled = (approved && !valid) || soldout;

  useAsyncEffect(async () => {
    setPurchasableTokens(await depositSummary.purchasableTokens());

    setBalance(await depositSummary.userBalance());
  }, []);

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
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Typography variant="h6">Bond</Typography>
      <Box mt={6}>
        <BigNumberInput
          value={amount}
          setValue={setAmount}
          maxValue={new BigNumber(balance)}
          setValid={setValid}
          EndAdornmentAssetProps={{
            assets:
            principalAsset && (principalAsset as any).baseAsset && (principalAsset as any).quoteAsset 
                ? [
                  {
                    icon: (principalAsset as any).baseAsset.icon,
                    label: (principalAsset as any).baseAsset.symbol,
                  },
                  {
                    icon: (principalAsset as any).quoteAsset.icon,
                    label: (principalAsset as any).quoteAsset.symbol,
                  },
                ]
                : (principalAsset as MockedAsset).icon && (principalAsset as MockedAsset).symbol ? [{ icon: (principalAsset as MockedAsset).icon, label: (principalAsset as MockedAsset).symbol }] : [],
            separator: "/",
            LabelProps: { variant: "body1" },
          }}
          buttonLabel="Max"
          ButtonProps={{
            onClick: () => setAmount(new BigNumber(balance)),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${balance} ${principalSymbol}`,
            },
          }}
          disabled={soldout}
        />
      </Box>
      <Box mt={3}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          disabled={disabled}
          onClick={handleButtonClick}
        >
          {buttonText}
        </Button>
      </Box>
      <Box mt={6} sx={{ opacity: soldout ? "50%" : "100%" }}>
        <Label {...defaultLabelProps("Your balance", `${balance} LP`)} />
        <Label
          {...defaultLabelProps(
            "You will get",
            `${depositSummary.rewardableTokens(amount.toNumber())} ${
              rewardAsset?.symbol
            }`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Max you can buy",
            `${purchasableTokens} ${rewardAsset?.symbol}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Vesting period", depositSummary.vestingPeriod)}
          mt={2}
        />
        <Label
          {...defaultLabelProps("ROI", `${depositSummary.roi.toNumber()}%`)}
          mt={2}
        />
      </Box>
      <PreviewPurchaseModal
        offerId={+offerId}
        selectedBondOffer={bond}
        nbOfBonds={depositSummary.nbOfBonds}
        rewardableTokens={depositSummary.rewardableTokens(amount.toNumber())}
        amount={amount}
        setAmount={setAmount}
        open={isOpenPreviewPurchaseModal}
      />
      <WrongAmountEnteredModal open={isWrongAmountEnteredModalOpen} />
    </Box>
  );
};
