import Default from "@/components/Templates/Default";
import Script from "next/script";
import * as React from "react";
import { useState } from "react";
import { CircularProgress, Grid, Stack } from "@mui/material";

export default function Feedback() {
  const [isLoading, setIsLoading] = useState(true);

  return (
    <Default>
      <Grid container mt={10} maxWidth={1032} mx="auto">
        <Grid item xs={12}>
          {isLoading && (
            <Stack
              alignItems="center"
              justifyContent="center"
              width="100%"
              height="100%"
            >
              <CircularProgress />
            </Stack>
          )}
          <div data-upvoty=""></div>
        </Grid>
      </Grid>
      <Script
        id="upvoty-initializer"
        strategy="afterInteractive"
        onLoad={() => {
          // @ts-ignore
          window?.upvoty.init("render", {
            boardHash:
              "09abccedb1fb8876cced2a2d4c96f08823df77447bb7daafa10d24dbb2661464",
            ssoToken: null,
            baseUrl: "composable-fi.upvoty.com",
          });
          setIsLoading(false);
        }}
        src={"https://composable-fi.upvoty.com/javascript/upvoty.embed.js"}
      />
    </Default>
  );
}
