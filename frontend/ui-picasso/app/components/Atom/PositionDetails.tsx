import { FC, ReactNode } from "react";
import { Box, Stack, Typography, useTheme } from "@mui/material";
import { alpha } from "@mui/material/styles";

type PositionDetailsProps = {
  children: ReactNode;
};

const PositionDetails: FC<PositionDetailsProps> = ({ children }) => {
  const theme = useTheme();

  return (
    <Box
      borderRadius={theme.spacing(1)}
      bgcolor={alpha(theme.palette.common.white, 0.02)}
    >
      <Typography textAlign="center" variant="h6">
        Details
      </Typography>

      <Stack mt="2.125rem" direction="column">
        {children}
      </Stack>
    </Box>
  );
};

export default PositionDetails;
