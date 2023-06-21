import { Token } from "tokens";
import { Box, BoxProps, Typography, TypographyProps } from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import { Asset } from "shared";

const defaultFlexBoxProps = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  gap: 3,
}

export type TokenValueProps = {
  token: Token | Asset,
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
        icon={
          token instanceof Asset ? token.getIconUrl() : token.icon
        }
        label={
          token instanceof Asset ? token.getSymbol() : token.symbol
        }
        LabelProps={LabelProps}
      />
      <Typography variant="body1" {...ValueProps}>
        {value}
      </Typography>
    </Box>
  )
};