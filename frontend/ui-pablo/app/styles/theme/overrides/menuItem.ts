import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const menuItemOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    gutters: {
      padding: theme.spacing(2.25, 3),
      "&.Mui-selected": {
        background: alpha(
          theme.palette.primary.light,
          theme.custom.opacity.light
        ),
        "&:hover": {
          background: alpha(
            theme.palette.primary.light,
            theme.custom.opacity.light
          ),
        },
      },
      "&:hover": {
        background: alpha(
          theme.palette.primary.light,
          theme.custom.opacity.lighter
        ),
      },
      [theme.breakpoints.down("sm")]: {
        padding: theme.spacing(1.875, 2),
      },
    },
  },
});