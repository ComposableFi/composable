import { BaseAsset, PairAsset } from "@/components/Atoms";
import { ArrowRightAlt } from "@mui/icons-material";
import {
  alpha,
  Box,
  BoxProps,
  Theme,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import TimerOutlinedIcon from "@mui/icons-material/TimerOutlined";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { MockedAsset } from "@/store/assets/assets.types";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useCallback } from "react";
import usePrincipalAssetSymbol from "@/defi/hooks/bonds/usePrincipalAssetSymbol";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";

const containerBoxProps = (theme: Theme) => ({
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  p: 4,
  borderRadius: 1,
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
  const rewardPriceInUSD = useUSDPriceByAssetId(
    rewardAsset?.network?.[DEFAULT_NETWORK_ID] || "-"
  );

  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;
  const vestingTime = useBondVestingTime(bond.selectedBondOffer);

  const renderIcons = useCallback(() => {
    if (baseAsset && quoteAsset) {
      return (
        <PairAsset
          assets={[
            {
              icon: baseAsset.icon,
              label: baseAsset.symbol,
            },
            {
              icon: quoteAsset.icon,
              label: quoteAsset.symbol,
            },
          ]}
          iconOnly
          iconSize={36}
        />
      );
    } else if (simplePrincipalAsset) {
      return (
        <BaseAsset
          label={simplePrincipalAsset.symbol}
          icon={simplePrincipalAsset.icon}
          LabelProps={{ variant: "body1" }}
          iconSize={36}
        />
      );
    }
    return null;
  }, [simplePrincipalAsset, baseAsset, quoteAsset]);

  const principalAssetSymbol = usePrincipalAssetSymbol(principalAsset);

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
          {renderIcons()}
          <Typography variant="body1">{principalAssetSymbol}</Typography>
        </Box>
        <ArrowRightAlt sx={{ color: "text.secondary" }} />
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Receive</Typography>
          {rewardAsset && (
            <BaseAsset
              label={(rewardAsset as MockedAsset).symbol}
              icon={(rewardAsset as MockedAsset).icon}
              LabelProps={{ variant: "body1" }}
              iconSize={36}
            />
          )}
          <Typography variant="body1">
            {rewardAsset && `${rewardAsset.symbol}`}
            <Typography variant="body1" fontWeight="600" component="span">
              {`${" " + bond.roi}%`}
            </Typography>
          </Typography>
        </Box>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Vesting period</Typography>
        <TimerOutlinedIcon sx={{ width: 36, height: 36 }} />
        <Typography variant="body1">{vestingTime}</Typography>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Market Price</Typography>
        <Box display="flex" justifyContent="center" alignItems="center">
          {`$${rewardPriceInUSD}`}
        </Box>
        <Typography variant="body1">{rewardAsset?.symbol}</Typography>
      </Box>
    </Box>
  );
};
