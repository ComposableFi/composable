import { Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const gridOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    item: {
      borderRadius: theme.shape.borderRadius,
    },
  },
});
