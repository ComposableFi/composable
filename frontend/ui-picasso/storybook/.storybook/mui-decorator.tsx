import { ColorModeContext } from "@/contexts/ColorMode";
import { getDesignTokens } from "@/styles/theme";
import { ThemeProvider as EmotionThemeProvider } from "@emotion/react";
import { Box, createTheme, CssBaseline, ThemeProvider } from "@mui/material";
import React from "react";

export const MUIDecorator: React.FC = ({ children }) => {
  const [mode, setMode] = React.useState<"light" | "dark">("dark");
  const colorMode = React.useMemo(
    () => ({
      toggleColorMode: () => {
        setMode((prevMode) => (prevMode === "light" ? "dark" : "light"));
      },
    }),
    []
  );
  return (
    <Box
      sx={{
        position: "relative",
      }}
    >
      <ColorModeContext.Provider value={colorMode}>
        <EmotionThemeProvider theme={createTheme(getDesignTokens(mode))}>
          <ThemeProvider theme={createTheme(getDesignTokens(mode))}>
            {/* CssBaseline kickstart an elegant, consistent, and simple baseline to build upon. */}
            <CssBaseline />
            {children}
          </ThemeProvider>
        </EmotionThemeProvider>
      </ColorModeContext.Provider>
    </Box>
  );
};
