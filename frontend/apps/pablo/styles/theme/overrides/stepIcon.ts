import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const stepIconOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      width: theme.spacing(4),
      height: theme.spacing(4),
    },

    text: {
      fontWeight: 600,
    }
  },
});
