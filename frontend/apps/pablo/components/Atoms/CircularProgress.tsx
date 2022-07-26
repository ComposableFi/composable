import {
  CircularProgress as MuiCircularProgress,
  CircularProgressProps,
  Box,
  alpha,
} from "@mui/material";
import { useTheme } from "@mui/material/styles";

export const CircularProgress: React.FC<CircularProgressProps> = ({
  ...props
}) => {
  const theme = useTheme();

  return (
    <Box sx={{ position: "relative" }}>
      <MuiCircularProgress
        variant="determinate"
        sx={{
          color: alpha(theme.palette.primary.main, theme.custom.opacity.light),
        }}
        thickness={4}
        {...props}
        value={100}
      />
      <MuiCircularProgress
        variant="indeterminate"
        disableShrink
        color="primary"
        sx={{
          position: "absolute",
          left: 0,
        }}
        thickness={4}
        {...props}
      />
    </Box>
  );
};
