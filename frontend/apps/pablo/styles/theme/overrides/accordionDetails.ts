import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const accordionDetailsOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: { 
      padding: theme.spacing(0), 
    },
  },
});