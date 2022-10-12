import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const alertOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      borderRadius: theme.shape.borderRadius,
      padding: theme.spacing(1.75, 3),
      backdropFilter: "blur(32px)",
      "& .MuiSvgIcon-root": {
        fill: alpha(theme.palette.common.white, theme.custom.opacity.dark),
        ml: theme.spacing(0.5),
      },
      "& .MuiAlert-icon": {
        display: "grid",
        alignItems: "center",
      },
      "& .MuiAlert-message": {
        display: "flex",
        alignItems: "center",
        width: "100%",
        padding: 0,
      },
      "& .MuiAlert-action": {
        display: "flex",
        alignItems: "center",
        padding: 0,
        marginRight: 0,
      },
    },
    filledSuccess: {
      background: alpha(theme.palette.success.main, theme.custom.opacity.light),
    },
    filledError: {
      background: alpha(theme.palette.error.main, theme.custom.opacity.light),
    },
    filledInfo: {
      background: alpha(theme.palette.info.main, theme.custom.opacity.light),
    },
    filledWarning: {
      background: alpha(theme.palette.warning.main, theme.custom.opacity.light),
    },
  },
});
