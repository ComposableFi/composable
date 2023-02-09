import Default from "@/components/Templates/Default";
import Script from "next/script";
import * as React from "react";
import { Grid } from "@mui/material";

export default function Feedback() {
  return (
    <Default>
      <Grid container mt={9} maxWidth={1032} mx="auto">
        <Grid item xs={12}>
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
        }}
        src={"https://composable-fi.upvoty.com/javascript/upvoty.embed.js"}
      />
    </Default>
  );
}
