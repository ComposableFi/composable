import { ColorModeContext } from "pablo/contexts/ColorMode";
import { createTheme } from "pablo/styles/theme";
import { ThemeProvider as EmotionThemeProvider } from "@emotion/react";
import { Box, CssBaseline, ThemeProvider } from "@mui/material";
import { Provider } from "react-redux";
import { useState, FC, useMemo } from "react";
import { store } from "pablo/stores/root";


export const MUIDecorator: FC = ({ children }) => {
  const [mode, setMode] = useState<"light" | "dark">("dark");
  const colorMode = useMemo(
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
      <Provider store={store}>
        <ColorModeContext.Provider value={colorMode}>
          <EmotionThemeProvider theme={createTheme(mode)}>
            <ThemeProvider theme={createTheme(mode)}>
              {/* CssBaseline kickstart an elegant, consistent, and simple baseline to build upon. */}
              <CssBaseline/>
              {children}
            </ThemeProvider>
          </EmotionThemeProvider>
        </ColorModeContext.Provider>
      </Provider>
    </Box>
  );
};
