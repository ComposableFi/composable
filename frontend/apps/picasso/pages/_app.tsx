import "defi-interfaces/types-lookup.d";
import "defi-interfaces/augment-api-tx";
import "defi-interfaces/augment-api-rpc";
import "defi-interfaces/augment-types";

import * as React from "react";
import { hotjar } from "react-hotjar";
import Head from "next/head";
import { AppProps } from "next/app";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import { CacheProvider, EmotionCache } from "@emotion/react";
import createEmotionCache from "@/styles/createEmotionCache";
import { getDesignTokens } from "@/styles/theme";
import { ColorModeContext } from "@/contexts/ColorMode";
import ParachainContextProvider from "@/defi/polkadot/context/ParachainContext";
import SubstrateBalancesUpdater from "@/stores/defi/polkadot/balances/PolkadotBalancesUpdater";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import CrowdloanRewardsUpdater from "@/stores/defi/polkadot/crowdloanRewards/CrowdloanRewardsUpdater";
import { PalletsContextProvider } from "@/defi/polkadot/context/PalletsContext";
import { BlockchainProvider } from "bi-lib";
import { NETWORKS } from "@/defi/Networks";
import { SnackbarProvider } from "notistack";
import { ThemeResponsiveSnackbar } from "@/components/Molecules/Snackbar";
import { ExecutorProvider } from "substrate-react";
import { ApolloProvider } from "@apollo/client";
import { client as apolloClient } from "@/apollo/apolloGraphql";
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
      <BlockchainProvider
        blockchainInfo={Object.entries(NETWORKS).map(([netId, net]) => {
          return {
            chainId: +netId,
            rpcUrl: net.rpcUrl,
          };
        })}
      >
        <ColorModeContext.Provider value={colorMode}>
          <ThemeProvider theme={theme}>
            {/* CssBaseline kickstart an elegant, consistent, and simple baseline to build upon. */}
            <CssBaseline />
            <ParachainContextProvider
              appName="Picasso UI"
              supportedChains={Object.values(SUBSTRATE_NETWORKS)}
            >
              <PalletsContextProvider>
                <ApolloProvider client={apolloClient}>
                  <SubstrateBalancesUpdater
                    substrateChains={Object.values(SUBSTRATE_NETWORKS)}
                  />
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
                    </ExecutorProvider>
                  </SnackbarProvider>
                </ApolloProvider>
              </PalletsContextProvider>
            </ParachainContextProvider>
          </ThemeProvider>
        </ColorModeContext.Provider>
      </BlockchainProvider>
    </CacheProvider>
  );
}
