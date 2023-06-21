import { PaletteMode } from "@mui/material";
import { createTheme } from "@mui/material/styles";
import { Shadows } from "@mui/material/styles/shadows";
import { paletteOptions } from "./options";

declare module "@mui/material/styles" {
  interface Theme {
    custom: {
      opacity: {
        lightest: number;
        lighter: number;
        light: number;
        main: number;
        dark: number;
        darker: number;
        darkest: number;
      };
      lineHeight: {
        larger: string;
        large: string;
        medium: string;
        small: string;
        smaller: string;
      };
      fontFamily: {
        primary: string;
        secondary: string;
        other: string;
      };
      drawerWidth: {
        desktop: number;
        tablet: number;
        mobile: number;
      };
    };
  }

  interface ThemeOptions {
    custom: {
      opacity?: {
        lightest?: number;
        lighter?: number;
        light?: number;
        main?: number;
        dark?: number;
        darker?: number;
        darkest?: number;
      };
      lineHeight?: {
        larger?: string;
        large?: string;
        medium?: string;
        small?: string;
        smaller?: string;
      };
      fontFamily?: {
        primary?: string;
        secondary?: string;
        other?: string;
      };
      drawerWidth?: {
        desktop?: number;
        tablet?: number;
        mobile?: number;
      };
    };
    shape: {
      borderRadius: string | number | string[] | undefined;
    };
  }

  interface Palette {
    featured: {
      main: string;
      dark: string;
      light: string;
    };
    gradient: {
      background: string;
      backdrop: string;
      primary: string;
      secondary: string;
      other: string;
      grapeBackground: string;
    };
  }

  interface TypeBackground {
    default: string;
    paper: string;
    transparentCharcoal: string;
  }

  interface PaletteOptions {
    featured: {
      main?: string;
      dark?: string;
      light?: string;
    };
    gradient?: {
      background?: string;
      backdrop?: string;
      primary?: string;
      secondary?: string;
      other?: string;
      grapeBackground?: string;
    };
  }
}

declare module "@mui/material/Typography" {
  interface TypographyPropsVariantOverrides {
    inputLabel: true;
  }
}

// create basic theme with basic design options
// @ts-ignore
export const createBaseTheme = (mode: PaletteMode) =>
  createTheme({
    breakpoints: {
      values: {
        xs: 0,
        sm: 560,
        md: 900,
        lg: 1080,
        xl: 1536,
      },
    },
    palette: {
      mode,
      ...paletteOptions[mode],
    },
    spacing: 8,
    mixins: {
      toolbar: {},
    },
    shape: {
      borderRadius: Array(4).fill("16px"),
    },
    zIndex: {
      drawer: 1200,
    },
    shadows: Array(25).fill("none") as Shadows,
    custom: {
      opacity: {
        lightest: 0.02,
        lighter: 0.05,
        light: 0.1,
        main: 0.3,
        dark: 0.5,
        darker: 0.6,
        darkest: 0.8,
      },
      lineHeight: {
        smaller: "110%",
        small: "120%",
        medium: "130%",
        large: "140%",
        larger: "155%",
      },
      fontFamily: {
        primary: "CodeNext",
        secondary: "NovaDeco",
        other: "CodeNext",
      },
      drawerWidth: {
        desktop: 320,
        tablet: 240,
        mobile: 240,
      },
    },
  });
