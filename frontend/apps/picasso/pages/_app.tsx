import { client as apolloClient } from "@/apollo/apolloGraphql";
import { ThemeResponsiveSnackbar } from "@/components/Molecules/Snackbar";
import { ColorModeContext } from "@/contexts/ColorMode";
import { NETWORKS } from "@/defi/Networks";
import { APP_NAME } from "@/defi/polkadot/constants";
import SubstrateBalancesUpdater from "@/stores/defi/polkadot/balances/PolkadotBalancesUpdater";
import CrowdloanRewardsUpdater from "@/stores/defi/polkadot/crowdloanRewards/CrowdloanRewardsUpdater";
import createEmotionCache from "@/styles/createEmotionCache";
import { getDesignTokens } from "@/styles/theme";
import { ApolloProvider } from "@apollo/client";
import { CacheProvider, EmotionCache } from "@emotion/react";
import CssBaseline from "@mui/material/CssBaseline";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { BlockchainProvider } from "bi-lib";
import "defi-interfaces";
import * as definitions from "defi-interfaces/definitions";
import { AppProps } from "next/app";
import Head from "next/head";
import { SnackbarProvider } from "notistack";

import * as React from "react";
import { hotjar } from "react-hotjar";
import { getEnvironment } from "shared/endpoints";
import { DotSamaContextProvider, ExecutorProvider } from "substrate-react";
import { Analytics } from "@/components/Organisms/Analytics";
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

  const theme = React.useMemo(() => createTheme(getDesignTokens(mode)), [mode]);

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
          <CssBaseline />
          <DotSamaContextProvider
            supportedRelaychains={[
              {
                chainId: "kusama",
                rpcUrl: getEnvironment("kusama"),
                rpc: {},
                types: {},
              },
            ]}
            supportedParachains={[
              {
                chainId: "statemine",
                rpcUrl: getEnvironment("statemine"),
                rpc: {},
                types: {},
              },
              {
                chainId: "picasso",
                rpcUrl: getEnvironment("picasso"),
                rpc,
                types,
              },
            ]}
            appName={APP_NAME}
          >
            <BlockchainProvider
              blockchainInfo={Object.entries(NETWORKS).map(([netId, net]) => {
                return {
                  chainId: +netId,
                  rpcUrl: net.rpcUrl,
                };
              })}
            >
              <ApolloProvider client={apolloClient}>
                <SubstrateBalancesUpdater />
                <CrowdloanRewardsUpdater />
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
                  <ExecutorProvider>
                    <Component {...pageProps} />
                    <Analytics />
                  </ExecutorProvider>
                </SnackbarProvider>
              </ApolloProvider>
            </BlockchainProvider>
          </DotSamaContextProvider>
        </ThemeProvider>
      </ColorModeContext.Provider>
    </CacheProvider>
  );
}
