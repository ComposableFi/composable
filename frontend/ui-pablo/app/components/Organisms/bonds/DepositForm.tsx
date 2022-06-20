import { BigNumberInput, Label } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { BondDetails } from "@/defi/types";
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
import { useEffect, useMemo, useState } from "react";
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
  depositSummary: IDepositSummary;
  supplySummary: ISupplySummary;
} & BoxProps;

export const DepositForm: React.FC<DepositFormProps> = ({
  depositSummary,
  supplySummary,
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

  const pablo = getToken("pablo");

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [approved, setApproved] = useState<boolean>(false);
  const [balance, setBalance] = useState("0");
  const [bondPriceInUSD, setBondPriceInUSD] = useState(0);
  const [marketPriceInUSD, setMarketPriceInUSD] = useState(0);
  const [purchasableTokens, setPurchasableTokens] = useState("0");

  const principalAsset = depositSummary.principalAsset;
  const soldout = balance === "0";
  const isWrongAmount = bondPriceInUSD < marketPriceInUSD;

  const handleDeposit = () => {
    dispatch(
      isWrongAmount ? openWrongAmountEnteredModal() : openConfirmingModal()
    );
  };

  const handleButtonClick = () => {
    approved ? handleDeposit() : setApproved(true);
  };

  const buttonText = soldout
    ? "Sold out"
    : approved
    ? "Deposit"
    : `Approve bonding ${
        "base" in principalAsset
          ? `${principalAsset.base.symbol}-${principalAsset.quote.symbol}`
          : principalAsset.symbol
      }`;
  const disabled = (approved && !valid) || soldout;

  useEffect(() => {
    (async () => {
      setPurchasableTokens(await depositSummary.purchasableTokens());
      setBondPriceInUSD(await supplySummary.bondPriceInUSD());
      setMarketPriceInUSD(await supplySummary.marketPriceInUSD());
      setBalance(await depositSummary.userBalance());
    })();
  }, []);

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
              "base" in principalAsset
                ? [
                    {
                      icon: principalAsset.base.icon,
                      label: principalAsset.base.symbol,
                    },
                    {
                      icon: principalAsset.quote.icon,
                      label: principalAsset.quote.symbol,
                    },
                  ]
                : [{ icon: principalAsset.icon, label: principalAsset.symbol }],
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
              balance: `${balance} ${
                "base" in principalAsset
                  ? `${principalAsset.base.symbol}/${principalAsset.quote.symbol}`
                  : principalAsset.symbol
              }`,
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
              pablo.symbol
            }`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Max you can buy",
            `${purchasableTokens} ${pablo.symbol}`
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
        bond={bond}
        amount={amount}
        setAmount={setAmount}
        open={isOpenPreviewPurchaseModal}
      />
      <WrongAmountEnteredModal open={isWrongAmountEnteredModalOpen} />
    </Box>
  );
};
