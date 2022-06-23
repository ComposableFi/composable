import { BaseAsset, PairAsset } from "@/components/Atoms";
import { ArrowRightAlt } from "@mui/icons-material";
import {
  Box,
  BoxProps,
  Typography,
  TypographyProps,
  Theme,
  useTheme,
  alpha,
} from "@mui/material";
import TimerOutlinedIcon from "@mui/icons-material/TimerOutlined";
import { ISupplySummary } from "../../../store/bonds/bonds.types";
import { useState } from "react";
import { useAsyncEffect } from "../../../hooks/useAsyncEffect";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { MockedAsset } from "@/store/assets/assets.types";

const containerBoxProps = (theme: Theme) => ({
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  p: 4,
  borderRadius: 1.5,
  sx: {
    background: theme.palette.gradient.secondary,
    border: `1px solid ${alpha(
      theme.palette.common.white,
      theme.custom.opacity.light
    )}`,
  },
});

const itemBoxProps: BoxProps = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: 3.5,
};

const itemTitleProps: TypographyProps = {
  variant: "body1",
  fontWeight: "600",
  color: "text.secondary",
};

export type SupplySummaryProps = {
  bond: SelectedBondOffer;
} & BoxProps;

export const SupplySummary: React.FC<SupplySummaryProps> = ({
  bond,
  ...boxProps
}) => {
  const theme = useTheme();
  const { principalAsset, rewardAsset } = bond;
  const [marketPriceInUSD, setMarketPriceInUSD] = useState(0);

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        gap={5.25}
      >
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Supply</Typography>
          {principalAsset && (principalAsset as any).baseAsset && (principalAsset as any).quoteAsset ? (
            <PairAsset
              assets={[
                {
                  icon: (principalAsset as any).baseAsset.icon,
                  label: (principalAsset as any).baseAsset.symbol,
                },
                {
                  icon: (principalAsset as any).quoteAsset.icon,
                  label: (principalAsset as any).quoteAsset.symbol,
                },
              ]}
              iconOnly
              iconSize={36}
            />
          ) : (principalAsset as MockedAsset).icon && (principalAsset as MockedAsset).symbol ? (
            <BaseAsset
            label={(principalAsset as MockedAsset).symbol}
            icon={(principalAsset as MockedAsset).icon}
            LabelProps={{ variant: "h4" }}
            iconSize={36}
          />
          ) : null}
          <Typography variant="body1">
            {principalAsset && (principalAsset as any).baseAsset && (principalAsset as any).quoteAsset ?
              `LP ${(principalAsset as any).baseAsset.symbol}-${(principalAsset as any).quoteAsset.symbol}`
              : (principalAsset as MockedAsset).symbol ? `${(principalAsset as MockedAsset).symbol}` : ""}
          </Typography>
        </Box>
        <ArrowRightAlt sx={{ color: "text.secondary" }} />
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Receive</Typography>
          {rewardAsset &&
          <BaseAsset icon={rewardAsset.icon} iconSize={36} />
          }
          <Typography variant="body1">
            {rewardAsset && `${rewardAsset.symbol}`}
            <Typography variant="body1" fontWeight="600" component="span">
              {`${bond.roi}%`}
            </Typography>
          </Typography>
        </Box>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Vesting period</Typography>
        <TimerOutlinedIcon sx={{ width: 36, height: 36 }} />
        <Typography variant="body1">{bond.vestingPeriod}</Typography>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Market Price</Typography>
        <Box display="flex" justifyContent="center" alignItems="center">
          {`$${marketPriceInUSD}`}
        </Box>
        <Typography variant="body1">{rewardAsset?.symbol}</Typography>
      </Box>
    </Box>
  );
};
