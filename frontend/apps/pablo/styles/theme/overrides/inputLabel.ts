import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const inputLabelOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      position: "relative",
      top: theme.spacing(-4),
      left: theme.spacing(-2),
    },
  },
});