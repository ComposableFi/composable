import { AlertColor } from "@mui/material";
import { createTheme, Theme } from "@mui/material/styles";
import { alpha } from "@mui/system";
import { VariantType } from "notistack";

const themeOverride = (defaultTheme: Theme, variant: VariantType) =>
  createTheme(defaultTheme, {
    palette: {
      info: {
        main: "#0286FF",
      },
      success: {
        main: "#009B6D",
      },
      error: {
        main: "#E10036",
      },
      warning: {
        main: "#C59A04",
      },
    },
    components: {
      MuiAlert: {
        styleOverrides: {
          action: {
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            padding: 0,
          },
          message: {
            display: "flex",
            flexDirection: "column",
            alignItems: "start",
            justifyContent: "flex-start",
            color: defaultTheme.palette.common.white,
          },
          filled: {
            minWidth: "500px",
            backdropFilter: "blur(32px)",
          },
          filledInfo: {
            backgroundColor: alpha(
              defaultTheme.palette[variant as AlertColor].main,
              0.1
            ),
            color: defaultTheme.palette.common.white,
          },
          filledSuccess: {
            backgroundColor: alpha(
              defaultTheme.palette[variant as AlertColor].main,
              0.1
            ),
            color: defaultTheme.palette.common.white,
          },
          filledError: {
            backgroundColor: alpha(
              defaultTheme.palette[variant as AlertColor].main,
              0.1
            ),
            color: defaultTheme.palette.common.white,
          },
          filledWarning: {
            backgroundColor: alpha(
              defaultTheme.palette[variant as AlertColor].main,
              0.1
            ),
            color: defaultTheme.palette.common.white,
          },
        },
      },
      MuiLinearProgress: {
        styleOverrides: {
          root: {
            marginTop: "-4px",
            "&::before": {
              background: "initial",
              borderTop: `4px solid ${
                defaultTheme.palette[variant as AlertColor].light
              }`,
              position: "absolute",
              top: 0,
              left: 0,
              width: "100%",
              content: '""',
            },
          },
        },
      },
    },
  }) as any;

export { themeOverride };
