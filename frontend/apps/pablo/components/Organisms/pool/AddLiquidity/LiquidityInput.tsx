import { Box, Skeleton, Typography, useTheme } from "@mui/material";
import { DropdownCombinedBigNumberInput } from "@/components";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { FC } from "react";
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
  if (config === null) {
    return <Skeleton variant="rectangular" width="100%" height="68px" />;
  }

  const threshold = config.balance.free
    .minus(transactionFee.multipliedBy(siteConfig.gasFeeMultiplier))
    .minus(gasFeeEd)
    .dp(gasFeeToken?.getDecimals(DEFAULT_NETWORK_ID) ?? 12);
  const inputBalance = config.balance.free.minus(
    config.asset.getExistentialDeposit(DEFAULT_NETWORK_ID) ?? 0
  );
  const maxAmount =
    config.asset.getSymbol() === gasFeeToken?.getSymbol()
      ? threshold.gte(0)
        ? threshold
        : new BigNumber(0)
      : inputBalance.lte(0)
      ? new BigNumber(0)
      : inputBalance;

  const fieldValue = value.gt(maxAmount) ? maxAmount : value;
  const isValueGreaterThanMax = value.gt(maxAmount);
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
