import {
  alpha,
  CommonColors,
  createTheme,
  PaletteMode,
  ThemeOptions,
} from "@mui/material";
import { grey } from "@mui/material/colors";
import { OverridesStyleRules } from "@mui/material/styles/overrides";
import { Shadows } from "@mui/material/styles/shadows";

declare module "@mui/material/styles" {
  interface Theme {
    custom: {
      opacity: {
        darker: number;
        dark: number;
        main: number;
        light: number;
        lighter: number;
      };
    };
  }

  interface CommonColors {
    darkWhite: string;
  }

  interface ThemeOptions {
    custom?: {
      opacity?: {
        darker?: number;
        dark?: number;
        main?: number;
        light?: number;
        lighter?: number;
      };
    };
  }

  interface Palette {
    featured: {
      lemon: "string";
    };
    common: CommonColors;
  }
}

declare module "@mui/material/Typography" {
  interface TypographyPropsVariantOverrides {
    inputLabel: true;
  }
}
const theme = createTheme({
  shadows: Array(25).fill("none") as Shadows,
});

export const brandPalette = {
  primary: {
    main: "#FF8500",
    light: "#F15700",
    dark: "#0C0600",
  },
  secondary: {
    main: "#AA2900",
    light: "#372B1E",
    dark: "#150B00",
  },
  info: {
    main: "#0286FF",
    light: "#004686",
    dark: "#001931",
  },
  success: {
    main: "#009B6D",
    light: "#005A3F",
    dark: "#002C1E",
  },
  error: {
    main: "#E10036",
    light: "#850020",
    dark: "#450011",
  },
  warning: {
    main: "#C59A04",
    light: "#846700",
    dark: "#2E2400",
  },
  featured: {
    lemon: "#33C500",
  },
  background: {
    default: "#000000",
    paper: alpha("#F15700", 0.02),
  },
  modal: {
    umber:
      "linear-gradient(180deg, rgba(12, 6, 0, 0.8) 0%, rgba(21, 11, 0, 0.8) 82.99%)",
    umberCut:
      "linear-gradient(180deg, rgba(12, 6, 0, 0) 63.64%, rgba(12, 6, 0, 0.8) 116.45%)",
  },
  common: {
    white: "#FFFFFF",
    darkWhite: alpha("#FFFFFF", 0.6),
  },
};

const customThemeOptions = {
  custom: {
    opacity: {
      darker: 0.6,
      dark: 0.5,
      main: 0.3,
      light: 0.1,
      lighter: 0.05,
    },
  },
};

const brandTypography = {
  fontFamily: '"TentangNanti", "Be Vietnam Pro", sans-serif',
  htmlFontSize: 16,
  h1: {
    fontFamily: "TentangNanti",
    lineHeight: "120%",
    fontSize: "6rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "4.5rem",
    },
  },
  h2: {
    fontFamily: "TentangNanti",
    lineHeight: "120%",
    fontSize: "4.5rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "4rem",
    },
  },
  h3: {
    fontFamily: "TentangNanti",
    lineHeight: "140%",
    fontSize: "4rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "3rem",
    },
  },
  h4: {
    fontFamily: "TentangNanti",
    lineHeight: "160%",
    fontSize: "3rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "2rem",
    },
  },
  h5: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "160%",
    fontSize: "2rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1.5rem",
    },
  },
  h6: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "160%",
    fontSize: "1.5rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1.25rem",
    },
  },
  subtitle1: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "160%",
    fontSize: "1.25rem",
    fontWeight: "lighter",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1.125rem",
    },
  },
  subtitle2: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "160%",
    fontSize: "1.125rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1rem",
    },
  },
  body1: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "155%",
    fontSize: "1.25rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1.125rem",
    },
  },
  body2: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "155%",
    fontSize: "1.125rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1rem",
    },
  },
  button: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "145%",
    fontSize: "1.125rem",
    width: "max-content",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1rem",
    },
  },
  caption: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "160%",
    fontSize: "0.75rem",
    fontWeight: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "0.625rem",
    },
  },
  inputLabel: {
    fontFamily: '"Be Vietnam Pro"',
    lineHeight: "155%",
    fontSize: "1rem",
    fontWeight: "lighter",
    [theme.breakpoints.down("sm")]: {
      fontSize: "0.875rem",
    },
  },
};

const buttonOverrides: Partial<OverridesStyleRules> = {
  styleOverrides: {
    root: {
      textTransform: "none",
      boxShadow: "none",
      whiteSpace: "no-wrap",
      minWidth: "maxContent",
      "&:hover": {
        boxShadow: "none",
      },
      color: brandPalette.common.white,
      fontFamily: "Be Vietnam Pro",
      lineHeight: "116%",
    },
    sizeLarge: {
      padding: "1.125rem",
      fontSize: "1.25rem",
      height: "4rem",
      [theme.breakpoints.down("sm")]: {
        padding: "0.9375rem",
        fontSize: "1.125rem",
        height: "3.5rem",
      },
    },
    sizeMedium: {
      padding: "0.9375rem",
      fontSize: "1.125rem",
      height: "3.5rem",
      [theme.breakpoints.down("sm")]: {
        padding: "0.75rem",
        fontSize: "1rem",
        height: "3rem",
      },
    },
    sizeSmall: {
      padding: "0.6875rem",
      fontSize: "1.125rem",
      height: "3rem",
      [theme.breakpoints.down("sm")]: {
        padding: "0.5625rem",
        fontSize: "1rem",
        height: "2.5rem",
      },
    },
    containedPrimary: {
      backgroundColor: brandPalette.primary.light,
      "&:hover": {
        backgroundColor: alpha(brandPalette.primary.light, 0.1),
      },
      "&:disabled": {
        backgroundColor: brandPalette.secondary.light,
        color: alpha(brandPalette.common.white, 0.3),
      },
    },
    outlinedPrimary: {
      borderColor: brandPalette.primary.light,
      color: brandPalette.common.white,
      "&:hover": {
        backgroundColor: alpha(brandPalette.primary.main, 0.15),
        borderColor: brandPalette.primary.light,
      },
      "&:disabled": {
        borderColor: "#372B1E",
        color: alpha(brandPalette.common.white, 0.3),
      },
    },
    textPrimary: {
      color: brandPalette.primary.light,
      "&:hover": {
        backgroundColor: alpha(brandPalette.primary.light, 0.05),
      },
      "&:disabled": {
        color: "#372B1E",
      },
    },
  },
};

const switchOverrides: Partial<OverridesStyleRules> = {
  styleOverrides: {
    root: {
      width: 64,
      height: 32,
      padding: 0,
      "& .MuiSwitch-switchBase": {
        padding: 0,
        margin: 4,
        transitionDuration: "200ms",
        "&.Mui-checked": {
          transform: "translateX(32px)",
          color: brandPalette.common.white,
          "& + .MuiSwitch-track": {
            backgroundColor:
              theme.palette.mode === "dark"
                ? brandPalette.primary.light
                : brandPalette.primary.light,
            opacity: 1,
            border: 0,
          },
          "&.Mui-disabled + .MuiSwitch-track": {
            opacity: 0.5,
          },
        },
        "&.Mui-disabled .MuiSwitch-thumb": {
          color:
            theme.palette.mode === "light"
              ? alpha(brandPalette.common.white, 0.3)
              : alpha(brandPalette.common.white, 0.3),
        },
        "&.Mui-disabled + .MuiSwitch-track": {
          opacity: theme.palette.mode === "light" ? 0.5 : 0.5,
        },
      },
      "& .MuiSwitch-thumb": {
        boxSizing: "border-box",
        width: 24,
        height: 24,
        color: brandPalette.common.white,
      },
      "& .MuiSwitch-track": {
        borderRadius: 16,
        backgroundColor:
          theme.palette.mode === "light"
            ? alpha(brandPalette.common.white, 0.1)
            : alpha(brandPalette.common.white, 0.1),
        opacity: 1,
        transition: theme.transitions.create(["background-color"], {
          duration: 300,
        }),
      },
    },
  },
};

const chipOverrides: Partial<OverridesStyleRules> = {
  styleOverrides: {
    root: {
      padding: "0.4rem",
      borderRadius: "0.5rem",
      fontFamily: '"Be Vietnam Pro"',
      fontSize: "1rem",
      backgroundColor: "rgba(255, 133, 0, 0.1)",
      color: brandPalette.warning.main,
      "& .MuiChip-icon": {
        height: "1.2rem",
      },
      "&.MuiChip-colorInfo": {
        backgroundColor: "rgba(2, 134, 255, 0.1)",
        color: brandPalette.info.main,
      },
      "&.MuiChip-colorSuccess": {
        backgroundColor: "rgba(0, 198, 138, 0.1)",
        color: brandPalette.success.main,
      },
      "&.MuiChip-colorError": {
        backgroundColor: "rgba(225, 0, 54, 0.1)",
        color: brandPalette.error.main,
      },
      "&.MuiChip-colorWarning": {
        backgroundColor: "rgba(197, 154, 4, 0.3)",
        color: brandPalette.warning.main,
      },
    },
  },
};

export const getDesignTokens = (mode: PaletteMode): ThemeOptions => ({
  ...theme,
  ...customThemeOptions,
  palette: {
    mode,
    ...(mode === "dark" && {
      text: {
        primary: brandPalette.common.white,
        secondary: brandPalette.common.darkWhite,
        disabled: alpha(theme.palette.common.white, 0.3),
      },
      ...brandPalette,
      actions: {
        active: grey[700],
        hover: grey[600],
        selected: grey[700],
        disabled: grey[500],
        disabledBackground: grey[800],
      },
      background: {
        default: "#000000",
        paper: alpha("#FF8500", 0.02),
      },
      divider: "#150B00",
    }),
    ...(mode === "light" && {
      text: {
        primary: grey[900],
        secondary: grey[600],
        disabled: grey[500],
      },
      actions: {
        active: grey[700],
        hover: grey[600],
        selected: grey[700],
        disabled: grey[500],
        disabledBackground: grey[800],
      },
      background: {
        default: grey[50],
        paper: grey[50],
      },
      divider: grey[200],
    }),
  },
  typography: brandTypography,
  components: {
    MuiCssBaseline: {
      styleOverrides: `
        @font-face {
          font-family: "TentangNanti";
          font-style: normal;
          font-display: swap;
          font-weight: 400;
          src: local('TentangNanti'), local('TentangNanti'), url("/static/TentangNanti.woff") format('woff');
        };
        body {
          background: ${brandPalette.primary.dark};
        }
      `,
    },
    MuiButton: buttonOverrides,
    MuiSwitch: switchOverrides,
    MuiAppBar: {
      styleOverrides: {
        root: {
          backgroundImage:
            "linear-gradient(to bottom, #0c0600 50%, rgba(0, 0, 0, 0.15) 134%)",
        },
      },
    },
    MuiToolbar: {
      styleOverrides: {
        root: {
          padding: "0.5rem 3rem",
        },
      },
    },
    MuiList: {
      styleOverrides: {
        root: {
          padding: theme.spacing(0, 3),
        },
      },
    },
    MuiListItem: {
      styleOverrides: {
        root: {
          padding: "1rem",
          paddingLeft: "1.5rem",
          paddingRight: "1rem",
          height: "5rem",
          borderRadius: theme.spacing(1.5),
          "&.Mui-selected": {
            backgroundColor: alpha(brandPalette.primary.main, 0.1),
            filter: "drop-shadow(0px 4px 4px rgba(0, 0, 0, 0.25))",
            "& .MuiListItemText-primary": {
              color: theme.palette.common.white,
            },
          },
        },
      },
    },
    MuiListItemIcon: {
      styleOverrides: {
        root: {
          minWidth: "2.625rem",
        },
      },
    },
    MuiListItemText: {
      styleOverrides: {
        primary: {
          fontSize: "1.125rem",
          color: alpha(theme.palette.common.white, 0.6),
        },
      },
    },
    MuiDialog: {
      styleOverrides: {
        paper: {
          backgroundColor: "rgba(7, 1, 5, 0.8)",
          backgroundImage:
            "linear-gradient(180deg, rgba(12, 6, 0, 0.8) 0%, rgba(21, 11, 0, 0.8) 82.99%)",
          boxShadow: "none",
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          padding: theme.spacing(2),
          backgroundImage: "none",
          /* width */
          "&::-webkit-scrollbar": {
            width: 16,
          },

          /* Track */
          "&::-webkit-scrollbar-track": {
            background: alpha(
              brandPalette.common.white,
              customThemeOptions.custom.opacity.light
            ),
            borderRadius: "0 12px 12px 0",
          },

          /* Handle */
          "&::-webkit-scrollbar-thumb": {
            background: brandPalette.common.white,
            border: "7px solid rgb(45 37 27)",
            borderRadius: 12,
          },

          /* Handle on hover */
          "&::-webkit-scrollbar-thumb:hover": {
            background: brandPalette.common.white,
          },
        },
        outlined: {
          border: `1px solid ${brandPalette.primary.main}`,
          "& img": {
            mixBlendMode: "luminosity",
          },
        },
      },
    },
    MuiTabs: {
      styleOverrides: {
        root: {
          "& .MuiTabs-indicator": {
            backgroundColor: brandPalette.primary.light,
          },
        },
      },
    },
    MuiTab: {
      styleOverrides: {
        root: {
          textTransform: "none",
          color: brandPalette.common.white,
          padding: theme.spacing(2.25, 3),
          lineHeight: 1.45,
          gap: theme.spacing(2),
          [theme.breakpoints.down("sm")]: {
            gap: theme.spacing(1.5),
            padding: theme.spacing(1.875, 0),
          },
          borderBottom: `3px solid ${alpha(
            brandPalette.primary.main,
            customThemeOptions.custom.opacity.light
          )}`,
          "&:hover": {
            background: alpha(
              brandPalette.primary.main,
              customThemeOptions.custom.opacity.lighter
            ),
            borderBottom: `3px solid ${alpha(
              brandPalette.primary.main,
              customThemeOptions.custom.opacity.main
            )}`,
          },
          "&.Mui-selected": {
            color: brandPalette.common.white,
            borderBottom: `2px solid ${brandPalette.primary.light}`,
          },
          "&.Mui-disabled": {
            color: brandPalette.common.white,
            borderBottom: `3px solid ${brandPalette.secondary.light}`,
            opacity: customThemeOptions.custom.opacity.main,
          },
          "&.MuiTab-labelIcon": {
            minHeight: "auto",
          },
        },
      },
    },
    MuiLinearProgress: {
      styleOverrides: {
        root: {
          backgroundColor: "transparent",
          height: "0.25rem",
          "&::before": {
            background: "transparent",
            borderTop: `1px dashed ${brandPalette.common.darkWhite}`,
            position: "absolute",
            top: 2,
            left: 0,
            width: "100%",
            content: '""',
          },
        },
        barColorPrimary: {
          backgroundColor: "#9e9a98",
        },
      },
    },
    MuiChip: chipOverrides,
    MuiLink: {
      styleOverrides: {
        root: {
          color: brandPalette.primary.light,
          "&:hover": {
            color: alpha(
              brandPalette.primary.light,
              customThemeOptions.custom.opacity.darker
            ),
          },
        },
      },
    },
    MuiTypography: {
      styleOverrides: {
        button: {
          textTransform: "none",
        },
      },
    },
    MuiInputLabel: {
      styleOverrides: {
        root: {
          position: "relative",
          top: "-2rem",
          left: "-1rem",
        },
      },
    },
    MuiOutlinedInput: {
      defaultProps: {
        notched: false,
      },
      styleOverrides: {
        root: {
          "&:hover": {
            "& .MuiOutlinedInput-notchedOutline": {
              borderColor: `${alpha(
                theme.palette.common.white,
                customThemeOptions.custom.opacity.main
              )}`,
            },
          },
          "&.Mui-error": {
            color: brandPalette.error.main,
            "&:hover": {
              "& .MuiOutlinedInput-notchedOutline": {
                borderColor: `${brandPalette.error.main}`,
              },
            },
          },
          "&.Mui-focused": {
            color: brandPalette.common.white,
            "& .MuiOutlinedInput-notchedOutline": {
              borderColor: `${brandPalette.primary.light}`,
            },
          },
          "&.Mui-disabled": {
            background: alpha(theme.palette.common.white, 0.02),
            "& .MuiOutlinedInput-notchedOutline": {
              borderColor: alpha(theme.palette.common.white, 0.1),
            },
          },
          "& .MuiSelect-icon": {
            right: theme.spacing(3),
            transform: "none",
            [theme.breakpoints.down("sm")]: {
              right: theme.spacing(2),
            },
          },
          "&.MuiInputBase-adornedStart": {
            paddingLeft: theme.spacing(2),
            [theme.breakpoints.down("sm")]: {
              paddingLeft: theme.spacing(1),
            },
          },
          "& .MuiSelect-select": {
            "&.MuiOutlinedInput-input.MuiInputBase-input": {
              paddingRight: theme.spacing(6),
            },
            "& .MuiBox-root": {
              overflow: "hidden",
            },
          },
        },
        input: {
          fontSize: 18,
          lineHeight: 1.55,
          height: "auto",
          padding: theme.spacing(2.25, 3),
          [theme.breakpoints.down("sm")]: {
            fontSize: 16,
            padding: theme.spacing(1.875, 2),
          },
        },
        inputSizeSmall: {
          padding: theme.spacing(1.75, 0),
        },
        notchedOutline: {
          borderColor: `${alpha(
            theme.palette.common.white,
            customThemeOptions.custom.opacity.main
          )}`,
        },
      },
    },
    MuiInputAdornment: {
      styleOverrides: {
        root: {
          flexShrink: 0,
        },
        positionStart: {
          marginRight: theme.spacing(4),
        },
      },
    },
    MuiListSubheader: {
      styleOverrides: {
        root: {
          background: brandPalette.background.paper,
          paddingTop: theme.spacing(1),
          paddingBottom: theme.spacing(1),
        },
      },
    },
    MuiMenu: {
      styleOverrides: {
        list: {
          minWidth: theme.spacing(22),
        },
      },
    },
    MuiMenuItem: {
      styleOverrides: {
        gutters: {
          padding: theme.spacing(2.25, 3),
          "&:hover": {
            background: alpha(
              brandPalette.primary.main,
              customThemeOptions.custom.opacity.lighter
            ),
          },
          [theme.breakpoints.down("sm")]: {
            padding: theme.spacing(1.875, 2),
          },
        },
      },
    },
    MuiTable: {
      styleOverrides: {
        root: {
          borderCollapse: "separate",
          borderSpacing: theme.spacing(0, 2),
          marginTop: "0",
        },
      },
    },
    MuiTableContainer: {
      styleOverrides: {
        root: {
          // padding: theme.spacing(1),
        },
      },
    },
    MuiTableCell: {
      styleOverrides: {
        head: {
          color: alpha(
            theme.palette.common.white,
            customThemeOptions.custom.opacity.darker
          ),
          border: "none",
          padding: 0,
          paddingLeft: theme.spacing(4),
        },
        body: {
          border: `1px solid ${alpha(
            theme.palette.common.white,
            customThemeOptions.custom.opacity.main
          )}`,
          borderStyle: "solid none",
          padding: theme.spacing(4),
          "&:first-of-type": {
            borderLeftStyle: "solid",
            borderTopLeftRadius: theme.spacing(1.5),
            borderBottomLeftRadius: theme.spacing(1.5),
          },
          "&:last-of-type": {
            borderRightStyle: "solid",
            borderTopRightRadius: theme.spacing(1.5),
            borderBottomRightRadius: theme.spacing(1.5),
          },
        },
      },
    },
    MuiTableBody: {},
    MuiAccordion: {
      styleOverrides: {
        root: {
          padding: 0,
          "&.Mui-expanded": {
            margin: 0,
          },
          "&.MuiPaper-root": {
            backgroundColor: "transparent",
          },
        },
      },
    },
    MuiAccordionSummary: {
      styleOverrides: {
        root: {
          paddingLeft: 0,
          margin: 0,
          minHeight: "none",
          "&.Mui-expanded": {
            margin: 0,
            minHeight: "none",
          },
          ".MuiAccordionSummary-content": {
            margin: 0,
          },
        },
      },
    },
    MuiAccordionDetails: {
      styleOverrides: {
        root: { padding: 0 },
      },
    },
    MuiAlert: {
      styleOverrides: {
        root: {
          borderRadius: "0.75rem",
          "& .MuiAlert-message": {
            display: "flex",
            alignItems: "center",
          },
          "& .MuiAlert-action": {
            display: "flex",
            alignItems: "center",
          },
        },
        icon: {
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        },
        filledSuccess: {
          background: alpha("#00c68a", 0.1),
          backdropFilter: "blur(16px)",
        },
        filledError: {
          background: alpha(brandPalette.error.main, 0.1),
          backdropFilter: "blur(16px)",
        },
        filledInfo: {
          background: alpha(brandPalette.info.main, 0.1),
          backdropFilter: "blur(16px)",
        },
        filledWarning: {
          background: alpha(brandPalette.warning.main, 0.1),
          backdropFilter: "blur(16px)",
        },
      },
    },
    MuiTooltip: {
      styleOverrides: {
        popper: {
          background: "transparent",
        },
        tooltip: {
          background: brandPalette.common.white,
          border: `1px solid ${brandPalette.common.white}`,
          borderRadius: "0.5rem",
          fontFamily: "Be Vietnam Pro",
          color: brandPalette.primary.dark,
          fontSize: "1.125rem",
          [theme.breakpoints.up("md")]: {
            padding: "1.5rem",
          },
          [theme.breakpoints.down("sm")]: {
            padding: "0.5rem",
          },
        },
        arrow: {
          color: brandPalette.common.white,
        },
      },
    },
  },
  shape: {
    borderRadius: 12,
  },
});
