import { BaseAsset, PairAsset } from "@/components/Atoms";
import { BondDetails } from "@/defi/types";
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
  supplySummary: ISupplySummary;
} & BoxProps;

export const SupplySummary: React.FC<SupplySummaryProps> = ({
  supplySummary,
  ...boxProps
}) => {
  const theme = useTheme();
  const rewardAsset = supplySummary.rewardAsset;
  const [marketPriceInUSD, setMarketPriceInUSD] = useState(0);

  useAsyncEffect(async () => {
    setMarketPriceInUSD(await supplySummary.marketPriceInUSD());
  }, []);

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
          {"base" in supplySummary.principalAsset ? (
            <PairAsset
              assets={[
                {
                  icon: supplySummary.principalAsset.base.icon,
                  label: supplySummary.principalAsset.base.symbol,
                },
                {
                  icon: supplySummary.principalAsset.quote.icon,
                  label: supplySummary.principalAsset.quote.symbol,
                },
              ]}
              iconOnly
              iconSize={36}
            />
          ) : (
            <BaseAsset
              label={supplySummary.principalAsset.symbol}
              icon={supplySummary.principalAsset.icon}
            />
          )}
          <Typography variant="body1">
            {"base" in supplySummary.principalAsset
              ? `LP ${supplySummary.principalAsset.base.symbol}-${supplySummary.principalAsset.quote.symbol}`
              : supplySummary.principalAsset.symbol}
          </Typography>
        </Box>
        <ArrowRightAlt sx={{ color: "text.secondary" }} />
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Receive</Typography>
          <BaseAsset icon={rewardAsset.icon} iconSize={36} />
          <Typography variant="body1">
            {`${rewardAsset.symbol} - `}
            <Typography variant="body1" fontWeight="600" component="span">
              {`${supplySummary.roi}%`}
            </Typography>
          </Typography>
        </Box>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Vesting period</Typography>
        <TimerOutlinedIcon sx={{ width: 36, height: 36 }} />
        <Typography variant="body1">{supplySummary.vestingPeriod}</Typography>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Market Price</Typography>
        <Box display="flex" justifyContent="center" alignItems="center">
          {`$${marketPriceInUSD}`}
        </Box>
        <Typography variant="body1">{rewardAsset.symbol}</Typography>
      </Box>
    </Box>
  );
};
