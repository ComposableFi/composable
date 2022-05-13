import { Label } from "@/components/Atoms";
import { DropdownCombinedBigNumberInput } from "@/components/Molecules";
import { FormTitle } from "@/components/Organisms/FormTitle";
import { Box, Button, useTheme, alpha, BoxProps, Grid, Typography, Theme, IconButton } from "@mui/material";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import { setCurrentStep, setCurrentSupply } from "@/stores/defi/pool";
import FormWrapper from "../FormWrapper";
import { TokenId } from "@/defi/types";
import { getToken, getTokenOptions, TOKEN_IDS } from "@/defi/Tokens";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useMobile } from "@/hooks/responsive";
import { TransactionSettings } from "@/components/Organisms/TransactionSettings";
import { openTransactionSettingsModal } from "@/stores/ui/uiSlice";

const selectLabelProps = (valid: boolean, label: string, balance: string) => ({
  label: label,
  BalanceProps:
    valid
      ? {
          title: <AccountBalanceWalletIcon color="primary" />,
          balance: balance,
        }
      : undefined,
} as const);

const priceBoxProps = (theme: Theme) => ({
  mt: 4,
  p: 2,
  borderRadius: 0.66,
  sx: {
    background: theme.palette.gradient.secondary,
  }
} as const);

const priceLabelProps = (label: string, balance?: string) => ({
  label: label,
  mb: 0,
  TypographyProps: {
    variant: "body1",
    fontWeight: 600
  },
  BalanceProps: {
    balance: balance,
    BalanceTypographyProps: {
      variant: "body1",
      fontWeight: 600,
    },
  },
} as const);

const combinedSelectProps = (
  tokenId: TokenId | "none",
  setValue: (v: BigNumber | TokenId) => any,
  isMobile?: boolean
) => ({
  value: tokenId,
  setValue: setValue,
  dropdownModal: true,
  forceHiddenLabel: isMobile ? true : false,
  options: getTokenOptions("Select"),
  borderLeft: false,
  minWidth: isMobile ? undefined : 150,
  searchable: true,
  renderShortLabel: true,
} as const);

const SetLiquidityStep: React.FC<BoxProps> = ({
  ...boxProps
}) => {

  const theme = useTheme();
  const isMobile = useMobile();
  const dispatch = useDispatch();

  const currentStep = useAppSelector((state) => state.pool.currentStep);

  const {
    tokenId1,
    tokenId2,
    balance1,
    balance2,
    pooledAmount1,
    pooledAmount2,
  } = useAppSelector((state) => state.pool.currentSupply);

  const currentPool = useAppSelector((state) => state.pool.currentPool);

  const [availableBalance] = useState<BigNumber>(new BigNumber(340));

  const [valid1, setValid1] = useState<boolean>(false);
  const [valid2, setValid2] = useState<boolean>(false);
  const [tokenToUSD1] = useState<BigNumber>(new BigNumber(1.6));
  const [tokenToUSD2] = useState<BigNumber>(new BigNumber(1.3));

  const validToken1 = tokenId1 !== "none";
  const validToken2 = tokenId2 !== "none";
  const usdAmount1 = pooledAmount1.multipliedBy(tokenToUSD1)
  const usdAmount2 = pooledAmount2.multipliedBy(tokenToUSD2)

  const setCurrentSupplyState = (property: string) => (v: BigNumber | TokenId | "none") => {
    dispatch(setCurrentSupply({ [property]: v }));
  };

  const onNextClickHandler = () => {
    dispatch(setCurrentStep(currentStep + 1));
  };

  const onBackHandler = () => {
    dispatch(setCurrentStep(currentStep - 1));
  };

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  return (
    <FormWrapper {...boxProps}>
      <FormTitle
        title="Set initial liquidity"
        onBackHandler={onBackHandler}
        onSettingHandler={onSettingHandler}
      />

      <Box mt={6}>
        <DropdownCombinedBigNumberInput
          maxValue={balance1}
          setValid={setValid1}
          noBorder
          value={pooledAmount1}
          setValue={setCurrentSupplyState("pooledAmount1")}
          InputProps={{
            disabled: !validToken1,
          }}
          buttonLabel={validToken1 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setCurrentSupplyState("pooledAmount1")(balance1),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={
            combinedSelectProps(tokenId1, setCurrentSupplyState("tokenId1"), isMobile)
          }
          LabelProps={selectLabelProps(validToken1, "Token 1", `${balance1}`)}
        />
        {valid1 && (
          <Typography variant="body2" mt={1.5}>
            {`≈$${usdAmount1}`}
          </Typography>
        )}
      </Box>

      <Box mt={4} textAlign="center">
        <IconButton
          sx={{
            width: 56,
            height: 56,
            border: `2px solid ${theme.palette.primary.main}`
          }}
        >
          <Typography variant="h5">+</Typography>
        </IconButton>
      </Box>

      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          maxValue={balance2}
          setValid={setValid2}
          noBorder
          value={pooledAmount2}
          setValue={setCurrentSupplyState("pooledAmount2")}
          InputProps={{
            disabled: !validToken2,
          }}
          buttonLabel={validToken2 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setCurrentSupplyState("pooledAmount2")(balance2),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={
            combinedSelectProps(tokenId2, setCurrentSupplyState("tokenId2"), isMobile)
          }
          LabelProps={selectLabelProps(validToken2, "Token 2", `${balance2}`)}
        />
        {valid2 && (
          <Typography variant="body2" mt={1.5}>
            {`≈$${usdAmount2}`}
          </Typography>
        )}
      </Box>

      <Box {...priceBoxProps(theme)}>
        <Label
          {...priceLabelProps("Total", `$${usdAmount1.plus(usdAmount2)}`)}
        />
        <Label
          {...priceLabelProps(`Available balance: $${availableBalance}`)}
          mt={0.5}
        />
      </Box>

      <Box mt={4}>
        <Button
          variant="contained"
          fullWidth
          size="large"
          onClick={onNextClickHandler}
          disabled={!valid1 || !valid2}
        >
          Preview
        </Button>
      </Box>
      <TransactionSettings />
    </FormWrapper>

  );
};

export default SetLiquidityStep;
