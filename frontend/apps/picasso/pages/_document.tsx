import * as React from "react";
import Document, { Head, Html, Main, NextScript } from "next/document";
import createEmotionServer from "@emotion/server/create-instance";
import createEmotionCache from "@/styles/createEmotionCache";
import { getDesignTokens } from "@/styles/theme";
import { createTheme } from "@mui/material";
import config from "@/constants/config";

export default class MyDocument extends Document {
  render() {
    const theme = createTheme(getDesignTokens("light"));
    return (
      <Html lang="en">
        <Head>
          <meta
            name="application-name"
            content="Composable Finance Picasso Parachain"
          />
          <meta name="apple-mobile-web-app-capable" content="yes" />
          <meta
            name="apple-mobile-web-app-status-bar-style"
            content="default"
          />
          <meta
            name="apple-mobile-web-app-title"
            content="Composable Finance Picasso Parachain"
          />
          <meta
            name="description"
            content="The interoperable crosschain DeFi Hub built on the DotSama ecosystem"
          />
          <meta name="format-detection" content="telephone=no" />
          <meta name="mobile-web-app-capable" content="yes" />
          <meta
            name="msapplication-config"
            content="/icons/browserconfig.xml"
          />
          <meta name="msapplication-TileColor" content="#FF8500" />
          <meta name="msapplication-tap-highlight" content="no" />
          <link rel="preconnect" href="https://fonts.googleapis.com" />
          <link
            rel="preconnect"
            href="https://fonts.gstatic.com"
            crossOrigin="true"
          />
          <link
            rel="preload"
            href="/static/TentangNanti.woff"
            as="font"
            type="font/woff"
            crossOrigin="anonymous"
          />
          <link
            href="https://fonts.googleapis.com/css2?family=Be+Vietnam+Pro:wght@300;400;600&display=swap"
            rel="stylesheet"
          />
          <link rel="apple-touch-icon" href="/pwa/apple-touch-icon.png" />
          <link
            rel="apple-touch-icon"
            sizes="152x152"
            href="/pwa/touch-icon-ipad.png"
          />
          <link
            rel="apple-touch-icon"
            sizes="180x180"
            href="/pwa/apple-touch-icon.png"
          />
          <link
            rel="apple-touch-icon"
            sizes="167x167"
            href="/pwa/touch-icon-ipad.png"
          />

          <link
            rel="icon"
            type="image/png"
            sizes="32x32"
            href="/pwa/favicon-32x32.png"
          />
          <link
            rel="icon"
            type="image/png"
            sizes="16x16"
            href="/pwa/favicon-32x32.png"
          />
          <link rel="manifest" href="/manifest.json" />
          <link rel="shortcut icon" href="/pwa/favicon.ico" />
          <meta name="twitter:card" content="summary" />
          <meta name="twitter:url" content={config.twitterUrl} />
          <meta
            name="twitter:title"
            content="Composable Finance Picasso Parachain"
          />
          <meta
            name="twitter:description"
            content="The interoperable crosschain DeFi Hub built on the DotSama ecosystem"
          />
          <meta
            name="twitter:image"
            content="/pwa/android-chrome-512x512.png"
          />
          <meta name="twitter:creator" content="@picasso_network" />
          <meta property="og:type" content="website" />
          <meta
            property="og:title"
            content="The interoperable crosschain DeFi Hub built on the DotSama ecosystem"
          />
          <meta
            property="og:description"
            content="The interoperable crosschain DeFi Hub built on the DotSama ecosystem"
          />
          <meta property="og:site_name" content="Picasso Parachain" />
          <meta property="og:url" content="https://picasso.xyz" />
          <meta property="og:image" content="/pwa/android-chrome-512x512.png" />

          {/* PWA primary color */}
          <meta name="theme-color" content={theme.palette.primary.main} />
          <link rel="shortcut icon" href="/pwa/favicon.ico" />
          <link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"
          />
          {/* Inject MUI styles first to match with the prepend: true configuration. */}
          {(this.props as any).emotionStyleTags}
        </Head>
        <body>
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

// `getInitialProps` belongs to `_document` (instead of `_app`),
// it's compatible with static-site generation (SSG).
MyDocument.getInitialProps = async (ctx) => {
  // Resolution order
  //
  // On the server:
  // 1. app.getInitialProps
  // 2. page.getInitialProps
  // 3. document.getInitialProps
  // 4. app.render
  // 5. page.render
  // 6. document.render
  //
  // On the server with error:
  // 1. document.getInitialProps
  // 2. app.render
  // 3. page.render
  // 4. document.render
  //
  // On the client
  // 1. app.getInitialProps
  // 2. page.getInitialProps
  // 3. app.render
  // 4. page.render

  const originalRenderPage = ctx.renderPage;

  // You can consider sharing the same emotion cache between all the SSR requests to speed up performance.
  // However, be aware that it can have global side effects.
  const cache = createEmotionCache();
  const { extractCriticalToChunks } = createEmotionServer(cache);

  ctx.renderPage = () =>
    originalRenderPage({
      enhanceApp: (App: any) =>
        function EnhanceApp(props) {
          return <App emotionCache={cache} {...props} />;
        },
    });

  const initialProps = await Document.getInitialProps(ctx);
  // This is important. It prevents emotion to render invalid HTML.
  // See https://github.com/mui-org/material-ui/issues/26561#issuecomment-855286153
  const emotionStyles = extractCriticalToChunks(initialProps.html);
  const emotionStyleTags = emotionStyles.styles.map((style) => (
    <style
      data-emotion={`${style.key} ${style.ids.join(" ")}`}
      key={style.key}
      // eslint-disable-next-line react/no-danger
      dangerouslySetInnerHTML={{ __html: style.css }}
    />
  ));

  return {
    ...initialProps,
    emotionStyleTags,
  };
};
