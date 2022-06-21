import { BaseAsset, PairAsset } from "@/components/Atoms";
import { Token } from "@/defi/types";
import ArrowRightAltIcon from "@mui/icons-material/ArrowRightAlt";
import { Box, Typography, BoxProps } from "@mui/material";
import { BondOffer } from "../../../store/bonds/bonds.types";

export type PageTitleProps = {
  principalAsset: BondOffer["asset"];
  rewardAsset: Token;
  iconSize?: number;
} & BoxProps;
export const PageTitle: React.FC<PageTitleProps> = ({
  principalAsset,
  rewardAsset,
  iconSize = 67,
  ...boxProps
}) => {
  return (
    <Box width="100%" {...boxProps}>
      <Box display="flex" justifyContent="center" alignItems="center" gap={3.5}>
        {"base" in principalAsset ? (
          <PairAsset
            assets={[
              {
                icon: principalAsset.base.icon,
                label: principalAsset.base.symbol,
              },
              {
                icon: principalAsset.quote.icon,
                label: principalAsset.quote.symbol,
              },
            ]}
            label={`LP ${principalAsset.base.symbol}-${principalAsset.quote.symbol}`}
            LabelProps={{ variant: "h4" }}
            iconSize={iconSize}
          />
        ) : (
          <BaseAsset
            label={principalAsset.symbol}
            icon={principalAsset.icon}
            LabelProps={{ variant: "h4" }}
            iconSize={iconSize}
          />
        )}
        <ArrowRightAltIcon sx={{ color: "text.secondary" }} />
        <BaseAsset
          icon={rewardAsset.icon}
          iconSize={67}
          label={rewardAsset.symbol}
          LabelProps={{ variant: "h4" }}
        />
      </Box>
      <Typography
        mt={3}
        variant="body1"
        color="text.secondary"
        textAlign="center"
        fontWeight="normal"
      >
        Buy {rewardAsset.name} while supplying tokens
      </Typography>
    </Box>
  );
};
