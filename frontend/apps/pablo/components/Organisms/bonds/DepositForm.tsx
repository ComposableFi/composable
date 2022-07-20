import { BigNumberInput, Label } from "@/components/Atoms";
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
  closeConfirmingModal,
  openConfirmingModal,
  openWrongAmountEnteredModal,
} from "@/stores/ui/uiSlice";
import { PreviewPurchaseModal } from "./PreviewPurchaseModal";
import { useDispatch } from "react-redux";
import { WrongAmountEnteredModal } from "./WrongAmountEnteredModal";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { useAssetBalance } from "@/store/assets/hooks";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { ConfirmingModal } from "../swap/ConfirmingModal";
import { usePrincipalAssetSymbol } from "@/defi/hooks/bonds/usePrincipalAssetSymbol";
import { usePurchaseBond } from "@/defi/hooks/bonds";

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
} & BoxProps;

export const DepositForm: React.FC<DepositFormProps> = ({
  bond,
  offerId,
  ...boxProps
}) => {
  const dispatch = useDispatch();
  const theme = useTheme();

  const isOpenPreviewPurchaseModal = useAppSelector(
    (state) => state.ui.isConfirmingModalOpen
  );
  const isWrongAmountEnteredModalOpen = useAppSelector(
    (state) => state.ui.isWrongAmountEnteredModalOpen
  );

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);

  const { rewardAsset } = bond;
  const soldout = bond.selectedBondOffer ? bond.selectedBondOffer.nbOfBonds.eq(0) : true;
  const isWrongAmount = bond.roi.lt(0);

  const handleDeposit = () => {
    dispatch(
      isWrongAmount ? openWrongAmountEnteredModal() : openConfirmingModal()
    );
  };

  const handleButtonClick = () => {
    handleDeposit();
  };

  const principalBalance = useAssetBalance(DEFAULT_NETWORK_ID, bond.selectedBondOffer ? bond.selectedBondOffer.asset : "0")
  const buttonText = soldout ? "Sold out" : "Deposit";
  const disabled = !valid || soldout;

  const principalSymbol = usePrincipalAssetSymbol(bond.principalAsset);

  const youWillGet = useMemo(() => {
    if (bond.selectedBondOffer) {
      return bond.rewardAssetPerBond.times(amount.dp(0))
    }
    return new BigNumber(0);
  }, [amount, bond]);

  const maxYouCanBuy = useMemo(() => {
    if (bond.selectedBondOffer) {
      let amountOfBondsBuyable = principalBalance.div(bond.principalAssetPerBond).decimalPlaces(0, BigNumber.ROUND_FLOOR)
      return amountOfBondsBuyable;
    }
    return new BigNumber(0);
  }, [principalBalance, bond]);

  const purchaseBond = usePurchaseBond(bond.selectedBondOffer ? bond.selectedBondOffer.offerId : new BigNumber(-1), amount);

  const [isTxProcessing, setIsTxProcessing] = useState(false);

  const onPurchaseBond = async () => {
    dispatch(closeConfirmingModal());
    setIsTxProcessing(true);
    try {
      await purchaseBond();
      bond.updateBondInfo();
    } catch(e: any) {
      console.error(e)
    } finally {
      setIsTxProcessing(false);
    }
  }

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
            onClick: () => setAmount(new BigNumber(bond.selectedBondOffer ? bond.selectedBondOffer.nbOfBonds : 0)),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${bond.selectedBondOffer ? bond.selectedBondOffer.nbOfBonds : new BigNumber(0)} ${principalSymbol} Bonds`,
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
        <Label {...defaultLabelProps("Your balance", `${principalBalance.toFixed(2)} ${principalSymbol}`)} />
        <Label
          {...defaultLabelProps(
            "You will get",
            `${youWillGet.toFixed(2)} ${
              rewardAsset?.symbol
            }`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Max you can buy",
            `${maxYouCanBuy}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Vesting period", bond.vestingPeriod ? bond.vestingPeriod : "0")}
          mt={2}
        />
        <Label
          {...defaultLabelProps("ROI", `${bond.roi.toNumber()}%`)}
          mt={2}
        />
      </Box>
      <PreviewPurchaseModal
        onPurchaseBond={onPurchaseBond}
        bond={bond}
        rewardableTokens={bond.selectedBondOffer ? bond.selectedBondOffer.nbOfBonds.toString() : "0"}
        amount={amount}
        setAmount={setAmount}
        open={isOpenPreviewPurchaseModal}
      />
      <WrongAmountEnteredModal open={isWrongAmountEnteredModalOpen} />
      <ConfirmingModal open={isTxProcessing} />
    </Box>
  );
};