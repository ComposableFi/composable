import { BaseAsset, PairAsset } from "@/components/Atoms";
import ArrowRightAltIcon from "@mui/icons-material/ArrowRightAlt";
import { Box, Typography, BoxProps } from "@mui/material";
import { useCallback } from "react";
import { Asset, LiquidityProviderToken } from "shared";

export type PageTitleProps = {
  bondedAsset_s: LiquidityProviderToken | Asset | undefined;
  rewardAsset: Asset | undefined;
  iconSize?: number;
} & BoxProps;
export const PageTitle: React.FC<PageTitleProps> = ({
  bondedAsset_s,
  rewardAsset,
  iconSize = 67,
  ...boxProps
}) => {
  const renderIcons = useCallback(() => {
    if (bondedAsset_s instanceof LiquidityProviderToken) {
      const underlyingAssets = bondedAsset_s.getUnderlyingAssetJSON();
      return (
        <PairAsset
          assets={underlyingAssets}
          label={`${bondedAsset_s.getSymbol()}`}
          LabelProps={{ variant: "h4" }}
          iconSize={iconSize}
        />
      );
    } else if (bondedAsset_s instanceof Asset) {
      return (
        <BaseAsset
          label={bondedAsset_s.getSymbol()}
          icon={bondedAsset_s.getIconUrl()}
          LabelProps={{ variant: "h4" }}
          iconSize={iconSize}
        />
      );
    }
    return null;
  }, [bondedAsset_s, iconSize]);

  return (
    <Box width="100%" {...boxProps}>
      <Box display="flex" justifyContent="center" alignItems="center" gap={3.5}>
        {renderIcons()}
        <ArrowRightAltIcon sx={{ color: "text.secondary" }} />
        {rewardAsset && (
          <BaseAsset
            icon={rewardAsset.getIconUrl()}
            iconSize={67}
            label={rewardAsset.getSymbol()}
            LabelProps={{ variant: "h4" }}
          />
        )}
      </Box>
      <Typography
        mt={3}
        variant="body1"
        color="text.secondary"
        textAlign="center"
        fontWeight="normal"
      >
        {rewardAsset ? `Buy ${rewardAsset.getSymbol()} while supplying tokens` : "-"}
      </Typography>
    </Box>
  );
};
