import { Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const tooltipOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    popper: {
      background: "transparent",
    },
    tooltip: {
      background: theme.palette.common.white,
      color: theme.palette.primary.dark,
      borderRadius: theme.shape.borderRadius,
      fontFamily: theme.custom.fontFamily.primary,
      fontSize: "1.125rem",
      padding: theme.spacing(3),
      [theme.breakpoints.down("sm")]: {
        padding: theme.spacing(1),
      },
      maxWidth: "max-content",
    },
    arrow: {
      color: theme.palette.common.white,
    },
  },
});
