import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const accordionSummaryOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      paddingLeft: theme.spacing(0),
      margin: theme.spacing(0),
      minHeight: "none",
      "&.Mui-expanded": {
        margin: theme.spacing(0),
        minHeight: "none",
      },
      ".MuiAccordionSummary-content": {
        margin: theme.spacing(0),
      },
    },
  },
});