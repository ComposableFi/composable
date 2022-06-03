import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const tableOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      borderCollapse: "separate",
      borderSpacing: theme.spacing(0, 2),
      marginTop: "0",
    },
  },
});