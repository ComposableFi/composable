import { OverridesStyleRules } from "@mui/material/styles/overrides";
import { alpha, Theme } from "@mui/material";

export const drawerOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    paper: {
      backgroundColor: "transparent",
      backgroundImage: "none",
      boxShadow: "none",
      borderRight: theme.spacing(0),
      boxSizing: "border-box",
      padding: theme.spacing(0),
    },
  },
});