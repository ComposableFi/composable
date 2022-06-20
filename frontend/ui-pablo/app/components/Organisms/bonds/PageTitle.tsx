import { BaseAsset, PairAsset } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { Token, TokenId } from "@/defi/types";
import ArrowRightAltIcon from "@mui/icons-material/ArrowRightAlt";
import { Box, Typography, useTheme, BoxProps } from "@mui/material";

export type PageTitleProps = {
  tokenId1: TokenId;
  tokenId2: TokenId;
  rewardAsset: Token;
  iconSize?: number;
} & BoxProps;
export const PageTitle: React.FC<PageTitleProps> = ({
  tokenId1,
  tokenId2,
  rewardAsset,
  iconSize = 67,
  ...boxProps
}) => {
  const token1 = getToken(tokenId1);
  const token2 = getToken(tokenId2);
  return (
    <Box width="100%" {...boxProps}>
      <Box display="flex" justifyContent="center" alignItems="center" gap={3.5}>
        <PairAsset
          assets={[
            { icon: token1.icon, label: token1.symbol },
            { icon: token2.icon, label: token2.symbol },
          ]}
          label={`LP ${token1.symbol}-${token2.symbol}`}
          LabelProps={{ variant: "h4" }}
          iconSize={iconSize}
        />
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
