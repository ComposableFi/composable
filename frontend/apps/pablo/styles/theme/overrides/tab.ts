import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const tabOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      textTransform: "none",
      color: theme.palette.common.white,
      padding: theme.spacing(2.25, 3),
      lineHeight: 1.45,
      gap: theme.spacing(2),
      [theme.breakpoints.down("sm")]: {
        gap: theme.spacing(1.5),
        padding: theme.spacing(1.875, 0),
      },
      borderBottom: `3px solid ${alpha(
        theme.palette.primary.main,
        theme.custom.opacity.light
      )}`,
      "&:hover": {
        background: alpha(
          theme.palette.primary.light,
          theme.custom.opacity.lighter
        ),
        borderBottom: `3px solid ${alpha(
          theme.palette.primary.light,
          theme.custom.opacity.main
        )}`,
      },
      "&.Mui-selected": {
        color: theme.palette.common.white,
        borderBottom: `2px solid ${theme.palette.primary.main}`,
      },
      "&.Mui-disabled": {
        color: theme.palette.common.white,
        borderBottom: `3px solid ${theme.palette.secondary.light}`,
        opacity: theme.custom.opacity.main,
      },
      "&.MuiTab-labelIcon": {
        minHeight: "auto",
      },
    },
  },
});