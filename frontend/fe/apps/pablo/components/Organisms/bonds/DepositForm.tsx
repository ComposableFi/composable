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
  bond: BondDetails;
} & BoxProps;

export const DepositForm: React.FC<DepositFormProps> = ({
  bond,
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
  const token1 = getToken(bond.tokenId1);
  const token2 = getToken(bond.tokenId2);
  const pablo = getToken("pablo");

  const [amount, setAmount] = useState<BigNumber>(new BigNumber(0));
  const [valid, setValid] = useState<boolean>(false);
  const [approved, setApproved] = useState<boolean>(false);

  const soldout = bond.balance.eq(0);
  const isWrongAmount = bond.discount_price < bond.market_price;

  const handleDeposit = () => {
    dispatch(isWrongAmount ? openWrongAmountEnteredModal() : openConfirmingModal());
  };

  const handleButtonClick = () => {
    approved ? handleDeposit() : setApproved(true);
  };

  const buttonText = soldout
                      ? "Sold out"
                      : (approved
                          ? "Deposit"
                          : `Approve bonding ${token1.symbol}-${token2.symbol}`
                      );
  const disabled = (approved && !valid) || soldout;

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Typography variant="h6">Bond</Typography>
      <Box mt={6}>
        <BigNumberInput
          value={amount}
          setValue={setAmount}
          maxValue={bond.balance}
          setValid={setValid}
          EndAdornmentAssetProps={{
            assets: [
              { icon: token1.icon, label: token1.symbol },
              { icon: token2.icon, label: token2.symbol },
            ],
            separator: "/",
            LabelProps: { variant: "body1" },
          }}
          buttonLabel="Max"
          ButtonProps={{
            onClick: () => setAmount(bond.balance),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          LabelProps={{
            label: "Amount",
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${bond.balance} ${token1.symbol}/${token2.symbol}`,
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
      <Box mt={6} sx={{opacity: soldout ? "50%" : "100%"}}>
        <Label {...defaultLabelProps("Your balance", `${bond.balance} LP`)} />
        <Label
          {...defaultLabelProps(
            "You will get",
            `${bond.rewardable_amount} ${pablo.symbol}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps(
            "Max you can buy",
            `${bond.buyable_amount} ${pablo.symbol}`
          )}
          mt={2}
        />
        <Label
          {...defaultLabelProps("Vesting term", `${bond.vesting_term} days`)}
          mt={2}
        />
        <Label {...defaultLabelProps("ROI", `${bond.roi}%`)} mt={2} />
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
