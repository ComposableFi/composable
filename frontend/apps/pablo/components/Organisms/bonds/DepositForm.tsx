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
import { useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import { PreviewPurchaseModal } from "./PreviewPurchaseModal";
import { WrongAmountEnteredModal } from "./WrongAmountEnteredModal";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { useAssetBalance } from "@/defi/hooks";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { ConfirmingModal } from "../swap/ConfirmingModal";
import { usePurchaseBond } from "@/defi/hooks/bonds";
import { useUiSlice, setUiState } from "@/store/ui/ui.slice";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";

const containerBoxProps = (theme: Theme) =>
({
  p: 4,
  borderRadius: 1,
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
} & BoxProps;

export const DepositForm: React.FC<DepositFormProps> = ({
  bond,
  offerId,
  ...boxProps
}) => {
  const theme = useTheme();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const {
    isOpenPreviewPurchaseModal,
    isWrongAmountEnteredModalOpen
  } = useUiSlice();

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);

  const soldOut = bond.selectedBondOffer
    ? (bond.selectedBondOffer.getNumberOfBonds(true) as BigNumber).eq(0)
    : true;
  const isWrongAmount = bond.roi.lt(0);

  const handleDeposit = () => {
    setUiState(
      isWrongAmount ? { isWrongAmountEnteredModalOpen: true } : { isOpenPreviewPurchaseModal: true }
    )
  };

  const handleButtonClick = () => {
    handleDeposit();
  };

  const bondedAssetBalance = useAssetBalance(
    bond.bondedAsset_s,
    "picasso"
  );

  const buttonText = soldOut ? "Sold out" : "Deposit";
  const disabled = !valid || soldOut;
  const vestingTime = useBondVestingTime(bond.selectedBondOffer);

  const youWillGet = useMemo(() => {
    if (bond.selectedBondOffer) {
      return bond.rewardAssetPerBond.times(amount.dp(0));
    }
    return new BigNumber(0);
  }, [amount, bond]);

  const maxYouCanBuy = useMemo(() => {
    if (bond.selectedBondOffer) {
      let amountOfBondsBuyable = bondedAssetBalance
        .div(bond.principalAssetPerBond)
        .decimalPlaces(0, BigNumber.ROUND_FLOOR);
      return amountOfBondsBuyable.lt(bond.selectedBondOffer.getNumberOfBonds(true) as BigNumber)
        ? amountOfBondsBuyable
        : bond.selectedBondOffer.getNumberOfBonds(true) as BigNumber;
    }
    return new BigNumber(0);
  }, [bondedAssetBalance, bond]);

  const purchaseBond = usePurchaseBond(
    bond.selectedBondOffer ? (bond.selectedBondOffer.getBondOfferId(true) as BigNumber) : new BigNumber(-1),
    amount
  );

  const isPendingPurchase = usePendingExtrinsic(
    "bond",
    "bondedFinance",
    selectedAccount?.address ?? "-"
  )

  const onPurchaseBond = async () => {
    try {
      setUiState({ isOpenPreviewPurchaseModal: false })
      await purchaseBond();
      bond.updateBondInfo();
    } catch (e: any) {
      console.error(e);
    }
  };

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Typography variant="h6">Bond</Typography>
      <Box mt={6}>
        <BigNumberInput
          value={amount}
          setValue={setAmount}
          maxValue={maxYouCanBuy}
          setValid={setValid}
          buttonLabel="Max"
          ButtonProps={{
            onClick: () =>
              setAmount(
                new BigNumber(
                  bond.selectedBondOffer ? bond.selectedBondOffer.getNumberOfBonds(true) : 0
                )
              ),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${bond.selectedBondOffer
                  ? bond.selectedBondOffer.getNumberOfBonds(true) as BigNumber
                  : new BigNumber(0)
                } ${bond.bondedAsset_s?.getSymbol()} Bonds`,
            },
          }}
          disabled={soldOut}
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
      <Box mt={6} sx={{ opacity: soldOut ? "50%" : "100%" }}>
        <Label
          {...defaultLabelProps(
            "Your balance",
            `${bondedAssetBalance.toFixed(2)} ${bond.bondedAsset_s?.getSymbol()}`
          )}
        />
        <Label
          {...defaultLabelProps(
            "You will get",
            `${youWillGet.toFixed(2)} ${bond.bondedAsset_s?.getSymbol()}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Max you can buy", `${maxYouCanBuy}`)}
          mt={2}
        />
        <Label {...defaultLabelProps("Vesting period", vestingTime)} mt={2} />
        <Label
          {...defaultLabelProps(
            "ROI",
            `${bond.roi.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}%`
          )}
          mt={2}
        />
      </Box>
      <PreviewPurchaseModal
        onPurchaseBond={onPurchaseBond}
        bond={bond}
        rewardableTokens={
          bond.selectedBondOffer
            ? (bond.selectedBondOffer.getRewardAssetAmount(true) as BigNumber).div(
              bond.selectedBondOffer.getNumberOfBonds()
            )
            : new BigNumber(0)
        }
        amount={amount}
        setAmount={setAmount}
        open={isOpenPreviewPurchaseModal}
      />
      <WrongAmountEnteredModal open={isWrongAmountEnteredModalOpen} />
      <ConfirmingModal open={isPendingPurchase} />
    </Box>
  );
};
