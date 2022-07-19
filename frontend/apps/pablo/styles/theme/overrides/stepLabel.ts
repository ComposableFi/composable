import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const stepLabelOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    label: {
      fontSize: 18,
      color: alpha(theme.palette.common.white, theme.custom.opacity.main),
      "&.Mui-active": {
        fontWeight: 300,
      },
    },
  },
});
