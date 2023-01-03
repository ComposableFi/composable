import { Box, Skeleton, Typography, useTheme } from "@mui/material";
import { DropdownCombinedBigNumberInput } from "@/components";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { FC, useMemo } from "react";
import { useMobile } from "@/hooks/responsive";
import {
  AssetDropdown,
  Config,
  Controlled,
} from "@/components/Organisms/liquidity/AddForm/types";
import { Asset } from "shared";
import BigNumber from "bignumber.js";
import siteConfig from "@/constants/config";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import useStore from "@/store/useStore";

type LiquidityInputProps = Controlled &
  AssetDropdown & { config: Config | null } & {
    label: string;
    onValidationChange: (valid: boolean) => void;
    gasFeeToken: Asset | null;
    transactionFee: BigNumber;
    gasFeeEd: BigNumber;
  };

export const LiquidityInput: FC<LiquidityInputProps> = ({
  config,
  value,
  onChange,
  assetDropdownItems,
  label,
  onValidationChange,
  transactionFee,
  gasFeeToken,
  gasFeeEd,
}) => {
  const theme = useTheme();
  const isMobile = useMobile();
  const isBYOGLoaded = useStore((store) => store.byog.isLoaded);
  const threshold = useMemo(() => {
    if (!config) return new BigNumber(0);
    return config.balance.free
      .minus(transactionFee.multipliedBy(siteConfig.gasFeeMultiplier))
      .minus(gasFeeEd)
      .dp(gasFeeToken?.getDecimals(DEFAULT_NETWORK_ID) ?? 12);
  }, [config, gasFeeEd, gasFeeToken, transactionFee]);
  const inputBalance = useMemo(() => {
    if (!config) return new BigNumber(0);
    return config.balance.free.minus(
      config.asset.getExistentialDeposit(DEFAULT_NETWORK_ID) ?? 0
    );
  }, [config]);
  const maxAmount = useMemo(() => {
    if (!config) return new BigNumber(0);
    return config.asset.getSymbol() === gasFeeToken?.getSymbol()
      ? threshold.gte(0)
        ? threshold
        : new BigNumber(0)
      : inputBalance.lte(0)
      ? new BigNumber(0)
      : inputBalance;
  }, [config, gasFeeToken, inputBalance, threshold]);

  const isValueGreaterThanMax = useMemo(() => {
    return value.gt(maxAmount);
  }, [value, maxAmount]);

  const fieldValue = useMemo(() => {
    return isValueGreaterThanMax ? maxAmount : value;
  }, [value, maxAmount, isValueGreaterThanMax]);

  if (config === null || !isBYOGLoaded) {
    return <Skeleton variant="rectangular" width="100%" height="68px" />;
  }

  return (
    <Box mt={4}>
      <DropdownCombinedBigNumberInput
        maxValue={maxAmount}
        setValid={(validationStatus) =>
          onValidationChange(validationStatus && !isValueGreaterThanMax)
        }
        noBorder
        value={fieldValue}
        setValue={onChange}
        InputProps={{
          disabled: maxAmount.isZero(),
        }}
        buttonLabel="Max"
        ButtonProps={{
          onClick: () => onChange(maxAmount),
          disabled: maxAmount.isZero(),
          sx: {
            padding: theme.spacing(1),
          },
        }}
        CombinedSelectProps={{
          disabled: false,
          value: (config.asset.getPicassoAssetId() as string) || "",
          setValue: (_v) => console.log("Setting token"),
          dropdownModal: true,
          forceHiddenLabel: isMobile,
          options: assetDropdownItems,
          borderLeft: false,
          minWidth: isMobile ? undefined : 150,
          searchable: true,
        }}
        LabelProps={{
          label: label,
          BalanceProps: {
            title: <AccountBalanceWalletIcon color="primary" />,
            balance: `${config.balance.free.toString()}`,
          },
        }}
      />
      {isValueGreaterThanMax || maxAmount.isZero() ? (
        <Typography
          variant="caption"
          color="error"
          textAlign="left"
          sx={{ display: "flex", ml: 2 }}
        >
          {isValueGreaterThanMax
            ? "Your token balance is too low to fulfill the pool ratio."
            : "Your token balance is too low to add liquidity."}
        </Typography>
      ) : null}
    </Box>
  );
};
