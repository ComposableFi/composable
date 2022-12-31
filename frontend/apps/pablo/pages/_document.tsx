import * as React from "react";
import Document, { Head, Html, Main, NextScript } from "next/document";
import createEmotionServer from "@emotion/server/create-instance";
import createEmotionCache from "@/styles/createEmotionCache";
import { createTheme } from "@/styles/theme";
import config from "@/constants/config";

export default class MyDocument extends Document {
  render() {
    const theme = createTheme("dark");
    return (
      <Html lang="en">
        <Head>
          {/* PWA Related */}
          <meta name="application-name" content={config.applicationName} />
          <meta name="apple-mobile-web-app-capable" content="yes" />
          <meta
            name="apple-mobile-web-app-status-bar-style"
            content="default"
          />
          <meta
            name="apple-mobile-web-app-title"
            content={config.applicationName}
          />
          <meta name="description" content={config.appDescription} />
          <meta name="format-detection" content="telephone=no" />
          <meta name="mobile-web-app-capable" content="yes" />
          <meta
            name="msapplication-config"
            content="/icons/browserconfig.xml"
          />
          <meta
            name="msapplication-TileColor"
            content={theme.palette.primary.main}
          />
          <meta name="msapplication-tap-highlight" content="no" />

          <link
            rel="apple-touch-icon"
            sizes="180x180"
            href="/pwa/apple-touch-icon.png"
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
            href="/pwa/favicon-16x16.png"
          />
          <link rel="manifest" href="/manifest.json" />
          <link rel="shortcut icon" href="/favicon.ico" />
          <meta name="twitter:card" content="summary_large_image" />
          <meta name="twitter:url" content={config.appUrl} />
          <meta name="twitter:title" content={config.applicationName} />
          <meta name="twitter:description" content={config.appDescription} />
          <meta
            name="twitter:image"
            content="https://raw.githubusercontent.com/fl-y/random-assets/main/assets/pablo-banner.jpg"
          />
          <meta name="twitter:creator" content={config.twitterHandle} />
          <meta property="og:type" content="website" />
          <meta property="og:title" content={config.applicationName} />
          <meta property="og:description" content={config.appDescription} />
          <meta property="og:site_name" content={config.applicationName} />
          <meta property="og:url" content={config.appUrl} />
          <meta
            name="og:image"
            content="https://raw.githubusercontent.com/fl-y/random-assets/main/assets/pablo-banner.jpg"
          />

          {/* Apple Splash screen */}
          <link
            rel="apple-touch-startup-image"
            href="/pwa/2048x2732.png"
            sizes="2048x2732"
          />
          <link
            rel="apple-touch-startup-image"
            href="/pwa/1668x2048.png"
            sizes="1668x2224"
          />
          <link
            rel="apple-touch-startup-image"
            href="/pwa/1536x2048.png"
            sizes="1536x2048"
          />
          <link
            rel="apple-touch-startup-image"
            href="/pwa/1125x2436.png"
            sizes="1125x2436"
          />
          <link
            rel="apple-touch-startup-image"
            href="/pwa/1242x2208.png"
            sizes="1242x2208"
          />
          <link
            rel="apple-touch-startup-image"
            href="/pwa/750x1334.png"
            sizes="750x1334"
          />
          <link
            rel="apple-touch-startup-image"
            href="/pwa/640x1136.png"
            sizes="640x1136"
          />

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
