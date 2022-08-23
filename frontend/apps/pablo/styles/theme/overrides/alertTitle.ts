import { Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const alertTitle = (theme: Theme): Partial<OverridesStyleRules> => ({
  root: {
    margin: "0",
  },
});
