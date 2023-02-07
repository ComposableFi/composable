import "defi-interfaces";

import * as React from "react";
import { hotjar } from "react-hotjar";
import { AppProps } from "next/app";
import { ThemeProvider } from "@mui/material/styles";
import { CacheProvider, EmotionCache } from "@emotion/react";
import { createTheme } from "@/styles/theme";
import { ColorModeContext } from "@/contexts/ColorMode";
import { APP_NAME } from "@/defi/constants";
import { DotSamaContextProvider, ExecutorProvider } from "substrate-react";
import "./global.css";
import Head from "next/head";
import CssBaseline from "@mui/material/CssBaseline";
import createEmotionCache from "@/styles/createEmotionCache";

import BaseUpdater from "@/updaters/BaseUpdater";
import * as definitions from "defi-interfaces/definitions";
import { SnackbarProvider } from "notistack";
import { ThemeResponsiveSnackbar } from "@/components";
import { getEnvironment } from "shared/endpoints";

const rpc = Object.keys(definitions)
  .filter((k) => {
    if (!(definitions as any)[k].rpc) {
      return false;
    } else {
      return Object.keys((definitions as any)[k].rpc).length > 0;
    }
  })
  .reduce(
    (accumulator, key) => ({
      ...accumulator,
      [key]: (definitions as any)[key].rpc,
    }),
    {}
  );
const types = Object.keys(definitions)
  .filter((key) => Object.keys((definitions as any)[key].types).length > 0)
  .reduce(
    (accumulator, key) => ({
      ...accumulator,
      ...(definitions as any)[key].types,
    }),
    {}
  );

// Client-side cache, shared for the whole session of the user in the browser.
const clientSideEmotionCache = createEmotionCache();

interface MyAppProps extends AppProps {
  emotionCache?: EmotionCache;
}

const initializeContentful = async () => {
  // const {fields} = await getEntry({
  //   id: "[ENTER_ENTRY_ID_TO_FETCH_JSON_FROM_CONTENTFUL]"
  // });
  // console.log(fields);
};

const initializeHotjar = () => {
  if (
    process.env.NODE_ENV === "production" &&
    process.env.NEXT_PUBLIC_HOTJAR_ID &&
    process.env.NEXT_PUBLIC_HOTJAR_SITE_ID
  ) {
    hotjar.initialize(
      parseInt(process.env.NEXT_PUBLIC_HOTJAR_ID),
      parseInt(process.env.NEXT_PUBLIC_HOTJAR_SITE_ID)
    );
  }
};

export default function MyApp(props: MyAppProps) {
  const { Component, emotionCache = clientSideEmotionCache, pageProps } = props;
  const [mode, setMode] = React.useState<"light" | "dark">("dark");
  const colorMode = React.useMemo(
    () => ({
      toggleColorMode: () => {
        setMode((prevMode) => (prevMode === "light" ? "dark" : "light"));
      },
    }),
    []
  );

  const theme = React.useMemo(() => createTheme(mode), [mode]);

  React.useEffect(initializeHotjar, []);
  React.useEffect(() => {
    initializeContentful();
  }, []);

  return (
    <CacheProvider value={emotionCache}>
      <Head>
        <meta name="viewport" content="initial-scale=1, width=device-width" />
      </Head>
      <ColorModeContext.Provider value={colorMode}>
        <ThemeProvider theme={theme}>
          {/* CssBaseline kickstart an elegant, consistent, and simple baseline to build upon. */}
          <CssBaseline />
          <SnackbarProvider
            Components={{
              info: ThemeResponsiveSnackbar,
              success: ThemeResponsiveSnackbar,
              error: ThemeResponsiveSnackbar,
              warning: ThemeResponsiveSnackbar,
            }}
            autoHideDuration={null}
            maxSnack={4}
            disableWindowBlurListener={true}
            anchorOrigin={{
              vertical: "bottom",
              horizontal: "center",
            }}
          >
            <DotSamaContextProvider
              supportedParachains={[
                {
                  chainId: "picasso",
                  rpcUrl: getEnvironment("picasso"),
                  rpc,
                  types,
                },
              ]}
              appName={APP_NAME}
            >
              <BaseUpdater />
              <ExecutorProvider>
                <Component {...pageProps} />
              </ExecutorProvider>
            </DotSamaContextProvider>
          </SnackbarProvider>
        </ThemeProvider>
      </ColorModeContext.Provider>
    </CacheProvider>
  );
}
