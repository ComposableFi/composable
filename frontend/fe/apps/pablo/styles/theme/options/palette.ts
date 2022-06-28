import { alpha, PaletteOptions } from "@mui/material";

export const paletteOptions = {
  dark: {
    primary: {
      main: "#1731F7",
      light: "#0085FF",
      dark: "#01041B",
    },
    secondary: {
      main: "#1F2764",
      light: "#0B1C9A",
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
    common: {
      white: "#FFFFFF",
      black: "#000000",
    },
    featured: {
      main: "#33C500",
    },
    gradient: {
      background:
        "linear-gradient(124deg, #162cd3 -50%, #000528 29%, #230008 85%, #7c001d 170%)",
      backdrop:
        "linear-gradient(180deg, rgba(1, 6, 50, 0.8) -16.41%, rgba(4, 3, 26, 0.8) 82.99%)",
      primary:
        "linear-gradient(146deg, #fff -5%, rgba(255, 255, 255, 0) 136%), \
                linear-gradient(320deg, rgba(255, 255, 255, 0.1) 74%, rgba(255, 255, 255, 0.2) 22%)",
      secondary:
        "linear-gradient(138deg, rgba(255, 255, 255, 0.04) -3%, rgba(255, 255, 255, 0.02) 137%)",
      other:
        "linear-gradient(130deg, #fff -2%, rgba(255, 255, 255, 0) 130%), \
              linear-gradient(124deg, #fff 10%, rgba(255, 255, 255, 0) 78%)",
    },
  } as PaletteOptions,
  light: {
    common: {
      white: "#FFFFFF",
      black: "#000000",
    },
    featured: {
      main: "#33C500",
    },
    gradient: {
      primary:
        "linear-gradient(180deg, rgba(12, 6, 0, 0.8) 0%, rgba(21, 11, 0, 0.8) 82.99%)",
      secondary:
        "linear-gradient(180deg, rgba(12, 6, 0, 0) 63.64%, rgba(12, 6, 0, 0.8) 116.45%)",
    },
  } as PaletteOptions,
};
