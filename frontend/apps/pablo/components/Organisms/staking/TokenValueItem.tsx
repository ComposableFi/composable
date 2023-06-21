import { Token } from "tokens";
import {
  alpha,
  Box,
  BoxProps,
  Theme,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import { BaseAsset } from "@/components/Atoms";

const defaultFlexBoxProps = (theme: Theme) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  gap: 3,
  borderRadius: 9999,
  px: 3,
  py: 2.25,
  sx: {
    background: alpha(theme.palette.common.white, theme.custom.opacity.lighter),
  }
} as const);

export type TokenValueItemProps = {
  token: Token,
  value: string,
  ValueProps?: TypographyProps,
} & BoxProps;

export const TokenValueItem: React.FC<TokenValueItemProps> = ({
  token,
  value,
  ValueProps,
  ...boxProps
}) => {
  const theme = useTheme();
  return (
    <Box {...defaultFlexBoxProps(theme)} {...boxProps} position="relative">
      <Box
        position="absolute"
        left={24}
      >
        <BaseAsset
          icon={token.icon}
        />
      </Box>
      <Typography variant="body1" {...ValueProps}>
        {value}
      </Typography>
    </Box>
  )
};