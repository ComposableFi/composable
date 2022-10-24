import { ColorModeContext } from "picasso/contexts/ColorMode";
import { getDesignTokens } from "picasso/styles/theme";
import { ThemeProvider as EmotionThemeProvider } from "@emotion/react";
import { Box, createTheme, CssBaseline, ThemeProvider } from "@mui/material";
import { FC, useMemo, useState } from "react";
import { client as apolloClient } from "picasso/apollo/apolloGraphql";
import { ApolloProvider } from "@apollo/client";

export const MUIDecorator: FC = ({ children }) => {
  const [mode, setMode] = useState<"light" | "dark">("dark");
  const colorMode = useMemo(
    () => ({
      toggleColorMode: () => {
        setMode((prevMode) => (prevMode === "light" ? "dark" : "light"));
      }
    }),
    []
  );


  return (
    <Box
      sx={{
        position: "relative"
      }}
    >
      <ApolloProvider client={apolloClient}>
        <ColorModeContext.Provider value={colorMode}>
          <EmotionThemeProvider theme={createTheme(getDesignTokens(mode))}>
            <ThemeProvider theme={createTheme(getDesignTokens(mode))}>
              {/* CssBaseline kickstart an elegant, consistent, and simple baseline to build upon. */}
              <CssBaseline />
              {children}
            </ThemeProvider>
          </EmotionThemeProvider>
        </ColorModeContext.Provider>
      </ApolloProvider>
    </Box>
  );
};
