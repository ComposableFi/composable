import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const appBarOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      marginTop: 1,
      backdropFilter: `blur(${theme.spacing(10)})`,
      background: `transparent`,
    },
  },
});