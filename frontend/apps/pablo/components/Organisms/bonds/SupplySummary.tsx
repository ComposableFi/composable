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
import { useCallback } from "react";
import { Asset, LiquidityProviderToken } from "shared";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";
import { useAssetIdOraclePrice } from "@/defi/hooks";

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
  const { bondedAsset_s, rewardAsset } = bond;
  const rewardPriceInUSD = useAssetIdOraclePrice(
    rewardAsset?.getPicassoAssetId() as string
  );

  const vestingTime = useBondVestingTime(bond.selectedBondOffer);
  const renderIcons = useCallback(() => {
    if (bondedAsset_s instanceof LiquidityProviderToken) {
      const underlyingAssets = bondedAsset_s.getUnderlyingAssetJSON();
      return (
        <PairAsset
          assets={underlyingAssets}
          iconOnly
          iconSize={36}
        />
      );
    } else if (bondedAsset_s instanceof Asset) {
      return (
        <BaseAsset
          label={bondedAsset_s.getSymbol()}
          icon={bondedAsset_s.getIconUrl()}
          LabelProps={{ variant: "body1" }}
          iconSize={36}
        />
      );
    }
    return null;
  }, [bondedAsset_s]);

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
          <Typography variant="body1">{bondedAsset_s?.getSymbol()}</Typography>
        </Box>
        <ArrowRightAlt sx={{ color: "text.secondary" }} />
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Receive</Typography>
          {rewardAsset && (
            <BaseAsset
              label={rewardAsset?.getSymbol()}
              icon={rewardAsset?.getIconUrl()}
              LabelProps={{ variant: "body1" }}
              iconSize={36}
            />
          )}
          <Typography variant="body1">
            {rewardAsset && `${rewardAsset.getSymbol()}`}&nbsp;
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
        <Typography variant="body1">{rewardAsset?.getSymbol()}</Typography>
      </Box>
    </Box>
  );
};
