import { Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const listItemIconOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      minWidth: theme.spacing(5.25),
      color: theme.palette.common.white,
      opacity: theme.custom.opacity.darker,
    },
  },
});
