import { alpha, Box, BoxProps, useTheme } from "@mui/material";

const FormWrapper: React.FC<BoxProps> = ({ children, ...boxProps }) => {
  const theme = useTheme();
  return (
    <Box
      borderRadius={1}
      p={{ xs: 2, md: 4 }}
      sx={{
        background: theme.palette.gradient.secondary,
        boxShadow: `-1px -1px ${alpha(
          theme.palette.common.white,
          theme.custom.opacity.light
        )}`,
      }}
      {...boxProps}
    >
      {children}
    </Box>
  );
};

export default FormWrapper;
