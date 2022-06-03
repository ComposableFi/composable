import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const accordionOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      padding: theme.spacing(0),
      "&.Mui-expanded": {
        margin: theme.spacing(0),
      },
      "&.MuiPaper-root": {
        backgroundColor: "transparent",
      },
    },
  },
});