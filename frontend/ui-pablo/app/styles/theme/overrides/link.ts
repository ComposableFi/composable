import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const linkOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      color: theme.palette.primary.light,
      "&:hover": {
        color: alpha(theme.palette.primary.light, theme.custom.opacity.darker),
      },
    },
  },
});