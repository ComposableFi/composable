import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const inputAdornmentOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      flexShrink: 0,
    },
    positionStart: {
      marginRight: theme.spacing(4),
    },
  },
});