import { BaseAsset, PairAsset } from "@/components/Atoms";
import ArrowRightAltIcon from "@mui/icons-material/ArrowRightAlt";
import { Box, Typography, BoxProps } from "@mui/material";
import { MockedAsset } from "@/store/assets/assets.types";
import { BondPrincipalAsset } from "@/defi/hooks/bonds/useBondOffers";

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

  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

  return (
    <Box width="100%" {...boxProps}>
      <Box display="flex" justifyContent="center" alignItems="center" gap={3.5}>
        {baseAsset &&
        quoteAsset ? (
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
        ) : simplePrincipalAsset ? (
          <BaseAsset
            label={simplePrincipalAsset.symbol}
            icon={simplePrincipalAsset.icon}
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
