import { Box, Skeleton, useTheme } from "@mui/material";
import { DropdownCombinedBigNumberInput } from "@/components";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { FC } from "react";
import { useMobile } from "@/hooks/responsive";
import {
  AssetDropdown,
  Config,
  Controlled,
} from "@/components/Organisms/liquidity/AddForm/types";

type LiquidityInputProps = Controlled &
  AssetDropdown & { config: Config | null } & {
    label: string;
    onValidationChange: (valid: boolean) => void;
  };

export const LiquidityInput: FC<LiquidityInputProps> = ({
  config,
  value,
  onChange,
  assetDropdownItems,
  label,
  onValidationChange,
}) => {
  const theme = useTheme();
  const isMobile = useMobile();
  if (config === null) {
    return <Skeleton variant="rectangular" width="100%" height="68px" />;
  }

  return (
    <Box mt={4}>
      <DropdownCombinedBigNumberInput
        // onMouseDown={(a) => console.log("onMouseDown", a)}
        maxValue={config.balance.free}
        setValid={onValidationChange}
        noBorder
        value={value}
        setValue={onChange}
        InputProps={{
          disabled: config.balance.free.isZero(),
        }}
        buttonLabel="Max"
        ButtonProps={{
          onClick: () => onChange(config.balance.free),
          sx: {
            padding: theme.spacing(1),
          },
        }}
        CombinedSelectProps={{
          disabled: false,
          value: (config.asset.getPicassoAssetId() as string) || "",
          setValue: (_v) => console.log("Setting token"),
          dropdownModal: true,
          forceHiddenLabel: isMobile ? true : false,
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
    </Box>
  );
};
