import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const chipOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      padding: "0.4rem",
      borderRadius: theme.shape.borderRadius,
      fontFamily: '"Be Vietnam Pro"',
      fontSize: "1rem",
      backgroundColor: alpha(
        theme.palette.primary.main,
        theme.custom.opacity.light
      ),
      color: theme.palette.warning.main,
      "& .MuiChip-icon": {
        height: "1.2rem",
      },
      "&.MuiChip-colorInfo": {
        backgroundColor: alpha(
          theme.palette.info.main,
          theme.custom.opacity.light
        ),
        color: theme.palette.info.main,
      },
      "&.MuiChip-colorSuccess": {
        backgroundColor: alpha(
          theme.palette.success.main,
          theme.custom.opacity.light
        ),
        color: theme.palette.success.main,
      },
      "&.MuiChip-colorError": {
        backgroundColor: alpha(
          theme.palette.info.main,
          theme.custom.opacity.light
        ),
        color: theme.palette.error.main,
      },
      "&.MuiChip-colorWarning": {
        backgroundColor: alpha(
          theme.palette.warning.main,
          theme.custom.opacity.light
        ),
        color: theme.palette.warning.main,
      },
    },
  },
});
