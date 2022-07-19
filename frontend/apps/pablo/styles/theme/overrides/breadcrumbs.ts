import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const breadcrumbsOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    separator: {
      marginLeft: theme.spacing(1.5),
      marginRight: theme.spacing(1.5),
    },
  },
});