import { Box, BoxProps, useTheme } from "@mui/material";

export const List = ({ children, ...props }: BoxProps) => {
  const theme = useTheme();
  return (
    <Box component="ol" {...props}>
      {children}
    </Box>
  );
};
