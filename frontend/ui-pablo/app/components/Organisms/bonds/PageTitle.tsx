import { BaseAsset, PairAsset } from "@/components/Atoms";
import ArrowRightAltIcon from "@mui/icons-material/ArrowRightAlt";
import { Box, Typography, BoxProps } from "@mui/material";
import { MockedAsset } from "@/store/assets/assets.types";
import { BondPrincipalAsset } from "@/defi/types";
import { useCallback } from "react";

export type PageTitleProps = {
  principalAsset: BondPrincipalAsset;
  rewardAsset: MockedAsset | undefined;
  iconSize?: number;
} & BoxProps;
export const PageTitle: React.FC<PageTitleProps> = ({
  principalAsset,
  rewardAsset,
  iconSize = 67,
  ...boxProps
}) => {
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

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
          label={`LP ${baseAsset.symbol}-${quoteAsset.symbol}`}
          LabelProps={{ variant: "h4" }}
          iconSize={iconSize}
        />
      );
    } else if (simplePrincipalAsset) {
      return (
        <BaseAsset
          label={simplePrincipalAsset.symbol}
          icon={simplePrincipalAsset.icon}
          LabelProps={{ variant: "h4" }}
          iconSize={iconSize}
        />
      );
    }
    return null;
  }, [simplePrincipalAsset, baseAsset, quoteAsset, iconSize]);

  return (
    <Box width="100%" {...boxProps}>
      <Box display="flex" justifyContent="center" alignItems="center" gap={3.5}>
        {renderIcons()}
        <ArrowRightAltIcon sx={{ color: "text.secondary" }} />
        {rewardAsset && (
          <BaseAsset
            icon={rewardAsset.icon}
            iconSize={67}
            label={rewardAsset.symbol}
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
        {rewardAsset ? `Buy ${rewardAsset.name} while supplying tokens` : "-"}
      </Typography>
    </Box>
  );
};
