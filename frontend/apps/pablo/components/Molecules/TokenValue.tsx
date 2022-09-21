import { Token } from "@/defi/types";
import { Box, BoxProps, Typography, TypographyProps } from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import { MockedAsset } from "@/store/assets/assets.types";

const defaultFlexBoxProps = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  gap: 3,
}

export type TokenValueProps = {
  token: Token | MockedAsset,
  value: string,
  LabelProps?: TypographyProps,
  ValueProps?: TypographyProps,
} & BoxProps;

export const TokenValue: React.FC<TokenValueProps> = ({
  token,
  value,
  LabelProps,
  ValueProps,
  ...boxProps
}) => {
  return (
    <Box {...defaultFlexBoxProps} {...boxProps}>
      <BaseAsset
        icon={token.icon}
        label={token.symbol}
        LabelProps={LabelProps}
      />
      <Typography variant="body1" {...ValueProps}>
        {value}
      </Typography>
    </Box>
  )
};