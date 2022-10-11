import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const buttonOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      textTransform: "none",
      boxShadow: "none",
      whiteSpace: "nowrap",
      minWidth: "max-content",
      "&:hover": {
        boxShadow: "none",
      },
      color: theme.palette.common.white,
      fontFamily: theme.custom.fontFamily.primary,
      lineHeight: theme.custom.lineHeight.small,
      borderRadius: theme.shape.borderRadius,
      "&.MuiButton-fullWidth": {
        padding: "initial",
      },
    },
    sizeLarge: {
      padding: theme.spacing(2.25, 6),
      fontSize: "1.25rem",
      height: "4rem",
      [theme.breakpoints.down("sm")]: {
        padding: theme.spacing(1.875, 6),
        fontSize: "1.125rem",
        height: "3.5rem",
      },
    },
    sizeMedium: {
      padding: theme.spacing(1.875, 4),
      fontSize: "1.125rem",
      height: "3.5rem",
      [theme.breakpoints.down("sm")]: {
        padding: theme.spacing(1.5, 4),
        fontSize: "1rem",
        height: "3rem",
      },
    },
    sizeSmall: {
      padding: theme.spacing(1.375, 3),
      fontSize: "1.125rem",
      height: "3rem",
      [theme.breakpoints.down("sm")]: {
        padding: theme.spacing(1.125, 3),
        fontSize: "1rem",
        height: "2.5rem",
      },
    },
    containedPrimary: {
      backgroundColor: theme.palette.primary.main,
      "&:hover": {
        backgroundColor: theme.palette.secondary.light,
      },
      "&:disabled": {
        backgroundColor: theme.palette.secondary.main,
        color: alpha(theme.palette.common.white, theme.custom.opacity.main),
      },
    },
    outlinedPrimary: {
      borderColor: theme.palette.primary.main,
      color: theme.palette.common.white,
      "&:hover": {
        backgroundColor: alpha(
          theme.palette.primary.light,
          theme.custom.opacity.light
        ),
        borderColor: theme.palette.primary.main,
      },
      "&:disabled": {
        borderColor: theme.palette.secondary.main,
        color: alpha(theme.palette.common.white, theme.custom.opacity.main),
      },
    },
    textPrimary: {
      color: theme.palette.primary.main,
      "&:hover": {
        backgroundColor: alpha(
          theme.palette.primary.light,
          theme.custom.opacity.light
        ),
      },
      "&:disabled": {
        color: theme.palette.secondary.main,
      },
    },
  },
});
