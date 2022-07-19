import { alpha, Box, BoxProps, Typography, useTheme } from "@mui/material"

export type BoxWrapperProps= {
  title?: string,
} & BoxProps;

export const BoxWrapper: React.FC<BoxWrapperProps> = ({
  title,
  children,
  ...boxProps
}) => {
  const theme = useTheme();
  return (
    <Box
      border={`1px solid ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`}
      p={4}
      borderRadius={0.6666}
      sx={{background: theme.palette.gradient.secondary}}
      {...boxProps}
    >
      {title && (
        <Typography variant="h6" mb={4}>{title}</Typography>
      )}
      {children}
    </Box>
  );
};
