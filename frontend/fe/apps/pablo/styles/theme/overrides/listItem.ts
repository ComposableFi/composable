import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const listItemOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      padding: theme.spacing(2, 2, 2, 3),
      height: theme.spacing(10),
      borderRadius: 9999,
      color: theme.palette.text.disabled,
      "&:hover": {
        "> .MuiListItemIcon-root": {
          opacity: 1,
          color: theme.palette.text.primary,
        },
        opacity: 1,
        color: theme.palette.text.primary,
        backgroundColor: alpha(
          theme.palette.primary.light,
          theme.custom.opacity.lighter
        ),
      },
      "&.Mui-selected": {
        "> .MuiListItemIcon-root": {
          color: theme.palette.primary.main,
          opacity: 1,
        },
        color: theme.palette.text.primary,
        opacity: 1,
        backgroundColor: alpha(
          theme.palette.primary.light,
          theme.custom.opacity.lighter
        ),
        "&:hover": {
          backgroundColor: alpha(
            theme.palette.primary.light,
            theme.custom.opacity.lighter
          ),
        },
      },
    },
  },
});
