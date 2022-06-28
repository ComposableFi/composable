import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const tableRowOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      height: "82px",
      "&:hover:not(.MuiTableRow-head)": {
        backgroundColor: alpha(theme.palette.primary.main, 0.1),
      },
    },
    head: {
      height: "fit-content",
    },
  },
});
