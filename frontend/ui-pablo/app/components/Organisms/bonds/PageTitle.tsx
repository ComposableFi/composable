import { BaseAsset, PairAsset } from "@/components/Atoms";
import { Token } from "@/defi/types";
import ArrowRightAltIcon from "@mui/icons-material/ArrowRightAlt";
import { Box, Typography, BoxProps } from "@mui/material";
import { MockedAsset } from "@/store/assets/assets.types";

export type PageTitleProps = {
  principalAsset:
    | {
        baseAsset: MockedAsset | undefined;
        quoteAsset: MockedAsset | undefined;
      }
    | MockedAsset
    | undefined;
  rewardAsset: MockedAsset | undefined;
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
        {principalAsset &&
        (principalAsset as any).baseAsset &&
        (principalAsset as any).quoteAsset ? (
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
            label={`LP ${(principalAsset as any).baseAsset.symbol}-${
              (principalAsset as any).quoteAsset.symbol
            }`}
            LabelProps={{ variant: "h4" }}
            iconSize={iconSize}
          />
        ) : principalAsset && (principalAsset as MockedAsset).icon &&
          (principalAsset as MockedAsset).symbol ? (
          <BaseAsset
            label={(principalAsset as MockedAsset).symbol}
            icon={(principalAsset as MockedAsset).icon}
            LabelProps={{ variant: "h4" }}
            iconSize={iconSize}
          />
        ) : null}
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
        Buy {rewardAsset?.name} while supplying tokens
      </Typography>
    </Box>
  );
};
