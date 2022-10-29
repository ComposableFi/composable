  
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