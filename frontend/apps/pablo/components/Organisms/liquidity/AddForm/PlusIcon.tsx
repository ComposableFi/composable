import { Box, Typography, useTheme } from "@mui/material";

export const PlusIcon = () => {
  const theme = useTheme();
  return (
    <Box mt={4} textAlign="center">
      <Box
        width={56}
        height={56}
        borderRadius="50%"
        display="flex"
        border={`2px solid ${theme.palette.primary.main}`}
        justifyContent="center"
        alignItems="center"
        margin="auto"
      >
        <Typography variant="h5">+</Typography>
      </Box>
    </Box>
  );
};
