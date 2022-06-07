import { alpha, Theme } from "@mui/material";
import { grey } from "@mui/material/colors";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const linearProgressOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      backgroundColor: "transparent",
      height: theme.spacing(0.5),
      "&::before": {
        background: "transparent",
        borderTop: `1px dashed ${alpha(theme.palette.common.white, theme.custom.opacity.darker)}`,
        position: "absolute",
        top: 2,
        left: 0,
        width: "100%",
        content: '""',
      },
    },
    barColorPrimary: {
      backgroundColor: grey[500],
    },
  },
});