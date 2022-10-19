  
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