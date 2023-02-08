import { Box, BoxProps, useTheme } from "@mui/material";

export const ListItem = ({ children, sx, ...props }: BoxProps) => {
  const theme = useTheme();
  return (
    <Box
      component="li"
      sx={{
        fontSize: theme.typography.body1.fontSize,
        mt: 4,
        mb: 0.5,
        ...sx,
      }}
      {...props}
    >
      {children}
    </Box>
  );
};
