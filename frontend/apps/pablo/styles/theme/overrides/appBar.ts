import { Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const appBarOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      marginTop: 1,
      background: `transparent`,
    },
  },
});
