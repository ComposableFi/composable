import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const typographyOverrides = (): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      textTransform: "none",
    },
  },
});