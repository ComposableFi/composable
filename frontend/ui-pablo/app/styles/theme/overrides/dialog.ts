import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const dialogOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    paper: {
      backgroundColor: "transparent",
      backgroundImage: "none",
      boxShadow: "none",
      "> .MuiIconButton-root": {
        color: alpha(theme.palette.common.white, theme.custom.opacity.darker),
        "&:hover": {
          color: theme.palette.common.white,
        },
      },
    },
  },
});
