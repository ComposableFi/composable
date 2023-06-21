import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const typographyOverrides = (): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      textTransform: "none",
    },
    gutterBottom: {
      marginBottom: "1.81rem",
    },
  },
});
