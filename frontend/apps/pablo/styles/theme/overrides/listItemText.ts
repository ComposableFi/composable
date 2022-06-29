import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const listItemTextOverrides = (): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    primary: {
      fontSize: "1.125rem",
    },
  },
});